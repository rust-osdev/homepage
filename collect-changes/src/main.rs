use chrono::{DateTime, Datelike, Duration, Utc};
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, LINK};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize, Debug, Clone)]
struct Repo {
    name: String,
    html_url: String,
}

#[derive(Deserialize, Debug, Clone)]
struct User {
    id: u64,
    login: String,
    html_url: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Pull {
    title: String,
    html_url: String,
    merged_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    user: Option<User>,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let last_month = {
        let twenty_days_ago = Utc::now().checked_sub_signed(Duration::days(20)).unwrap();
        twenty_days_ago
            .with_day(1)
            .unwrap()
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
    };
    let year = last_month.year();
    let month = last_month.month();
    println!("Creating changelog for {year}/{month}");

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/vnd.github+json".parse().unwrap());
    headers.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());
    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        println!("Using GITHUB_TOKEN");
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {token}").parse().unwrap(),
        );
    }
    let client = reqwest::Client::builder()
        .user_agent("rust-osdev/collect-changes")
        .default_headers(headers)
        .build()?;

    let all_repos: Vec<Repo> = fetch_all(
        &client,
        "https://api.github.com/orgs/rust-osdev/repos\
         ?type=sources&sort=pushed&direction=desc&per_page=100",
    )
    .await?;

    type MergedPulls = Vec<(Pull, DateTime<Utc>)>;
    let mut changes: BTreeMap<String, (Repo, MergedPulls)> = BTreeMap::new();
    for repo in all_repos {
        let mut merged = Vec::new();
        let mut next = Some(format!(
            "https://api.github.com/repos/rust-osdev/{}/pulls\
             ?state=closed&sort=updated&direction=desc&per_page=100",
            repo.name
        ));
        while let Some(url) = next.take() {
            let resp = client.get(&url).send().await?.error_for_status()?;
            let next_url = link_next(resp.headers());
            let items: Vec<Pull> = resp.json().await?;
            let oldest_updated = items.last().and_then(|p| p.updated_at);
            for p in &items {
                if let Some(t) = p.merged_at {
                    if t.year() == year && t.month() == month {
                        merged.push((p.clone(), t));
                    }
                }
            }
            // PRs are returned sorted by updated_at descending; once we're past
            // the target month, no earlier page can contain matches.
            if oldest_updated.map(|u| u < last_month).unwrap_or(true) {
                break;
            }
            next = next_url;
        }
        if !merged.is_empty() {
            changes.insert(repo.name.clone(), (repo, merged));
        }
    }

    for (_, (repo, mut pulls)) in changes {
        println!("\n\n### [`{}`]({})\n", repo.name, repo.html_url);

        let mut thanks: BTreeMap<u64, User> = BTreeMap::new();
        pulls.sort_by_key(|(_, merged_at)| *merged_at);
        for (pull, _) in pulls {
            println!("- [{}]({})", pull.title, pull.html_url);
            if let Some(u) = pull.user {
                thanks.insert(u.id, u);
            }
        }

        if !thanks.is_empty() {
            print!("\nThanks to ");
            let last_idx = thanks.len() - 1;
            for (i, author) in thanks.into_values().enumerate() {
                match i {
                    0 => {}
                    i if i == last_idx => print!(", and "),
                    _ => print!(", "),
                }
                print!("[@{}]({})", author.login, author.html_url);
            }
            println!(" for their contributions!");
        }
    }

    Ok(())
}

async fn fetch_all<T: for<'de> Deserialize<'de>>(
    client: &reqwest::Client,
    first_url: &str,
) -> eyre::Result<Vec<T>> {
    let mut out = Vec::new();
    let mut next = Some(first_url.to_string());
    while let Some(url) = next.take() {
        let resp = client.get(&url).send().await?.error_for_status()?;
        next = link_next(resp.headers());
        let items: Vec<T> = resp.json().await?;
        out.extend(items);
    }
    Ok(out)
}

fn link_next(headers: &HeaderMap) -> Option<String> {
    let link = headers.get(LINK)?.to_str().ok()?;
    for part in link.split(',') {
        let (url_part, rel_part) = part.trim().split_once(';')?;
        let url = url_part.trim().trim_start_matches('<').trim_end_matches('>');
        if rel_part.trim() == r#"rel="next""# {
            return Some(url.to_string());
        }
    }
    None
}
