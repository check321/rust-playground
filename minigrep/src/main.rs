use std::env;
use std::process;
use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("an error occured while processing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config){
        println!("an error occured while processing serach: {e}");
        process::exit(1);
    }
   
}

