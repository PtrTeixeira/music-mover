use audiotags::Tag;
use clap::Parser;
use std::ffi::OsStr;
use std::fs::{self};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    source: String,
    destination: String,
}

struct SourceDetails<'a> {
    artist: &'a str,
    album: Option<&'a str>,
    title: Option<&'a str>,
    track: Option<u16>,
    ext: Option<&'a OsStr>,
    file_name: &'a OsStr,
}

fn get_relative_path(source_details: SourceDetails) -> Box<Path> {
    let mut dest = PathBuf::new();
    dest.push(source_details.artist);

    if let Some(album) = source_details.album {
        dest.push(album);
    }

    if source_details.ext == None || source_details.title == None {
        dest.push(source_details.file_name);
    } else {
        let ext = source_details.ext.expect("Unreachable");
        let title = source_details.title.expect("Unreachable");

        if let Some(track) = source_details.track {
            dest.push(format!("{} - {}", track, title));
        } else {
            dest.push(title);
        }

        dest.set_extension(ext);
    }

    dest.into_boxed_path()
}

fn main() {
    let args = Args::parse();

    println!("Source: {}, Dest: {}", args.source, args.destination);

    let destination_path = Path::new(&args.destination);
    for entry in fs::read_dir(args.source).unwrap() {
        let file = entry.unwrap();
        let source_path = file.path();
        let tag_source = Tag::new().read_from_path(&source_path);
        if let Ok(tags) = tag_source {
            let artist = tags.artist();
            if artist == None {
                println!("Could not determine artist for track");
                continue;
            }

            let file_name = file.file_name();
            let artist = artist.expect("Unreachable");
            let album = tags.album_title();
            let title = tags.title();
            let track = tags.track_number();
            let ext = source_path.extension();

            let source_details = SourceDetails {
                artist,
                album,
                title,
                track,
                ext,
                file_name: &file_name,
            };

            let relative_dest = get_relative_path(source_details);
            let dest = destination_path.join(relative_dest);

            if let Some(dest_parent) = dest.parent() {
                let _ = fs::create_dir_all(dest_parent);
            }

            println!("Copying from {:?} to {:?}", source_path, dest);
            let _ = fs::copy(source_path, dest);
        } else {
            println!("Failed reading file {:?}", file.path());
        }
    }
}
