# clinicaltrials-gov-rs

Async Rust client for the [ClinicalTrials.gov API v2](https://clinicaltrials.gov/data-about-studies/learn-about-api). All v2 endpoints. JSON, CSV, FHIR, and RIS formats. Essie expression search. Distance-based geographic filtering.

Generated from the official OpenAPI 3.0.3 spec. Every parameter in the spec is a parameter on the Rust function — the signatures are wide and positional.

## Install

```toml
[dependencies]
clinicaltrials-gov-api = "0.1.0"
```

## Quick Start

```rust
use clinicaltrials_gov_api::apis::{configuration::Configuration, studies_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::default();
    let studies = studies_api::list_studies(
        &config,
        Some("json"), None, Some("lung cancer"),
        // remaining OpenAPI parameters default to None
        None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None,
        None, None, None, Some(true), Some(10), None,
    ).await?;
    println!("Found {} studies", studies.studies.len());
    Ok(())
}
```

Run `cargo doc --open` for the full signatures.

## Healthcare examples

**Phase 3 oncology trials within 50 km of Helsinki, as FHIR:**

```rust
let studies = studies_api::list_studies(
    &config,
    Some("fhir.json"), None, Some("cancer"), Some("phase 3"),
    None, None, None, None, None, None, None, None,
    Some("distance(60.1699,24.9384,50km)"),
    None, None, None, None, None, None, None, None, None, None,
    None, None, Some(true), Some(50), None,
).await?;
```

**Currently recruiting COVID-19 vaccine studies:**

```rust
let studies = studies_api::list_studies(
    &config,
    Some("json"), None, Some("COVID-19"), Some("vaccine"),
    None, None, None, None, None, None, None,
    Some(vec!["RECRUITING".to_string()]),
    None, None, None, None, None, None, None, None, None,
    None, None, None, Some(true), Some(20), None,
).await?;
```

**Single study by NCT ID:**

```rust
let study = studies_api::fetch_study(
    &config, "NCT04000165", Some("json"), None, None,
).await?;
```

## Endpoints

All URIs relative to `https://clinicaltrials.gov/api/v2`.

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

See [MODELS.md](MODELS.md) for the model list.

## Develop

Requires Rust 1.70+ and Node.js for the OpenAPI Generator CLI.

```bash
make install    # install tools, generate client
make generate   # regenerate from OpenAPI spec
make compile
make test
```

See [ADR-001](adrs/01-generate-client.md) for the generation rationale.

## Links

- [ClinicalTrials.gov API docs](https://clinicaltrials.gov/data-about-studies/learn-about-api)
- [OpenAPI spec](ctg-oas-v2.yaml)
- [Architecture decisions](adrs/)

## License

MIT — see [LICENSE](LICENSE).

---

Built by [Aktagon](https://aktagon.com). Applied AI for regulated markets. Commercial support: [christian@aktagon.com](mailto:christian@aktagon.com).
