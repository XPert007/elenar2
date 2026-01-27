use reqwest;
use std::io;

#[tokio::main]
async fn main() {
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
    let response = reqwest::get(link).await.unwrap(); //probably errors if no
    //internet
    if response.status().is_success() {
        let body = response.text().await.unwrap();
        println!("{}", body);
    } else {
        println!("response failed");
    }
}
