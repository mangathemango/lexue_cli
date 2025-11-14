use anyhow::Result;
use clap::{Parser, Subcommand};
use dirs::home_dir;
use reqwest::Response;
use scraper::{ElementRef, Html, Node, Selector};
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

fn render_elem_to_md<W: Write>(el: &ElementRef, out: &mut W) -> std::io::Result<()> {
    let tag = el.value().name();

    match tag {
        "p" => {
            let txt = el.text().collect::<String>().trim().to_string();
            if !txt.is_empty() {
                writeln!(out, "{}\n", txt)?;
            }

            // recurse into nested elements (like div inside p)
            for child in el.children() {
                if let Some(child_el) = ElementRef::wrap(child) {
                    render_elem_to_md(&child_el, out)?;
                }
            }
        }

        "h3" => {
            let txt = el.text().collect::<String>().trim().to_string();
            writeln!(out, "### {}\n", txt)?;
        }

        "ul" => {
            for li in el.select(&scraper::Selector::parse("li").unwrap()) {
                render_elem_to_md(&li, out)?;
            }
        }

        "li" => {
            write!(out, "- ")?;
            let mut txt = el.text().collect::<String>().trim().to_string();
            if !txt.is_empty() {
                writeln!(out, "{}", txt)?;
            }

            // recurse into *nested* elements inside <li>
            for child in el.children() {
                if let Some(child_el) = ElementRef::wrap(child) {
                    render_elem_to_md(&child_el, out)?;
                }
            }
        }

        "img" => {
            if let Some(src) = el.value().attr("src") {
                writeln!(out, "![image]({})\n", src)?;
            }
        }

        "div" => {
            // div contains images, captions, etc.
            for child in el.children() {
                if let Some(child_el) = ElementRef::wrap(child) {
                    render_elem_to_md(&child_el, out)?;
                }
            }
        }

        _ => {
            // fallback — recurse into children
            for child in el.children() {
                if let Some(child_el) = ElementRef::wrap(child) {
                    render_elem_to_md(&child_el, out)?;
                }
            }
        }
    }

    Ok(())
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
                    if let Some(elem_ref) = ElementRef::wrap(child) {
                        render_elem_to_md(&elem_ref, &mut readme_writer)?;
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
