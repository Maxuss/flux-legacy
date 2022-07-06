use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tokio::io::{AsyncWrite, AsyncWriteExt};

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
}

#[cfg(feature = "async_runtime")]
pub struct AsyncFunctionWriter<W> {
    writer: W
}

#[cfg(feature = "async_runtime")]
impl<W> AsyncFunctionWriter<W> where W: AsyncWrite {
    pub fn new(to: W) -> Self {
        Self {
            writer: to
        }
    }

    pub async fn write_line<S: Into<String> + Send + Sync>(&mut self, line: S) -> anyhow::Result<usize> {
        Ok(self.writer.write(line.into().as_bytes()).await?)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct FunctionGenerator;

impl FunctionGenerator {
    pub fn new(named: String, namespace_dir: &PathBuf) -> anyhow::Result<FunctionWriter<File>> {
        let dir = namespace_dir.clone().join("functions").join(format!("{}.mcfunction", named));
        Ok(FunctionWriter::new(File::create(dir)?))
    }

    #[cfg(feature = "async_runtime")]
    pub fn new_async(named: String, namespace_dir: &PathBuf) -> anyhow::Result<AsyncFunctionWriter<File>> {
        let dir = namespace_dir.clone().join("functions").join(format!("{}.mcfunction", named));
        Ok(AsyncFunctionWriter::new(tokio::fs::File::create(dir)?))
    }
}