# \StatsApi

All URIs are relative to *https://clinicaltrials.gov/api/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**field_values_stats**](StatsApi.md#field_values_stats) | **GET** /stats/field/values | Field Values
[**list_field_sizes_stats**](StatsApi.md#list_field_sizes_stats) | **GET** /stats/field/sizes | List Field Sizes
[**size_stats**](StatsApi.md#size_stats) | **GET** /stats/size | Study Sizes



## field_values_stats

> Vec<models::FieldValuesStats> field_values_stats(types, fields)
Field Values

Value statistics of the study leaf fields.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**types** | Option<[**Vec<models::FieldStatsType>**](models::FieldStatsType.md)> | Filter by field types |  |[default to []]
**fields** | Option<[**Vec<String>**](String.md)> | Filter by piece names or field paths of leaf fields. See [Data Structure](/data-api/about-api/study-data-structure) for the available values.  If specified, must be non-empty comma- or pipe-separated list of fields to return. |  |

### Return type

[**Vec<models::FieldValuesStats>**](FieldValuesStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_field_sizes_stats

> Vec<models::ListSizes> list_field_sizes_stats(fields)
List Field Sizes

Sizes of list/array fields.  To search studies by a list field size, use `AREA[FieldName:size]` search operator. For example, [AREA\\[Phase:size\\] 2](https://clinicaltrials.gov/search?term=AREA%5BPhase:size%5D%202) query finds studies with 2 phases.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**fields** | Option<[**Vec<String>**](String.md)> | Filter by piece names or field paths of leaf fields. See [Data Structure](/data-api/about-api/study-data-structure) for the available values.  If specified, must be non-empty comma- or pipe-separated list of fields to return. If unspecified, all available stats will be returned. |  |

### Return type

[**Vec<models::ListSizes>**](ListSizes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## size_stats

> models::GzipStats size_stats()
Study Sizes

Statistics of study JSON sizes.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GzipStats**](GzipStats.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

