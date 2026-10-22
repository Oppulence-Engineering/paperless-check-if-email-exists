# OverviewSectionOverviewReverificationData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Enabled** | **bool** |  | [required]
**NextRunAt** | Pointer to **NullableTime** |  | [optional]

## Methods

### NewOverviewSectionOverviewReverificationData

`func NewOverviewSectionOverviewReverificationData(enabled bool) *OverviewSectionOverviewReverificationData`

NewOverviewSectionOverviewReverificationData instantiates a new OverviewSectionOverviewReverificationData object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOverviewSectionOverviewReverificationDataWithDefaults

`func NewOverviewSectionOverviewReverificationDataWithDefaults() *OverviewSectionOverviewReverificationData`

NewOverviewSectionOverviewReverificationDataWithDefaults instantiates a new OverviewSectionOverviewReverificationData object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEnabled

`func (o *OverviewSectionOverviewReverificationData) GetEnabled() bool`

GetEnabled returns the Enabled field if non-nil, zero value otherwise.

### GetEnabledOk

`func (o *OverviewSectionOverviewReverificationData) GetEnabledOk() (*bool, bool)`

GetEnabledOk returns a tuple with the Enabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnabled

`func (o *OverviewSectionOverviewReverificationData) SetEnabled(v bool)`

SetEnabled sets Enabled field to given value.


### GetNextRunAt

`func (o *OverviewSectionOverviewReverificationData) GetNextRunAt() time.Time`

GetNextRunAt returns the NextRunAt field if non-nil, zero value otherwise.

### GetNextRunAtOk

`func (o *OverviewSectionOverviewReverificationData) GetNextRunAtOk() (*time.Time, bool)`

GetNextRunAtOk returns a tuple with the NextRunAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNextRunAt

`func (o *OverviewSectionOverviewReverificationData) SetNextRunAt(v time.Time)`

SetNextRunAt sets NextRunAt field to given value.

### HasNextRunAt

`func (o *OverviewSectionOverviewReverificationData) HasNextRunAt() bool`

HasNextRunAt returns a boolean if a field has been set.

### SetNextRunAtNil

`func (o *OverviewSectionOverviewReverificationData) SetNextRunAtNil()`

 SetNextRunAtNil sets the value for NextRunAt to be an explicit nil

### UnsetNextRunAt
`func (o *OverviewSectionOverviewReverificationData) UnsetNextRunAt()`

UnsetNextRunAt ensures that no value is present for NextRunAt, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
