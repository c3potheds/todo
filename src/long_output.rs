use std::io::Write;

pub struct RequiresPrimary {
    max_lines: usize,
}

pub fn max_lines(max_lines: usize) -> RequiresPrimary {
    RequiresPrimary {
        max_lines: max_lines,
    }
}

pub struct RequiresAlternate<A: Write> {
    max_lines: usize,
    primary: A,
}

impl RequiresPrimary {
    pub fn primary<A: Write>(self, primary: A) -> RequiresAlternate<A> {
        RequiresAlternate {
            max_lines: self.max_lines,
            primary: primary,
        }
    }
}

enum Lazy<T, F: FnOnce() -> T> {
    NotCreated(Option<F>),
    Created(T),
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    fn create(&mut self) -> &mut T {
        if let Lazy::NotCreated(f) = self {
            *self = Lazy::Created((f.take().unwrap())());
        }
        match self {
            Lazy::Created(t) => t,
            _ => panic!(),
        }
    }
}

pub struct PipeToAlternateIfOutputIsLong<A: Write, B: Write, F: FnOnce() -> B> {
    max_lines: usize,
    primary: A,
    alternate: Lazy<B, F>,
    buffer: Vec<u8>,
    lines_printed: usize,
}

impl<A: Write> RequiresAlternate<A> {
    pub fn alternate<B: Write, F: FnOnce() -> B>(
        self,
        alternate: F,
    ) -> PipeToAlternateIfOutputIsLong<A, B, F> {
        PipeToAlternateIfOutputIsLong {
            max_lines: self.max_lines,
            primary: self.primary,
            alternate: Lazy::NotCreated(Some(alternate)),
            buffer: Vec::new(),
            lines_printed: 0,
        }
    }
}

impl<A: Write, B: Write, F: FnOnce() -> B> Write
    for PipeToAlternateIfOutputIsLong<A, B, F>
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Lazy::Created(out) = &mut self.alternate {
            out.write(buf)
        } else {
            const ASCII_NEWLINE: u8 = 10;
            self.lines_printed +=
                buf.iter().copied().filter(|&b| b == ASCII_NEWLINE).count();
            if self.lines_printed > self.max_lines {
                // Create the alternate output.
                let alternate = self.alternate.create();
                // Flush the buffer into alternate.
                alternate
                    .write_all(&self.buffer)
                    .and_then(|_| alternate.write_all(buf))
                    .map(|_| buf.len())
            } else {
                self.buffer.write(buf)
            }
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match &mut self.alternate {
            Lazy::Created(out) => out.flush(),
            _ => self.buffer.flush(),
        }
    }
}

impl<A: Write, B: Write, F: FnOnce() -> B> Drop
    for PipeToAlternateIfOutputIsLong<A, B, F>
{
    fn drop(&mut self) {
        match &mut self.alternate {
            Lazy::Created(out) => {
                out.flush().ok();
            }
            _ => {
                self.primary.write_all(&self.buffer).ok();
            }
        }
    }
}

pub struct Less {
    process: std::process::Child,
    stdin: Option<std::process::ChildStdin>,
}

impl Less {
    pub fn new(cmd: &str, args: &[String]) -> std::io::Result<Self> {
        std::process::Command::new(cmd)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                let stdin = child.stdin.take().unwrap();
                Ok(Less {
                    process: child,
                    stdin: Some(stdin),
                })
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
