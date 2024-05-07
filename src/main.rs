use chrono::naive::NaiveDate;
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

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

#[derive(Debug, Deserialize)]
struct AlgoritmeregisterRecord {
    name: Option<String>,
    organization: Option<String>,
    description_short: Option<String>,
    type_: Option<String>,
    category: Option<String>,
    website: Option<String>,
    status: Option<String>,
    goal: Option<String>,
    proportionality: Option<String>,
    lawful_basis: Option<String>,
    iama_description: Option<String>,
    standard_version: Option<String>,
    uuid: Option<String>,
    url: Option<String>,
    contact_email: Option<String>,
    lang: Option<String>,
    publiccode: Option<String>,
    source_data: Option<String>,
    methods_and_models: Option<String>,
    human_intervention: Option<String>,
    risks: Option<String>,
    provider: Option<String>,
    tags: Option<String>,
    source_id: Option<String>,
    begin_date: Option<String>,
    end_date: Option<String>,
    lawful_basis_link: Option<String>,
    impacttoetsen: Option<String>,
    source_data_link: Option<String>,
    process_index_url: Option<String>,
    publication_category: Option<String>,
    lawful_basis_grouping: Option<String>,
    impacttoetsen_grouping: Option<String>,
    source_data_grouping: Option<String>,
    department: Option<String>,
    impact: Option<String>,
    decision_making_process: Option<String>,
    documentation: Option<String>,
    competent_authority: Option<String>,
    iama: Option<String>,
    dpia: Option<String>,
    dpia_description: Option<String>,
    objection_procedure: Option<String>,
    area: Option<String>,
    revision_date: Option<String>,
    description: Option<String>,
    application_url: Option<String>,
    mprd: Option<String>,
    monitoring: Option<String>,
    performance_standard: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct SystemCard {
    version: String,
    name: Option<String>,
    upl: Option<String>,
    owners: Option<Owners>,
    description: Option<String>,
    labels: Option<Vec<Labels>>,
    status: Option<String>,
    publication_category: Option<String>,
    begin_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    goal_and_impact: Option<String>,
    considerations: Option<String>,
    risk_management: Option<String>,
    human_intervention: Option<String>,
    legal_base: Option<Vec<LegalBase>>,
    used_data: Option<String>,
    technical_design: Option<String>,
    external_providers: Option<Vec<String>>,
    references: Option<Vec<String>>,
    models: Option<Vec<ModelCard>>,
    assessments: Option<Vec<AssessmentCard>>,
}

impl SystemCard {
    fn new() -> Self {
        SystemCard {
            version: "0.1a3".to_string(),
            name: None,
            upl: None,
            owners: None,
            description: None,
            labels: None,
            status: None,
            publication_category: None,
            begin_date: None,
            end_date: None,
            goal_and_impact: None,
            considerations: None,
            risk_management: None,
            human_intervention: None,
            legal_base: None,
            used_data: None,
            technical_design: None,
            external_providers: None,
            references: None,
            models: None,
            assessments: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Labels {
    name: Option<String>,
    value: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct LegalBase {
    name: Option<String>,
    link: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Owners {
    oin: Option<String>,
    organization: Option<String>,
    name: Option<String>,
    email: Option<String>,
    role: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ModelCard {}

#[derive(Serialize, Deserialize)]
struct AssessmentCard {}

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
    file.write_all(&output.as_bytes()).expect("could not write to file");
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
