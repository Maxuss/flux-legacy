use proc_macro::TokenStream;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum MinecraftVersion {
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
            MinecraftVersion::v1_18_2 => 9,
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
    path: PathBuf,
    pkg: Configuration,
}

impl ModuleLoader {
    pub fn new(path: PathBuf, pkg: &Configuration) -> Self {
        Self {
            path,
            pkg: pkg.clone(),
        }
    }

    pub async fn compile(&self) -> anyhow::Result<()> {
        let mut cache = self.path.clone();
        cache.push(".flux_cache");
        let mut res_dir = cache.clone();
        let mut dp_dir = cache.clone();
        res_dir.push("resources");
        dp_dir.push("datapack");

        tokio::fs::create_dir_all(res_dir).await?;
        tokio::fs::create_dir_all(dp_dir).await?;


        // TODO: actual compilation, tbd later
        Ok(())
    }
}

pub trait Module {
    fn load(&mut self);
    fn init(&mut self);
}
