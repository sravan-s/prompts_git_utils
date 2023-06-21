use git2::{Commit, Repository};
use std::{error::Error, io::stdin, path::PathBuf};

// need to keep a list of-
// commitId, commitMessage

// there are 3 types of commits-
// feature, bugfix, chore(others)
// go through each commit ->
// * pick p
// * drop d
// * stop s
// * cancel c
enum KeyPress {
    P,
    D,
    S,
    C,
}

// cargo watch -x run
fn get_commits(repo: &Repository) -> Result<Vec<Commit>, Box<dyn Error>> {
    let mut static_commits: Vec<Commit> = Vec::new();
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;
    for rev in revwalk {
        let commit = repo.find_commit(rev?)?;
        let message = commit
            .summary_bytes()
            .unwrap_or_else(|| commit.message_bytes());
        println!("{}\t{}", commit.id(), String::from_utf8_lossy(message));
        println!("Press: p to Pick, d to Drop, s to Stop, c to Cancel, then press Enter");
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let i = input.chars().last().unwrap_or_default();
        if i == 'p' {
            static_commits.push(commit.clone());
        }
        if i == 'd' {
            continue;
        }
        if i == 's' {
            break;
        }
        if i == 'c' {
            panic!("terminating the program")
        }
    }
    return Ok(static_commits);
}

fn get_repo(path: PathBuf) -> Result<Repository, Box<dyn Error>> {
    let repo: Repository = Repository::discover(&path)?;
    Ok(repo)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::new();
    println!("Input relative repo path");
    let mut repo_path = String::new();
    stdin().read_line(&mut repo_path)?;
    path.push(repo_path);

    let repo = get_repo(path)?;
    let commits = get_commits(&repo)?;
    println!(
        "repo is worktree-> {}; commits len {}",
        repo.is_worktree(),
        commits.len()
    );
    Ok(())
}
