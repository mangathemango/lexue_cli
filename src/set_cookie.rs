use anyhow::Result;
use std::fs;
use dirs::home_dir;

// Save cookie to a file in user's home directory
pub fn set_cookie(cookie: &str) -> Result<std::path::PathBuf> {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".lexue_cli");
    fs::create_dir_all(&path)?;
    path.push("session.txt");
    fs::write(path.clone(), cookie)?;
    Ok(path)
}