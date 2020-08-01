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
    pub fn return_slug(&self) -> String {
        self.title.replace(" ", "-").replace("/", "-")
    }

    pub fn get_video_url(&self) -> String {
        let mut videos: Vec<&Video> = self
            .videos
            .iter()
            .filter(|x| x.format == Some("mp4".to_string()))
            .filter(|x| {
                x.bitrate == Some("720p".to_string())
                    || x.bitrate == Some("4500k".to_string())
                    || x.bitrate == Some("1200k".to_string())
            })
            .collect();

        videos.sort_by(|a, b| a.quality_rating().cmp(&b.quality_rating()));

        return videos[0].proper_url();
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Video {
    pub url: String,
    pub bitrate: Option<String>,
    pub format: Option<String>,
}

impl Video {
    fn quality_rating(&self) -> u8 {
        if self.bitrate == Some("720p".to_string()) {
            return 1;
        }

        if self.bitrate == Some("4500k".to_string()) {
            return 2;
        }

        if self.bitrate == Some("1200k".to_string()) {
            return 3;
        }

        return 99;
    }

    fn proper_url(&self) -> String {
        return self.url.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn episode_quality() {
        let video1: Video = Video {
            url: "https://1200.com".to_string(),
            bitrate: Some("1200k".to_string()),
            format: Some("mp4".to_string()),
        };
        let video2: Video = Video {
            url: "https://900k.com".to_string(),
            bitrate: Some("900k".to_string()),
            format: Some("mp4".to_string()),
        };
        let video3: Video = Video {
            url: "https://720.com".to_string(),
            bitrate: Some("720p".to_string()),
            format: Some("mp4".to_string()),
        };
        let video4: Video = Video {
            url: "https://second-1200k.com".to_string(),
            bitrate: Some("1200k".to_string()),
            format: Some("mp4".to_string()),
        };
        let video5: Video = Video {
            url: "https://last.com".to_string(),
            bitrate: Some("1000k".to_string()),
            format: Some("h264".to_string()),
        };
        let test: Episode = Episode {
            id: "123".to_string(),
            nola_episode: "SAST4921".to_string(),
            videos: vec![video1, video2, video3, video4, video5],
            title: "Foo".to_string(),
        };

        assert_eq!(test.get_video_url(), "https://720.com");
    }
}
