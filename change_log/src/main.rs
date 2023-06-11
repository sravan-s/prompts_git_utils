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

        let origin = match repo.find_remote("origin") {
            Ok(r) => r,
            Err(e) => panic!("No remotes{}", e),
        };

        println!("origin is {}", origin.url().unwrap());
        panic!("Exit");
    }
}
