# GetV1Bulk200ResponseSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**TotalSafe** | **int32** | The number of emails where &#x60;is_reachable&#x60; is \&quot;safe\&quot;. | [optional]
**TotalInvalid** | **int32** | The number of emails where &#x60;is_reachable&#x60; is \&quot;invalid\&quot;. | [optional]
**TotalRisky** | **int32** | The number of emails where &#x60;is_reachable&#x60; is \&quot;risky\&quot;. | [optional]
**TotalUnknown** | **int32** | The number of emails where &#x60;is_reachable&#x60; is \&quot;unknown\&quot;. | [optional]

## Methods

### NewGetV1Bulk200ResponseSummary

`func NewGetV1Bulk200ResponseSummary(totalSafe int32, totalInvalid int32, totalRisky int32, totalUnknown int32) *GetV1Bulk200ResponseSummary`

NewGetV1Bulk200ResponseSummary instantiates a new GetV1Bulk200ResponseSummary object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewGetV1Bulk200ResponseSummaryWithDefaults

`func NewGetV1Bulk200ResponseSummaryWithDefaults() *GetV1Bulk200ResponseSummary`

NewGetV1Bulk200ResponseSummaryWithDefaults instantiates a new GetV1Bulk200ResponseSummary object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetTotalSafe

`func (o *GetV1Bulk200ResponseSummary) GetTotalSafe() int32`

GetTotalSafe returns the TotalSafe field if non-nil, zero value otherwise.

### GetTotalSafeOk

`func (o *GetV1Bulk200ResponseSummary) GetTotalSafeOk() (*int32, bool)`

GetTotalSafeOk returns a tuple with the TotalSafe field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalSafe

`func (o *GetV1Bulk200ResponseSummary) SetTotalSafe(v int32)`

SetTotalSafe sets TotalSafe field to given value.


### GetTotalInvalid

`func (o *GetV1Bulk200ResponseSummary) GetTotalInvalid() int32`

GetTotalInvalid returns the TotalInvalid field if non-nil, zero value otherwise.

### GetTotalInvalidOk

`func (o *GetV1Bulk200ResponseSummary) GetTotalInvalidOk() (*int32, bool)`

GetTotalInvalidOk returns a tuple with the TotalInvalid field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalInvalid

`func (o *GetV1Bulk200ResponseSummary) SetTotalInvalid(v int32)`

SetTotalInvalid sets TotalInvalid field to given value.


### GetTotalRisky

`func (o *GetV1Bulk200ResponseSummary) GetTotalRisky() int32`

GetTotalRisky returns the TotalRisky field if non-nil, zero value otherwise.

### GetTotalRiskyOk

`func (o *GetV1Bulk200ResponseSummary) GetTotalRiskyOk() (*int32, bool)`

GetTotalRiskyOk returns a tuple with the TotalRisky field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalRisky

`func (o *GetV1Bulk200ResponseSummary) SetTotalRisky(v int32)`

SetTotalRisky sets TotalRisky field to given value.


### GetTotalUnknown

`func (o *GetV1Bulk200ResponseSummary) GetTotalUnknown() int32`

GetTotalUnknown returns the TotalUnknown field if non-nil, zero value otherwise.

### GetTotalUnknownOk

`func (o *GetV1Bulk200ResponseSummary) GetTotalUnknownOk() (*int32, bool)`

GetTotalUnknownOk returns a tuple with the TotalUnknown field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalUnknown

`func (o *GetV1Bulk200ResponseSummary) SetTotalUnknown(v int32)`

SetTotalUnknown sets TotalUnknown field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


