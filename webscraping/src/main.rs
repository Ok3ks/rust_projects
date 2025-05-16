
use serde::{Serialize, Deserialize};
use std::fmt::Display;
use serde_json::Result;


#[derive(Serialize, Deserialize, Debug)]
pub struct Lyrics {
    title: String,
    lyrics: String,
    //genre?
}

impl Display for Lyrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.title, self.lyrics)
    }
}

pub fn search<'a>(query: String, contents: String) -> Vec<String> {
    let mut results = Vec::<String>::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line.to_string());
        }
    }
    results
}


fn main() -> {

    //Parameterize to ensure this scraping process can be parallelized

    let _base_url = "https://www.musixmatch.com/lyrics/";
    let music_title = String::from("https://www.musixmatch.com/lyrics/Kendrick-Lamar/luther");

    let response = reqwest::blocking::get(&music_title);
    let response = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);
    
    let lyrics_selector = scraper::Selector::parse("div").unwrap();
    let selections = document.select(&lyrics_selector).next().unwrap().text().map(|s| s.to_string()).collect::<Vec<_>>().join("\n");

    let lyric = Lyrics {title: music_title, lyrics: selections};
    // let j = serde_json::to_string(&lyric);

    // Print, write to a file, or send to an HTTP server.
    // println!("{}", j);

    println!("{}", lyric);
    //write to file 

}
