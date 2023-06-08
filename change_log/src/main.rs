use git2::{Repository, Commit};
// need to keep a list of-
// commitId, commitMessage

// there are 3 types of commits-
// feature, bugfix,

fn main() {
    let _commits: Vec<Commit> = Vec::new();
    loop {
        let repo: Repository = match Repository::open("/path/to/a/repo") {
            Ok(repo) => repo,
            Err(e) => panic!("Failed to open a repo here: {}", e),
        };

        let remotes = match repo.remotes() {
            Ok (remotes) => remotes,
            Err(e) => panic!("There are no remotes {}", e),
        };

        let remote_name = match remotes.get(0) {
            Some(r) => r,
            None => panic!("Couldnt find repo"),
        };

        print!("Welcome: to {:?}", remote_name);
    }
}
