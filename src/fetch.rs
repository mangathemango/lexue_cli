use scraper::{ElementRef, Html, Selector};
use std::fs;
use anyhow::Result;
use std::fs::File;
use std::io::{BufWriter, Write};
use percent_encoding::percent_decode_str;
use crate::get::get;



async fn download_image(url: &str, dir: String) -> anyhow::Result<String> {
    // Extract the filename from URL (strip query) or generate one
    let raw = url.split('/').last().unwrap_or("image.bin");
    let raw = raw.split('?').next().unwrap_or(raw);

    // Percent-decode (e.g. "%E5%85%AC..." -> "公司那点事1.JPG")
    let decoded = percent_decode_str(raw).decode_utf8_lossy().to_string();

    // Sanitize filename for filesystem (avoid illegal Windows characters)
    let filename: String = decoded
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            _ => c,
        })
        .collect();

    // Build path with PathBuf for cross-platform safety
    let mut path = std::path::PathBuf::from(&dir);
    path.push("assets");
    fs::create_dir_all(&path)?;

    path.push(&filename);

    // Fetch the image (this uses your cookie-authenticated client)
    let bytes = get(url).await?.bytes().await?;
    fs::write(&path, &bytes)?;

    Ok(format!("assets/{}", filename))
}

fn render_elem_to_md<'a, W: Write + 'a>(
    el: ElementRef<'a>,
    out: &'a mut W,
    dir: &'a String,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = std::io::Result<()>> + 'a>> {
    Box::pin(async move {
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
                        render_elem_to_md(child_el, out, dir).await?;
                    }
                }
            }

            "h3" => {
                let txt = el.text().collect::<String>().trim().to_string();
                writeln!(out, "### {}\n", txt)?;
            }

            "ul" => {
                for li in el.select(&scraper::Selector::parse("li").unwrap()) {
                    render_elem_to_md(li, out, dir).await?;
                }
            }

            "li" => {
                write!(out, "- ")?;
                let mut has_child = false;
                // recurse into *nested* elements inside <li>
                for child in el.children() {
                    if let Some(child_el) = ElementRef::wrap(child) {
                        render_elem_to_md(child_el, out, dir).await?;
                        has_child = true;
                    }
                }

                if has_child {
                    return Ok(());
                }
                let txt = el.text().collect::<String>().trim().to_string();
                if !txt.is_empty() {
                    writeln!(out, "{}", txt)?;
                }


            }

            "img" => {
                if let Some(src) = el.value().attr("src") {
                    writeln!(
                        out,
                        "![image]({})\n",
                        download_image(src, dir.clone()).await.unwrap()
                    )?;
                }
            }

            "div" => {
                // div contains images, captions, etc.
                for child in el.children() {
                    if let Some(child_el) = ElementRef::wrap(child) {
                        render_elem_to_md(child_el, out, dir).await?;
                    }
                }
            }

            _ => {
                // fallback — recurse into children
                for child in el.children() {
                    if let Some(child_el) = ElementRef::wrap(child) {
                        render_elem_to_md(child_el, out, dir).await?;
                    }
                }
            }
        }

        Ok(())
    })
}


pub async fn fetch(url: &String) -> Result<()> {
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

    writeln!(readme_writer, "# {}", title)?;
    let div_sel = Selector::parse(".no-overflow").unwrap();
    if let Some(div) = document.select(&div_sel).next() {
        // iterate over children
        for child in div.children() {
            if let Some(elem_ref) = ElementRef::wrap(child) {
                render_elem_to_md(elem_ref, &mut readme_writer, &title).await?;
            }
        }
    }
    Ok(())
}
