use serde::{Serialize, Deserialize};
use std::fmt::Display;


#[derive(Serialize, Deserialize, Debug)]
pub struct Lyrics {
    pub title: String,
    pub lyrics: String,
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
