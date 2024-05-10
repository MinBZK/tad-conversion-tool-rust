use anyhow::Context;
use clap::{Parser, ValueEnum};
use std::fs;
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

    #[arg(short, long, default_value = "out/")]
    output_dir: PathBuf,
}

fn convert_algoritmeregister(
    in_path: impl AsRef<Path>,
    out_dir: impl AsRef<Path>,
) -> anyhow::Result<()> {
    let input_records =
        AlgoritmeregisterRecord::load_from_csv(in_path).context("Could not load from csv file.")?;

    // For now only consider version 1.0.0 of algoritmeregister.
    let input_records: Vec<AlgoritmeregisterRecord> = input_records
        .into_iter()
        .filter(|record| record.standard_version.as_deref() == Some("1.0.0"))
        .collect();

    fs::create_dir_all(&out_dir).context("Could not create output directory")?;

    for record in input_records {
        let system_card = SystemCard::from(&record);
        let filename = format!(
            "{}_{}.yaml",
            record
                .name
                // We know each entry in algoritmeregister has a name, so we can unwrap.
                .unwrap()
                .to_lowercase()
                .replace(" ", "_")
                .replace("/", "")
                .replace(":", ""),
            record
                .organization
                // We know each entry in algoritmeregister has an organization, so we can unwrap.
                .unwrap()
                .to_lowercase()
                .replace(" ", "_")
        );
        let savepath = out_dir.as_ref().join(filename);
        system_card.save(savepath)?;
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.input_format {
        InputFormat::Algoritmeregister => convert_algoritmeregister(&args.path, &args.output_dir)?,
        InputFormat::Toetsingskader => unimplemented!(),
    }

    Ok(())
}
