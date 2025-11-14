use anyhow::Result;
use clap::{Parser, Subcommand};
use dirs::home_dir;
use reqwest::Response;
use scraper::{ElementRef, Html, Selector};
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use tokio;

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
        /// The cookie value (MoodleSession)
        cookie: String,
    },
    /// Fetch programming exercises
    Fetch { url: String },
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

// Save cookie to a file in user's home directory
fn save_cookie(cookie: &str) -> Result<std::path::PathBuf> {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".lexue_cli");
    fs::create_dir_all(&path)?;
    path.push("session.txt");
    fs::write(path.clone(), cookie)?;
    Ok(path)
}

async fn get(url: &str) -> anyhow::Result<Response> {
    let mut path = home_dir().expect("Could not find home directory");
    path.push(".lexue_cli");
    path.push("session.txt");
    let cookie = fs::read_to_string(path)?;
    let client = reqwest::Client::new();
    let resp: Response = client
        .get(url)
        .header("Cookie", format!("MoodleSession={}", cookie))
        .send()
        .await?;
    Ok(resp)
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Login { cookie } => {
            println!(
                "Saved cookie {} at {}",
                cookie,
                save_cookie(cookie)?.to_str().unwrap()
            );
        }
        Commands::Fetch { url } => {
            println!("Sending get request to {}... ", url);
            let resp = get(url).await?;
            let html = resp.text().await?;
            let document = Html::parse_document(&html);

            let title_sel = "#region-main div h2";
            let title = document
                .select(&Selector::parse(title_sel).unwrap())
                .next()
                .expect("Invalid website!")
                .text()
                .collect::<String>()
                .trim()
                .to_string();

            println!("{}", title);
            fs::create_dir_all(&title)?;

            // create starter files
            fs::write(format!("{}/main.c", title), "// start coding")?;

            let readme = File::create(format!("{}/README.md", title))?;
            let mut readme_writer = BufWriter::new(readme);

            let div_sel = Selector::parse(".no-overflow").unwrap();
            if let Some(div) = document.select(&div_sel).next() {
                // iterate over children
                for child in div.children() {
                    if let Some(elem) = child.value().as_element() {
                        if let Some(elem_ref) = ElementRef::wrap(child) {

                            match elem.name() {
                                "p" => {
                                    let txt = elem_ref.text().collect::<String>();
                                    writeln!(readme_writer, "{}\n", txt)?;
                                }
                                "h3" => {
                                    let txt = elem_ref.text().collect::<String>();
                                    writeln!(readme_writer, "### {}\n", txt)?;
                                }
                                "ul" => {
                                    for li in elem_ref.select(&Selector::parse("li").unwrap()) {
                                        let txt = li.text().collect::<String>();
                                        writeln!(readme_writer, "- {}", txt)?;
                                    }
                                }
                                "div" => {
                                    for img in elem_ref.select(&Selector::parse("img").unwrap()) {
                                        if let Some(url) = img.value().attr("src") {
                                            writeln!(readme_writer, "![image]({})\n", url)?;
                                        }
                                    }
                                    if let Some(p) = elem_ref.select(&Selector::parse("p").unwrap()).next()
                                    {
                                        let text = p.text().collect::<String>();
                                        writeln!(readme_writer, "{}", text)?;
                                    }
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
        Commands::Submit { path } => {
            println!("Submitting solution from path: {}", path);
            // later: zip and upload
        }
        Commands::Ping => {
            println!("Pinging Lexue Servers... ");
            let resp = get("https://lexue.bit.edu.cn/my/").await?;
            println!("Response: {}", resp.text().await?);
        }
        Commands::Get { url } => {
            println!("Sending get request to {}... ", url);
            let resp = get(url).await?;
            println!("Response: {}", resp.text().await?);
        }
    }

    Ok(())
}
