# UpdateReverificationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BatchSize** | Pointer to **int32** | Addresses re-checked in each run (1 to 1,000). | [optional]
**Enabled** | **bool** |  | [required]
**StalenessDays** | **int32** | Re-check results older than this many days (1 to 365). | [required]

## Methods

### NewUpdateReverificationRequest

`func NewUpdateReverificationRequest(enabled bool, stalenessDays int32) *UpdateReverificationRequest`

NewUpdateReverificationRequest instantiates a new UpdateReverificationRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateReverificationRequestWithDefaults

`func NewUpdateReverificationRequestWithDefaults() *UpdateReverificationRequest`

NewUpdateReverificationRequestWithDefaults instantiates a new UpdateReverificationRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBatchSize

`func (o *UpdateReverificationRequest) GetBatchSize() int32`

GetBatchSize returns the BatchSize field if non-nil, zero value otherwise.

### GetBatchSizeOk

`func (o *UpdateReverificationRequest) GetBatchSizeOk() (*int32, bool)`

GetBatchSizeOk returns a tuple with the BatchSize field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBatchSize

`func (o *UpdateReverificationRequest) SetBatchSize(v int32)`

SetBatchSize sets BatchSize field to given value.

### HasBatchSize

`func (o *UpdateReverificationRequest) HasBatchSize() bool`

HasBatchSize returns a boolean if a field has been set.

### GetEnabled

`func (o *UpdateReverificationRequest) GetEnabled() bool`

GetEnabled returns the Enabled field if non-nil, zero value otherwise.

### GetEnabledOk

`func (o *UpdateReverificationRequest) GetEnabledOk() (*bool, bool)`

GetEnabledOk returns a tuple with the Enabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnabled

`func (o *UpdateReverificationRequest) SetEnabled(v bool)`

SetEnabled sets Enabled field to given value.


### GetStalenessDays

`func (o *UpdateReverificationRequest) GetStalenessDays() int32`

GetStalenessDays returns the StalenessDays field if non-nil, zero value otherwise.

### GetStalenessDaysOk

`func (o *UpdateReverificationRequest) GetStalenessDaysOk() (*int32, bool)`

GetStalenessDaysOk returns a tuple with the StalenessDays field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStalenessDays

`func (o *UpdateReverificationRequest) SetStalenessDays(v int32)`

SetStalenessDays sets StalenessDays field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
