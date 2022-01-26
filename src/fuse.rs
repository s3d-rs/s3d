use crate::daemon::Daemon;
use fuser::{FileAttr, FileType, Filesystem, Request};
use std::time::Duration;

impl Daemon {
    pub async fn start_fuse_mount(&'static self) -> anyhow::Result<()> {
        let mountpoint = self.conf.local.fuse_mount_point.to_owned();
        if mountpoint.is_empty() {
            return Ok(());
        }
        let mut session = fuser::Session::new(
            self,
            mountpoint.as_ref(),
            &[fuser::MountOption::AutoUnmount],
        )?;
        // run the fuse event loop in a separate thread
        let res = tokio::task::spawn_blocking(move || session.run()).await;
        Ok(res??)
    }

    fn make_fuse_attr(&self, ino: u64, kind: FileType, size: u64) -> FileAttr {
        let now = std::time::SystemTime::now();
        let blksize: u32 = 512;
        FileAttr {
            ino,
            size,
            blocks: (size + (blksize as u64) - 1) / blksize as u64,
            atime: now,
            mtime: now,
            ctime: now,
            crtime: now,
            kind,
            perm: 0o644,
            nlink: 1,
            uid: 0,
            gid: 0,
            rdev: 0,
            blksize,
            flags: 0,
        }
    }
}

impl Filesystem for &Daemon {
    fn statfs(&mut self, _req: &Request<'_>, ino: u64, reply: fuser::ReplyStatfs) {
        debug!("FUSE::statfs() ino={}", ino);
        reply.statfs(0, 0, 0, 0, 0, 512, 255, 0);
    }

    fn open(&mut self, _req: &Request<'_>, ino: u64, flags: i32, reply: fuser::ReplyOpen) {
        debug!("FUSE::open() ino={} flags={}", ino, flags);
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
        debug!(
            "FUSE::release() ino={} fh={} flags={} lock_owner={:?} flush={}",
            ino, fh, flags, lock_owner, flush
        );
        reply.ok();
    }

    fn opendir(&mut self, _req: &Request<'_>, ino: u64, flags: i32, reply: fuser::ReplyOpen) {
        debug!("FUSE::opendir() ino={} flags={}", ino, flags);
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
        debug!("FUSE::releasedir() ino={} fh={} flags={}", ino, fh, flags);
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
        debug!("FUSE::lookup() ino={} name={}", ino, name);
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
        debug!("FUSE::readdir() ino={} fh={} offset={}", ino, fh, offset);
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
        debug!(
            "FUSE::readdirplus() ino={} fh={} offset={}",
            ino, fh, offset
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
        debug!("FUSE::getattr() ino={}", ino);
        let ttl = Duration::from_secs(60);
        if ino < 1000 {
            let attr = self.make_fuse_attr(ino, FileType::Directory, 0);
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
        debug!(
            "FUSE::read() ino={} fh={} offset={} size={} flags={} lock_owner={:?}",
            ino, fh, offset, size, flags, lock_owner
        );
        if ino < 1000 {
            reply.error(libc::EISDIR);
        } else {
            reply.data("0123456789\n".to_string().as_bytes());
        }
    }
}
