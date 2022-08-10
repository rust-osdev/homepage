use chrono::{Datelike, Duration, Utc};
use octocrab::params;
use std::collections::{BTreeMap, HashMap};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let last_month = {
        let twenty_days_ago = Utc::now().checked_sub_signed(Duration::days(20)).unwrap();
        let first_of_month = twenty_days_ago.with_day(1).unwrap().date();
        first_of_month.and_hms(0, 0, 0)
    };
    let year = last_month.year();
    let month = last_month.month();
    println!("Creating changelog for {year}/{month}",);

    let gh = {
        let mut builder = octocrab::OctocrabBuilder::new();
        builder = if let Ok(token) = std::env::var("GITHUB_TOKEN") {
            println!("Using GITHUB_TOKEN");
            builder.personal_token(token)
        } else {
            builder
        };
        builder.build()?
    };
    let repo_page = gh
        .orgs("rust-osdev")
        .list_repos()
        .repo_type(params::repos::Type::Sources)
        .sort(params::repos::Sort::Pushed)
        .direction(params::Direction::Descending)
        .per_page(100)
        .send()
        .await?;
    let all_repos = gh.all_pages(repo_page).await?;

    let mut changes: HashMap<_, Vec<_>> = HashMap::new();
    let mut repos = HashMap::new();
    for repo in all_repos {
        let mut pulls = gh
            .pulls("rust-osdev", &repo.name)
            .list()
            .state(params::State::Closed)
            .sort(params::pulls::Sort::Updated)
            .direction(params::Direction::Descending)
            .per_page(100)
            .send()
            .await?;
        let entry = changes.entry(repo.name.clone()).or_default();
        repos.insert(repo.name.clone(), repo);
        loop {
            let items = pulls.take_items();
            let merged = items
                .iter()
                .cloned()
                .filter_map(|p| p.merged_at.map(|t| (p, t)))
                .filter(|(_, t)| t.year() == year && t.month() == month);
            entry.extend(merged);
            if items
                .last()
                .and_then(|p| p.updated_at)
                .map(|u| u < last_month)
                .unwrap_or(true)
            {
                break;
            }
            match gh.get_page(&pulls.next).await? {
                None => break,
                Some(next_page) => pulls = next_page,
            }
        }
    }
    changes.retain(|_, pulls| !pulls.is_empty());

    for (repo_name, mut pulls) in changes {
        let repo = &repos[&repo_name];
        println!(
            "\n\n### [`{}`]({})\n",
            repo.name,
            repo.html_url.as_ref().unwrap()
        );

        let mut thanks = BTreeMap::new();

        pulls.sort_by_key(|(_, merged_at)| *merged_at);
        for (pull, _) in pulls {
            println!("- [{}]({})", pull.title.unwrap(), pull.html_url.unwrap());

            let author = pull.user.unwrap();
            thanks.insert(author.id, author);
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
