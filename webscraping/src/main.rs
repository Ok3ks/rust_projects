use clap::{builder::Str, Parser};
use std::collections::*;
use std::path::Path;
use std::thread;
use std::time::Duration;
use webscraping::{Args, Lyrics};

pub fn get_lyrics(url: String) -> Lyrics {
    _get_lyrics_internal(url)
}
fn _get_lyrics_internal(url: String) -> Lyrics {
    let parts: Vec<String> = _parse_url_path(&url).unwrap();
    let artist: String = parts.get(3).unwrap().to_string();
    let title: String = parts.get(4).unwrap().to_string();

    // let mut lyrics = Vec::<String>::new();
    // let mut meaning= String::new();
    // let mut moods= String::new();
    // let mut rating =Vec::<String>::new();

    let response = reqwest::blocking::get(&url);
    let response = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);

    let lyrics_selector = scraper::Selector::parse("div").unwrap();
    let selections = document
        .select(&lyrics_selector)
        .next()
        .unwrap()
        .text()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    let lyric_section = selections.split("Writer").nth(0).unwrap().to_string();
    let other_section = match selections.split("Writer").nth(1) {
        Some(after_writer) => {
            match after_writer.split("Identify the song's sections").nth(1) {
                Some(section) => section.to_string(),
                None => {
                    println!("Error: Could not find 'Identify the song's sections' text");
                    String::new() // or return/handle error however you want
                }
            }
        }
        None => {
            println!("Error: Could not find 'Writer' section");
            String::new() // or return/handle error however you want
        }
    };

    let song_lyrics = Lyrics {
        artist: artist,
        title: title,
        lyrics_section: lyric_section,
        other_section: other_section,
    };

    song_lyrics
}

fn _parse_url_path(url: &str) -> Result<Vec<String>, String> {
    if url.is_empty() {
        return Err(String::from("URL cannot be empty"));
    }

    let parts: Vec<String> = url
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    Ok(parts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func_get_albums() {
        let artist_url = String::from("https://www.musixmatch.com/artist/Kendrick-Lamar/albums");
        let albums = get_albums(artist_url);

        for i in (albums) {
            assert_eq!(i.starts_with("/album/"), true);
        }
    }

    #[test]
    fn test_func_get_songs() {
        let album_url = String::from("https://www.musixmatch.com/artist/Taylor-Swift/album/Taylor-Swift/Taylor-Swift-Big-Machine-Radio-Release-Special");
        let songs = get_songs(album_url);

        for i in (songs) {
            assert_eq!(i.starts_with("/lyrics/"), true);
        }
    }

    #[test]
    fn test_func_get_lyrics() {
        let song_url =
            String::from("https://www.musixmatch.com/lyrics/Taylor-Swift/champagne-problems");
        let songs = get_lyrics(song_url);

        ////Get Lyrics
        assert_eq!(songs.lyrics_section.contains("Lyrics"), true);
        assert_eq!(songs.other_section.contains("Mood"), true);
        assert_eq!(songs.other_section.contains("Rating"), true);
        assert_eq!(songs.other_section.contains("Meaning"), true);
    }
}

pub fn _get_songs(album_url: String) -> HashSet<String> {
    get_songs(album_url)
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

    let _song_links = links
        .into_iter()
        .filter(|x| x.starts_with("/lyrics/"))
        .collect::<HashSet<_>>();
    _song_links
}

pub fn _get_albums(url: String) -> HashSet<String> {
    get_albums(url)
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

    let _album_links = links
        .into_iter()
        .filter(|x| x.starts_with("/album/"))
        .collect::<HashSet<_>>();
    _album_links
}

fn get_artist_name() -> String {
    let args = Args::parse();
    let _album_base_url: String = format!("https://www.musixmatch.com/artist/{0}/albums", {
        args.artist
    })
    .to_string();
    _album_base_url
}

fn main() {
    let _base_url = String::from("https://www.musixmatch.com/");
    let _album_url = get_artist_name();

    let albums = get_albums(_album_url.clone());
    let mut count = 0;

    for element in albums {
        for song in get_songs(_base_url.clone() + &element.clone().trim_start_matches('/')) {
            let lyric = get_lyrics(_base_url.clone() + &song.clone().trim_start_matches('/'));

            count += 1;

            lyric.save(Path::new(
                format!("../lyrics/{0}/{1}", { &lyric.artist }, { &lyric.title }).as_str(),
            ));
        }
        thread::sleep(Duration::from_millis(10000));
    }
}

//add rayon now!

// slam into database?

//Hyphenify consumer input i.e artist name

//Extract exact song lyrics

//Build RAG on top? in Rust or in python? --use ahnlich?

//tests

//frontend (leptos or react)

//understand owned and borrowed references

//unwrap_or_else, match , if let
