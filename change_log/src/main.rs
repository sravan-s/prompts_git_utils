use git2::{Commit, Repository};
// need to keep a list of-
// commitId, commitMessage

// there are 3 types of commits-
// feature, bugfix,

// cargo watch -x run
fn main() {
    let _commits: Vec<Commit> = Vec::new();
    loop {
        let repo: Repository = match Repository::open("../") {
            Ok(repo) => repo,
            Err(e) => panic!("Failed to open a repo here: {}", e),
        };

        let remotes = match repo.remotes() {
            Ok(r) => r,
            Err(e) => panic!("repo doesnt have a name {}", e),
        };

        let remote_name = remotes.iter().find(|x| x.is_some()).unwrap().unwrap();

        if !remote_name.is_empty() {
            panic!("remote_name {}", remote_name);
        }
        panic!("no remotes");
    }
}
