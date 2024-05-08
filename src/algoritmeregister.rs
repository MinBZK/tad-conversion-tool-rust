use anyhow::Context;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct AlgoritmeregisterRecord {
    pub name: Option<String>,
    pub organization: Option<String>,
    pub description_short: Option<String>,
    pub type_: Option<String>,
    pub category: Option<String>,
    pub website: Option<String>,
    pub status: Option<String>,
    pub goal: Option<String>,
    pub proportionality: Option<String>,
    pub lawful_basis: Option<String>,
    pub iama_description: Option<String>,
    pub standard_version: Option<String>,
    pub uuid: Option<String>,
    pub url: Option<String>,
    pub contact_email: Option<String>,
    pub lang: Option<String>,
    pub publiccode: Option<String>,
    pub source_data: Option<String>,
    pub methods_and_models: Option<String>,
    pub human_intervention: Option<String>,
    pub risks: Option<String>,
    pub provider: Option<String>,
    pub tags: Option<String>,
    pub source_id: Option<String>,
    pub begin_date: Option<String>,
    pub end_date: Option<String>,
    pub lawful_basis_link: Option<String>,
    pub impacttoetsen: Option<String>,
    pub source_data_link: Option<String>,
    pub process_index_url: Option<String>,
    pub publication_category: Option<String>,
    pub lawful_basis_grouping: Option<String>,
    pub impacttoetsen_grouping: Option<String>,
    pub source_data_grouping: Option<String>,
    pub department: Option<String>,
    pub impact: Option<String>,
    pub decision_making_process: Option<String>,
    pub documentation: Option<String>,
    pub competent_authority: Option<String>,
    pub iama: Option<String>,
    pub dpia: Option<String>,
    pub dpia_description: Option<String>,
    pub objection_procedure: Option<String>,
    pub area: Option<String>,
    pub revision_date: Option<String>,
    pub description: Option<String>,
    pub application_url: Option<String>,
    pub mprd: Option<String>,
    pub monitoring: Option<String>,
    pub performance_standard: Option<String>,
}

impl AlgoritmeregisterRecord {
    pub fn load_from_csv(path: impl AsRef<Path>) -> anyhow::Result<Vec<Self>> {
        let file = File::open(path)?;
        let mut reader = csv::Reader::from_reader(file);

        let mut output = Vec::new();
        for record in reader.deserialize() {
            let record: AlgoritmeregisterRecord = record.context("Could not parse record")?;
            output.push(record);
        }
        Ok(output)
    }
}
