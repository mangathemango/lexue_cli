use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lexue-cli", version, about = "Lexue Automation CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Log into your Lexue account manually
    Login {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
    /// Fetch programming exercises
    Fetch {
        #[arg(short, long)]
        course: String,
    },
    /// Submit your finished exercise
    Submit {
        #[arg(short, long)]
        path: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Login { username, password } => {
            println!("Logging in as {} with password {}", username, password);
            // later: store token or cookie file here
        }
        Commands::Fetch { course } => {
            println!("Fetching assignments for course: {}", course);
            // later: download problem statements, setup folder
        }
        Commands::Submit { path } => {
            println!("Submitting solution from path: {}", path);
            // later: zip and upload
        }
    }

    Ok(())
}
