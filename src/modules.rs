#![allow(dead_code)]

pub mod functions;
pub mod context;

use std::any::Any;
use std::fmt::{Debug, format};
use std::fs::create_dir_all;
use std::io::Read;
use std::path::{Path, PathBuf};
use lazy_static::lazy_static;
use std::sync::{Mutex, Arc};
use anyhow::bail;
use colored::{Color, Colorize};

use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use crate::utils::log_warn;

lazy_static! {
     pub static ref GLOBAL_MODULE_LOADER: Arc<Mutex<ModuleLoader>> = Arc::new(Mutex::new(ModuleLoader::new(get_loader_info())));
}

fn get_loader_info() -> GlobalFluxConfiguration {
    let cfg_path = Path::new("./flux.toml");
    if !cfg_path.exists() {
        log_warn("No Flux configuration provided! Using default config...");

        return GlobalFluxConfiguration { libraries: vec![], merge_packs: true }
    }
    let mut buf = String::new();
    std::fs::File::open(cfg_path).unwrap().read_to_string(&mut buf).unwrap();
    toml::de::from_str(&buf).unwrap()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalFluxConfiguration {
    pub libraries: Vec<PathBuf>,
    pub merge_packs: bool,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum MinecraftVersion {
    #[serde(rename = "1.19")]
    v1_19,
    #[serde(rename = "1.18.2")]
    v1_18_2,
    #[serde(rename = "1.18")]
    v1_18,
    #[serde(rename = "1.17")]
    v1_17,
    #[serde(rename = "1.16")]
    v1_16,
    #[serde(rename = "1.15")]
    v1_15,
    #[serde(rename = "1.13")]
    v1_13,
    #[serde(rename = "latest")]
    Latest,
}

impl MinecraftVersion {
    pub fn id(&self) -> i32 {
        match self {
            MinecraftVersion::v1_18_2 | MinecraftVersion::v1_19 => 9,
            MinecraftVersion::v1_18 => 8,
            MinecraftVersion::v1_17 => 7,
            MinecraftVersion::v1_16 => 6,
            MinecraftVersion::v1_15 => 5,
            MinecraftVersion::v1_13 => 4,
            MinecraftVersion::Latest => 9,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    pub project: ModuleDeclaration,
    pub minecraft: MinecraftDeclaration,
    pub build: Option<BuildDeclaration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDeclaration {
    pub name: String,
    pub version: String,
    pub authors: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildDeclaration {
    pub datapack_output: Option<String>,
    pub textures_output: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftDeclaration {
    pub version: MinecraftVersion,
}

pub struct ModuleLoader {
    config: GlobalFluxConfiguration,
    modules: Vec<Box<dyn Module>>
}


impl ModuleLoader {
    pub fn new(cfg: GlobalFluxConfiguration) -> Self {
        Self {
            config: cfg,
            modules: vec![]
        }
    }

    pub fn load<M: Module>(&mut self, module: M) -> anyhow::Result<()> {
        let mut b = Box::new(module);

        b.load();

        self.modules.push(b);

        Ok(())
    }

    #[cfg(feature = "async_runtime")]
    pub async fn compile_async(&self) -> anyhow::Result<()> {
        // TODO: actual compilation, tbd later
        Ok(())
    }

    #[cfg(not(feature = "async_runtime"))]
    pub fn compile(&self) -> anyhow::Result<()> {
        // TODO: actual compilation, tbd later
        Ok(())
    }
}

pub trait Module: Any + Send + Sync {
    fn name(&self) -> String;

    fn load(&mut self);
    fn init(&mut self);
}
