use crate::modules::functions::{FunctionGenerator, FunctionWriter};
use crate::modules::Module;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug)]
pub struct GlobalModuleContext<'a, M> {
    dp_dir: PathBuf,
    res_dir: PathBuf,
    module: &'a M,
    namespace_dir: PathBuf,
}

impl<'a, M> GlobalModuleContext<'a, M>
where
    M: Module,
{
    pub(crate) fn new(dp_dir: PathBuf, res_dir: PathBuf, module: &'a mut M) -> Self {
        Self {
            dp_dir: dp_dir.clone(),
            res_dir,
            module,
            namespace_dir: dp_dir.join("data").join(module.name()),
        }
    }

    pub fn create_function<S: Into<String>>(
        &self,
        named: S,
    ) -> anyhow::Result<FunctionWriter<File>> {
        FunctionGenerator::new(named.into(), &self.namespace_dir)
    }

    #[cfg(feature = "async_runtime")]
    pub fn create_function_async<S: Into<String>>(
        &self,
        named: S,
    ) -> anyhow::Result<crate::modules::functions::AsyncFunctionWriter<File>> {
        FunctionGenerator::new_async(named.into(), &self.namespace_dir)
    }
}
