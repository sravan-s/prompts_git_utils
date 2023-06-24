use git2::Commit;

pub fn generate_changelog(commits: &Vec<Commit>) -> String {
    let commit_msgs: Vec<&str> = commits
        .iter()
        .map(|commit| {
            let msg = commit.message().unwrap();
            println!("msg: {}", msg);
            msg
        })
        .collect();
    commit_msgs.join("\n")
}
