use std::fs::File;
use std::io::prelude::*;
use std::string;
use std::env;
use std::path::Path;



fn main() { 
    let args: Vec<String> = env::args().collect();

    let mut input: String;
    let mut input_file: Path;
    input_file = Path::new(&args[1]);

    println!("Minecraft Lexer & Compiler Tool by vopi181");
    println!("https://github.com/vopi181/");
    println!("{}", input_file.to_str);
    load_file(input_file);
}


fn load_file(mut filePath: Path) -> () {
    let mut file = File::open(filePath);
    


}