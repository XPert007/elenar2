use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use dialoguer::FuzzySelect;
use reqwest;
use scraper::{self, Html, Selector};
use std::io::stdout;
use std::{collections::HashMap, io};
#[tokio::main]
async fn main() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    println!("Enter the light novel you'd like to read");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Couldn't read the name");
    let words: Vec<&str> = name.split_whitespace().collect();
    let mut sub_search = String::new();
    for word in words {
        sub_search.push_str(word);
        sub_search.push_str("+");
    }
    let link = format!("https://novelbin.me/search?keyword={sub_search}");
    let response = reqwest::get(link).await.unwrap(); //probably errors if no internet
    let mut name_with_link: HashMap<String, String> = HashMap::new();
    if response.status().is_success() {
        let body = response.text().await.unwrap();
        let document = Html::parse_document(&body);
        let row_sel = Selector::parse(".list.list-novel.col-xs-12 > *").unwrap();
        let link_sel = Selector::parse("h3.novel-title > a").unwrap();

        for row in document.select(&row_sel) {
            if let Some(a) = row.select(&link_sel).next() {
                let title = a.text().collect::<String>().trim().to_string();
                let href = a.value().attr("href").unwrap_or("").to_string();
                name_with_link.insert(title, href);
            }
        }
    } else {
        println!("response failed");
    }
    let items: Vec<String> = name_with_link.keys().map(|x| x.to_string()).collect();
    let selected = FuzzySelect::new()
        .with_prompt("Select the one you'd like to read")
        .items(&items)
        .interact()
        .unwrap();

    let mut chapter_with_link: HashMap<String, String> = HashMap::new();
    let link = name_with_link.get(&items[selected]).unwrap().clone();

    let slug = link.trim_end_matches('/').rsplit('/').next().unwrap();
    let link = format!("https://novelbin.me/ajax/chapter-archive?novelId={slug}");
    println!("{}", link);
    let response = reqwest::get(link).await.unwrap();
    if response.status().is_success() {
        let body = response.text().await.unwrap();
        let document = Html::parse_document(&body);

        let chapter_sel = Selector::parse("div.panel-body ul.list-chapter li > a").unwrap();

        for a in document.select(&chapter_sel) {
            let title = a.value().attr("title").unwrap_or("").to_string();

            let href = a.value().attr("href").unwrap_or("").to_string();

            let href = href.replace("/novel-book/", "/b/");
            chapter_with_link.insert(title, href);
        }
    } else {
        println!("doesnt work");
    }

    let items: Vec<String> = chapter_with_link.keys().map(|x| x.to_string()).collect();
    let selected = FuzzySelect::new()
        .with_prompt("Select the chapter: ")
        .items(&items)
        .interact()
        .unwrap();
    let link = chapter_with_link.get(&items[selected]).unwrap();
    println!("{}", link);
    let response = reqwest::get(link).await.unwrap();

    let mut paragraphs = Vec::new();
    if response.status().is_success() {
        let body = response.text().await.unwrap();
        let document = Html::parse_document(&body);
        let para_sel = Selector::parse("#chr-content > p").unwrap();

        for p in document.select(&para_sel) {
            let text = p.text().collect::<Vec<_>>().join(" ").trim().to_string();

            if !text.is_empty() {
                paragraphs.push(text);
            }
        }
    } else {
        println!("failed");
    }
    for p in paragraphs {
        println!("{}", p);
    }
}
