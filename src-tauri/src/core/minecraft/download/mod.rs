use tauri::AppHandle;

use super::NovaError;

pub mod vanilla;

pub trait Downloader {
    #[allow(async_fn_in_trait)]
    async fn download(&self) -> Result<(), NovaError>;
}

pub trait OnlineFetch {
    async fn fetch() -> Result<Self, NovaError>
    where
        Self: Sized;
}

#[derive(PartialEq, Eq)]
pub enum DownloadSource {
    Mojang,
    BMCLAPI,
}

impl DownloadSource {
    pub fn get(dsource: &Self, url: &String) -> String {
        if *dsource == Self::BMCLAPI {
            let mut processed = url.clone();
            if !processed.ends_with('/') {
                processed += "/";
            }
            return processed
                .replace("http://", "https://")
                .replace("piston-meta.mojang.com", "bmclapi2.bangbang93.com")
                .replace(
                    "resources.download.minecraft.net",
                    "bmclapi2.bangbang93.com/assets",
                )
                .replace("libraries.minecraft.net", "bmclapi2.bangbang93.com/maven")
                .replace("files.minecraftforge.net", "bmclapi2.bangbang93.com")
                .replace(
                    "dl.liteloader.com/versions",
                    "bmclapi.bangbang93.com/maven/com/mumfrey/liteloader",
                )
                .replace(
                    "authlib-injector.yushi.moe",
                    "bmclapi2.bangbang93.com/mirrors/authlib-injector",
                )
                .replace("meta.fabricmc.net", "bmclapi2.bangbang93.com/fabric-meta")
                .replace("maven.fabricmc.net", "bmclapi2.bangbang93.com/maven")
                .replace(
                    "maven.neoforged.net/releases",
                    "bmclapi2.bangbang93.com/maven",
                )
                .replace(
                    "maven.quiltmc.org/repository/release",
                    "bmclapi2.bangbang93.com/maven",
                )
                .replace("meta.quiltmc.org", "bmclapi2.bangbang93.com/quilt-meta");
        }
        return url.clone();
    }
}

#[tauri::command]
fn download(app: AppHandle) {
    todo!()
}
