use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Payload {
    pub items: Vec<Episode>,
}

#[derive(Debug, Deserialize)]
pub struct List {
    pub collections: Collections,
}

#[derive(Debug, Deserialize)]
pub struct Collections {
    #[serde(rename = "kids-programs-tier-1")]
    pub tier_1: Tier,
    #[serde(rename = "kids-programs-tier-2")]
    pub tier_2: Tier,
    #[serde(rename = "kids-programs-tier-3")]
    pub tier_3: Tier,
}

#[derive(Debug, Deserialize)]
pub struct Tier {
    pub content: Vec<Show>,
}

#[derive(Debug, Deserialize)]
pub struct Show {
    pub nola_root: String,
    pub slug: String,
    pub title: String,
    pub content_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Episode {
    pub id: String,
    pub nola_episode: String,
    pub videos: Vec<Video>,
    pub title: String,
}

impl Episode {
    fn return_episode_number(&self) -> String {
        let length = self.nola_episode.len();
        self.nola_episode[6..length].to_string()
    }

    fn return_season_number(&self) -> String {
        self.nola_episode[4..6].to_string()
    }

    pub fn return_slug(&self) -> String {
        self.title.replace(" ", "-").replace("/", "-")
    }

    pub fn get_video_url(&self) -> String {
        self.videos
            .iter()
            .filter(|x| {
                x.format == Some("mp4".to_string())
                    && (x.bitrate == Some("720p".to_string())
                        || x.bitrate == Some("4500k".to_string()))
            })
            .map(|x| x.url.to_string())
            .fold("".to_string(), |_acc, x| x)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Video {
    url: String,
    bitrate: Option<String>,
    format: Option<String>,
}
