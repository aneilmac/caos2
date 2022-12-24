use caos2::parser::parse_caos_script;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let args: Vec<String> = args.collect();

    let path = args.join(" ");
    println!("Opening file: {}", path);
    let file_content = std::fs::read_to_string(path).expect("Valid file");

    match parse_caos_script(&file_content) {
        Ok((res, _)) => {
            if res.is_empty() {
                print!("Successful parse!");
            } else {
                println!("Failed with remaining results:\n{}", res);
            }
        }
        Err(e) => {
            let err = nom::error::convert_error(file_content.as_str(), e);
            println!("Failed with error:\n{}", err);
        }
    }
}
