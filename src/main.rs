use clap::Parser;
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::probe::Hint;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    source: String,
    destination: String,
}

fn main() {
    let args = Args::parse();

    println!("Source: {}, Dest: {}", args.source, args.destination);
    let src = std::fs::File::open(&args.source).expect("Failed to open!");
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    let mut hint = Hint::new();
    hint.with_extension("mp3");
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let probed = symphonia::default::get_probe().format(&hint, mss, &fmt_opts, &meta_opts)
        .expect("Unsupported format");
    let mut format = probed.format;

    let track = format.tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .expect("No supported audio tracks");
    let dec_opts: DecoderOptions = Default::default();

    let decoder = symphonia::default::get_codecs().make(&track.codec_params, &dec_opts)
        .expect("Unsupported codec");
    let _track_id = track.id;

    loop {

        let _packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(_) => unimplemented!(),
        };

        // println!("format = {:?}", format);
        // println!("Preparing to read tags ....");
        while !format.metadata().is_latest() {
            println!("Reading tags ...");
            format.metadata().pop();

            if let Some(rev) = format.metadata().current() {
                println!("tags = {:?}", rev.tags());
            }
        }
        // println!("Done reading tags");
    }

    println!("Hello, world!");
}
