use caos2::parser::parse_caos_script;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let args: Vec<String> = args.collect();

    let path = args.join(" ");
    println!("Opening file: {}", path);
    let file_content = std::fs::read_to_string(path).expect("Valid file");
    let commands = parse_caos_script(&file_content).expect("Successful parse");
    dbg!(commands);
}
