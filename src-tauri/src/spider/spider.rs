use std::collections::HashMap;
use super::model::Item;
use crate::spider::model::MainPage;
use anyhow::{Ok, Result};
use reqwest::{header, Client};
use scraper::{Html, Selector};
use regex::Regex;

const DOMAIN: &str = "http://dilidili11.com";
const MAIN_PAGE_URI: &str = "/acg/0/0/all/1.html";

async fn fetch_html(url: &str) -> Result<Html> {
    let html = get_by_url(url).await?;
    let document = Html::parse_document(&html);
    Ok(document)
}

macro_rules! headers {
    ( $( $key:expr => $value: literal ), * $(,)?) => {{
        let mut headers = header::HeaderMap::new();
        $(
            headers.insert($key, header::HeaderValue::from_static($value));
        )*

        headers
    }};
}

// #[retry::retry(7)]
async fn get_by_url(url: &str) -> Result<String> {
    let client = Client::new();
    let headers = headers!(header::USER_AGENT => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36");

    let response = client.get(url).headers(headers).send().await?;

    let text = response.text().await?;

    Ok(text)
}

pub async fn page(sub_url: Option<&str>) -> Result<MainPage> {
    let url = match sub_url {
        None => {
            format!("{}{}", DOMAIN, MAIN_PAGE_URI)
        }
        Some(sub_url) => {
            format!("{}{}", DOMAIN, sub_url)
        }
    };
    let html = fetch_html(url.as_str()).await?;

    let items_page_html = item_page_html(&html).await?;

    let items = parse_items(&items_page_html)?;
    let pages = parse_pages(&html)?;

    Ok(MainPage {
        items,
        pre_page: pages.0,
        next_pagr: pages.1,
    })
}

async fn item_page_html(html: &Html) -> Result<Html> {
    let items_page_url = items_page_url(&html);

    fetch_html(items_page_url.as_str()).await
}

pub fn items_page_url(html: &Html) -> String {
    let url_script = html
        .select(&Selector::parse("body > div.topall > div > div.nav-down-mb.filter.shadow.clearfix > ul > script").unwrap())
        .next()
        .map(|s| s.text().collect::<String>())
        .unwrap_or_default();

    let base_url_map = parse_js_script(url_script.as_str());
    let base_url = base_url_map.get("_yu_gda_s_sp").unwrap();

    let params_script = html.select(&Selector::parse("body > div.wrap > div.index-tj.mb.clearfix > ul.main > script").unwrap())
        .next()
        .map(|s| s.text().collect::<String>())
        .unwrap_or_default();
    let params_map = parse_js_script(params_script.as_str());

    format!("{base_url}?action={}&page={}&year={}&area={}&class={}&dect={}&id={}",
            params_map.get("fj_action_").unwrap_or(&String::default()),
            params_map.get("fj_page_").unwrap_or(&String::default()),
            params_map.get("fj_year").unwrap_or(&String::default()),
            params_map.get("fj_year").unwrap_or(&String::default()),
            params_map.get("fj_class").unwrap_or(&String::default()),
            params_map.get("dect").unwrap_or(&String::default()),
            params_map.get("fj_id_").unwrap_or(&String::default()),
    )

}

fn parse_js_script(script: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();

    let re = Regex::new("var\\s+(\\w+)_?\\s*=\\s*[\"\']?(.*?)[\"\']?;").unwrap();

    for cap in re.captures_iter(script) {
        if let (Some(key), Some(value)) = (cap.get(1), cap.get(2)) {
            result.insert(key.as_str().to_string(), value.as_str().to_string());
        }
    }

    result
}

fn parse_items(html: &Html) -> Result<Vec<Item>> {
    // 预先解析所有选择器，避免在循环中重复解析
    let li_selector = Selector::parse("li.mb").unwrap();
    let link_selector = Selector::parse("a.li-hv").unwrap();
    let name_selector = Selector::parse("a > div.text > p.name").unwrap();
    let img_selector = Selector::parse("a > div.img > img").unwrap();
    let desc_selector = Selector::parse("a > div.img > p.bz").unwrap();

    let mut items = Vec::<Item>::new();
    for li in html.select(&li_selector) {
        let page_url = li
            .select(&link_selector)
            .next()
            .and_then(|a| a.value().attr("href"))
            .unwrap_or_default();

        let name = li
            .select(&name_selector)
            .next()
            .map(|p| p.text().collect::<String>())
            .unwrap_or_default();

        let image_url = li
            .select(&img_selector)
            .next()
            .and_then(|img| img.value().attr("src"))
            .unwrap_or_default();

        let desc = li
            .select(&desc_selector)
            .next()
            .map(|p| p.text().collect::<String>())
            .unwrap_or_default();

        let id = page_url
            .trim_start_matches("/acg/")
            .trim_end_matches("/")
            .to_string();

        items.push(Item {
            id,
            name,
            image_url: image_url.to_string(),
            desc,
            page_url: page_url.to_string(),
        })
    }

    Ok(items)
}

fn parse_pages(html: &Html) -> Result<(Option<String>, Option<String>)> {
    let selector = Selector::parse("body > div.wrap > div.page.mb.clearfix > a").unwrap();
    let mut result = (None, None);

    for page in html.select(&selector) {
        let href = page.value().attr("href");
        let text = page.text().collect::<String>().trim().to_string();

        if text == "上一页" {
            result.0 = href.map(|s| s.to_string());
        } else if text == "下一页" {
            result.1 = href.map(|s| s.to_string());
        }
    }

    Ok(result)
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[tokio::test]
    async fn test_retry() {
        let _ = get_by_url("https://www.baidu.com").await;
    }

    #[tokio::test]
    async fn test_main_page() {
        let main_page = page(None).await;

        println!("main_page: {main_page:#?}");
    }
}
