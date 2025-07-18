# ADR-001: Generate Rust API Client Using OpenAPI Generator

## Status

Accepted

## Context

We need to create a Rust API library for the ClinicalTrials.gov REST API v2. The API has an OpenAPI 3.0.3 specification with:

- 8 endpoints across 3 main categories (Studies, Stats, Version)
- Complex data structures with 100+ schema components
- Multiple response formats (JSON, CSV, FHIR, RIS, ZIP)
- Extensive query parameters and filtering options

Manual implementation would be time-consuming and error-prone.

## Decision

Use OpenAPI Generator to automatically generate the Rust client library from the OpenAPI specification.

**Generator**: `rust` generator from OpenAPI Generator
**Rationale**:

- Eliminates manual coding of models and API calls
- Ensures type safety and consistency with the API spec
- Automatically handles complex nested structures
- Generates documentation from OpenAPI descriptions
- Supports regeneration when API spec changes

## Implementation

```bash
openapi-generator-cli generate \
  -i ctg-oas-v2.yaml \
  -g rust \
  -o . \
  --additional-properties=packageName=clinicaltrials-gov-api
```

## Consequences

### Positive

- Fast initial development
- Type-safe API client
- Automatic updates when spec changes
- Comprehensive model coverage
- Built-in serialization/deserialization

### Negative

- Generated code may not follow all Rust idioms
- Limited customization of generated APIs
- Dependency on OpenAPI Generator tool
- May generate unused code for large specs

### Mitigation

- Review and test generated code before use
- Add custom wrapper functions for common use cases
- Keep generator version pinned for reproducible builds

