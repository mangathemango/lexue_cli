use std::fs;
use dirs::home_dir;
use anyhow::Result;

// Save cookie to a file in user's home directory
fn save_cookie(cookie: &str) -> Result<std::path::PathBuf> {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".lexue_cli");
    fs::create_dir_all(&path)?;
    path.push("session.txt");
    fs::write(path.clone(), cookie)?;
    Ok(path)
}

pub fn login(cookie: &String) -> Result<()> {
    println!(
        "Saved cookie {} at {}",
        cookie,
        save_cookie(cookie)?.to_str().unwrap()
    );
    Ok(())
}