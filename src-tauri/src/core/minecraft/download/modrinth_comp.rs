use std::string;

use reqwest;
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
    let url = format!("https://api.modrinth.com/v2/project/{}", project_id);
    let client = reqwest::Client::new();
    let request = client.get(&url);
    let response = request.send().await.unwrap();

    let response: ProjectJson = serde_json::from_str(&response.text().await.unwrap())?;

    Ok(())
}
