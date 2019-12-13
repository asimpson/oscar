extern crate log;
extern crate reqwest;
extern crate simple_logger;
use log::info;
use serde::Deserialize;
use std::fs;
use std::io::copy;
use std::path::PathBuf;
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

    fn get_video_object(&self) -> Vec<Video> {
        self.videos
            .iter()
            .filter(|x| x.format == "mp4" && x.bitrate == Some("720p".to_string()))
            .cloned()
            .collect()
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

fn get_history() -> Result<String, Box<dyn std::error::Error>> {
    let history_path = format!(
        "{}/.oscar_history",
        dirs::home_dir()
            .expect("Home dir is expanded.")
            .to_string_lossy()
    );
    let ids = fs::read_to_string(history_path);
    let history = match ids {
        Ok(f) => f,
        _ => "".to_string(),
    };

    Ok(history)
}

fn update_history(episodes: &str) {
    info!("Updating history");
    let history_path = format!(
        "{}/.oscar_history",
        dirs::home_dir()
            .expect("Home dir is expanded.")
            .to_string_lossy()
    );
    fs::write(history_path, episodes).expect("ID of first episode is written to .oscar_history");
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
        simple_logger::init().unwrap();
    }
    let output = opts.output.to_str().expect("Output path as str.");
    let episodes = fetch_video_info().expect("Returns a Vec of Episodes.");
    let client = reqwest::Client::new();
    let mut new: bool = false;

    for episode in episodes.iter() {
        let history = get_history().expect("Got history.");
        if !history.contains(&episode.id) {
            new = true;
            let video_obj = episode
                .get_video_object()
                .pop()
                .expect("Returns the only element in the Vec which contains the URL.");

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
                let mut history = get_history().expect("Get history.");
                let mut file = fs::File::create(&name).expect("Is valid file path.");
                let id = format!("\n{}", &episode.id);
                info!("Downloading episode {}", &name);
                let mut dl = client
                    .get(&video_obj.url)
                    .send()
                    .expect("Initiates download of movie file.");
                copy(&mut dl, &mut file).expect("Movie files are written.");
                history.push_str(&id);
                update_history(&history);
            }
        }
    }

    if !new {
        info!("No new episodes found.");
    }
}
