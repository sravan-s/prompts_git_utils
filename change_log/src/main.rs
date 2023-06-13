use git2::{Commit, Repository};
use std::error;
use std::path::PathBuf;

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
fn print_commits(path: PathBuf) -> Result<i32, Box<dyn error::Error>> {
    let repo = Repository::discover(path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;
    for rev in revwalk {
        let commit = repo.find_commit(rev?)?;
        let message = commit
            .summary_bytes()
            .unwrap_or_else(|| commit.message_bytes());
        println!("{}\t{}", commit.id(), String::from_utf8_lossy(message));
    }
    Ok(0)
}

fn main() {
    let _commits: Vec<Commit> = Vec::new();
    let mut path = PathBuf::new();
    path.push("../");
    let _ = print_commits(path);
}
