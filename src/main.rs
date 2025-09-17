use audiotags::Tag;
use clap::Parser;
use std::fs::{self};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    source: String,
    destination: String,
}

fn main() {
    let args = Args::parse();

    println!("Source: {}, Dest: {}", args.source, args.destination);

    for entry in fs::read_dir(args.source).unwrap() {
        let file = entry.unwrap();
        let tag_source = Tag::new().read_from_path(file.path());
        match tag_source {
            Ok(tags) => println!("Title {:?}, Album {:?}", tags.title(), tags.album_title()),
            Err(_) => println!("Failed reading file {:?}", file.path()),
        }
    }
}
