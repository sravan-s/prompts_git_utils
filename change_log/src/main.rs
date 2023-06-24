use git2::{Commit, Repository};
use std::{
    error::Error,
    io::{stdin, Read},
    path::PathBuf,
    process::exit,
};

// need to keep a list of-
// commitId, commitMessage

// there are 3 types of commits-
// feature, bugfix, chore(others)
// go through each commit ->
// * pick p
// * drop d
// * stop s
// * cancel c

// cargo watch -x run
fn get_commits(repo: &Repository) -> Result<Vec<Commit>, Box<dyn Error>> {
    // initialize
    let mut static_commits: Vec<Commit> = Vec::new();
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    // start walking through commits
    'outer: for rev in revwalk {
        let commit = repo.find_commit(rev?)?;
        let message = commit
            .summary_bytes()
            .unwrap_or_else(|| commit.message_bytes());

        // Show commit & read input
        let commit_id = commit.id();
        println!("{}\t{}", commit_id, String::from_utf8_lossy(message));
        println!("Press: p to Pick, d to Drop, s to Stop(or finish), c to Cancel");

        let mut input = String::new();
        stdin().read_line(&mut input)?;
        let input_char = input.chars().nth(0).unwrap();
        match input_char {
            'p' => {
                static_commits.push(commit.clone());
                println!("Pushed commit {}", commit_id);
                continue 'outer;
            }
            'd' => {
                println!("Skipping to next commit {}", commit_id);
                continue 'outer;
            }
            's' => {
                println!("Finished the task");
                break 'outer;
            }
            'c' => {
                println!("Closing the program");
                exit(0);
            }
            _ => {
                println!("Unknown command: {}", input_char);
            }
        }
    }

    return Ok(static_commits);
}

fn get_repo(path: PathBuf) -> Result<Repository, Box<dyn Error>> {
    let repo: Repository = Repository::discover(&path)?;
    Ok(repo)
}

fn get_path() -> Result<PathBuf, Box<dyn Error>> {
    println!("Input relative repo path");
    let mut path = PathBuf::new();

    let mut repo_path = String::new();
    stdin().read_line(&mut repo_path)?;

    let last_char = repo_path.chars().last().unwrap();
    if last_char == '\n' {
        repo_path.pop();
    }
    path.push(repo_path);
    Ok(path)
}

fn generate_changelog(commits: &Vec<Commit>) -> String {
    println!("{}", commits.len());
    "Hola".to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = get_path()?;
    let repo = get_repo(path)?;
    let commits = get_commits(&repo)?;
    println!(
        "repo is worktree-> {}; commits len {}",
        repo.is_worktree(),
        commits.len()
    );
    Ok(())
}
