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

fn get_songs(album_url: String) -> HashSet<String> {

    let response = reqwest::blocking::get(&album_url);
    let response = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);

    let mut links = Vec::<String>::new();
    let lyrics_selector = scraper::Selector::parse("a").unwrap();

    for element in document.select(&lyrics_selector) {
        if let Some(link) = element.value().attr("href") {
            links.push(String::from(link));
        }
    }

    let _song_links = links.into_iter().filter(|x| x.starts_with("/lyrics/")).collect::<HashSet<_>>();
    _song_links
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

fn main() {

    let _base_url = String::from("https://www.musixmatch.com/");
    let albums = get_albums(_albums.clone());
    let mut count = 0;

    for element in albums {

        println!("{}{}", count, element);

        for song in get_songs(_base_url.clone() + &element.clone()){
            let _lyric = get_lyrics(_base_url.clone() + &song.clone());
            println!("{}", _lyric);
        }
        count += 1;
        break
    }

}
