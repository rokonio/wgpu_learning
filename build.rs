use naga::front::wgsl::Parser;
use std::fs;
use std::io::prelude::*;

fn main() {
    let shaders_files = fs::read_dir("src/shaders").unwrap();
    let mut parser = Parser::new();
    for file in shaders_files {
        let mut file = fs::File::open(file.unwrap().path()).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        parser.parse(content.as_str()).unwrap();
    }
}
