use std::str::FromStr;

use crate::download::{Downloader, OnlineFetch};
use crate::base::{GamePath, MinecraftVersion, VersionManifestOverall};
use crate::NovaMCoreError;

struct VanillaDownloader {
    game_path: GamePath,
    name: String,
    version: String,
}

impl VanillaDownloader {
    fn new<S: ToString + ?Sized>(game_path: &S, name: &S, version: &S) -> Self {
        Self {
            game_path: GamePath::from_str(game_path.to_string().as_str())
                .expect("Failed to create game path."),
            name: name.to_string(),
            version: version.to_string(),
        }
    }
}

impl Downloader for VanillaDownloader {
    async fn download(&self) -> Result<(), NovaMCoreError> {
        let manifest = VersionManifestOverall::fetch()
            .await
            .expect("Failed to fetch version manifest.");
        let mut found = false;
        let mut version: Option<MinecraftVersion> = None;
        let mut default_headers = reqwest::header::HeaderMap::new();
        default_headers.insert(
            reqwest::header::USER_AGENT, 
            reqwest::header::HeaderValue::from_str("PCL2 Nova MCore/0.0.0")
                .map_err(|e| {
                    NovaMCoreError::msg(&e.to_string())
                })?
            );
        let http_client = reqwest::ClientBuilder::new()
            .default_headers(default_headers)
            .build()
            .map_err(|e| {
                NovaMCoreError::msg(&e.to_string())
            })?;
        for i in manifest.versions.iter() {
            if i.id == self.version {
                found = true;
                version = serde_json::from_str(
                    &http_client
                        .get(&i.url)
                        .send()
                        .await
                        .expect("Failed to fetch version json file.")
                        .text()
                        .await
                        .expect("Failed to fetch version json file.")
                )
                .expect("Failed to fetch version json file.");
                break;
            }
        }
        if !found {
            return Err(NovaMCoreError::msg("Failed to find target version."));
        }

        Ok(())
    }
}
