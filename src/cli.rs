use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lexue-cli", version, about = "Lexue Automation CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Log into your Lexue account manually
    Login,
    /// Fetch a programming exercise into a folder
    Fetch { 
        // The id of the exercise
        id: String 
    },
    /// Submit your finished exercise
    Submit,
    /// Ping the https://lexue.bit.edu.cn/my/ server
    Ping,
    /// Sends a GET request to the specified url
    Get { url: String },
    /// Set the cookie of this session
    SetCookie {cookie: String,},
    /// Opens a web page on your browser based on an URI
    Open { 
        /// The url of the page, e.g "/my", "", "course/view.php?id=*"
        uri: String 
    }
}