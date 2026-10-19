# SourceQualityResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Sources** | [**[]SourceQualityRow**](SourceQualityRow.md) |  | [required]

## Methods

### NewSourceQualityResponse

`func NewSourceQualityResponse(sources []SourceQualityRow) *SourceQualityResponse`

NewSourceQualityResponse instantiates a new SourceQualityResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSourceQualityResponseWithDefaults

`func NewSourceQualityResponseWithDefaults() *SourceQualityResponse`

NewSourceQualityResponseWithDefaults instantiates a new SourceQualityResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetSources

`func (o *SourceQualityResponse) GetSources() []SourceQualityRow`

GetSources returns the Sources field if non-nil, zero value otherwise.

### GetSourcesOk

`func (o *SourceQualityResponse) GetSourcesOk() ([]SourceQualityRow, bool)`

GetSourcesOk returns a tuple with the Sources field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSources

`func (o *SourceQualityResponse) SetSources(v []SourceQualityRow)`

SetSources sets Sources field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
