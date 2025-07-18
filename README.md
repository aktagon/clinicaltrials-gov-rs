# clinicaltrials-gov-rs

A Rust client library for the [ClinicalTrials.gov API v2](https://clinicaltrials.gov/data-about-studies/learn-about-api).

## Features

- **Type-safe API client** generated from OpenAPI 3.0.3 specification
- **Comprehensive coverage** of all ClinicalTrials.gov API endpoints
- **Multiple response formats** - JSON, CSV, FHIR, RIS, ZIP
- **Advanced search capabilities** with Essie expression syntax
- **Geographic filtering** with distance-based queries
- **Async/await support** with modern Rust patterns

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

### Prerequisites

- Rust 1.70+
- Node.js (for OpenAPI Generator CLI)

### Building

```bash
# Install dependencies and generate client
make install

# Generate Rust client from OpenAPI spec
make generate

# Build the project
make compile

# Run tests
make test

# Full development cycle
make all
```

### Code Generation

This library is automatically generated from the official ClinicalTrials.gov OpenAPI specification using [OpenAPI Generator](https://openapi-generator.tech/). See [ADR-001](adrs/01-generate-client.md) for details.

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

## Documentation For Models

- [AdverseEvent](docs/AdverseEvent.md)
- [AdverseEventsModule](docs/AdverseEventsModule.md)
- [AgencyClass](docs/AgencyClass.md)
- [AgreementRestrictionType](docs/AgreementRestrictionType.md)
- [AnalysisDispersionType](docs/AnalysisDispersionType.md)
- [AnnotationModule](docs/AnnotationModule.md)
- [AnnotationSection](docs/AnnotationSection.md)
- [ArmGroup](docs/ArmGroup.md)
- [ArmGroupType](docs/ArmGroupType.md)
- [ArmsInterventionsModule](docs/ArmsInterventionsModule.md)
- [AvailIpd](docs/AvailIpd.md)
- [BaselineCharacteristicsModule](docs/BaselineCharacteristicsModule.md)
- [BaselineMeasure](docs/BaselineMeasure.md)
- [BioSpec](docs/BioSpec.md)
- [BioSpecRetention](docs/BioSpecRetention.md)
- [BooleanStats](docs/BooleanStats.md)
- [BrowseBranch](docs/BrowseBranch.md)
- [BrowseLeaf](docs/BrowseLeaf.md)
- [BrowseLeafRelevance](docs/BrowseLeafRelevance.md)
- [BrowseModule](docs/BrowseModule.md)
- [CertainAgreement](docs/CertainAgreement.md)
- [ConditionsModule](docs/ConditionsModule.md)
- [ConfidenceIntervalNumSides](docs/ConfidenceIntervalNumSides.md)
- [Contact](docs/Contact.md)
- [ContactRole](docs/ContactRole.md)
- [ContactsLocationsModule](docs/ContactsLocationsModule.md)
- [DateStats](docs/DateStats.md)
- [DateStruct](docs/DateStruct.md)
- [DateType](docs/DateType.md)
- [Denom](docs/Denom.md)
- [DenomCount](docs/DenomCount.md)
- [DerivedSection](docs/DerivedSection.md)
- [DescriptionModule](docs/DescriptionModule.md)
- [DesignAllocation](docs/DesignAllocation.md)
- [DesignInfo](docs/DesignInfo.md)
- [DesignMasking](docs/DesignMasking.md)
- [DesignModule](docs/DesignModule.md)
- [DesignTimePerspective](docs/DesignTimePerspective.md)
- [DistItem](docs/DistItem.md)
- [DocumentSection](docs/DocumentSection.md)
- [DropWithdraw](docs/DropWithdraw.md)
- [EligibilityModule](docs/EligibilityModule.md)
- [EnrollmentInfo](docs/EnrollmentInfo.md)
- [EnrollmentType](docs/EnrollmentType.md)
- [EnumInfo](docs/EnumInfo.md)
- [EnumItem](docs/EnumItem.md)
- [EnumStats](docs/EnumStats.md)
- [EventAssessment](docs/EventAssessment.md)
- [EventGroup](docs/EventGroup.md)
- [EventStats](docs/EventStats.md)
- [ExpandedAccessInfo](docs/ExpandedAccessInfo.md)
- [ExpandedAccessStatus](docs/ExpandedAccessStatus.md)
- [ExpandedAccessTypes](docs/ExpandedAccessTypes.md)
- [FieldNode](docs/FieldNode.md)
- [FieldStatsType](docs/FieldStatsType.md)
- [FieldValuesStats](docs/FieldValuesStats.md)
- [FirstMcpInfo](docs/FirstMcpInfo.md)
- [FlowGroup](docs/FlowGroup.md)
- [FlowMilestone](docs/FlowMilestone.md)
- [FlowPeriod](docs/FlowPeriod.md)
- [FlowStats](docs/FlowStats.md)
- [GeoPoint](docs/GeoPoint.md)
- [GzipStats](docs/GzipStats.md)
- [IdentificationModule](docs/IdentificationModule.md)
- [IntegerStats](docs/IntegerStats.md)
- [Intervention](docs/Intervention.md)
- [InterventionType](docs/InterventionType.md)
- [InterventionalAssignment](docs/InterventionalAssignment.md)
- [IpdSharing](docs/IpdSharing.md)
- [IpdSharingInfoType](docs/IpdSharingInfoType.md)
- [IpdSharingStatementModule](docs/IpdSharingStatementModule.md)
- [LargeDoc](docs/LargeDoc.md)
- [LargeDocumentModule](docs/LargeDocumentModule.md)
- [LimitationsAndCaveats](docs/LimitationsAndCaveats.md)
- [ListSize](docs/ListSize.md)
- [ListSizes](docs/ListSizes.md)
- [Location](docs/Location.md)
- [LongestString](docs/LongestString.md)
- [MaskingBlock](docs/MaskingBlock.md)
- [MeasureAnalysis](docs/MeasureAnalysis.md)
- [MeasureCategory](docs/MeasureCategory.md)
- [MeasureClass](docs/MeasureClass.md)
- [MeasureDispersionType](docs/MeasureDispersionType.md)
- [MeasureGroup](docs/MeasureGroup.md)
- [MeasureParam](docs/MeasureParam.md)
- [Measurement](docs/Measurement.md)
- [Mesh](docs/Mesh.md)
- [MiscInfoModule](docs/MiscInfoModule.md)
- [MoreInfoModule](docs/MoreInfoModule.md)
- [NonInferiorityType](docs/NonInferiorityType.md)
- [NumberStats](docs/NumberStats.md)
- [ObservationalModel](docs/ObservationalModel.md)
- [Official](docs/Official.md)
- [OfficialRole](docs/OfficialRole.md)
- [OrgStudyIdInfo](docs/OrgStudyIdInfo.md)
- [OrgStudyIdType](docs/OrgStudyIdType.md)
- [Organization](docs/Organization.md)
- [Outcome](docs/Outcome.md)
- [OutcomeMeasure](docs/OutcomeMeasure.md)
- [OutcomeMeasureType](docs/OutcomeMeasureType.md)
- [OutcomeMeasuresModule](docs/OutcomeMeasuresModule.md)
- [OutcomesModule](docs/OutcomesModule.md)
- [OversightModule](docs/OversightModule.md)
- [PagedStudies](docs/PagedStudies.md)
- [PartialDateStruct](docs/PartialDateStruct.md)
- [ParticipantFlowModule](docs/ParticipantFlowModule.md)
- [Phase](docs/Phase.md)
- [PointOfContact](docs/PointOfContact.md)
- [PrimaryPurpose](docs/PrimaryPurpose.md)
- [ProtocolSection](docs/ProtocolSection.md)
- [RecruitmentStatus](docs/RecruitmentStatus.md)
- [Reference](docs/Reference.md)
- [ReferenceType](docs/ReferenceType.md)
- [ReferencesModule](docs/ReferencesModule.md)
- [ReportingStatus](docs/ReportingStatus.md)
- [ResponsibleParty](docs/ResponsibleParty.md)
- [ResponsiblePartyType](docs/ResponsiblePartyType.md)
- [ResultsSection](docs/ResultsSection.md)
- [Retraction](docs/Retraction.md)
- [SamplingMethod](docs/SamplingMethod.md)
- [SearchArea](docs/SearchArea.md)
- [SearchDocument](docs/SearchDocument.md)
- [SearchPart](docs/SearchPart.md)
- [SecondaryIdInfo](docs/SecondaryIdInfo.md)
- [SecondaryIdType](docs/SecondaryIdType.md)
- [SeeAlsoLink](docs/SeeAlsoLink.md)
- [Sex](docs/Sex.md)
- [Sponsor](docs/Sponsor.md)
- [SponsorCollaboratorsModule](docs/SponsorCollaboratorsModule.md)
- [StandardAge](docs/StandardAge.md)
- [Status](docs/Status.md)
- [StatusModule](docs/StatusModule.md)
- [StringStats](docs/StringStats.md)
- [Study](docs/Study.md)
- [StudySize](docs/StudySize.md)
- [StudyType](docs/StudyType.md)
- [SubmissionInfo](docs/SubmissionInfo.md)
- [SubmissionTracking](docs/SubmissionTracking.md)
- [UnpostedAnnotation](docs/UnpostedAnnotation.md)
- [UnpostedEvent](docs/UnpostedEvent.md)
- [UnpostedEventType](docs/UnpostedEventType.md)
- [ValueCount](docs/ValueCount.md)
- [Version](docs/Version.md)
- [ViolationAnnotation](docs/ViolationAnnotation.md)
- [ViolationEvent](docs/ViolationEvent.md)
- [ViolationEventType](docs/ViolationEventType.md)
- [WebLink](docs/WebLink.md)
- [WhoMasked](docs/WhoMasked.md)

To get access to the crate's generated documentation, use:

```
cargo doc --open
```

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
