//! FUSE
//!
//! - Filesystems in the Linux kernel - FUSE
//!   https://www.kernel.org/doc/html/latest/filesystems/fuse.html
//!
//! - To FUSE or Not to FUSE: Performance of User-Space File Systems
//!   https://www.usenix.org/system/files/conference/fast17/fast17-vangoor.pdf

use crate::config;
use crate::utils::staticify;
use fuser::{FileAttr, FileType, Filesystem, Request};
use std::time::Duration;

pub const BLOCK_SIZE: u32 = 4096;
pub const NAMELEN: u32 = 1024;
pub const KB: u64 = 1u64 << 10;
pub const MB: u64 = 1u64 << 20;
pub const GB: u64 = 1u64 << 30;
pub const TB: u64 = 1u64 << 40;
pub const PB: u64 = 1u64 << 50;

pub struct Fuse {}

impl Fuse {
    pub async fn start_fuse_mount() -> anyhow::Result<()> {
        if *config::S3D_FUSE_MOUNT != "true" {
            debug!("Fuse mount disabled");
            return Ok(());
        }
        info!("Fuse mount enabled");

        let fuse = staticify(Fuse {});

        let mountpoint = config::S3D_FUSE_MOUNT_DIR.as_str();
        if mountpoint.is_empty() {
            return Ok(());
        }
        let mut session = fuser::Session::new(
            fuse,
            mountpoint.as_ref(),
            &[
                fuser::MountOption::RW,
                // fuser::MountOption::RO,
                // fuser::MountOption::Sync,
                // fuser::MountOption::DirSync,
                // fuser::MountOption::Async,
                fuser::MountOption::AllowRoot,
                fuser::MountOption::AllowOther,
                fuser::MountOption::AutoUnmount,
                fuser::MountOption::DefaultPermissions,
                fuser::MountOption::NoDev,
                fuser::MountOption::NoSuid,
                fuser::MountOption::NoAtime,
                fuser::MountOption::CUSTOM("nobrowse".to_string()),
                fuser::MountOption::FSName("s3d".to_string()),
                fuser::MountOption::Subtype("s3d".to_string()),
            ],
        )?;
        // run the fuse event loop in a separate thread
        let res = tokio::task::spawn_blocking(move || session.run()).await;
        Ok(res??)
    }

    fn make_fuse_attr(&self, ino: u64, kind: FileType, size: u64) -> FileAttr {
        let now = std::time::SystemTime::now();
        FileAttr {
            ino, // inode's number
            size,
            blocks: (size + (BLOCK_SIZE as u64) - 1) / BLOCK_SIZE as u64,
            blksize: BLOCK_SIZE,
            kind,
            rdev: 0,                         // device type, for special file inode
            uid: unsafe { libc::geteuid() }, // user-id of owner
            gid: unsafe { libc::getegid() }, // group-id of owner
            perm: if kind == FileType::Directory {
                0o755
            } else {
                0o644
            }, // inode protection mode
            nlink: if kind == FileType::Directory {
                2 // parent + '.' + (subdirs * '..')
            } else {
                1
            }, // number of hard links to the file
            flags: 0,
            atime: now,
            mtime: now,
            ctime: now,
            crtime: now,
        }
    }
}

impl Filesystem for &Fuse {
    fn statfs(&mut self, _req: &Request<'_>, ino: u64, reply: fuser::ReplyStatfs) {
        trace!("FUSE::statfs() ino={}", ino);
        reply.statfs(
            42,            // total data blocks in file system
            1u64 << 38,    // free blocks in fs
            1u64 << 38,    // free blocks avail to non-superuser
            42,            // total file nodes in file system
            1_000_000_000, // free file nodes in fs
            BLOCK_SIZE,    // fundamental file system block size
            1024,          // namelen
            1024 * 1024,   // optimal transfer block size
        );
    }

    fn open(&mut self, _req: &Request<'_>, ino: u64, flags: i32, reply: fuser::ReplyOpen) {
        trace!("FUSE::open() ino={} flags={}", ino, flags);
        reply.opened(ino, flags as u32);
    }

    fn release(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        flags: i32,
        lock_owner: Option<u64>,
        flush: bool,
        reply: fuser::ReplyEmpty,
    ) {
        trace!(
            "FUSE::release() ino={} fh={} flags={} lock_owner={:?} flush={}",
            ino,
            fh,
            flags,
            lock_owner,
            flush
        );
        reply.ok();
    }

    fn opendir(&mut self, _req: &Request<'_>, ino: u64, flags: i32, reply: fuser::ReplyOpen) {
        trace!("FUSE::opendir() ino={} flags={}", ino, flags);
        if ino < 1000 {
            reply.opened(ino, flags as u32);
        } else {
            reply.error(libc::ENOTDIR);
        }
    }

    fn releasedir(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        flags: i32,
        reply: fuser::ReplyEmpty,
    ) {
        trace!("FUSE::releasedir() ino={} fh={} flags={}", ino, fh, flags);
        if ino < 1000 {
            reply.ok();
        } else {
            reply.error(libc::ENOTDIR);
        }
    }

    fn lookup(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        name: &std::ffi::OsStr,
        reply: fuser::ReplyEntry,
    ) {
        let name = name.to_str().unwrap();
        trace!("FUSE::lookup() ino={} name={}", ino, name);
        if ino >= 1000 {
            reply.error(libc::ENOTDIR);
            return;
        }
        if !name.starts_with("file") {
            reply.error(libc::ENOENT);
            return;
        }
        let i = name[4..].parse::<u64>().unwrap();
        let kind = if i < 1000 {
            FileType::Directory
        } else {
            FileType::RegularFile
        };
        let attr = self.make_fuse_attr(i, kind, 10);
        let ttl = Duration::from_secs(60);
        reply.entry(&ttl, &attr, 0);
    }

    fn readdir(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        mut reply: fuser::ReplyDirectory,
    ) {
        trace!("FUSE::readdir() ino={} fh={} offset={}", ino, fh, offset);
        if ino >= 1000 {
            reply.error(libc::ENOTDIR);
            return;
        }
        for i in 1000..1003 as u64 {
            if i > offset as u64 {
                if reply.add(i, i as i64, FileType::RegularFile, &format!("file{}", i)) {
                    break;
                }
            }
        }
        reply.ok();
    }

    fn readdirplus(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        mut reply: fuser::ReplyDirectoryPlus,
    ) {
        trace!(
            "FUSE::readdirplus() ino={} fh={} offset={}",
            ino,
            fh,
            offset
        );
        if ino >= 1000 {
            reply.error(libc::ENOTDIR);
            return;
        }
        let ttl = Duration::from_secs(60);
        for i in 1000..1003 as u64 {
            if i >= offset as u64 {
                let attr = self.make_fuse_attr(i as u64, FileType::RegularFile, 10);
                reply.add(i, i as i64, &format!("file{}", i), &ttl, &attr, 0);
            }
        }
        reply.ok();
    }

    fn getattr(&mut self, _req: &Request<'_>, ino: u64, reply: fuser::ReplyAttr) {
        trace!("FUSE::getattr() ino={}", ino);
        let ttl = Duration::from_secs(60);
        if ino < 1000 {
            let attr = self.make_fuse_attr(ino, FileType::Directory, 10);
            reply.attr(&ttl, &attr);
        } else {
            let attr = self.make_fuse_attr(ino, FileType::RegularFile, 10);
            reply.attr(&ttl, &attr);
        }
    }

    fn read(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        size: u32,
        flags: i32,
        lock_owner: Option<u64>,
        reply: fuser::ReplyData,
    ) {
        trace!(
            "FUSE::read() ino={} fh={} offset={} size={} flags={} lock_owner={:?}",
            ino,
            fh,
            offset,
            size,
            flags,
            lock_owner
        );
        if ino < 1000 {
            reply.error(libc::EISDIR);
        } else {
            reply.data("0123456789\n".to_string().as_bytes());
        }
    }
}
