# \StudiesApi

All URIs are relative to *https://clinicaltrials.gov/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**enums**](StudiesApi.md#enums) | **GET** /studies/enums | Enums
[**fetch_study**](StudiesApi.md#fetch_study) | **GET** /studies/{nctId} | Single Study
[**list_studies**](StudiesApi.md#list_studies) | **GET** /studies | Studies
[**search_areas**](StudiesApi.md#search_areas) | **GET** /studies/search-areas | Search Areas
[**studies_metadata**](StudiesApi.md#studies_metadata) | **GET** /studies/metadata | Data Model Fields



## enums

> Vec<models::EnumInfo> enums()
Enums

Returns enumeration types and their values.  Every item of the returning array represents enum type and contains the following properties: * `type` - enum type name * `pieces` - array of names of all data pieces having the enum type * `values` - all available values of the enum; every item contains the following properties:   * `value` - data value   * `legacyValue` - data value in legacy API   * `exceptions` - map from data piece name to legacy value when different from `legacyValue`     (some data pieces had special enum values in legacy API)

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::EnumInfo>**](EnumInfo.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fetch_study

> String fetch_study(nct_id, format, markup_format, fields)
Single Study

Returns data of a single study.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**nct_id** | **String** | NCT Number of a study. If found in [NCTIdAlias](data-api/about-api/study-data-structure#NCTIdAlias) field, 301 HTTP redirect to the actual study will be returned. | [required] |
**format** | Option<**String**> | Must be one of the following: * `csv`- return CSV table; available fields are listed on [CSV Download](/data-api/about-api/csv-download) * `json`- return JSON object; format of `markup` fields depends on `markupFormat` parameter * `json.zip`- put JSON object into a .json file and download it as zip archive; field values of type `markup` are in [markdown](https://spec.commonmark.org/0.28/) format * `fhir.json` - return FHIR JSON; fields are not customizable; see [Access Data in FHIR](/data-api/fhir) * `ris`- return RIS record; available tags are listed on [RIS Download](/data-api/about-api/ris-download) |  |[default to json]
**markup_format** | Option<**String**> | Format of `markup` type fields: * `markdown`- [markdown](https://spec.commonmark.org/0.28/) format * `legacy`- compatible with classic PRS  Applicable only to `json` format. |  |[default to markdown]
**fields** | Option<[**Vec<String>**](String.md)> | If specified, must be non-empty comma- or pipe-separated list of fields to return. If unspecified, all fields will be returned. Order of the fields does not matter.  For `csv` format, specify list of columns. The column names are available on [CSV Download](/data-api/about-api/csv-download).  For `json` and `json.zip` formats, every list item is either area name, piece name, or field name. If a piece or a field is a branch node, all descendant fields will be included. All area names are available on [Search Areas](/data-api/about-api/search-areas), the piece and field names - on [Data Structure](/data-api/about-api/study-data-structure) and also can be retrieved at `/studies/metadata` endpoint.  For `fhir.json` format, all available fields are returned and this parameter must be unspecified.  For `ris` format, specify list of tags. The tag names are available on [RIS Download](/data-api/about-api/ris-download). |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/csv, application/json, application/zip, application/fhir+json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_studies

> models::PagedStudies list_studies(format, markup_format, query_period_cond, query_period_term, query_period_locn, query_period_titles, query_period_intr, query_period_outc, query_period_spons, query_period_lead, query_period_id, query_period_patient, filter_period_overall_status, filter_period_geo, filter_period_ids, filter_period_advanced, filter_period_synonyms, post_filter_period_overall_status, post_filter_period_geo, post_filter_period_ids, post_filter_period_advanced, post_filter_period_synonyms, agg_filters, geo_decay, fields, sort, count_total, page_size, page_token)
Studies

Returns data of studies matching query and filter parameters. The studies are returned page by page. If response contains `nextPageToken`, use its value in `pageToken` to get next page. The last page will not contain `nextPageToken`. A page may have empty `studies` array. Request for each subsequent page **must** have the same parameters as for the first page, except `countTotal`, `pageSize`, and `pageToken` parameters.  If neither queries nor filters are set, all studies will be returned. If any query parameter contains only NCT IDs (comma- and/or space-separated), filters are ignored.  `query.*` parameters are in [Essie expression syntax](/find-studies/constructing-complex-search-queries). Those parameters affect ranking of studies, if sorted by relevance. See `sort` parameter for details.  `filter.*` and `postFilter.*` parameters have same effect as there is no aggregation calculation.  Both are available just to simplify applying parameters from search request. Both do not affect ranking of studies.  Note: When trying JSON format in your browser, do not set too large `pageSize` parameter, if `fields` is unlimited. That may return too much data for the browser to parse and render.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**format** | Option<**String**> | Must be one of the following: * `csv`- return CSV table with one page of study data; first page will contain header with column names; available fields are listed on [CSV Download](/data-api/about-api/csv-download) page * `json`- return JSON with one page of study data; every study object is placed in a separate line; `markup` type fields format depends on `markupFormat` parameter |  |[default to json]
**markup_format** | Option<**String**> | Format of `markup` type fields: * `markdown`- [markdown](https://spec.commonmark.org/0.28/) format * `legacy`- compatible with classic PRS  Applicable only to `json` format. |  |[default to markdown]
**query_period_cond** | Option<**String**> | \"Conditions or disease\" query in [Essie expression syntax](/find-studies/constructing-complex-search-queries). See \"ConditionSearch Area\" on [Search Areas](/data-api/about-api/search-areas#ConditionSearch) for more details. |  |
**query_period_term** | Option<**String**> | \"Other terms\" query in [Essie expression syntax](/find-studies/constructing-complex-search-queries). See \"BasicSearch Area\" on [Search Areas](/data-api/about-api/search-areas#BasicSearch) for more details. |  |
**query_period_locn** | Option<**String**> | \"Location terms\" query in [Essie expression syntax](/find-studies/constructing-complex-search-queries). See \"LocationSearch Area\" on [Search Areas](/data-api/about-api/search-areas#LocationSearch) for more details. |  |
**query_period_titles** | Option<**String**> | \"Title / acronym\" query in [Essie expression syntax](/find-studies/constructing-complex-search-queries). See \"TitleSearch Area\" on [Search Areas](/data-api/about-api/search-areas#TitleSearch) for more details. |  |
**query_period_intr** | Option<**String**> | \"Intervention / treatment\" query in [Essie expression syntax](/find-studies/constructing-complex-search-queries). See \"InterventionSearch Area\" on [Search Areas](/data-api/about-api/search-areas#InterventionSearch) for more details. |  |
**query_period_outc** | Option<**String**> | \"Outcome measure\" query in [Essie expression syntax](/find-studies/constructing-complex-search-queries). See \"OutcomeSearch Area\" on [Search Areas](/data-api/about-api/search-areas#OutcomeSearch) for more details. |  |
**query_period_spons** | Option<**String**> | \"Sponsor / collaborator\" query in [Essie expression syntax](/find-studies/constructing-complex-search-queries). See \"SponsorSearch Area\" on [Search Areas](/data-api/about-api/search-areas#SponsorSearch) for more details. |  |
**query_period_lead** | Option<**String**> | Searches in \"LeadSponsorName\" field. See [Study Data Structure](/data-api/about-api/study-data-structure#LeadSponsorName) for more details. The query is in [Essie expression syntax](/find-studies/constructing-complex-search-queries). |  |
**query_period_id** | Option<**String**> | \"Study IDs\" query in [Essie expression syntax](/find-studies/constructing-complex-search-queries). See \"IdSearch Area\" on [Search Areas](/data-api/about-api/search-areas#IdSearch) for more details. |  |
**query_period_patient** | Option<**String**> | See \"PatientSearch Area\" on [Search Areas](/data-api/about-api/search-areas#PatientSearch) for more details. |  |
**filter_period_overall_status** | Option<[**Vec<models::Status>**](models::Status.md)> | Filter by comma- or pipe-separated list of statuses |  |
**filter_period_geo** | Option<**String**> | Filter by geo-function. Currently only distance function is supported. Format: `distance(latitude,longitude,distance)` |  |
**filter_period_ids** | Option<[**Vec<String>**](String.md)> | Filter by comma- or pipe-separated list of NCT IDs (a.k.a. ClinicalTrials.gov identifiers). The provided IDs will be searched in [NCTId](data-api/about-api/study-data-structure#NCTId) and [NCTIdAlias](data-api/about-api/study-data-structure#NCTIdAlias) fields. |  |
**filter_period_advanced** | Option<**String**> | Filter by query in [Essie expression syntax](/find-studies/constructing-complex-search-queries) |  |
**filter_period_synonyms** | Option<[**Vec<String>**](String.md)> | Filter by comma- or pipe-separated list of `area`:`synonym_id` pairs |  |
**post_filter_period_overall_status** | Option<[**Vec<models::Status>**](models::Status.md)> | Filter by comma- or pipe-separated list of statuses |  |
**post_filter_period_geo** | Option<**String**> | Filter by geo-function. Currently only distance function is supported. Format: `distance(latitude,longitude,distance)` |  |
**post_filter_period_ids** | Option<[**Vec<String>**](String.md)> | Filter by comma- or pipe-separated list of NCT IDs (a.k.a. ClinicalTrials.gov identifiers). The provided IDs will be searched in [NCTId](data-api/about-api/study-data-structure#NCTId) and [NCTIdAlias](data-api/about-api/study-data-structure#NCTIdAlias) fields. |  |
**post_filter_period_advanced** | Option<**String**> | Filter by query in [Essie expression syntax](/find-studies/constructing-complex-search-queries) |  |
**post_filter_period_synonyms** | Option<[**Vec<String>**](String.md)> | Filter by comma- or pipe-separated list of `area`:`synonym_id` pairs |  |
**agg_filters** | Option<**String**> | Apply aggregation filters, aggregation counts will not be provided. The value is comma- or pipe-separated list of pairs `filter_id`:`space-separated list of option keys` for the checked options. |  |
**geo_decay** | Option<**String**> | Set proximity factor by distance from `filter.geo` location to the closest [LocationGeoPoint](/data-api/about-api/study-data-structure#LocationGeoPoint) of a study. Ignored, if `filter.geo` parameter is not set or response contains more than 10,000 studies. |  |[default to func:exp,scale:300mi,offset:0mi,decay:0.5]
**fields** | Option<[**Vec<String>**](String.md)> | If specified, must be non-empty comma- or pipe-separated list of fields to return. If unspecified, all fields will be returned. Order of the fields does not matter.  For `csv` format, specify list of columns. The column names are available on [CSV Download](/data-api/about-api/csv-download).  For `json` format, every list item is either area name, piece name, field name, or special name. If a piece or a field is a branch node, all descendant fields will be included. All area names are available on [Search Areas](/data-api/about-api/search-areas), the piece and field names — on [Data Structure](/data-api/about-api/study-data-structure) and also can be retrieved at `/studies/metadata` endpoint. There is a special name, `@query`, which expands to all fields queried by search. |  |
**sort** | Option<[**Vec<String>**](String.md)> | Comma- or pipe-separated list of sorting options of the studies. The returning studies are not sorted by default for a performance reason. Every list item contains a field/piece name and an optional sort direction (`asc` for ascending or `desc` for descending) after colon character.  All piece and field names can be found on [Data Structure](/data-api/about-api/study-data-structure) and also can be retrieved at `/studies/metadata` endpoint. Currently, only date and numeric fields are allowed for sorting. There is a special \"field\" `@relevance` to sort by relevance to a search query.  Studies missing sort field are always last. Default sort direction: * Date field - `desc` * Numeric field - `asc` * `@relevance` - `desc` |  |[default to []]
**count_total** | Option<**bool**> | Count total number of studies in all pages and return `totalCount` field with first page, if `true`. For CSV, the result can be found in `x-total-count` response header. The parameter is ignored for the subsequent pages. |  |[default to false]
**page_size** | Option<**i32**> | Page size is maximum number of studies to return in response. It does not have to be the same for every page. If not specified or set to 0, the default value will be used. It will be coerced down to  1,000, if greater than that. |  |[default to 10]
**page_token** | Option<**String**> | Token to get next page. Set it to a `nextPageToken` value returned with the previous page in JSON format. For CSV, it can be found in `x-next-page-token` response header. Do not specify it for first page. |  |

### Return type

[**models::PagedStudies**](PagedStudies.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_areas

> Vec<models::SearchDocument> search_areas()
Search Areas

Search Docs and their Search Areas.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::SearchDocument>**](SearchDocument.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## studies_metadata

> Vec<models::FieldNode> studies_metadata(include_indexed_only, include_historic_only)
Data Model Fields

Returns study data model fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include_indexed_only** | Option<**bool**> | Include indexed-only fields, if `true` |  |[default to false]
**include_historic_only** | Option<**bool**> | Include fields available only in historic data, if `true` |  |[default to false]

### Return type

[**Vec<models::FieldNode>**](FieldNode.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

