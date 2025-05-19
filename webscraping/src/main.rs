use webscraping::Lyrics;
use std::collections::*;

fn get_lyrics(url: String) -> String {
    let response = reqwest::blocking::get(&url);
    let response = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);
    
    let lyrics_selector = scraper::Selector::parse("div").unwrap();
    let selections = document.select(&lyrics_selector).next().unwrap().text().map(|s| s.to_string()).collect::<Vec<_>>().join("\n");

    selections
}

fn get_albums(url: String) -> HashSet<String> {

    let response = reqwest::blocking::get(&url);
    let response = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);

    let mut links = Vec::<String>::new();
    let lyrics_selector = scraper::Selector::parse("a").unwrap();

    for element in document.select(&lyrics_selector) {
        if let Some(link) = element.value().attr("href") {
            links.push(String::from(link));
        }
    }

    let _album_links = links.into_iter().filter(|x| x.starts_with("/album/")).collect::<HashSet<_>>();
    _album_links

}

// fn extract_songs from albums
// fn extract_lyrics for each song

fn main() {

    //Parameterize to ensure this scraping process can be parallelized

    let _albums = String::from("https://www.musixmatch.com/artist/Kendrick-Lamar/albums");

    let _base_url = "https://www.musixmatch.com/lyrics/";
    let music_title = String::from("https://www.musixmatch.com/lyrics/Kendrick-Lamar/luther");

    //modularise this into a function
    let albums = get_albums(_albums.clone());
    let mut count = 0;
    for element in albums {
        println!("{}{}", count, element);
        count += 1;
    }

    // let selections = get_lyrics(music_title.clone());
    // let lyric = Lyrics {title: music_title, lyrics: selections};
    // println!("{}", lyric);
}
