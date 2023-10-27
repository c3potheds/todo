use std::io::Write;

pub struct Less {
    process: std::process::Child,
    stdin: Option<std::process::ChildStdin>,
}

#[derive(Debug)]
pub struct CouldNotSpawnPaginator(std::io::Error);

impl Less {
    pub fn new(cmd: &[String]) -> Result<Self, CouldNotSpawnPaginator> {
        let mut child = std::process::Command::new(&cmd[0])
            .args(&cmd[1..])
            .stdin(std::process::Stdio::piped())
            .spawn()
            .map_err(CouldNotSpawnPaginator)?;
        let stdin = child.stdin.take().unwrap();
        Ok(Less {
            process: child,
            stdin: Some(stdin),
        })
    }
}

impl Write for Less {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stdin.as_ref().unwrap().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.stdin.as_ref().unwrap().flush()
    }
}

impl Drop for Less {
    fn drop(&mut self) {
        std::mem::drop(self.stdin.take().unwrap());
        self.process.wait().ok();
    }
}
