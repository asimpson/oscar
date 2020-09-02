extern crate log;
extern crate reqwest;
extern crate simple_logger;
use log::error;
use log::info;
use std::fs;
use std::io::copy;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

mod json;

#[derive(StructOpt, Debug)]
#[structopt(name = "oscar")]
struct Opt {
    /// Silence all output
    #[structopt(short, long)]
    silent: bool,

    /// Dry-run. Don't actually download anything
    #[structopt(short, long)]
    dry_run: bool,

    /// Show slug. The show to download. Get slugs by running oscar list.
    #[structopt(short = "S", long, default_value = "sesame-street")]
    show_slug: String,

    /// Output directory
    #[structopt(short, long, parse(from_os_str), default_value = "/tmp/")]
    output: PathBuf,

    #[structopt(subcommand)]
    command: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// List available shows
    #[structopt(name = "list")]
    List,
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

fn fetch_video_info(show_slug: &str) -> Result<Vec<json::Episode>, Box<dyn std::error::Error>> {
    let url = format!("https://producerplayer.services.pbskids.org/show-list/?shows={}&page=1&page_size=20&available=public&sort=-encored_on&type=episode"
                      , show_slug);
    let client = reqwest::Client::new();
    let videos: json::Payload = client.get(&url).send()?.json()?;

    Ok(videos.items)
}

fn fetch_show_list() -> Result<json::Collections, Box<dyn std::error::Error>> {
    let url = "https://content.services.pbskids.org/v2/kidspbsorg/home";
    let client = reqwest::Client::new();
    let list: json::List = client.get(url).send()?.json()?;

    Ok(list.collections)
}

fn main() {
    let opts = Opt::from_args();

    if let Some(Command::List) = opts.command {
        let mut list = fetch_show_list().unwrap_or_else(|e| {
            error!("List API request failed: {}", e);
            process::exit(1)
        });

        list.tier_1.content.append(&mut list.tier_2.content);
        list.tier_1.content.append(&mut list.tier_3.content);

        for item in list.tier_1.content.iter() {
            println!("{:?}", item.slug);
        }

        process::exit(0)
    }

    if !opts.silent {
        simple_logger::init().unwrap_or_else(|e| {
            error!("Problem initiating logging: {}", e);
        })
    }
    let show = opts.show_slug;
    let output = opts.output.to_string_lossy();

    let episodes = fetch_video_info(&show).unwrap_or_else(|e| {
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

            let name = format!("{}{}-{}.mp4", output, &show, episode.return_slug());
            info!("Found episode {}", name);
            info!("Episode ID {}", episode.id);
            info!("Episode url {}", url);

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
                println!("Downloaded {}", &name);
            }
        }
    }

    if !new {
        info!("No new episodes found.");
    }
}
