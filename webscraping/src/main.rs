use webscraping::{ Lyrics, Args };
use std::collections::*;
use clap::Parser;

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
            println!("{}", link);
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

fn get_artist_name() -> String {

    let args = Args::parse();
    let _album_base_url = format!("https://www.musixmatch.com/artist/{0}/albums", {args.artist}).to_string();
    
    _album_base_url
}

fn main() {

    let _base_url = String::from("https://www.musixmatch.com/");
    let _artist = get_artist_name();

    let albums = get_albums(_artist.clone());
    let mut count = 0;
    
    for element in albums {

        for song in get_songs(_base_url.clone() + &element.clone().trim_start_matches('/')) {
            
            let _lyric = get_lyrics(_base_url.clone() + &song.clone().trim_start_matches('/'));
            println!("{}{}", count, _lyric);

            count += 1;
        }
        break

    }

}


//Hyphenify consumer input i.e artist name
//Extract exact song lyrics
