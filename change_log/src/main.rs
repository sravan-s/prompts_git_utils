use std::error::Error;

mod commits;
mod repo_utils;

fn main() -> Result<(), Box<dyn Error>> {
    let path = repo_utils::get_path()?;
    let repo = repo_utils::get_repo(path)?;
    let selected_commits = repo_utils::get_commits(&repo)?;
    let final_message = commits::generate_changelog(&selected_commits);
    println!("/n/n/n/final message:");
    println!("{}", final_message);
    Ok(())
}
