use crate::cli::Cli;
use crate::table::MarkdownTable;
use octocrab::models::pulls::{MergeableState, ReviewState};
use octocrab::Octocrab;

mod cli;
mod table;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::init()?;
    let github = Octocrab::builder()
        .personal_token(cli.personal_token.clone())
        .build()?;
    octocrab::initialise(github);
    let prs = if let Some(pr) = cli.pr {
        let pr = octocrab::instance()
            .pulls(&cli.owner, &cli.repo)
            .get(pr)
            .await?;

        vec![pr]
    } else {
        let prs = octocrab::instance()
            .pulls(&cli.owner, &cli.repo)
            .list()
            .send()
            .await?;
        prs.items
    };

    println!(
        "Collecting data for {} PRs... This may take a few seconds.",
        prs.len()
    );
    println!();

    let mut md = MarkdownTable::new();
    md.set_titles(vec!["PR", "Author", "Mergeable", "Review state"]);
    for pr in prs {
        // Exclude draft PRs
        if pr.draft.unwrap_or(true) {
            continue;
        }

        let age = (chrono::Utc::now() - pr.updated_at.unwrap())
            .to_std()
            .unwrap();
        if age > std::time::Duration::from_secs(60 * 60 * 24 * 60) {
            continue;
        }

        let commits: serde_json::Value = octocrab::instance()
            .get(&pr.commits_url.unwrap(), None::<&()>)
            .await?;
        let last_commit = commits.as_array().unwrap().last().unwrap();

        // let check_suites_url = format!(
        //     "https://api.github.com/repos/tari-project/tari-dan/commits/{}/check-suites",
        //     last_commit["sha"].as_str().unwrap()
        // );
        // let check_suites: serde_json::Value = octocrab::instance()
        //     .get(&check_suites_url, None::<&()>)
        //     .await?;

        let check_runs_url = format!(
            "https://api.github.com/repos/{}/{}/commits/{}/check-runs",
            cli.owner,
            cli.repo,
            last_commit["sha"].as_str().unwrap()
        );
        let check_runs = octocrab::instance()
            .get::<serde_json::Value, _, _>(&check_runs_url, None::<&()>)
            .await?;

        let tests_passed = check_runs["check_runs"]
            .as_array()
            .unwrap()
            .iter()
            .all(|s| s["conclusion"] == "success");
        let tests_pending = !tests_passed
            // && check_suites["check_suites"]
            //     .as_array()
            //     .unwrap()
            //     .iter()
            //     .any(|s| s["status"] != "completed")
            && check_runs["check_runs"]
                .as_array()
                .unwrap()
                .iter()
                .any(|s| s["status"] != "completed");

        let reviews = octocrab::instance()
            .pulls(&cli.owner, &cli.repo)
            .list_reviews(pr.number)
            .send()
            .await?;
        let review_state = reviews
            .into_iter()
            .find(|r| {
                !matches!(
                    r.state,
                    Some(ReviewState::Open) | Some(ReviewState::Pending)
                )
            })
            .map(|r| match r.state {
                Some(ReviewState::Approved) => format!(
                    "Approved{}",
                    r.user
                        .map(|u| format!(" by {}", u.login))
                        .unwrap_or_default()
                ),
                Some(ReviewState::Dismissed) => "Dismissed".to_string(),
                Some(ReviewState::ChangesRequested) => format!(
                    "Changes requested{}",
                    r.user
                        .map(|u| format!(" by {}", u.login))
                        .unwrap_or_default()
                ),
                Some(ReviewState::Commented) => format!(
                    "Commented{}",
                    r.user
                        .map(|u| format!(" by {}", u.login))
                        .unwrap_or_default()
                ),
                None => "Needs review".to_string(),
                s => unreachable!("Unexpected review state: {:?}", s),
            })
            .unwrap_or_else(|| "Needs review".to_string());

        let pr = octocrab::instance()
            .pulls(&cli.owner, &cli.repo)
            .get(pr.number)
            .await?;

        let row = row![
            format!(
                "[#{}]({}) {}",
                pr.number,
                pr.html_url.unwrap(),
                if tests_passed {
                    "🟢"
                } else if tests_pending {
                    "🟡"
                } else {
                    "🔴"
                },
            ),
            pr.user.map(|u| u.login).unwrap_or_default(),
            pr.mergeable_state
                .map(|s| to_better_state(&s))
                .unwrap_or_else(|| "Unknown".to_string()),
            review_state
        ];
        md.add_row(row);
    }

    println!("{}", md);
    Ok(())
}

fn to_better_state(state: &MergeableState) -> String {
    match state {
        MergeableState::Dirty => "Conflicts".to_string(),
        MergeableState::Blocked | MergeableState::Unknown => "No".to_string(),
        s => format!("{:?}", s),
    }
}
