use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("args incomplete.");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // println!("With text: \n{contents}");
    
    for line in search(&config.query, &contents){
        println!("{}", line);
    } 

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut rs = Vec::new(); 
    for line in contents.lines() {
        if line.contains(query){
            rs.push(line);
        }
    }
    rs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "rust";
        let contents = "java 
go rust
python
haskell";

        assert_eq!(vec!["go rust"], search(query, contents));
    }
}
