use anyhow::Context;
use chrono::naive::NaiveDate;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::algoritmeregister::AlgoritmeregisterRecord;

#[derive(Serialize, Deserialize)]
pub struct SystemCard {
    pub version: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub upl: Option<String>,

    #[serde(skip_serializing_if = "Owners::is_empty")]
    pub owners: Owners,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Labels::is_empty")]
    pub labels: Labels,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub publication_category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_date: Option<NaiveDate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub goal_and_impact: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub considerations: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_management: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_intervention: Option<String>,

    #[serde(skip_serializing_if = "LegalBases::is_empty")]
    pub legal_base: LegalBases,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub technical_design: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub external_providers: Vec<String>,

    #[serde(skip_serializing_if = "References::is_empty")]
    pub references: References,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<ModelCard>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessments: Option<Vec<AssessmentCard>>,
}

impl From<&AlgoritmeregisterRecord> for SystemCard {
    fn from(item: &AlgoritmeregisterRecord) -> Self {
        // For now we only accept dates in format YYYY-MM from the algoritmeregister. If
        // date is not in this format we set begin_date or end_date to None.
        let begin_date = if let Some(begin_date) = item.begin_date.as_ref() {
            parse_date(begin_date).ok()
        } else {
            None
        };
        let end_date = if let Some(end_date) = item.end_date.as_ref() {
            parse_date(end_date).ok()
        } else {
            None
        };

        // Populate external_providers vector. There is a maximum of 1 external provider in the
        // algoritmeregister.
        let mut external_providers = Vec::with_capacity(1);

        if let Some(provider) = item.provider.as_ref() {
            external_providers.push(provider.to_string());
        }

        // Populate references vector.

        SystemCard {
            version: "0.1a3".to_string(),
            name: item.name.clone(),
            upl: None,
            owners: Owners::from(item),
            description: item.description_short.clone(),
            labels: Labels::from(item),
            status: item.status.clone(),
            publication_category: item.publication_category.clone(),
            begin_date,
            end_date,
            goal_and_impact: item.goal.clone(),
            considerations: item.proportionality.clone(),
            risk_management: item.risks.clone(),
            human_intervention: item.human_intervention.clone(),
            legal_base: LegalBases::from(item),
            used_data: item.source_data.clone(),
            technical_design: item.methods_and_models.clone(),
            external_providers,
            references: References::from(item),
            models: None,
            assessments: None,
        }
    }
}

impl SystemCard {
    pub fn save(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let output =
            serde_yml::to_string(&self).context("Could not serialize SystemCard as yaml.")?;
        let mut file = File::create(path).context("Could not create file.")?;
        file.write_all(&output.as_bytes())
            .context("Could not write contents to file")?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Label {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

type Labels = Vec<Label>;

impl From<&AlgoritmeregisterRecord> for Labels {
    fn from(item: &AlgoritmeregisterRecord) -> Self {
        let mut labels = Vec::with_capacity(5);

        if let Some(category) = item.category.as_ref() {
            labels.push(Label {
                name: Some("category".to_string()),
                value: Some(category.clone()),
            });
        };

        if let Some(tags) = item.tags.as_ref() {
            labels.push(Label {
                name: Some("tags".to_string()),
                value: Some(tags.clone()),
            });
        };

        if let Some(lawful_basis) = item.lawful_basis.as_ref() {
            labels.push(Label {
                name: Some("lawful_basis".to_string()),
                value: Some(lawful_basis.clone()),
            });
        };

        if let Some(impacttoetsen) = item.impacttoetsen.as_ref() {
            labels.push(Label {
                name: Some("impacttoetsen".to_string()),
                value: Some(impacttoetsen.clone()),
            });
        };

        if let Some(record_impacttoetsen_grouping) = item.impacttoetsen_grouping.as_ref() {
            labels.push(Label {
                name: Some("impacttoetsen_grouping".to_string()),
                value: Some(record_impacttoetsen_grouping.clone()),
            });
        };

        labels
    }
}

#[derive(Serialize, Deserialize)]
pub struct LegalBase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

type LegalBases = Vec<LegalBase>;

impl From<&AlgoritmeregisterRecord> for LegalBases {
    fn from(item: &AlgoritmeregisterRecord) -> Self {
        // Regex is valid so we can safely use unwrap.
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"\d+: (.+?), (https:\/\/\S+)(?:\.|$)").unwrap());

        let mut legal_base = Vec::new();
        if let Some(input) = item.lawful_basis_grouping.as_ref() {
            for (_, [name, link]) in RE.captures_iter(input).map(|c| c.extract()) {
                legal_base.push(LegalBase {
                    name: Some(name.to_string()),
                    link: Some(link.to_string()),
                })
            }
        }
        legal_base
    }
}

type References = Vec<String>;

impl From<&AlgoritmeregisterRecord> for References {
    fn from(item: &AlgoritmeregisterRecord) -> Self {
        let mut references = Vec::with_capacity(5);

        if let Some(publiccode) = item.publiccode.as_ref() {
            references.push(publiccode.clone());
        }
        if let Some(website) = item.website.as_ref() {
            references.push(website.clone());
        }
        if let Some(url) = item.url.as_ref() {
            references.push(url.clone());
        }
        if let Some(source_data_link) = item.source_data_link.as_ref() {
            references.push(source_data_link.clone());
        }
        if let Some(process_index_url) = item.process_index_url.as_ref() {
            references.push(process_index_url.clone());
        }
        references
    }
}

#[derive(Serialize, Deserialize)]
pub struct Owner {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oin: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

type Owners = Vec<Owner>;

impl From<&AlgoritmeregisterRecord> for Owners {
    fn from(item: &AlgoritmeregisterRecord) -> Self {
        let mut owners = Vec::with_capacity(1);

        owners.push(Owner {
            oin: None,
            organization: item.organization.clone(),
            name: None,
            email: item.contact_email.clone(),
            role: None,
        });

        owners
    }
}

// TODO: implement ModelCard.
#[derive(Serialize, Deserialize)]
pub struct ModelCard {}

// TODO: implement AssessmentCard.
#[derive(Serialize, Deserialize)]
pub struct AssessmentCard {}

fn parse_date(date: &str) -> anyhow::Result<NaiveDate> {
    /* Parses a date from the format used in the algoritmeregister YYYY-MM into a NaiveDate.
     * Sets the begin day to the first of the month, so YYYY-DD is converted to YYYY-DD-01.
     */
    let date = format!("{}-01", &date);
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")?;
    Ok(date)
}
