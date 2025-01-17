
use std::{ fs, error::Error };

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
	pub query: String, 
	pub file_path: String,
}


impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
	    if args.len() < 3 {
		return Err("not enough arguments");	
	} 
	     
	     let query = args[1].clone();
	     let file_path = args[2].clone();
		
	     Ok(Config { query, file_path })

	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}


