use git2::{Commit, Repository};
use std::{error::Error, io::stdin, path::PathBuf, process::exit};

enum CommitPrompt {
    Pick,
    Drop,
    Stop,
    Cancel,
    Unknown,
}

fn get_commit_prompt() -> Result<CommitPrompt, Box<dyn Error>> {
    println!("Press: p to Pick, d to Drop, s to Stop(or finish), c to Cancel");

    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let input_char = input.chars().nth(0).unwrap();
    let input_enum = match input_char {
        'p' => CommitPrompt::Pick,
        'd' => CommitPrompt::Drop,
        's' => CommitPrompt::Stop,
        'c' => CommitPrompt::Cancel,
        _ => CommitPrompt::Unknown,
    };
    Ok(input_enum)
}

pub fn get_commits(repo: &Repository) -> Result<Vec<Commit>, Box<dyn Error>> {
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

        loop {
            let prompt = get_commit_prompt()?;
            match prompt {
                CommitPrompt::Pick => {
                    static_commits.push(commit.clone());
                    println!("Pushed commit {}", commit_id);
                    break;
                }
                CommitPrompt::Drop => {
                    println!("Skipping to next commit {}", commit_id);
                    break;
                }
                CommitPrompt::Stop => {
                    println!("Finished iterating through commits");
                    break 'outer;
                }
                CommitPrompt::Cancel => {
                    println!("Closing the program");
                    exit(0);
                }
                _ => {
                    println!("Unknown command");
                    continue;
                }
            }
        }
    }

    return Ok(static_commits);
}

pub fn get_repo(path: PathBuf) -> Result<Repository, Box<dyn Error>> {
    let repo: Repository = Repository::discover(&path)?;
    Ok(repo)
}

pub fn get_path() -> Result<PathBuf, Box<dyn Error>> {
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
