use crate::core::utils::net;
use std::str::FromStr;

use crate::core::minecraft::{minecraft::{GamePath, VersionManifestOverall, MinecraftVersion}, NovaError};
use super::{Downloader, OnlineFetch};

struct VanillaDownloader {
    game_path: GamePath,
    name: String,
    version: String
}

impl VanillaDownloader {
    fn new<S: ToString + ?Sized>(game_path: &S, name: &S, version: &S) -> Self {
        Self { 
            game_path: GamePath::from_str(&game_path.to_string().as_str())
                .expect("Failed to create game path."), 
            name: name.to_string(), 
            version: version.to_string()
        }
    }
}

impl Downloader for VanillaDownloader {
    async fn download(&self) -> Result<(), NovaError> {
        let manifest = VersionManifestOverall::fetch().await
            .expect("Failed to fetch version manifest.");
        let mut found = false;
        let mut version: Option<MinecraftVersion> = None;
        let http_client= net::HttpClient::new();
        for i in manifest.versions.iter() { 
            if i.id == self.version {
                found = true;
                version = serde_json::from_str(&http_client.get(&i.url).await.expect("Failed to fetch version json file.").body.unwrap())
                    .expect("Failed to fetch version json file.");
                break;
            }
        }
        if !found {
            return Err(NovaError::msg("Failed to find target version."));
        }


        Ok(())
    }
}