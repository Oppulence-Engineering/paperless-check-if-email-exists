# ProviderEndpointListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ProviderEndpoints** | [**[]ProviderEndpointView**](ProviderEndpointView.md) |  | [required]

## Methods

### NewProviderEndpointListResponse

`func NewProviderEndpointListResponse(providerEndpoints []ProviderEndpointView) *ProviderEndpointListResponse`

NewProviderEndpointListResponse instantiates a new ProviderEndpointListResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewProviderEndpointListResponseWithDefaults

`func NewProviderEndpointListResponseWithDefaults() *ProviderEndpointListResponse`

NewProviderEndpointListResponseWithDefaults instantiates a new ProviderEndpointListResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetProviderEndpoints

`func (o *ProviderEndpointListResponse) GetProviderEndpoints() []ProviderEndpointView`

GetProviderEndpoints returns the ProviderEndpoints field if non-nil, zero value otherwise.

### GetProviderEndpointsOk

`func (o *ProviderEndpointListResponse) GetProviderEndpointsOk() ([]ProviderEndpointView, bool)`

GetProviderEndpointsOk returns a tuple with the ProviderEndpoints field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderEndpoints

`func (o *ProviderEndpointListResponse) SetProviderEndpoints(v []ProviderEndpointView)`

SetProviderEndpoints sets ProviderEndpoints field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
