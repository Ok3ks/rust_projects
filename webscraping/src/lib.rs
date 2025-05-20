use serde::{Serialize, Deserialize};
use std::fmt::Display;
use clap::Parser;


#[derive(Serialize, Deserialize, Debug)]
pub struct Lyrics {
    pub artist: String,
    pub title: String,
    pub lyrics: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub artist: String,
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
