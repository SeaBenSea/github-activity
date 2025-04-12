use clap::Parser;
use log::{debug, info, warn};
use model::Event;
use std::error::Error;

mod model;

#[derive(Parser, Debug)]
#[command(version, about = "A GitHub activity CLI tool", long_about = None)]
struct Cli {
    #[arg(short, long)]
    username: Option<String>,

    #[arg(short, long, help = "Use local events endpoint")]
    local: bool,
}

fn set_url(username: Option<String>, is_local: bool) -> String {
    if is_local {
        "http://localhost:5000/events".to_string()
    } else {
        match username {
            Some(user) => format!("https://api.github.com/users/{}/events?per_page=100", user),
            None => "https://api.github.com/events?per_page=100".to_string(),
        }
    }
}

fn process_event(event: &Event, index: usize) {
    print!("{}: {} ", index, event.actor.display_login);

    match event.event_type.as_str() {
        "CommitCommentEvent" => println!(
            "{} a commit comment on {}",
            event.payload["action"], event.repo.name
        ),
        "CreateEvent" => {
            if !event.payload["ref"].is_null() {
                println!(
                    "created {}:{} on {}",
                    event.payload["ref_type"], event.payload["ref"], event.repo.name
                );
            } else {
                println!(
                    "created {} on {}",
                    event.payload["ref_type"], event.repo.name
                );
            }
        }
        "DeleteEvent" => println!(
            "deleted {}:{} on {}",
            event.payload["ref_type"], event.payload["ref"], event.repo.name
        ),
        "ForkEvent" => println!(
            "created {} by forking from {}",
            event.repo.name, event.payload["forkee"]["full_name"],
        ),
        "GollumEvent" => {
            if let Some(pages) = event.payload["pages"].as_array() {
                println!("updated/edited {} wiki pages", pages.len());

                for wiki in pages {
                    println!(
                        "\t- {} {}:{} on {}",
                        wiki["action"], wiki["page_name"], wiki["title"], wiki["html_url"]
                    );
                }
            } else {
                warn!("GollumEvent payload missing 'pages' array");
            }
        }
        "IssueCommentEvent" => println!(
            "{} a comment [{}] on the issue (#{} [{}]) on {}",
            event.payload["action"],
            event.payload["comment"]["html_url"],
            event.payload["issue"]["number"],
            event.payload["issue"]["html_url"],
            event.repo.name,
        ),
        "IssuesEvent" => println!(
            "{} issue (#{} [{}]) on {}",
            event.payload["action"],
            event.payload["issue"]["number"],
            event.payload["issue"]["html_url"],
            event.repo.name,
        ),
        "MemberEvent" => {
            if let Some(action) = event.payload["action"].as_str() {
                match action {
                    "added" => println!(
                        "added {} on {}",
                        event.payload["member"]["login"], event.repo.name
                    ),
                    "edited" => println!(
                        "changed {} from {} on {}",
                        event.payload["member"]["login"],
                        event.payload["changes"]["old_permission"]["from"],
                        event.repo.name
                    ),
                    _ => println!(
                        "{} {} on {}",
                        action, event.payload["member"]["login"], event.repo.name
                    ),
                }
            } else {
                warn!("MemberEvent with no action specified");
            }
        }
        "PublicEvent" => println!("made {} public", event.repo.name),
        "PullRequestEvent" => println!(
            "{} PR #{} on {} ({})",
            event.payload["action"],
            event.payload["number"],
            event.repo.name,
            event.payload["pull_request"]["html_url"]
        ),
        "PullRequestReviewEvent" => {
            println!(
                "{} a review on PR #{}",
                event.payload["action"], event.payload["pull_request"]["number"],
            );
            println!(
                "\t{} - {}",
                event.payload["review"]["state"], event.payload["review"]["body"]
            )
        }
        "PullRequestReviewCommentEvent" => println!(
            "{} a comment [{}] on the PR (#{} [{}]) on {}",
            event.payload["action"],
            event.payload["comment"]["html_url"],
            event.payload["pull_request"]["number"],
            event.payload["pull_request"]["html_url"],
            event.repo.name
        ),
        "PullRequestReviewThreadEvent" => println!(
            "{} a thread [{}] on the PR (#{} [{}]) on {}",
            event.payload["action"],
            event.payload["thread"]["html_url"],
            event.payload["pull_request"]["number"],
            event.payload["pull_request"]["html_url"],
            event.repo.name
        ),
        "PushEvent" => println!(
            "pushed {} commit(s) to {} of {}",
            event.payload["distinct_size"], event.payload["ref"], event.repo.name
        ),
        "ReleaseEvent" => println!(
            "{} a release {} - {} on {}",
            event.payload["action"],
            event.payload["release"]["tag_name"],
            event.payload["release"]["body"],
            event.repo.name
        ),
        "SponsorshipEvent" => println!(
            "{} a sponsorship on {}",
            event.payload["action"], event.repo.name
        ),
        "WatchEvent" => {
            println!("{} watching {}", event.payload["action"], event.repo.name)
        }
        _ => println!("[WARN] Unknown event type: {}", event.event_type),
    }
}

async fn get_user_events(url: String) -> Result<(), Box<dyn Error>> {
    info!("Fetching events from URL: {}", url);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "github-activity-cli")
        .send()
        .await?
        .error_for_status()?;

    let events: Vec<model::Event> = response.json().await?;

    if events.is_empty() {
        warn!("No events found");
    }

    for (index, event) in events.iter().enumerate() {
        process_event(event, index);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let cli = Cli::parse();
    debug!("Parsed CLI arguments: {:?}", cli);

    let url = set_url(cli.username, cli.local);

    get_user_events(url).await?;

    Ok(())
}
