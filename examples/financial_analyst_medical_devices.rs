use clinicaltrials_gov_api::apis::configuration::Configuration;
use clinicaltrials_gov_api::apis::studies_api;
use clinicaltrials_gov_api::models::Status;
use std::collections::HashMap;

const LARGE_PAGE_SIZE: i32 = 50;
const MEDIUM_PAGE_SIZE: i32 = 30;
const SMALL_PAGE_SIZE: i32 = 25;
const TINY_PAGE_SIZE: i32 = 20;

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

    fn condition(mut self, cond: &str) -> Self {
        self.query_cond = Some(cond.to_string());
        self
    }

    fn term(mut self, term: &str) -> Self {
        self.query_term = Some(term.to_string());
        self
    }

    fn location(mut self, location: &str) -> Self {
        self.query_locn = Some(location.to_string());
        self
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

/// Counts the number of trials by phase for pipeline strength assessment
fn count_phases(studies: &[clinicaltrials_gov_api::models::Study]) -> HashMap<String, i32> {
    let mut phase_counts = HashMap::new();
    for study in studies {
        if let Some(protocol) = &study.protocol_section {
            if let Some(design) = &protocol.design_module {
                if let Some(phases) = &design.phases {
                    for phase in phases {
                        *phase_counts.entry(phase.to_string()).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    phase_counts
}

/// Counts trials that involve medical devices by checking intervention types
fn count_device_trials(studies: &[clinicaltrials_gov_api::models::Study]) -> i32 {
    let mut device_trials = 0;
    for study in studies {
        if let Some(protocol) = &study.protocol_section {
            if let Some(arms_interventions) = &protocol.arms_interventions_module {
                if let Some(interventions) = &arms_interventions.interventions {
                    for intervention in interventions {
                        if let Some(intervention_type) = &intervention.r#type {
                            if intervention_type
                                .to_string()
                                .to_lowercase()
                                .contains("device")
                            {
                                device_trials += 1;
                                break; // Only count each study once
                            }
                        }
                    }
                }
            }
        }
    }
    device_trials
}

/// Extracts industry sponsor names for competitive landscape analysis
fn extract_industry_sponsors(studies: &[clinicaltrials_gov_api::models::Study]) -> Vec<String> {
    let mut industry_sponsors = Vec::new();
    for study in studies {
        if let Some(protocol) = &study.protocol_section {
            if let Some(sponsors) = &protocol.sponsor_collaborators_module {
                if let Some(lead_sponsor) = &sponsors.lead_sponsor {
                    if let Some(name) = &lead_sponsor.name {
                        if let Some(class) = &lead_sponsor.class {
                            // Filter for industry-sponsored trials only
                            if class.to_string().to_lowercase().contains("industry") {
                                industry_sponsors.push(name.clone());
                            }
                        }
                    }
                }
            }
        }
    }
    industry_sponsors
}

fn calculate_total_enrollment(studies: &[clinicaltrials_gov_api::models::Study]) -> i32 {
    let mut total = 0;
    for study in studies {
        if let Some(protocol) = &study.protocol_section {
            if let Some(design) = &protocol.design_module {
                if let Some(enrollment) = &design.enrollment_info {
                    if let Some(count) = enrollment.count {
                        total += count;
                    }
                }
            }
        }
    }
    total
}

fn analyze_state_distribution(
    studies: &[clinicaltrials_gov_api::models::Study],
) -> HashMap<String, i32> {
    let mut state_counts = HashMap::new();
    for study in studies {
        if let Some(protocol) = &study.protocol_section {
            if let Some(contacts_locations) = &protocol.contacts_locations_module {
                if let Some(locations) = &contacts_locations.locations {
                    for location in locations {
                        if let Some(state) = &location.state {
                            if let Some(country) = &location.country {
                                if country == "United States" {
                                    *state_counts.entry(state.clone()).or_insert(0) += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    state_counts
}

/// Finds recently completed trials (2023-2024) for competitive intelligence
/// Returns (total_completed_count, recent_completions_with_dates)
fn find_recent_completions(
    studies: &[clinicaltrials_gov_api::models::Study],
) -> (i32, Vec<(String, String)>) {
    let mut completed_trials = 0;
    let mut recent_completions = Vec::new();

    for study in studies {
        if let Some(protocol) = &study.protocol_section {
            if let Some(status_module) = &protocol.status_module {
                if let Some(overall_status) = &status_module.overall_status {
                    if *overall_status == Status::Completed {
                        completed_trials += 1;

                        // Check if completion was recent (2023-2024)
                        if let Some(completion_date) = &status_module.completion_date_struct {
                            if let Some(date) = &completion_date.date {
                                if date.starts_with("2023") || date.starts_with("2024") {
                                    if let Some(identification) = &protocol.identification_module {
                                        if let Some(title) = &identification.brief_title {
                                            recent_completions.push((title.clone(), date.clone()));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    (completed_trials, recent_completions)
}

/// Prints top sponsors by trial count, sorted by frequency
fn print_top_sponsors(sponsors: &[String], limit: usize) {
    // Count sponsor frequency using fold for functional style
    let sponsor_counts: HashMap<_, _> = sponsors.iter().fold(HashMap::new(), |mut acc, sponsor| {
        *acc.entry(sponsor).or_insert(0) += 1;
        acc
    });

    let mut sorted_sponsors: Vec<_> = sponsor_counts.iter().collect();
    sorted_sponsors.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count descending

    for (sponsor, count) in sorted_sponsors.iter().take(limit) {
        println!("     {}: {} trials", sponsor, count);
    }
}

fn print_top_states(state_counts: &HashMap<String, i32>, limit: usize) {
    let mut sorted_states: Vec<_> = state_counts.iter().collect();
    sorted_states.sort_by(|a, b| b.1.cmp(a.1));

    for (state, count) in sorted_states.iter().take(limit) {
        println!("     {}: {} trial sites", state, count);
    }
}

/// Example: Financial analyst searching for clinical trials related to a major US medical device manufacturer
///
/// This example demonstrates how a financial analyst might use the ClinicalTrials.gov API to:
/// - Search for clinical trials sponsored by major medical device companies
/// - Filter by trial status and phase to assess pipeline strength
/// - Analyze geographic distribution of trials
/// - Extract key financial indicators like trial size and duration
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::default();

    println!("[+] Financial Analysis: Medical Device Clinical Trials");
    println!("==================================================\n");

    // Example 1: Search for trials by major medical device manufacturer (Medtronic)
    println!("[-] Searching for Medtronic-sponsored trials...");
    let medtronic_trials = StudySearchBuilder::new()
        .sponsor("Medtronic")
        .status(vec![Status::Recruiting, Status::ActiveNotRecruiting])
        .page_size(LARGE_PAGE_SIZE)
        .search(&config)
        .await?;

    let studies = &medtronic_trials.studies;
    if !studies.is_empty() {
        println!("[*] Found {} active Medtronic trials", studies.len());

        let phase_counts = count_phases(studies);
        let device_trials = count_device_trials(studies);

        println!("\n[>] Pipeline Analysis:");
        println!("   Device-related trials: {}", device_trials);
        println!("   Phase breakdown:");
        for (phase, count) in &phase_counts {
            println!("     {}: {} trials", phase, count);
        }
    }

    // Example 2: Search for cardiovascular device trials (high-value market segment)
    println!("\n[^] Analyzing cardiovascular device market...");
    let cardio_device_trials = StudySearchBuilder::new()
        .condition("cardiovascular disease OR heart disease OR cardiac")
        .term("device OR implant OR stent OR pacemaker")
        .status(vec![Status::Recruiting, Status::ActiveNotRecruiting])
        .page_size(SMALL_PAGE_SIZE)
        .search(&config)
        .await?;

    let studies = &cardio_device_trials.studies;
    if !studies.is_empty() {
        println!(
            "[*] Found {} late-stage cardiovascular device trials",
            studies.len()
        );

        let industry_sponsors = extract_industry_sponsors(studies);
        let enrollment_total = calculate_total_enrollment(studies);

        println!("\n[$] Market Intelligence:");
        println!("   Industry-sponsored trials: {}", industry_sponsors.len());
        println!("   Total patient enrollment: {} patients", enrollment_total);
        println!("   Top industry sponsors:");

        print_top_sponsors(&industry_sponsors, 5);
    }

    // Example 3: Geographic analysis for market penetration assessment
    println!("\n[~] Geographic market analysis...");
    let us_trials = StudySearchBuilder::new()
        .term("device OR medical device")
        .location("United States")
        .status(vec![Status::Recruiting])
        .page_size(MEDIUM_PAGE_SIZE)
        .search(&config)
        .await?;

    let studies = &us_trials.studies;
    if !studies.is_empty() {
        println!(
            "[*] Found {} US-based device trials currently recruiting",
            studies.len()
        );

        let state_counts = analyze_state_distribution(studies);

        println!("\n[@] Top US states by trial activity:");
        print_top_states(&state_counts, 10);
    }

    // Example 4: Competitive intelligence - search for specific device categories
    println!("\n[?] Competitive analysis: Orthopedic devices...");
    let ortho_trials = StudySearchBuilder::new()
        .condition("orthopedic OR joint replacement OR knee replacement OR hip replacement")
        .term("device OR implant OR prosthesis")
        .status(vec![
            Status::Recruiting,
            Status::ActiveNotRecruiting,
            Status::Completed,
        ])
        .page_size(TINY_PAGE_SIZE)
        .search(&config)
        .await?;

    let studies = &ortho_trials.studies;
    if !studies.is_empty() {
        println!("[*] Found {} orthopedic device trials", studies.len());

        let (completed_trials, recent_completions) = find_recent_completions(studies);

        println!("\n[#] Competitive Intelligence:");
        println!("   Completed trials: {}", completed_trials);
        println!(
            "   Recent completions (2023-2024): {}",
            recent_completions.len()
        );

        if !recent_completions.is_empty() {
            println!("\n   Recently completed studies to monitor:");
            for (title, date) in recent_completions.iter().take(5) {
                println!("     - {} ({})", title, date);
            }
        }
    }

    println!("\n[=] Financial Analysis Summary:");
    println!("=====================================");
    println!("[+] Pipeline strength assessment completed");
    println!("[+] Market size estimation performed");
    println!("[+] Competitive landscape analyzed");
    println!("[+] Geographic market penetration mapped");
    println!("[+] Recent trial completions identified for results monitoring");

    println!("\n[!] Analyst Recommendations:");
    println!("- Monitor Phase 3 cardiovascular device trials for upcoming market entries");
    println!("- Track Medtronic's trial progress for competitive positioning");
    println!("- Focus on high-enrollment trials in key therapeutic areas");
    println!("- Watch for trial results from recently completed studies");

    Ok(())
}
