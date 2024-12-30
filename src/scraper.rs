use std::time::Duration;
use tokio::time::timeout;

use playwright::{
    api::{Browser, Page},
    Error, Playwright,
};
use spinners::{Spinner, Spinners};

pub struct Scraper {
    playwright: Playwright,
    browser: Browser,
}

impl Scraper {
    pub async fn new() -> Result<Self, Error> {
        let playwright = Playwright::initialize().await?;

        playwright.prepare()?;

        let browser = playwright
            .chromium()
            .launcher()
            .headless(true)
            .launch()
            .await?;

        Ok(Scraper {
            playwright,
            browser,
        })
    }

    pub async fn new_page(&self) -> Result<Page, Error> {
        let context = self.browser.context_builder().build().await?;
        let page = context.new_page().await?;
        Ok(page)
    }

    pub async fn wait_for_non_empty_elements(
        &self,
        page: &Page,
        selector: &str,
        timeout_duration: Duration,
    ) -> Result<Vec<String>, playwright::Error> {
        // Timeout para evitar esperas indefinidas
        let result = timeout(timeout_duration, async {
            loop {
                // Busca los elementos en el DOM
                let elements = page.query_selector_all(selector).await?;
                let mut texts = Vec::new();

                for element in &elements {
                    if let Some(text) = element.inner_text().await.ok() {
                        let text: String = text.chars().into_iter().filter(|c| c.is_ascii()).collect();
                        if !text.trim().is_empty() {
                            texts.push(text);
                        }
                    }
                }

                // Si al menos un elemento tiene contenido, salimos del bucle
                if !texts.is_empty() {
                    return Ok(texts);
                }

                // Espera un momento antes de volver a intentarlo
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        })
        .await;

        // Devuelve el resultado o un error si el tiempo se agotó
        result.map_err(|_| playwright::Error::Timeout)?
    }

    pub async fn kata(&self, url: &str) -> Result<Vec<String>, Error> {
        let page = self.new_page().await?;

        let mut loading = Spinner::new(Spinners::Dots, "Scraping...".into());

        page.goto_builder(url).goto().await?;

        let texts = self
            .wait_for_non_empty_elements(&page, ".CodeMirror-code pre", Duration::from_secs(10))
            .await?;

        loading.stop_with_message("Scraping completed successfully.".into());

        Ok(texts)
    }

    pub async fn close(&self) -> Result<(), Error> {
        self.browser.close().await?;
        Ok(())
    }

    pub fn get_playwright(&self) -> &Playwright {
        &self.playwright
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_kata_data() {
        let scraper = Scraper::new().await.unwrap();

        let urls = [
            "https://www.codewars.com/kata/59971e64bfccc70748000068/train/rust",
            "https://www.codewars.com/kata/559ac78160f0be07c200005a/train/rust",
            "https://www.codewars.com/kata/59971e64bfccc70748000068/train/rust",
            "https://www.codewars.com/kata/59971e64bfccc70748000068/train/rust",
        ];

        for url in urls {
            let kata = scraper.kata(url).await.unwrap_or_else(|e| {
                panic!("Test fallido para URL '{}': {:?}", url, e);
            });

            println!("Resultados para {}:", url);
            println!("_____________________________________");
            for text in &kata {
                println!("{text}");
            }
            println!("_____________________________________");
            assert!(!kata.is_empty(), "The vector should not be empty.");
            assert!(
                !kata.iter().all(|k| k.is_empty()),
                "All strings in the vector should not be empty."
            );
        }

        // Asegúrate de cerrar el navegador después de completar los tests
        scraper.close().await.unwrap();
    }
}
