use anyhow::Context;
use clap::{Parser, ValueEnum};
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
    path: PathBuf,

    #[arg(short, long)]
    #[clap(value_enum)]
    input_format: InputFormat,
}

fn convert_algoritmeregister(
    in_path: impl AsRef<Path>,
    out_dir: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let algreg_records =
        AlgoritmeregisterRecord::load_from_csv(in_path).context("Could not load from csv file.")?;

    // For now only consider version 1.0.0 of algoritmeregister.
    let version = "1.0.0".to_string();
    let algreg_records: Vec<AlgoritmeregisterRecord> = algreg_records
        .into_iter()
        .filter(|record| record.standard_version.as_ref() == Some(&version))
        .collect();

    for record in algreg_records {
        let system_card = SystemCard::from_algoritmeregister_record(&record);
        let filename = format!(
            "{}_{}.yaml",
            record
                .name
                .unwrap()
                .to_lowercase()
                .replace(" ", "_")
                .replace("/", "")
                .replace(":", ""),
            record
                .organization
                .unwrap()
                .to_lowercase()
                .replace(" ", "_")
        );
        let savepath = out_dir.as_ref().join(filename);
        println!("{:?}", savepath);
        system_card.save(savepath)?;
    }

    Ok(())
}

fn convert_toetsingskader(in_path: impl AsRef<Path>, out_path: impl AsRef<Path>) {
    unimplemented!()
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.input_format {
        InputFormat::Algoritmeregister => convert_algoritmeregister(&args.path, "out/")?,
        InputFormat::Toetsingskader => convert_toetsingskader(&args.path, "out/"),
    }

    Ok(())
}
