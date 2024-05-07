use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use tad_conversion_tool::algoritmeregister::AlgoritmeregisterRecord;
use tad_conversion_tool::system_card::SystemCard;

#[derive(Clone, Debug, ValueEnum)]
enum InputFormat {
    Algoritmeregister,
    Toetsingskader,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: PathBuf,

    #[arg(short, long)]
    #[clap(value_enum)]
    input_format: InputFormat,

    #[arg(short, long)]
    card_version: String,
}

fn convert_algoritmeregister(file: &impl AsRef<Path>, card_version: &str) {
    let contents = File::open(file).expect("could not read file");
    let mut reader = csv::Reader::from_reader(contents);

    for result in reader.deserialize() {
        let record: AlgoritmeregisterRecord = result.expect("could not read result");
        println!("{:?}", record);
    }

    let system_card = SystemCard::new();
    let output = serde_yml::to_string(&system_card).expect("could not write to yaml");
    let mut file = File::create("system_card.yaml").expect("could not create file");
    file.write_all(&output.as_bytes())
        .expect("could not write to file");
}

fn convert_toetsingskader(file: &impl AsRef<Path>, card_version: &str) {
    unimplemented!()
}

fn main() {
    let args = Args::parse();

    match args.input_format {
        InputFormat::Algoritmeregister => convert_algoritmeregister(&args.file, &args.card_version),
        InputFormat::Toetsingskader => convert_toetsingskader(&args.file, &args.card_version),
    }
}
