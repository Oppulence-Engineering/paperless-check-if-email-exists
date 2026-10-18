# AuditEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Actor** | Pointer to **NullableString** |  | [optional]
**CreatedAt** | **time.Time** |  | [required]
**EventData** | Pointer to **map[string]interface{}** |  | [optional]
**EventType** | **string** |  | [required]
**Id** | **int64** |  | [required]
**JobId** | **int32** |  | [required]
**TaskId** | Pointer to **NullableInt32** |  | [optional]

## Methods

### NewAuditEvent

`func NewAuditEvent(createdAt time.Time, eventType string, id int64, jobId int32) *AuditEvent`

NewAuditEvent instantiates a new AuditEvent object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAuditEventWithDefaults

`func NewAuditEventWithDefaults() *AuditEvent`

NewAuditEventWithDefaults instantiates a new AuditEvent object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetActor

`func (o *AuditEvent) GetActor() string`

GetActor returns the Actor field if non-nil, zero value otherwise.

### GetActorOk

`func (o *AuditEvent) GetActorOk() (*string, bool)`

GetActorOk returns a tuple with the Actor field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetActor

`func (o *AuditEvent) SetActor(v string)`

SetActor sets Actor field to given value.

### HasActor

`func (o *AuditEvent) HasActor() bool`

HasActor returns a boolean if a field has been set.

### SetActorNil

`func (o *AuditEvent) SetActorNil()`

 SetActorNil sets the value for Actor to be an explicit nil

### UnsetActor
`func (o *AuditEvent) UnsetActor()`

UnsetActor ensures that no value is present for Actor, not even an explicit nil

### GetCreatedAt

`func (o *AuditEvent) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *AuditEvent) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *AuditEvent) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetEventData

`func (o *AuditEvent) GetEventData() map[string]interface{}`

GetEventData returns the EventData field if non-nil, zero value otherwise.

### GetEventDataOk

`func (o *AuditEvent) GetEventDataOk() (map[string]interface{}, bool)`

GetEventDataOk returns a tuple with the EventData field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEventData

`func (o *AuditEvent) SetEventData(v map[string]interface{})`

SetEventData sets EventData field to given value.

### HasEventData

`func (o *AuditEvent) HasEventData() bool`

HasEventData returns a boolean if a field has been set.

### GetEventType

`func (o *AuditEvent) GetEventType() string`

GetEventType returns the EventType field if non-nil, zero value otherwise.

### GetEventTypeOk

`func (o *AuditEvent) GetEventTypeOk() (*string, bool)`

GetEventTypeOk returns a tuple with the EventType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEventType

`func (o *AuditEvent) SetEventType(v string)`

SetEventType sets EventType field to given value.


### GetId

`func (o *AuditEvent) GetId() int64`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *AuditEvent) GetIdOk() (*int64, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *AuditEvent) SetId(v int64)`

SetId sets Id field to given value.


### GetJobId

`func (o *AuditEvent) GetJobId() int32`

GetJobId returns the JobId field if non-nil, zero value otherwise.

### GetJobIdOk

`func (o *AuditEvent) GetJobIdOk() (*int32, bool)`

GetJobIdOk returns a tuple with the JobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobId

`func (o *AuditEvent) SetJobId(v int32)`

SetJobId sets JobId field to given value.


### GetTaskId

`func (o *AuditEvent) GetTaskId() int32`

GetTaskId returns the TaskId field if non-nil, zero value otherwise.

### GetTaskIdOk

`func (o *AuditEvent) GetTaskIdOk() (*int32, bool)`

GetTaskIdOk returns a tuple with the TaskId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTaskId

`func (o *AuditEvent) SetTaskId(v int32)`

SetTaskId sets TaskId field to given value.

### HasTaskId

`func (o *AuditEvent) HasTaskId() bool`

HasTaskId returns a boolean if a field has been set.

### SetTaskIdNil

`func (o *AuditEvent) SetTaskIdNil()`

 SetTaskIdNil sets the value for TaskId to be an explicit nil

### UnsetTaskId
`func (o *AuditEvent) UnsetTaskId()`

UnsetTaskId ensures that no value is present for TaskId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
