# PipelineRunView

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BilledEmails** | **int32** |  | [required]
**CompletedAt** | Pointer to **NullableTime** |  | [optional]
**CreatedAt** | **time.Time** |  | [required]
**DeliveryAttempts** | **int32** |  | [required]
**DeliveryError** | Pointer to **NullableString** |  | [optional]
**DeliveryStatus** | [**PipelineDeliveryStatus**](PipelineDeliveryStatus.md) |  | [required]
**ErrorCode** | Pointer to **NullableString** |  | [optional]
**ErrorMessage** | Pointer to **NullableString** |  | [optional]
**Id** | **int64** |  | [required]
**JobId** | Pointer to **NullableInt32** |  | [optional]
**LastDeliveryAttemptAt** | Pointer to **NullableTime** |  | [optional]
**ListId** | Pointer to **NullableInt32** |  | [optional]
**NextDeliveryAttemptAt** | Pointer to **NullableTime** |  | [optional]
**PipelineId** | **int64** |  | [required]
**ResultLocation** | Pointer to **interface{}** |  | [optional]
**ScheduledFor** | Pointer to **NullableTime** |  | [optional]
**SourceSnapshot** | **interface{}** |  | [required]
**StartedAt** | Pointer to **NullableTime** |  | [optional]
**Stats** | **interface{}** |  | [required]
**Status** | [**PipelineRunStatus**](PipelineRunStatus.md) |  | [required]
**TenantId** | **string** |  | [required]
**TriggerType** | **string** |  | [required]
**UpdatedAt** | **time.Time** |  | [required]

## Methods

### NewPipelineRunView

`func NewPipelineRunView(billedEmails int32, createdAt time.Time, deliveryAttempts int32, deliveryStatus PipelineDeliveryStatus, id int64, pipelineId int64, sourceSnapshot interface{}, stats interface{}, status PipelineRunStatus, tenantId string, triggerType string, updatedAt time.Time) *PipelineRunView`

NewPipelineRunView instantiates a new PipelineRunView object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPipelineRunViewWithDefaults

`func NewPipelineRunViewWithDefaults() *PipelineRunView`

NewPipelineRunViewWithDefaults instantiates a new PipelineRunView object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBilledEmails

`func (o *PipelineRunView) GetBilledEmails() int32`

GetBilledEmails returns the BilledEmails field if non-nil, zero value otherwise.

### GetBilledEmailsOk

`func (o *PipelineRunView) GetBilledEmailsOk() (*int32, bool)`

GetBilledEmailsOk returns a tuple with the BilledEmails field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBilledEmails

`func (o *PipelineRunView) SetBilledEmails(v int32)`

SetBilledEmails sets BilledEmails field to given value.


### GetCompletedAt

`func (o *PipelineRunView) GetCompletedAt() time.Time`

GetCompletedAt returns the CompletedAt field if non-nil, zero value otherwise.

### GetCompletedAtOk

`func (o *PipelineRunView) GetCompletedAtOk() (*time.Time, bool)`

GetCompletedAtOk returns a tuple with the CompletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCompletedAt

`func (o *PipelineRunView) SetCompletedAt(v time.Time)`

SetCompletedAt sets CompletedAt field to given value.

### HasCompletedAt

`func (o *PipelineRunView) HasCompletedAt() bool`

HasCompletedAt returns a boolean if a field has been set.

### SetCompletedAtNil

`func (o *PipelineRunView) SetCompletedAtNil(b bool)`

 SetCompletedAtNil sets the value for CompletedAt to be an explicit nil

### UnsetCompletedAt
`func (o *PipelineRunView) UnsetCompletedAt()`

UnsetCompletedAt ensures that no value is present for CompletedAt, not even an explicit nil
### GetCreatedAt

`func (o *PipelineRunView) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *PipelineRunView) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *PipelineRunView) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetDeliveryAttempts

`func (o *PipelineRunView) GetDeliveryAttempts() int32`

GetDeliveryAttempts returns the DeliveryAttempts field if non-nil, zero value otherwise.

### GetDeliveryAttemptsOk

`func (o *PipelineRunView) GetDeliveryAttemptsOk() (*int32, bool)`

GetDeliveryAttemptsOk returns a tuple with the DeliveryAttempts field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeliveryAttempts

`func (o *PipelineRunView) SetDeliveryAttempts(v int32)`

SetDeliveryAttempts sets DeliveryAttempts field to given value.


### GetDeliveryError

`func (o *PipelineRunView) GetDeliveryError() string`

GetDeliveryError returns the DeliveryError field if non-nil, zero value otherwise.

### GetDeliveryErrorOk

`func (o *PipelineRunView) GetDeliveryErrorOk() (*string, bool)`

GetDeliveryErrorOk returns a tuple with the DeliveryError field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeliveryError

`func (o *PipelineRunView) SetDeliveryError(v string)`

SetDeliveryError sets DeliveryError field to given value.

### HasDeliveryError

`func (o *PipelineRunView) HasDeliveryError() bool`

HasDeliveryError returns a boolean if a field has been set.

### SetDeliveryErrorNil

`func (o *PipelineRunView) SetDeliveryErrorNil(b bool)`

 SetDeliveryErrorNil sets the value for DeliveryError to be an explicit nil

### UnsetDeliveryError
`func (o *PipelineRunView) UnsetDeliveryError()`

UnsetDeliveryError ensures that no value is present for DeliveryError, not even an explicit nil
### GetDeliveryStatus

`func (o *PipelineRunView) GetDeliveryStatus() PipelineDeliveryStatus`

GetDeliveryStatus returns the DeliveryStatus field if non-nil, zero value otherwise.

### GetDeliveryStatusOk

`func (o *PipelineRunView) GetDeliveryStatusOk() (*PipelineDeliveryStatus, bool)`

GetDeliveryStatusOk returns a tuple with the DeliveryStatus field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeliveryStatus

`func (o *PipelineRunView) SetDeliveryStatus(v PipelineDeliveryStatus)`

SetDeliveryStatus sets DeliveryStatus field to given value.


### GetErrorCode

`func (o *PipelineRunView) GetErrorCode() string`

GetErrorCode returns the ErrorCode field if non-nil, zero value otherwise.

### GetErrorCodeOk

`func (o *PipelineRunView) GetErrorCodeOk() (*string, bool)`

GetErrorCodeOk returns a tuple with the ErrorCode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorCode

`func (o *PipelineRunView) SetErrorCode(v string)`

SetErrorCode sets ErrorCode field to given value.

### HasErrorCode

`func (o *PipelineRunView) HasErrorCode() bool`

HasErrorCode returns a boolean if a field has been set.

### SetErrorCodeNil

`func (o *PipelineRunView) SetErrorCodeNil(b bool)`

 SetErrorCodeNil sets the value for ErrorCode to be an explicit nil

### UnsetErrorCode
`func (o *PipelineRunView) UnsetErrorCode()`

UnsetErrorCode ensures that no value is present for ErrorCode, not even an explicit nil
### GetErrorMessage

`func (o *PipelineRunView) GetErrorMessage() string`

GetErrorMessage returns the ErrorMessage field if non-nil, zero value otherwise.

### GetErrorMessageOk

`func (o *PipelineRunView) GetErrorMessageOk() (*string, bool)`

GetErrorMessageOk returns a tuple with the ErrorMessage field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrorMessage

`func (o *PipelineRunView) SetErrorMessage(v string)`

SetErrorMessage sets ErrorMessage field to given value.

### HasErrorMessage

`func (o *PipelineRunView) HasErrorMessage() bool`

HasErrorMessage returns a boolean if a field has been set.

### SetErrorMessageNil

`func (o *PipelineRunView) SetErrorMessageNil(b bool)`

 SetErrorMessageNil sets the value for ErrorMessage to be an explicit nil

### UnsetErrorMessage
`func (o *PipelineRunView) UnsetErrorMessage()`

UnsetErrorMessage ensures that no value is present for ErrorMessage, not even an explicit nil
### GetId

`func (o *PipelineRunView) GetId() int64`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *PipelineRunView) GetIdOk() (*int64, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *PipelineRunView) SetId(v int64)`

SetId sets Id field to given value.


### GetJobId

`func (o *PipelineRunView) GetJobId() int32`

GetJobId returns the JobId field if non-nil, zero value otherwise.

### GetJobIdOk

`func (o *PipelineRunView) GetJobIdOk() (*int32, bool)`

GetJobIdOk returns a tuple with the JobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobId

`func (o *PipelineRunView) SetJobId(v int32)`

SetJobId sets JobId field to given value.

### HasJobId

`func (o *PipelineRunView) HasJobId() bool`

HasJobId returns a boolean if a field has been set.

### SetJobIdNil

`func (o *PipelineRunView) SetJobIdNil(b bool)`

 SetJobIdNil sets the value for JobId to be an explicit nil

### UnsetJobId
`func (o *PipelineRunView) UnsetJobId()`

UnsetJobId ensures that no value is present for JobId, not even an explicit nil
### GetLastDeliveryAttemptAt

`func (o *PipelineRunView) GetLastDeliveryAttemptAt() time.Time`

GetLastDeliveryAttemptAt returns the LastDeliveryAttemptAt field if non-nil, zero value otherwise.

### GetLastDeliveryAttemptAtOk

`func (o *PipelineRunView) GetLastDeliveryAttemptAtOk() (*time.Time, bool)`

GetLastDeliveryAttemptAtOk returns a tuple with the LastDeliveryAttemptAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastDeliveryAttemptAt

`func (o *PipelineRunView) SetLastDeliveryAttemptAt(v time.Time)`

SetLastDeliveryAttemptAt sets LastDeliveryAttemptAt field to given value.

### HasLastDeliveryAttemptAt

`func (o *PipelineRunView) HasLastDeliveryAttemptAt() bool`

HasLastDeliveryAttemptAt returns a boolean if a field has been set.

### SetLastDeliveryAttemptAtNil

`func (o *PipelineRunView) SetLastDeliveryAttemptAtNil(b bool)`

 SetLastDeliveryAttemptAtNil sets the value for LastDeliveryAttemptAt to be an explicit nil

### UnsetLastDeliveryAttemptAt
`func (o *PipelineRunView) UnsetLastDeliveryAttemptAt()`

UnsetLastDeliveryAttemptAt ensures that no value is present for LastDeliveryAttemptAt, not even an explicit nil
### GetListId

`func (o *PipelineRunView) GetListId() int32`

GetListId returns the ListId field if non-nil, zero value otherwise.

### GetListIdOk

`func (o *PipelineRunView) GetListIdOk() (*int32, bool)`

GetListIdOk returns a tuple with the ListId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListId

`func (o *PipelineRunView) SetListId(v int32)`

SetListId sets ListId field to given value.

### HasListId

`func (o *PipelineRunView) HasListId() bool`

HasListId returns a boolean if a field has been set.

### SetListIdNil

`func (o *PipelineRunView) SetListIdNil(b bool)`

 SetListIdNil sets the value for ListId to be an explicit nil

### UnsetListId
`func (o *PipelineRunView) UnsetListId()`

UnsetListId ensures that no value is present for ListId, not even an explicit nil
### GetNextDeliveryAttemptAt

`func (o *PipelineRunView) GetNextDeliveryAttemptAt() time.Time`

GetNextDeliveryAttemptAt returns the NextDeliveryAttemptAt field if non-nil, zero value otherwise.

### GetNextDeliveryAttemptAtOk

`func (o *PipelineRunView) GetNextDeliveryAttemptAtOk() (*time.Time, bool)`

GetNextDeliveryAttemptAtOk returns a tuple with the NextDeliveryAttemptAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNextDeliveryAttemptAt

`func (o *PipelineRunView) SetNextDeliveryAttemptAt(v time.Time)`

SetNextDeliveryAttemptAt sets NextDeliveryAttemptAt field to given value.

### HasNextDeliveryAttemptAt

`func (o *PipelineRunView) HasNextDeliveryAttemptAt() bool`

HasNextDeliveryAttemptAt returns a boolean if a field has been set.

### SetNextDeliveryAttemptAtNil

`func (o *PipelineRunView) SetNextDeliveryAttemptAtNil(b bool)`

 SetNextDeliveryAttemptAtNil sets the value for NextDeliveryAttemptAt to be an explicit nil

### UnsetNextDeliveryAttemptAt
`func (o *PipelineRunView) UnsetNextDeliveryAttemptAt()`

UnsetNextDeliveryAttemptAt ensures that no value is present for NextDeliveryAttemptAt, not even an explicit nil
### GetPipelineId

`func (o *PipelineRunView) GetPipelineId() int64`

GetPipelineId returns the PipelineId field if non-nil, zero value otherwise.

### GetPipelineIdOk

`func (o *PipelineRunView) GetPipelineIdOk() (*int64, bool)`

GetPipelineIdOk returns a tuple with the PipelineId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPipelineId

`func (o *PipelineRunView) SetPipelineId(v int64)`

SetPipelineId sets PipelineId field to given value.


### GetResultLocation

`func (o *PipelineRunView) GetResultLocation() interface{}`

GetResultLocation returns the ResultLocation field if non-nil, zero value otherwise.

### GetResultLocationOk

`func (o *PipelineRunView) GetResultLocationOk() (*interface{}, bool)`

GetResultLocationOk returns a tuple with the ResultLocation field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResultLocation

`func (o *PipelineRunView) SetResultLocation(v interface{})`

SetResultLocation sets ResultLocation field to given value.

### HasResultLocation

`func (o *PipelineRunView) HasResultLocation() bool`

HasResultLocation returns a boolean if a field has been set.

### SetResultLocationNil

`func (o *PipelineRunView) SetResultLocationNil(b bool)`

 SetResultLocationNil sets the value for ResultLocation to be an explicit nil

### UnsetResultLocation
`func (o *PipelineRunView) UnsetResultLocation()`

UnsetResultLocation ensures that no value is present for ResultLocation, not even an explicit nil
### GetScheduledFor

`func (o *PipelineRunView) GetScheduledFor() time.Time`

GetScheduledFor returns the ScheduledFor field if non-nil, zero value otherwise.

### GetScheduledForOk

`func (o *PipelineRunView) GetScheduledForOk() (*time.Time, bool)`

GetScheduledForOk returns a tuple with the ScheduledFor field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScheduledFor

`func (o *PipelineRunView) SetScheduledFor(v time.Time)`

SetScheduledFor sets ScheduledFor field to given value.

### HasScheduledFor

`func (o *PipelineRunView) HasScheduledFor() bool`

HasScheduledFor returns a boolean if a field has been set.

### SetScheduledForNil

`func (o *PipelineRunView) SetScheduledForNil(b bool)`

 SetScheduledForNil sets the value for ScheduledFor to be an explicit nil

### UnsetScheduledFor
`func (o *PipelineRunView) UnsetScheduledFor()`

UnsetScheduledFor ensures that no value is present for ScheduledFor, not even an explicit nil
### GetSourceSnapshot

`func (o *PipelineRunView) GetSourceSnapshot() interface{}`

GetSourceSnapshot returns the SourceSnapshot field if non-nil, zero value otherwise.

### GetSourceSnapshotOk

`func (o *PipelineRunView) GetSourceSnapshotOk() (*interface{}, bool)`

GetSourceSnapshotOk returns a tuple with the SourceSnapshot field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceSnapshot

`func (o *PipelineRunView) SetSourceSnapshot(v interface{})`

SetSourceSnapshot sets SourceSnapshot field to given value.


### SetSourceSnapshotNil

`func (o *PipelineRunView) SetSourceSnapshotNil(b bool)`

 SetSourceSnapshotNil sets the value for SourceSnapshot to be an explicit nil

### UnsetSourceSnapshot
`func (o *PipelineRunView) UnsetSourceSnapshot()`

UnsetSourceSnapshot ensures that no value is present for SourceSnapshot, not even an explicit nil
### GetStartedAt

`func (o *PipelineRunView) GetStartedAt() time.Time`

GetStartedAt returns the StartedAt field if non-nil, zero value otherwise.

### GetStartedAtOk

`func (o *PipelineRunView) GetStartedAtOk() (*time.Time, bool)`

GetStartedAtOk returns a tuple with the StartedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStartedAt

`func (o *PipelineRunView) SetStartedAt(v time.Time)`

SetStartedAt sets StartedAt field to given value.

### HasStartedAt

`func (o *PipelineRunView) HasStartedAt() bool`

HasStartedAt returns a boolean if a field has been set.

### SetStartedAtNil

`func (o *PipelineRunView) SetStartedAtNil(b bool)`

 SetStartedAtNil sets the value for StartedAt to be an explicit nil

### UnsetStartedAt
`func (o *PipelineRunView) UnsetStartedAt()`

UnsetStartedAt ensures that no value is present for StartedAt, not even an explicit nil
### GetStats

`func (o *PipelineRunView) GetStats() interface{}`

GetStats returns the Stats field if non-nil, zero value otherwise.

### GetStatsOk

`func (o *PipelineRunView) GetStatsOk() (*interface{}, bool)`

GetStatsOk returns a tuple with the Stats field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStats

`func (o *PipelineRunView) SetStats(v interface{})`

SetStats sets Stats field to given value.


### SetStatsNil

`func (o *PipelineRunView) SetStatsNil(b bool)`

 SetStatsNil sets the value for Stats to be an explicit nil

### UnsetStats
`func (o *PipelineRunView) UnsetStats()`

UnsetStats ensures that no value is present for Stats, not even an explicit nil
### GetStatus

`func (o *PipelineRunView) GetStatus() PipelineRunStatus`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *PipelineRunView) GetStatusOk() (*PipelineRunStatus, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *PipelineRunView) SetStatus(v PipelineRunStatus)`

SetStatus sets Status field to given value.


### GetTenantId

`func (o *PipelineRunView) GetTenantId() string`

GetTenantId returns the TenantId field if non-nil, zero value otherwise.

### GetTenantIdOk

`func (o *PipelineRunView) GetTenantIdOk() (*string, bool)`

GetTenantIdOk returns a tuple with the TenantId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantId

`func (o *PipelineRunView) SetTenantId(v string)`

SetTenantId sets TenantId field to given value.


### GetTriggerType

`func (o *PipelineRunView) GetTriggerType() string`

GetTriggerType returns the TriggerType field if non-nil, zero value otherwise.

### GetTriggerTypeOk

`func (o *PipelineRunView) GetTriggerTypeOk() (*string, bool)`

GetTriggerTypeOk returns a tuple with the TriggerType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTriggerType

`func (o *PipelineRunView) SetTriggerType(v string)`

SetTriggerType sets TriggerType field to given value.


### GetUpdatedAt

`func (o *PipelineRunView) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *PipelineRunView) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *PipelineRunView) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


