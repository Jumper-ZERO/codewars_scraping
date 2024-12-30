use clap::Parser;
use codewars_scraping::codewars::api::kata;
use codewars_scraping::file::{kata_file, markdown_file};
use codewars_scraping::scraper::Scraper;
use regex::Regex;
use tokio::time::Instant;

#[derive(Parser, Default, Debug)]
#[command(author = "Jumper-ZERO", version)]
/// Scraping kata exercises from codewars
struct Cli {
    #[arg(short, long, required = false, value_parser = validate_kata)]
    /// Url o code of kata exercise
    kata: Option<String>,

    #[arg(required = false, value_parser = validate_code)]
    /// Code of the kata exercise
    code: Option<String>,
}

fn validate_code(code: &str) -> Result<String, String> {
    if code.len() != 24 {
        Err(String::from("Length in code in incorrect"))
    } else {
        Ok(code.to_string())
    }
}

fn validate_kata(kata: &str) -> Result<String, String> {
    let pattern = r"https?://www\.codewars\.com/kata/(?P<code>[a-zA-Z0-9]{24})(/train/rust)?|^(?P<code_only>[a-zA-Z0-9]{24})$";
    let re = Regex::new(pattern).map_err(|e| e.to_string())?;
    if let Some(captures) = re.captures(kata) {
        if let Some(code) = captures.name("code").or(captures.name("code_only")) {
            Ok(code.as_str().to_string())
        } else {
            Err(String::from("Código inválido. No se encontró un código válido en la URL o el código proporcionado."))
        }
    } else {
        Err(String::from(
            "Código inválido. No se encontró un código válido en la URL o el código proporcionado.",
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<(dyn std::error::Error + 'static)>> {
    let start = Instant::now();
    let cli = Cli::parse();
    let code = match &cli.code {
        Some(code) => code.clone(),
        None => match &cli.kata {
            Some(kata) => kata.clone(),
            None => {
                eprintln!("Error: Se requiere 'kata' o 'code'.");
                std::process::exit(1);
            }
        },
    };
    let kata = kata(&code).await?;
    let url = kata.train_url().unwrap();
    let codewars_page = Scraper::new().await.unwrap();

    let content = codewars_page.kata(&url).await.unwrap_or_else(|e| {
        panic!("No se consiguio contenido de: {}, {}", url, e);
    });

    codewars_page.close().await.unwrap();

    println!("Time elapsed: {:?}", start.elapsed());
    markdown_file(&kata).await?;
    kata_file(&kata, content).await?;
    Ok(())
}
