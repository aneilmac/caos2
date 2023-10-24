use caos2::{parse_cos, CaosError};
use std::env;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    args.next();
    let args: Vec<String> = args.collect();
    let path = args.join(" ");
    println!("Reading directory: {}", path);

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let p = entry.path();
        if p.extension().map(|p| p == "cos").unwrap_or(false) {
            println!("{}", p.display());
            let file_content = std::fs::read_to_string(&p)?;
            scan_file(&file_content).expect("Successful");
        }
    }
    Ok(())
}

fn scan_file(file_content: &str) -> Result<(), CaosError> {
    let d = parse_cos(file_content)?;
    // match CaosParser::parse(caos2::Rule::program, &file_content) {
    //     Ok(..) => {
    //         println!(" -- successful parse!");
    //         true
    //     }
    //     Err(e) => {
    //         let err = e.to_string();
    //         println!("-- failed with error:\n{}", err);
    //         false
    //     }
    // }
    println!("{:#?}", d);
    Ok(())
}
