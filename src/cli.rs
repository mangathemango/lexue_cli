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
    Login {
        /// The cookie value (MoodleSession)
        cookie: String,
    },
    /// Fetch a programming exercise into a folder
    Fetch { 
        // The id of the exercise
        id: String 
    },
    /// Submit your finished exercise
    Submit {
        #[arg(short, long)]
        path: String,
    },
    /// Ping the https://lexue.bit.edu.cn/my/ server
    Ping,
    /// Sends a GET request to the specified url
    Get { url: String },
}