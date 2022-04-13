use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    // process::{Child, ChildStdin, Command, Stdio},
};

/// CodeWriter pipes generated code through rustfmt and then into an output file.
/// However rustfmt seems to get stuck so we had to disable it for now.
pub struct CodeWriter {
    path: PathBuf,
    w: Option<BufWriter<File>>,
    // w: Box<dyn Write>,
    // rustfmt: Option<Child>,
}

impl CodeWriter {
    pub fn new(file_path: &Path) -> Self {
        println!("CodeWriter file {:?}", file_path);
        let file = File::create(file_path).unwrap();
        // let mut rustfmt = Command::new("rustfmt")
        //     .arg("--edition=2021")
        //     .stdin(Stdio::piped())
        //     .stdout(file)
        //     .spawn()
        //     .unwrap();
        // println!("CodeWriter rustfmt {:?}", rustfmt);
        // let w = BufWriter::new(rustfmt.stdin.take().unwrap());
        let w = BufWriter::new(file);
        CodeWriter {
            path: file_path.to_path_buf(),
            w: Some(w),
            // w: Box::new(w),
            // rustfmt: Some(rustfmt),
            // rustfmt: None,
        }
    }

    pub fn write_code<T: ToString>(&mut self, code: T) {
        self.write_all(code.to_string().as_bytes()).unwrap();
        self.write_all(b"\n\n").unwrap();
    }

    pub fn done(mut self) {
        self.flush().unwrap();
    }
}

impl Write for CodeWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.w.as_mut().unwrap().write(buf)
        // self.w.as_mut().write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        println!("CodeWriter flush buffers {}", self.path.display());
        self.w.take().unwrap().flush()?;
        // self.w.flush()?;
        // println!("CodeWriter wait rustfmt {}", self.path.display());
        // self.rustfmt.take().unwrap().wait()?;
        println!("CodeWriter done {}", self.path.display());
        Ok(())
    }
}
