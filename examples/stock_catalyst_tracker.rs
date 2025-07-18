use clinicaltrials_gov_api::apis::configuration::Configuration;
use clinicaltrials_gov_api::apis::studies_api;
use clinicaltrials_gov_api::models::Status;
use chrono::{Utc, NaiveDate};
use std::collections::HashMap;

const LARGE_PAGE_SIZE: i32 = 100;

#[derive(Debug, Clone)]
struct CatalystEvent {
    company: String,
    trial_title: String,
    nct_id: String,
    catalyst_type: CatalystType,
    expected_date: String,
    market_impact: MarketImpact,
    enrollment_size: Option<i32>,
    phase: Option<String>,
    indication: String,
}

#[derive(Debug, Clone)]
enum CatalystType {
    PrimaryCompletion,
    StudyCompletion,
}

#[derive(Debug, Clone)]
enum MarketImpact {
    High,    // Large Phase 3, major indication, >$1B market
    Medium,  // Phase 2/3, significant market opportunity
    Low,     // Early phase, niche indication
}

#[derive(Default)]
struct StudySearchBuilder {
    format: Option<String>,
    query_cond: Option<String>,
    query_term: Option<String>,
    query_locn: Option<String>,
    query_spons: Option<String>,
    filter_overall_status: Option<Vec<Status>>,
    count_total: Option<bool>,
    page_size: Option<i32>,
}

impl StudySearchBuilder {
    fn new() -> Self {
        Self {
            format: Some("json".to_string()),
            count_total: Some(true),
            ..Default::default()
        }
    }


    fn sponsor(mut self, sponsor: &str) -> Self {
        self.query_spons = Some(sponsor.to_string());
        self
    }

    fn status(mut self, status: Vec<Status>) -> Self {
        self.filter_overall_status = Some(status);
        self
    }

    fn page_size(mut self, size: i32) -> Self {
        self.page_size = Some(size);
        self
    }

    async fn search(
        self,
        config: &Configuration,
    ) -> Result<clinicaltrials_gov_api::models::PagedStudies, Box<dyn std::error::Error>> {
        let result = studies_api::list_studies(
            config,
            self.format.as_deref(),
            None,
            self.query_cond.as_deref(),
            self.query_term.as_deref(),
            self.query_locn.as_deref(),
            None,
            None,
            None,
            self.query_spons.as_deref(),
            None,
            None,
            None,
            self.filter_overall_status,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            self.count_total,
            self.page_size,
            None,
        )
        .await;

        match result {
            Ok(studies) => Ok(studies),
            Err(e) => Err(Box::new(e)),
        }
    }
}

/// Identifies upcoming catalyst events that could move stock prices
fn find_upcoming_catalysts(studies: &[clinicaltrials_gov_api::models::Study]) -> Vec<CatalystEvent> {
    let mut catalysts = Vec::new();
    
    for study in studies {
        if let Some(protocol) = &study.protocol_section {
            let mut catalyst_event = CatalystEvent {
                company: extract_sponsor_name(study).unwrap_or("Unknown").to_string(),
                trial_title: extract_trial_title(study).unwrap_or("Unknown").to_string(),
                nct_id: extract_nct_id(study).unwrap_or("Unknown").to_string(),
                catalyst_type: CatalystType::PrimaryCompletion,
                expected_date: "TBD".to_string(),
                market_impact: MarketImpact::Low,
                enrollment_size: extract_enrollment_size(study),
                phase: extract_phase(study),
                indication: extract_indication(study).unwrap_or("Unknown").to_string(),
            };

            // Check for primary completion date
            if let Some(status_module) = &protocol.status_module {
                if let Some(primary_completion) = &status_module.primary_completion_date_struct {
                    if let Some(date) = &primary_completion.date {
                        catalyst_event.expected_date = date.clone();
                        catalyst_event.catalyst_type = CatalystType::PrimaryCompletion;
                        
                        // Assess market impact
                        catalyst_event.market_impact = assess_market_impact(study);
                        
                        // Only include if within next 2 years and high/medium impact
                        if is_within_timeframe(date, 24) && 
                           matches!(catalyst_event.market_impact, MarketImpact::High | MarketImpact::Medium) {
                            catalysts.push(catalyst_event.clone());
                        }
                    }
                }
                
                // Check for study completion date if no primary completion
                if catalyst_event.expected_date == "TBD" {
                    if let Some(completion) = &status_module.completion_date_struct {
                        if let Some(date) = &completion.date {
                            catalyst_event.expected_date = date.clone();
                            catalyst_event.catalyst_type = CatalystType::StudyCompletion;
                            catalyst_event.market_impact = assess_market_impact(study);
                            
                            if is_within_timeframe(date, 24) && 
                               matches!(catalyst_event.market_impact, MarketImpact::High | MarketImpact::Medium) {
                                catalysts.push(catalyst_event);
                            }
                        }
                    }
                }
            }
        }
    }
    
    catalysts
}

/// Assesses the potential market impact of a trial
fn assess_market_impact(study: &clinicaltrials_gov_api::models::Study) -> MarketImpact {
    let mut score = 0;
    
    // Phase scoring
    if let Some(phase) = extract_phase(study) {
        match phase.as_str() {
            "PHASE3" => score += 3,
            "PHASE2" => score += 2,
            "PHASE1" => score += 1,
            _ => score += 0,
        }
    }
    
    // Enrollment size scoring
    if let Some(enrollment) = extract_enrollment_size(study) {
        if enrollment > 1000 {
            score += 3;
        } else if enrollment > 300 {
            score += 2;
        } else if enrollment > 100 {
            score += 1;
        }
    }
    
    // Indication scoring (major therapeutic areas)
    if let Some(indication) = extract_indication(study) {
        let indication_lower = indication.to_lowercase();
        if indication_lower.contains("cardiovascular") || 
           indication_lower.contains("oncology") || 
           indication_lower.contains("diabetes") ||
           indication_lower.contains("alzheimer") {
            score += 2;
        }
    }
    
    // Market impact classification
    match score {
        7..=10 => MarketImpact::High,
        4..=6 => MarketImpact::Medium,
        _ => MarketImpact::Low,
    }
}

/// Checks if a date is within the specified number of months from now
fn is_within_timeframe(date_str: &str, months: i32) -> bool {
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let now = Utc::now().naive_utc().date();
        let future_date = now + chrono::Duration::days((months * 30) as i64);
        date <= future_date && date >= now
    } else {
        false
    }
}

/// Identifies trials with recent status changes that could indicate imminent catalysts
fn detect_status_changes(studies: &[clinicaltrials_gov_api::models::Study]) -> Vec<String> {
    let mut status_alerts = Vec::new();
    
    for study in studies {
        if let Some(protocol) = &study.protocol_section {
            if let Some(status_module) = &protocol.status_module {
                if let Some(status) = &status_module.overall_status {
                    if let Some(last_update) = &status_module.last_update_post_date_struct {
                        if let Some(date) = &last_update.date {
                            if is_within_timeframe(date, 1) { // Within last month
                                let nct_id = extract_nct_id(study).unwrap_or("Unknown");
                                let title = extract_trial_title(study).unwrap_or("Unknown");
                                
                                match status {
                                    Status::ActiveNotRecruiting => {
                                        status_alerts.push(format!("[STATUS] {} - Enrollment Complete: {}", nct_id, title));
                                    },
                                    Status::Completed => {
                                        status_alerts.push(format!("[COMPLETE] {} - Study Completed: {}", nct_id, title));
                                    },
                                    Status::Suspended => {
                                        status_alerts.push(format!("[SUSPEND] {} - Study Suspended: {}", nct_id, title));
                                    },
                                    Status::Terminated => {
                                        status_alerts.push(format!("[TERM] {} - Study Terminated: {}", nct_id, title));
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    status_alerts
}

/// Tracks pipeline value by company
fn calculate_pipeline_value(catalysts: &[CatalystEvent]) -> HashMap<String, (i32, i32, i32)> {
    let mut pipeline_value = HashMap::new();
    
    for catalyst in catalysts {
        let entry = pipeline_value.entry(catalyst.company.clone()).or_insert((0, 0, 0));
        match catalyst.market_impact {
            MarketImpact::High => entry.0 += 1,
            MarketImpact::Medium => entry.1 += 1,
            MarketImpact::Low => entry.2 += 1,
        }
    }
    
    pipeline_value
}

// Helper functions for data extraction
fn extract_sponsor_name(study: &clinicaltrials_gov_api::models::Study) -> Option<&str> {
    study.protocol_section.as_ref()?
        .sponsor_collaborators_module.as_ref()?
        .lead_sponsor.as_ref()?
        .name.as_deref()
}

fn extract_trial_title(study: &clinicaltrials_gov_api::models::Study) -> Option<&str> {
    study.protocol_section.as_ref()?
        .identification_module.as_ref()?
        .brief_title.as_deref()
}

fn extract_nct_id(study: &clinicaltrials_gov_api::models::Study) -> Option<&str> {
    study.protocol_section.as_ref()?
        .identification_module.as_ref()?
        .nct_id.as_deref()
}

fn extract_enrollment_size(study: &clinicaltrials_gov_api::models::Study) -> Option<i32> {
    study.protocol_section.as_ref()?
        .design_module.as_ref()?
        .enrollment_info.as_ref()?
        .count
}

fn extract_phase(study: &clinicaltrials_gov_api::models::Study) -> Option<String> {
    study.protocol_section.as_ref()?
        .design_module.as_ref()?
        .phases.as_ref()?
        .get(0)
        .map(|p| p.to_string())
}

fn extract_indication(study: &clinicaltrials_gov_api::models::Study) -> Option<&str> {
    study.protocol_section.as_ref()?
        .conditions_module.as_ref()?
        .conditions.as_ref()?
        .get(0)
        .map(|s| s.as_str())
}

/// Example: Stock catalyst tracker for biotech and medical device companies
/// 
/// This example demonstrates how to identify upcoming clinical trial events
/// that could significantly impact stock prices:
/// - Primary endpoint readouts
/// - Study completions
/// - Enrollment milestones
/// - Regulatory submission timelines
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::default();

    println!("[*] Stock Catalyst Tracker: Clinical Trial Events");
    println!("==============================================\n");

    // Track major biotech companies
    let companies = vec![
        "Moderna", "BioNTech", "Pfizer", "Johnson & Johnson", 
        "Merck", "AbbVie", "Gilead", "Amgen", "Biogen"
    ];

    let mut all_catalysts = Vec::new();

    for company in &companies {
        println!("[>] Analyzing {} pipeline...", company);
        
        let company_trials = StudySearchBuilder::new()
            .sponsor(company)
            .status(vec![Status::Recruiting, Status::ActiveNotRecruiting])
            .page_size(LARGE_PAGE_SIZE)
            .search(&config)
            .await?;

        let catalysts = find_upcoming_catalysts(&company_trials.studies);
        all_catalysts.extend(catalysts);

        // Check for recent status changes
        let status_changes = detect_status_changes(&company_trials.studies);
        if !status_changes.is_empty() {
            println!("   [+] Recent Status Changes:");
            for change in status_changes.iter().take(3) {
                println!("     {}", change);
            }
        }
    }

    // Sort catalysts by expected date
    all_catalysts.sort_by(|a, b| a.expected_date.cmp(&b.expected_date));

    println!("\n[!] HIGH-IMPACT CATALYSTS (Next 12 Months):");
    println!("==========================================");
    
    let high_impact_catalysts: Vec<_> = all_catalysts.iter()
        .filter(|c| matches!(c.market_impact, MarketImpact::High))
        .filter(|c| is_within_timeframe(&c.expected_date, 12))
        .collect();

    for catalyst in high_impact_catalysts.iter().take(10) {
        println!("[HIGH] {} - {} ({})", 
                 catalyst.expected_date, 
                 catalyst.company, 
                 catalyst.phase.as_ref().unwrap_or(&"Unknown".to_string()));
        println!("   Title: {}", catalyst.trial_title);
        println!("   Enrollment: {} patients", 
                 catalyst.enrollment_size.unwrap_or(0));
        println!("   Indication: {}", catalyst.indication);
        println!("   NCT ID: {}\n", catalyst.nct_id);
    }

    println!("[~] MEDIUM-IMPACT CATALYSTS (Next 6 Months):");
    println!("===========================================");
    
    let medium_impact_catalysts: Vec<_> = all_catalysts.iter()
        .filter(|c| matches!(c.market_impact, MarketImpact::Medium))
        .filter(|c| is_within_timeframe(&c.expected_date, 6))
        .collect();

    for catalyst in medium_impact_catalysts.iter().take(5) {
        println!("[MED] {} - {} ({})", 
                 catalyst.expected_date, 
                 catalyst.company, 
                 catalyst.phase.as_ref().unwrap_or(&"Unknown".to_string()));
        println!("   Title: {}", catalyst.trial_title);
        println!("   NCT ID: {}\n", catalyst.nct_id);
    }

    // Pipeline value analysis
    let pipeline_values = calculate_pipeline_value(&all_catalysts);
    
    println!("[$] PIPELINE VALUE ANALYSIS:");
    println!("===========================");
    
    let mut sorted_companies: Vec<_> = pipeline_values.iter().collect();
    sorted_companies.sort_by(|a, b| {
        let score_a = a.1.0 * 3 + a.1.1 * 2 + a.1.2;
        let score_b = b.1.0 * 3 + b.1.1 * 2 + b.1.2;
        score_b.cmp(&score_a)
    });

    for (company, (high, medium, low)) in sorted_companies.iter().take(5) {
        let total_score = high * 3 + medium * 2 + low;
        println!("[PIPELINE] {}: {} catalysts (High: {}, Medium: {}, Low: {}) - Score: {}", 
                 company, high + medium + low, high, medium, low, total_score);
    }

    println!("\n[!!] IMMEDIATE CATALYSTS (Next 30 Days):");
    println!("======================================");
    
    let immediate_catalysts: Vec<_> = all_catalysts.iter()
        .filter(|c| is_within_timeframe(&c.expected_date, 1))
        .collect();

    if immediate_catalysts.is_empty() {
        println!("No immediate catalysts identified in the next 30 days.");
    } else {
        for catalyst in immediate_catalysts.iter().take(5) {
            println!("[URGENT] {} - {} ({})", 
                     catalyst.expected_date, 
                     catalyst.company, 
                     catalyst.phase.as_ref().unwrap_or(&"Unknown".to_string()));
            println!("   Title: {}", catalyst.trial_title);
            println!("   Impact: {:?}\n", catalyst.market_impact);
        }
    }

    println!("[=] SUMMARY:");
    println!("===========");
    println!("Total catalysts tracked: {}", all_catalysts.len());
    println!("High-impact events: {}", all_catalysts.iter().filter(|c| matches!(c.market_impact, MarketImpact::High)).count());
    println!("Medium-impact events: {}", all_catalysts.iter().filter(|c| matches!(c.market_impact, MarketImpact::Medium)).count());
    println!("Companies analyzed: {}", companies.len());

    Ok(())
}