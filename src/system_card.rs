use anyhow::Context;
use chrono::naive::NaiveDate;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::algoritmeregister::AlgoritmeregisterRecord;

#[derive(Serialize, Deserialize)]
pub struct SystemCard {
    pub version: String,
    pub name: Option<String>,
    pub upl: Option<String>,
    pub owners: Option<Vec<Owner>>,
    pub description: Option<String>,
    pub labels: Option<Vec<Label>>,
    pub status: Option<String>,
    pub publication_category: Option<String>,
    pub begin_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub goal_and_impact: Option<String>,
    pub considerations: Option<String>,
    pub risk_management: Option<String>,
    pub human_intervention: Option<String>,
    pub legal_base: Option<Vec<LegalBase>>,
    pub used_data: Option<String>,
    pub technical_design: Option<String>,
    pub external_providers: Option<Vec<String>>,
    pub references: Option<Vec<String>>,
    pub models: Option<Vec<ModelCard>>,
    pub assessments: Option<Vec<AssessmentCard>>,
}

impl SystemCard {
    pub fn new() -> Self {
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

    pub fn from_algoritmeregister_record(record: &AlgoritmeregisterRecord) -> Self {
        let owner = Owner {
            oin: None,
            organization: record.organization.clone(),
            name: None,
            email: record.contact_email.clone(),
            role: None,
        };

        let begin_date = if let Some(begin_date) = record.begin_date.as_ref() {
            let begin_date = format!("{}-01", &begin_date);
            match NaiveDate::parse_from_str(&begin_date, "%Y-%m-%d") {
                Ok(begin_date) => Some(begin_date),
                Err(_) => None,
            }
        } else {
            None
        };

        let end_date = if let Some(end_date) = record.end_date.as_ref() {
            let end_date = format!("{}-01", &end_date);
            match NaiveDate::parse_from_str(&end_date, "%Y-%m-%d") {
                Ok(end_date) => Some(end_date),
                Err(_) => None,
            }
        } else {
            None
        };

        let mut labels = Vec::with_capacity(5);

        if let Some(category) = record.category.as_ref() {
            labels.push(Label {
                name: Some("category".to_string()),
                value: Some(category.clone()),
            });
        };

        if let Some(tags) = record.tags.as_ref() {
            labels.push(Label {
                name: Some("tags".to_string()),
                value: Some(tags.clone()),
            });
        };

        if let Some(lawful_basis) = record.lawful_basis.as_ref() {
            labels.push(Label {
                name: Some("lawful_basis".to_string()),
                value: Some(lawful_basis.clone()),
            });
        };

        if let Some(impacttoetsen) = record.impacttoetsen.as_ref() {
            labels.push(Label {
                name: Some("impacttoetsen".to_string()),
                value: Some(impacttoetsen.clone()),
            });
        };

        if let Some(record_impacttoetsen_grouping) = record.impacttoetsen_grouping.as_ref() {
            labels.push(Label {
                name: Some("impacttoetsen_grouping".to_string()),
                value: Some(record_impacttoetsen_grouping.clone()),
            });
        };

        let re = Regex::new(r"\d+: (.+?), (https:\/\/\S+)(?:\.|$)").unwrap(); // is valid regex so we can
                                                                              // unwrap
        let mut legal_base = Vec::new();
        if let Some(lawful_basis_grouping) = record.lawful_basis_grouping.as_ref() {
            for (_, [name, link]) in re.captures_iter(lawful_basis_grouping).map(|c| c.extract()) {
                legal_base.push(LegalBase {
                    name: Some(name.to_string()),
                    link: Some(link.to_string()),
                })
            }
        };

        let mut external_providers = Vec::with_capacity(1);
        if let Some(provider) = record.provider.as_ref() {
            external_providers.push(provider.to_string());
        }

        let mut references = Vec::with_capacity(5);
        if let Some(publiccode) = record.publiccode.as_ref() {
            references.push(publiccode.to_string());
        }
        if let Some(website) = record.website.as_ref() {
            references.push(website.to_string());
        }
        if let Some(url) = record.url.as_ref() {
            references.push(url.to_string());
        }
        if let Some(source_data_link) = record.source_data_link.as_ref() {
            references.push(source_data_link.to_string());
        }
        if let Some(process_index_url) = record.process_index_url.as_ref() {
            references.push(process_index_url.to_string());
        }

        SystemCard {
            version: "0.1a3".to_string(),
            name: record.name.clone(),
            upl: None,
            owners: Some(vec![owner]),
            description: record.description_short.clone(),
            labels: Some(labels),
            status: record.status.clone(),
            publication_category: record.publication_category.clone(),
            begin_date,
            end_date,
            goal_and_impact: record.goal.clone(),
            considerations: record.proportionality.clone(),
            risk_management: record.risks.clone(),
            human_intervention: record.human_intervention.clone(),
            legal_base: Some(legal_base),
            used_data: record.source_data.clone(),
            technical_design: record.methods_and_models.clone(),
            external_providers: Some(external_providers),
            references: Some(references),
            models: None,
            assessments: None,
        }
    }

    pub fn save(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let output = serde_yml::to_string(&self).context("Could not write into yaml format.")?;
        let mut file = File::create(path).context("Could not create file.")?;
        file.write_all(&output.as_bytes())
            .context("Could not wiret contents to file")?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Label {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LegalBase {
    pub name: Option<String>,
    pub link: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Owner {
    pub oin: Option<String>,
    pub organization: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ModelCard {}

#[derive(Serialize, Deserialize)]
struct AssessmentCard {}
