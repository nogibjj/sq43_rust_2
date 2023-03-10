#![allow(unused)]
extern crate clap;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    /// A specific word to find
    pattern: String,
    /// file to open
    file: std::path::PathBuf,
}

fn main() {
    // let matches = App::new("grec")
    //                     .version(1.0)
    //                     .author("Shan Qing")
    //                     .arg(Arg::with_name("file")
    //                         .short("f")
    //                         .long("file")
    //                         .value_name("FILE")
    //                         .required(true))
    //                     .arg(Arg::with_name("pattern")
    //                         .short("p")
    //                         .long("pattern")
    //                         .value_name("PATTERN"))
    //                     .get_matches();

    // let path = PathBuf::from(matches.value_of("file").unwrap());
    let args = Cli::parse();
    use std::collections::HashMap;
    let mut map: HashMap<&str, i32> = HashMap::new();
    let args = Cli::parse();
    let result = std::fs::read_to_string(args.file);
    let content = match result {
        Ok(content) => { content },
        Err(error) => { panic!("Can't deal with {}, just exit here", error); }
    };
    let mut count = 0;

    for line in content.lines() {
        for word in line.split_whitespace() {
            if !map.contains_key(word){
                map.insert(word, 1);
            }
            else {
                *map.get_mut(word).unwrap() += 1;
            }
        }
    }
    // println!("{}", count)
    if args.pattern =="all" {
        for (word, occur) in &map {
            println!("{word:?} : {occur}");
        }
    }
    else {
        if !map.contains_key(&args.pattern as &str) {
            println!("{} : 0", args.pattern);
        }
        else {
            println!("\"{}\" : {}", args.pattern, map.get(&args.pattern as &str).unwrap());
        }
    }
}