use clap::Parser;

#[derive(Parser)]
#[command(name = "lexue-cli")]
struct Cli {
    /// a sample flag
    #[arg(short, long)]
    name: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let who = cli.name.unwrap_or_else(|| "stranger".into());
    println!("hello, {}! welcome to lexue-cli 👋", who);
    Ok(())
}
