use anyhow::Result;
pub fn open(uri: &String) -> Result<()> {
    webbrowser::open(format!("https://lexue.bit.edu.cn{}",uri).as_str())?;
    Ok(())
}