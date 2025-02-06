use std::string;

use crate::core::utils::net;
use reqwest::{self, Body};
use serde::Deserialize;
use serde_json;

#[derive(Deserialize, Debug)]
struct ProjectJson {
    id: String,
    team: String,
    body_url: Option<String>,
    moderator_message: ProjectJsonModeratorMessage,
    published: String,
    updated: String,
    approved: Option<String>,
    queued: Option<String>,
    followers: i32,
    license: ProjectJsonLicense,
    versions: Vec<String>,
    game_versions: Vec<String>,
    loaders: Vec<String>,
    gallery: Vec<ProjectJsonGallery>,
}

#[derive(Deserialize, Debug)]
struct ProjectJsonModeratorMessage {
    message: String,
    body: String,
}

#[derive(Deserialize, Debug)]
struct ProjectJsonLicense {
    id: String,
    name: String,
    url: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ProjectJsonGallery {
    url: String,
    featured: bool,
    title: Option<String>,
    description: Option<String>,
    created: String,
    ordering: i32,
}

pub struct ModrinthProject {
    id: String,
    name: String,
    description: String,
    version: Vec<String>,
}

pub async fn get_project_byid(
    project_id: &str,
) -> Result<ModrinthProject, Box<dyn std::error::Error>> {
    let client = net::HttpClient::new();
    if let Ok(response) = client
        .get(&format!(
            "https://api.modrinth.com/v2/project/{}",
            project_id
        ))
        .await
    {
        let res: ProjectJson = serde_json::from_str(&response.body)?;
    }

    Ok(())
}
