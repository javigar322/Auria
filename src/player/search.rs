use std::{collections::HashSet, process::Command};

use anyhow::Result;
use serde_json::Value;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct VideoFormat {
    pub title: String,
    pub url: String,
}

impl VideoFormat {
    pub fn new(title: String, url: String) -> Self {
        Self { title, url }
    }
}

pub async fn search_videos(query: String) -> Result<HashSet<VideoFormat>> {
    let limit = 10;

    let output = Command::new("yt-dlp")
        .arg("--dump-json")
        .arg("--default-search")
        .arg("ytsearch")
        .arg(format!("ytsearch{}:{}", limit, query))
        .arg("--no-playlist")
        .output()?;

    let stdout = String::from_utf8(output.stdout)?;

    let mut videos: HashSet<VideoFormat> = HashSet::new();
    eprintln!("STDOUT:\n{}", stdout);

    for line in stdout.lines() {
        if line.is_empty() {
            continue;
        }
        let json: Value = serde_json::from_str(line)?;
        let id = json["id"].as_str().unwrap_or("");
        let title = json["title"].as_str().unwrap_or("Unknown title");
        let url = format!("https://www.youtube.com/watch?v={}", id);
        videos.insert(VideoFormat::new(title.to_string(), url));
        println!("buscando {}", title)
    }

    Ok(videos)
}
