use caos2::CaosParser;
use pest::Parser;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    args.next();
    let args: Vec<String> = args.collect();
    let path = args.join(" ");
    println!("Reading directory: {}", path);

    let paths = std::fs::read_dir(path)?;
    for p in paths {
        let p = p?;

        let path = p.path();
        if path.extension().map(|e| e == "cos").unwrap_or(false) {
            print!("Reading file: {}", path.display());
            let file_content = std::fs::read_to_string(path)?;
            let res = scan_file(&file_content);
            if !res {
                break;
            }
        }
    }
    Ok(())
}

fn scan_file(file_content: &str) -> bool {
    match CaosParser::parse(caos2::Rule::program, &file_content) {
        Ok(..) => {
            println!(" -- successful parse!");
            true
        }
        Err(e) => {
            let err = e.to_string();
            println!("-- failed with error:\n{}", err);
            false
        }
    }
}
