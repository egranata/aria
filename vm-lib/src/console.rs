// SPDX-License-Identifier: Apache-2.0
pub trait Console: std::any::Any {
    fn print(&mut self, s: &str) -> std::io::Result<()> {
        print!("{}", s);
        Ok(())
    }
    fn println(&mut self, s: &str) -> std::io::Result<()> {
        println!("{}", s);
        Ok(())
    }
    fn eprintln(&mut self, s: &str) -> std::io::Result<()> {
        eprintln!("{}", s);
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any;
}

#[derive(Default, Clone, Copy)]
pub struct StdConsole;
impl Console for StdConsole {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Default, Clone)]
pub struct TestConsole {
    pub stdout: String,
    pub stderr: String,
}

impl Console for TestConsole {
    fn print(&mut self, s: &str) -> std::io::Result<()> {
        self.stdout.push_str(s);
        Ok(())
    }

    fn println(&mut self, s: &str) -> std::io::Result<()> {
        self.stdout.push_str(s);
        self.stdout.push('\n');
        Ok(())
    }

    fn eprintln(&mut self, s: &str) -> std::io::Result<()> {
        self.stderr.push_str(s);
        self.stderr.push('\n');
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl TestConsole {
    pub fn clear(&mut self) {
        self.stdout.clear();
        self.stderr.clear();
    }
}

impl std::io::Write for dyn Console {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let s = std::str::from_utf8(buf)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;
        self.print(s)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
