use std::fs::File;
use std::io::Read;

fn main() {
        match read_file(){
            // Hola Rust, Happy with 2024.
            Ok(content) => println!("{}",content),
            // Err: Error opening file: No such file or directory (os error 2)
            Err(err) => println!("Err: {}",err),
        }

    // let rs = read_file()?;
}

/*fn read_file() -> Result<String, String> {
    File::open("src/temp/simple_file_1.txt")
        .map_err(|err| format!("Error opening file: {}", err))
        .and_then(|mut file|{
            let mut content = String::new();
            file.read_to_string(&mut content)
                .map_err(|err| format!("Error reading file: {}",err))
                .map(|_| content)
        })
}*/

// question mark version.
fn read_file() -> Result<String, String> {

    let mut file = File::open("../temp/simple_file_1.txt").map_err(|err| format!("Error opening file: {}", err))?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|err| format!("Error reading file: {}", err))?;

    Ok(content)
}