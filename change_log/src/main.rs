use git2::{Commit, Repository};
use std::{
    error::Error,
    io::{stdin, Read},
    path::PathBuf,
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
enum KeyPress {
    P,
    D,
    S,
    C,
}

// cargo watch -x run
fn get_commits(repo: &Repository) -> Result<Vec<Commit>, Box<dyn Error>> {
    // initialize
    let mut static_commits: Vec<Commit> = Vec::new();
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    // start walking through commits
    for rev in revwalk {
        let commit = repo.find_commit(rev?)?;
        let message = commit
            .summary_bytes()
            .unwrap_or_else(|| commit.message_bytes());

        // Show commit & read input
        println!("{}\t{}", commit.id(), String::from_utf8_lossy(message));
        println!("Press: p to Pick, d to Drop, s to Stop(or finish), c to Cancel");
        let mut buffer = [0u8; 1];
        let i: char = match stdin().read_exact(&mut buffer) {
            Ok(_) => buffer[0] as char,
            Err(e) => {
                println!("Error reading input{}", e);
                '\0'
            }
        };

        // Let user select what to do with input
        if i == 'p' {
            static_commits.push(commit.clone());
            continue;
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
        println!("Unknown input");
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

    let last_char = repo_path.chars().last().unwrap();
    if last_char == '\n' {
        repo_path.pop();
    }
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
