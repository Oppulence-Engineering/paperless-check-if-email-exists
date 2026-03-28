# ReverificationStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BatchSize** | Pointer to **int32** |  | [optional]
**EmailsRequeued** | Pointer to **int32** |  | [optional]
**Enabled** | **bool** |  | [required]
**LastJobId** | Pointer to **NullableInt32** |  | [optional]
**LastRunAt** | Pointer to **NullableTime** |  | [optional]
**NextRunAt** | Pointer to **NullableTime** |  | [optional]
**StalenessDays** | Pointer to **int32** |  | [optional]

## Methods

### NewReverificationStatusResponse

`func NewReverificationStatusResponse(enabled bool) *ReverificationStatusResponse`

NewReverificationStatusResponse instantiates a new ReverificationStatusResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewReverificationStatusResponseWithDefaults

`func NewReverificationStatusResponseWithDefaults() *ReverificationStatusResponse`

NewReverificationStatusResponseWithDefaults instantiates a new ReverificationStatusResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBatchSize

`func (o *ReverificationStatusResponse) GetBatchSize() int32`

GetBatchSize returns the BatchSize field if non-nil, zero value otherwise.

### GetBatchSizeOk

`func (o *ReverificationStatusResponse) GetBatchSizeOk() (*int32, bool)`

GetBatchSizeOk returns a tuple with the BatchSize field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBatchSize

`func (o *ReverificationStatusResponse) SetBatchSize(v int32)`

SetBatchSize sets BatchSize field to given value.

### HasBatchSize

`func (o *ReverificationStatusResponse) HasBatchSize() bool`

HasBatchSize returns a boolean if a field has been set.

### GetEmailsRequeued

`func (o *ReverificationStatusResponse) GetEmailsRequeued() int32`

GetEmailsRequeued returns the EmailsRequeued field if non-nil, zero value otherwise.

### GetEmailsRequeuedOk

`func (o *ReverificationStatusResponse) GetEmailsRequeuedOk() (*int32, bool)`

GetEmailsRequeuedOk returns a tuple with the EmailsRequeued field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmailsRequeued

`func (o *ReverificationStatusResponse) SetEmailsRequeued(v int32)`

SetEmailsRequeued sets EmailsRequeued field to given value.

### HasEmailsRequeued

`func (o *ReverificationStatusResponse) HasEmailsRequeued() bool`

HasEmailsRequeued returns a boolean if a field has been set.

### GetEnabled

`func (o *ReverificationStatusResponse) GetEnabled() bool`

GetEnabled returns the Enabled field if non-nil, zero value otherwise.

### GetEnabledOk

`func (o *ReverificationStatusResponse) GetEnabledOk() (*bool, bool)`

GetEnabledOk returns a tuple with the Enabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnabled

`func (o *ReverificationStatusResponse) SetEnabled(v bool)`

SetEnabled sets Enabled field to given value.


### GetLastJobId

`func (o *ReverificationStatusResponse) GetLastJobId() int32`

GetLastJobId returns the LastJobId field if non-nil, zero value otherwise.

### GetLastJobIdOk

`func (o *ReverificationStatusResponse) GetLastJobIdOk() (*int32, bool)`

GetLastJobIdOk returns a tuple with the LastJobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastJobId

`func (o *ReverificationStatusResponse) SetLastJobId(v int32)`

SetLastJobId sets LastJobId field to given value.

### HasLastJobId

`func (o *ReverificationStatusResponse) HasLastJobId() bool`

HasLastJobId returns a boolean if a field has been set.

### SetLastJobIdNil

`func (o *ReverificationStatusResponse) SetLastJobIdNil(b bool)`

 SetLastJobIdNil sets the value for LastJobId to be an explicit nil

### UnsetLastJobId
`func (o *ReverificationStatusResponse) UnsetLastJobId()`

UnsetLastJobId ensures that no value is present for LastJobId, not even an explicit nil
### GetLastRunAt

`func (o *ReverificationStatusResponse) GetLastRunAt() time.Time`

GetLastRunAt returns the LastRunAt field if non-nil, zero value otherwise.

### GetLastRunAtOk

`func (o *ReverificationStatusResponse) GetLastRunAtOk() (*time.Time, bool)`

GetLastRunAtOk returns a tuple with the LastRunAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastRunAt

`func (o *ReverificationStatusResponse) SetLastRunAt(v time.Time)`

SetLastRunAt sets LastRunAt field to given value.

### HasLastRunAt

`func (o *ReverificationStatusResponse) HasLastRunAt() bool`

HasLastRunAt returns a boolean if a field has been set.

### SetLastRunAtNil

`func (o *ReverificationStatusResponse) SetLastRunAtNil(b bool)`

 SetLastRunAtNil sets the value for LastRunAt to be an explicit nil

### UnsetLastRunAt
`func (o *ReverificationStatusResponse) UnsetLastRunAt()`

UnsetLastRunAt ensures that no value is present for LastRunAt, not even an explicit nil
### GetNextRunAt

`func (o *ReverificationStatusResponse) GetNextRunAt() time.Time`

GetNextRunAt returns the NextRunAt field if non-nil, zero value otherwise.

### GetNextRunAtOk

`func (o *ReverificationStatusResponse) GetNextRunAtOk() (*time.Time, bool)`

GetNextRunAtOk returns a tuple with the NextRunAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNextRunAt

`func (o *ReverificationStatusResponse) SetNextRunAt(v time.Time)`

SetNextRunAt sets NextRunAt field to given value.

### HasNextRunAt

`func (o *ReverificationStatusResponse) HasNextRunAt() bool`

HasNextRunAt returns a boolean if a field has been set.

### SetNextRunAtNil

`func (o *ReverificationStatusResponse) SetNextRunAtNil(b bool)`

 SetNextRunAtNil sets the value for NextRunAt to be an explicit nil

### UnsetNextRunAt
`func (o *ReverificationStatusResponse) UnsetNextRunAt()`

UnsetNextRunAt ensures that no value is present for NextRunAt, not even an explicit nil
### GetStalenessDays

`func (o *ReverificationStatusResponse) GetStalenessDays() int32`

GetStalenessDays returns the StalenessDays field if non-nil, zero value otherwise.

### GetStalenessDaysOk

`func (o *ReverificationStatusResponse) GetStalenessDaysOk() (*int32, bool)`

GetStalenessDaysOk returns a tuple with the StalenessDays field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStalenessDays

`func (o *ReverificationStatusResponse) SetStalenessDays(v int32)`

SetStalenessDays sets StalenessDays field to given value.

### HasStalenessDays

`func (o *ReverificationStatusResponse) HasStalenessDays() bool`

HasStalenessDays returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


