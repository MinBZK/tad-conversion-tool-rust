use chrono::naive::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SystemCard {
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
}

#[derive(Serialize, Deserialize)]
pub struct Labels {
    name: Option<String>,
    value: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LegalBase {
    name: Option<String>,
    link: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Owners {
    oin: Option<String>,
    organization: Option<String>,
    name: Option<String>,
    email: Option<String>,
    role: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ModelCard {}

#[derive(Serialize, Deserialize)]
struct AssessmentCard {}
