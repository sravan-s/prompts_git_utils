use git2::Commit;
use regex::Regex;

enum CommitType {
    Feat,
    Fix,
    Doc,
    Chore, // if something is not either of the three above, we make them misc
}

struct ChangeLog {
    feats: String,
    fixes: String,
    docs: String,
    chores: String,
}

impl ChangeLog {
    fn make_changelog(&self) -> String {
        let ChangeLog {
            feats,
            fixes,
            docs,
            chores,
        } = &self;
        let changelog = format!("## features \n {feats} \n ## fixes \n {fixes} \n ## chores \n {chores} \n ##docs \n {docs}");
        changelog
    }
}

// regex:  ^(feat|fix|chore|doc):?
fn map_commit_types(commit_msg: &String) -> CommitType {
    let re = Regex::new(r"^(feat|fix|chore|doc):?").unwrap();
    let commit_type = re.captures(commit_msg).unwrap();
    let commit_type_enum: CommitType = match &commit_type[1] {
        "feat" => CommitType::Feat,
        "fix" => CommitType::Fix,
        "doc" => CommitType::Doc,
        _ => CommitType::Chore,
    };
    commit_type_enum
}

pub fn generate_changelog(commits: &Vec<Commit>) -> String {
    let change_log: ChangeLog = commits
        .iter()
        .map(|commit| {
            let msg = commit.message().unwrap();
            let commit_type: CommitType = map_commit_types(&msg.to_string());
            (commit_type, msg)
        })
        .fold(
            ChangeLog {
                feats: "".to_string(),
                fixes: "".to_string(),
                docs: "".to_string(),
                chores: "".to_string(),
            },
            |mut acc, i| {
                let (c, m): (CommitType, &str) = i;
                match c {
                    CommitType::Feat => acc.feats += m,
                    CommitType::Fix => acc.fixes += m,
                    CommitType::Doc => acc.docs += m,
                    _ => acc.chores += m,
                }
                acc
            },
        );
    change_log.make_changelog()
}
