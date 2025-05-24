use webscraping::{ Lyrics, Args };
use std::collections::*;
use clap::{builder::Str, Parser};

pub fn get_lyrics(url: String) -> Lyrics {
    _get_lyrics_internal(url)
}
fn _get_lyrics_internal(url: String) -> Lyrics {
    println!("{}", url);

    let mut lyrics = Vec::<String>::new();

    let mut meaning= String::new();
    let mut moods= String::new();
    let mut rating =Vec::<String>::new();


    let response = reqwest::blocking::get(&url);
    let response = response.unwrap().text().unwrap();
    let document = scraper::Html::parse_document(&response);
    
    let lyrics_selector = scraper::Selector::parse("div").unwrap();
    let selections = document.select(&lyrics_selector).next().unwrap().text().map(|s| s.to_string()).collect::<Vec<_>>().join("\n");

    let mut lyric_section = selections.split("Writer").nth(0).unwrap().to_string();
    let mut other_section= selections.split("Writer").nth(1).unwrap().to_string();

    // let lyric_start_index = lyric_section.position(|x: &str| x.starts_with("Lyrics")).unwrap(); //overflow on length of characters makes this untrusted
    // let _ = lyric_section.nth(lyric_start_index); //consumes all elements up until the start index

    let song_lyrics = Lyrics{
                                url: url, 
                                lyrics_section: lyric_section, 
                                other_section: other_section
                            };

    song_lyrics

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

        let album_url = String::from("https://www.musixmatch.com/ \n
                                artist/Taylor-Swift/album/Taylor-Swift/Taylor-Swift-Big-Machine-Radio-Release-Special/ \n
                                lyrics/Taylor-Swift/champagne-problems");
        let songs = get_lyrics(album_url);
        
        ////Get Lyrics
        assert_eq!("/lyric/", "/lyric/");
     }
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

// pub fn _get_songs

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
    let _album_base_url :String = format!("https://www.musixmatch.com/artist/{0}/albums", {args.artist}).to_string();
    _album_base_url
}


fn main() {

    let _base_url = String::from("https://www.musixmatch.com/");
    let _album_url = get_artist_name();

    let albums = get_albums(_album_url.clone());
    let mut count = 0;
    
    for element in albums {
        for song in get_songs(_base_url.clone() + &element.clone().trim_start_matches('/')) {
            let _lyric = get_lyrics(_base_url.clone() + &song.clone().trim_start_matches('/'));

            count += 1;
            break
        }
        break

    }

}


//Hyphenify consumer input i.e artist name
//Extract exact song lyrics

//tests
//frontend (leptos or react)
