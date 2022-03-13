use crate::utils::un_gzip;
use anyhow::bail;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncReadExt;

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
    pub dependencies: Option<HashMap<String, DependencyNotation>>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyNotation {
    pub local: Option<PathBuf>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DependencyResolver {
    mc_ver: i32,
    name: String,
    ver: String,
    ext_dep: Vec<DependencyNotation>,
    path: PathBuf,
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
        let mut res_dir = self.path.clone();
        res_dir.push(".flux_cache");
        res_dir.push("build_intermediate");
        let mut dp_dir = res_dir.clone();
        res_dir.push("resources");
        dp_dir.push("datapack");

        tokio::fs::create_dir_all(res_dir).await?;
        tokio::fs::create_dir_all(dp_dir).await?;

        if let Some(deps) = &self.pkg.dependencies {
            for (k, v) in deps {
                self.resolve_dependency(k.clone(), v.clone()).await?;
            }
        };

        // TODO: actual compilation, tbd later
        Ok(())
    }

    pub async fn resolve_dependency(
        &self,
        name: String,
        dep: DependencyNotation,
    ) -> anyhow::Result<()> {
        let mut cache = self.path.clone();
        cache.push(".flux_cache");
        cache.push(name.clone());
        if !cache.exists() {
            create_dir_all(cache.clone()).await?;
        };
        if dep.local.is_none() && dep.url.is_some() {
            bail!("Repository resolving is not yet implemented!");
        };
        if let Some(resolver) = dep.local {
            let mut file = File::open(resolver).await?;
            let mut str = String::new();
            file.read_to_string(&mut str).await?;
            let resolver: DependencyResolver = toml::from_str(&str)?;
            if resolver.mc_ver > self.pkg.minecraft.version.id() {
                println!(
                    "[Err] {} {} {}",
                    "Could not resolve dependency".red(),
                    name.as_str().yellow(),
                    ", the version is newer than project's version!".red()
                );
                return Ok(());
            }

            let mut bins = resolver.path;
            bins.push("build");
            let mut rp = bins.clone();
            let mut dp = bins.clone();
            rp.push("resources.flux");
            dp.push("datapack.flux");

            if rp.exists() {
                un_gzip(rp, cache.clone()).await;
            }
            if dp.exists() {
                un_gzip(dp, cache.clone()).await
            }
        } else {
            println!(
                "[Err] {} {} {}",
                "Could not resolve dependency".red(),
                name.as_str().yellow(),
                ", the dependency notation does not provide any resolvers!".red()
            );
            return Ok(());
        }

        Ok(())
    }
}
