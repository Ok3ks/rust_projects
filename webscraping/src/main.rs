
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
fn main() {

    let response = reqwest::blocking::get("https://www.musixmatch.com/lyrics/Kendrick-Lamar/luther");
    let response = response.unwrap().text().unwrap();


    let document = scraper::Html::parse_document(&response);
    let lyrics_selector = scraper::Selector::parse("div").unwrap();

    let selections = document.select(&lyrics_selector).next().unwrap().text().collect::<Vec<_>>();

    for line in selections {
        println!("{}", line);
    }
}
