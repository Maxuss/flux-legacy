use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FunctionWriter<W> {
    writer: W
}

impl<W> FunctionWriter<W> where W: Write {
    pub fn new(to: W) -> Self {
        Self {
            writer: to
        }
    }

    pub fn write_line<S: Into<String>>(&mut self, line: S) -> anyhow::Result<usize> {
        Ok(self.writer.write(line.into().as_bytes())?)
    }

    pub fn close(&mut self) {
        self.writer.flush().expect("Could not flush function writer.");
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FunctionGenerator;

impl FunctionGenerator {
    pub fn new(named: String, namespace_dir: &PathBuf) -> anyhow::Result<FunctionWriter<File>> {
        let dir = namespace_dir.clone().join("functions").join(format!("{}.mcfunction", named));
        Ok(FunctionWriter::new(File::create(dir)?))
    }
}