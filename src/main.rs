use std::fs;

fn main() {
    println!("Hello, world!");
    let base_path = fs::read_dir(".").unwrap();
    for path in base_path {
        println!("Entry: {}", path.unwrap().path().display());
    }
}
