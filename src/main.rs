extern crate log;
extern crate reqwest;
extern crate simple_logger;
use log::error;
use log::info;
use serde::Deserialize;
use std::fs;
use std::io::copy;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "oscar")]
struct Opt {
    /// Silence all output
    #[structopt(short, long)]
    silent: bool,

    /// Dry-run. Don't actually download anything
    #[structopt(short, long)]
    dry_run: bool,

    /// Output directory
    #[structopt(short, long, parse(from_os_str), default_value = "/tmp/")]
    output: PathBuf,
}

#[derive(Debug, Deserialize)]
struct Payload {
    items: Vec<Episode>,
}

#[derive(Debug, Deserialize)]
struct Episode {
    id: String,
    nola_episode: String,
    videos: Vec<Video>,
    title: String,
}

impl Episode {
    fn return_episode_number(&self) -> String {
        self.nola_episode[6..8].to_string()
    }

    fn return_season_number(&self) -> String {
        self.nola_episode[4..6].to_string()
    }

    fn return_slug(&self) -> String {
        self.title.replace(" ", "-")
    }

    fn get_video_url(&self) -> String {
        self.videos
            .iter()
            .filter(|x| x.format == "mp4" && x.bitrate == Some("720p".to_string()))
            .map(|x| x.url.to_string())
            .fold("".to_string(), |_acc, x| x)
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Video {
    url: String,
    bitrate: Option<String>,
    format: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn season_number() {
        let video: Video = Video {
            url: "https://url.com".to_string(),
            bitrate: Some("720p".to_string()),
            format: "mp4".to_string(),
        };
        let test: Episode = Episode {
            id: "123".to_string(),
            nola_episode: "SAST4921".to_string(),
            videos: vec![video],
            title: "Foo".to_string(),
        };

        assert_eq!(test.return_season_number(), "49");
    }

    #[test]
    fn episode_number() {
        let video: Video = Video {
            url: "https://url.com".to_string(),
            bitrate: Some("720p".to_string()),
            format: "mp4".to_string(),
        };
        let test: Episode = Episode {
            id: "123".to_string(),
            nola_episode: "SAST4921".to_string(),
            videos: vec![video],
            title: "Foo".to_string(),
        };

        assert_eq!(test.return_episode_number(), "21");
    }
}

fn gen_history_path() -> String {
    let home = match dirs::home_dir() {
        Some(p) => p.to_string_lossy().into_owned(),
        None => "/".to_string(),
    };

    format!("{}/.oscar_history", home)
}

fn get_history() -> String {
    let history_path = gen_history_path();
    let ids = fs::read_to_string(history_path);

    match ids {
        Ok(f) => f,
        _ => "".to_string(),
    }
}

fn update_history(episodes: &str) {
    info!("Updating history");
    let history_path = gen_history_path();
    fs::write(history_path.as_str(), episodes).unwrap_or_else(|e| {
        error!(
            "An error occured updating the history at {}: {}",
            history_path, e
        )
    });
}

fn fetch_video_info() -> Result<Vec<Episode>, Box<dyn std::error::Error>> {
    let url = "https://producerplayer.services.pbskids.org/show-list/?shows=sesame-street&shows_title=Sesame+Street&page=1&page_size=20&available=public&sort=-encored_on&type=episode";
    let client = reqwest::Client::new();
    let videos: Payload = client.get(url).send()?.json()?;

    Ok(videos.items)
}

fn main() {
    let opts = Opt::from_args();

    if !opts.silent {
        simple_logger::init().unwrap_or_else(|e| {
            error!("Problem initiating logging: {}", e);
        })
    }
    let output = opts.output.to_string_lossy();

    let episodes = fetch_video_info().unwrap_or_else(|e| {
        error!("API request failed: {}", e);
        process::exit(1)
    });

    let client = reqwest::Client::new();
    let mut new: bool = false;

    for episode in episodes.iter() {
        let history = get_history();

        if !history.contains(&episode.id) {
            new = true;
            let url = episode.get_video_url();

            let name = format!(
                "{}Sesame-Street-S{}E{}-{}.mp4",
                output,
                episode.return_season_number(),
                episode.return_episode_number(),
                episode.return_slug()
            );
            info!("Found episode {}", name);
            info!("Episode ID {}", episode.id);

            if !opts.dry_run {
                let id = format!("\n{}", &episode.id);
                let mut history = get_history();
                let mut file = fs::File::create(&name).unwrap_or_else(|_e| {
                    error!("Had a problem downloading video to this path: {}", &name);
                    process::exit(1)
                });

                info!("Downloading episode {}", &name);
                let mut movie = client.get(&url).send().unwrap_or_else(|e| {
                    error!("Failed to download video: {}", e);
                    process::exit(1)
                });

                copy(&mut movie, &mut file).unwrap_or_else(|e| {
                    error!("Failed to write movie to location: {}", e);
                    process::exit(1)
                });

                history.push_str(&id);
                update_history(&history);
            }
        }
    }

    if !new {
        info!("No new episodes found.");
    }
}
