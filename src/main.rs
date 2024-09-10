use std::env;
use std::fs;


fn main() {
   let args: Vec<String> = env::args().collect();

   if args.len() < 2{
        eprintln!("Usage: word_counter <file_path>");
        return;
   }

   let file_path = &args[1];

   match fs::read_to_string(file_path) {
        Ok(contents) => {

            let (words,lines,chars) = count_words_lines_chars(&contents);
            println!("Words: {}", words);
            println!("Lines: {}", lines);
            println!("Characters: {}", chars);
       }
       Err(e)=>{
            eprintln!("Error reading file: {}",e);
       }
   }
}

fn count_words_lines_chars(contents: &str) -> (usize, usize, usize){
    let words = contents.split_whitespace().count();
    let lines = contents.lines().count();
    let chars = contents.chars().count();

    (words,lines,chars)
}
