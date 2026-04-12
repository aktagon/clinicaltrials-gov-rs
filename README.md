# clinicaltrials-gov-rs

Rust client for the [ClinicalTrials.gov API v2](https://clinicaltrials.gov/data-about-studies/learn-about-api). Generated from the official OpenAPI 3.0.3 spec. All v2 endpoints. JSON, CSV, FHIR, and RIS output formats. Essie expression search. Distance-based geographic filtering. Async with tokio/reqwest.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
clinicaltrials-gov-api = "0.1.0"
```

## Quick Start

```rust
use clinicaltrials_gov_api::apis::configuration::Configuration;
use clinicaltrials_gov_api::apis::studies_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::default();

    // Search for lung cancer studies
    let studies = studies_api::list_studies(
        &config,
        Some("json"),           // format
        None,                   // markupFormat
        Some("lung cancer"),    // query.cond
        None,                   // Other query parameters...
        None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None,
        None, None, None,
        Some(true),             // countTotal
        Some(10),               // pageSize
        None,                   // pageToken
    ).await?;

    println!("Found {} studies", studies.studies.len());

    Ok(())
}
```

## Development

Requires Rust 1.70+ and Node.js (for OpenAPI Generator CLI).

```bash
make install    # install dependencies and generate client
make generate   # regenerate from OpenAPI spec
make compile    # build
make test       # run tests
make all        # full cycle
```

Generated from the official ClinicalTrials.gov OpenAPI spec using [OpenAPI Generator](https://openapi-generator.tech/). See [ADR-001](adrs/01-generate-client.md).

## Documentation for API Endpoints

All URIs are relative to *https://clinicaltrials.gov/api/v2*

| Class        | Method                                                                | HTTP request                  | Description       |
| ------------ | --------------------------------------------------------------------- | ----------------------------- | ----------------- |
| _StatsApi_   | [**field_values_stats**](docs/StatsApi.md#field_values_stats)         | **GET** /stats/field/values   | Field Values      |
| _StatsApi_   | [**list_field_sizes_stats**](docs/StatsApi.md#list_field_sizes_stats) | **GET** /stats/field/sizes    | List Field Sizes  |
| _StatsApi_   | [**size_stats**](docs/StatsApi.md#size_stats)                         | **GET** /stats/size           | Study Sizes       |
| _StudiesApi_ | [**enums**](docs/StudiesApi.md#enums)                                 | **GET** /studies/enums        | Enums             |
| _StudiesApi_ | [**fetch_study**](docs/StudiesApi.md#fetch_study)                     | **GET** /studies/{nctId}      | Single Study      |
| _StudiesApi_ | [**list_studies**](docs/StudiesApi.md#list_studies)                   | **GET** /studies              | Studies           |
| _StudiesApi_ | [**search_areas**](docs/StudiesApi.md#search_areas)                   | **GET** /studies/search-areas | Search Areas      |
| _StudiesApi_ | [**studies_metadata**](docs/StudiesApi.md#studies_metadata)           | **GET** /studies/metadata     | Data Model Fields |
| _VersionApi_ | [**version**](docs/VersionApi.md#version)                             | **GET** /version              | Version           |

## Models

See [MODELS.md](MODELS.md) for the full list, or run `cargo doc --open`.

## Examples

### Search by Condition

```rust
// Find COVID-19 vaccine studies
let studies = studies_api::list_studies(
    &config,
    Some("json"),
    None,
    Some("COVID-19"),          // condition
    Some("vaccine"),           // other terms
    None, None, None, None, None, None, None,
    Some(vec!["RECRUITING".to_string()]), // only recruiting
    None, None, None, None, None, None, None, None, None,
    None, None, None,
    Some(true),  // count total
    Some(20),    // page size
    None
).await?;
```

### Geographic Search

```rust
// Find studies within 50 miles of NIH (Bethesda, MD)
let studies = studies_api::list_studies(
    &config,
    Some("json"), None, None, None, None, None, None, None, None, None, None,
    None,
    Some("distance(39.0035707,-77.1013313,50mi)"), // geo filter
    None, None, None, None, None, None, None, None, None, None, None,
    Some(true), Some(10), None
).await?;
```

### Get Single Study

```rust
// Get specific study by NCT ID
let study = studies_api::fetch_study(
    &config,
    "NCT04000165",  // nctId
    Some("json"),   // format
    None,           // markupFormat
    None            // fields
).await?;

println!("Study: {}", study.protocol_section.unwrap().identification_module.unwrap().brief_title.unwrap());
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `make test`
5. Submit a pull request

## Support

For questions, issues, or commercial support, contact [Aktagon Ltd.](mailto:christian@aktagon.com)

## Links

- [ClinicalTrials.gov API Documentation](https://clinicaltrials.gov/data-about-studies/learn-about-api)
- [OpenAPI Specification](ctg-oas-v2.yaml)
- [Architecture Decision Records](adrs/)
