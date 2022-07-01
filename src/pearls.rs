use std::time::Duration;

use scraper::{ElementRef, Html, Selector};
use ureq::Agent;

fn http_request(path: &str) -> anyhow::Result<String> {
    let agent: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();
    let body = agent.get(path).call()?.into_string()?;
    Ok(body)
}

fn inner_text(selection: ElementRef, selector_contents: &Selector) -> String {
    selection
        .select(selector_contents)
        .flat_map(|el| el.text())
        .collect()
}

pub fn get_pages_in_category<S: Into<String>>(category: S) -> anyhow::Result<usize> {
    let body = http_request(&format!("https://www.inpearls.ru/{}", category.into()))?;
    let document = Html::parse_document(&body);
    let mut pub_count = 1;

    let pub_element = document
        .select(&Selector::parse("div.fst-italic.text-muted.mb-3").unwrap())
        .next();

    if let Some(value) = pub_element {
        let text: String = value.text().collect();
        let mut text_value = text.trim().split(' ').collect::<Vec<&str>>()[0];

        if text_value.ends_with('K') {
            text_value = &text_value[1..];
            pub_count = text_value.parse().unwrap_or(1) * 1000;
        } else {
            pub_count = text_value.parse().unwrap_or(1);
        }
    }

    Ok((pub_count / 20).clamp(1, usize::MAX))
}

pub fn ip_get<S: Into<String>>(category: S, page: usize) -> anyhow::Result<Vec<String>> {
    let body = http_request(&format!("https://www.inpearls.ru/{}?page={}", category.into(), page))?;
    let document = Html::parse_document(&body);
    let mut quotes = Vec::new();

    let selector_contents = &Selector::parse("div.main").unwrap();
    let sections = document.select(selector_contents);

    for content in sections {
        let inner: String = inner_text(content, &Selector::parse("p").unwrap());
        let inner_author: String = inner_text(content, &Selector::parse("a").unwrap());
        if !inner.ends_with("… показать весь текст …") {
            quotes.push(format!("{}\n\t© {}", inner, inner_author));
        }
    }

    Ok(quotes)
}
