# SuppressionEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ActorId** | Pointer to **NullableString** |  | [optional]
**ActorType** | **string** |  | [required]
**CanonicalEmail** | **string** |  | [required]
**CreatedAt** | **time.Time** |  | [required]
**EntryId** | Pointer to **NullableInt32** |  | [optional]
**EventType** | **string** |  | [required]
**FromStatus** | Pointer to **NullableString** |  | [optional]
**Id** | **int64** |  | [required]
**Metadata** | **map[string]interface{}** |  | [required]
**ReasonCode** | Pointer to **NullableString** |  | [optional]
**ReasonDetail** | Pointer to **NullableString** |  | [optional]
**SourceRef** | Pointer to **NullableString** |  | [optional]
**SourceType** | Pointer to **NullableString** |  | [optional]
**ToStatus** | Pointer to **NullableString** |  | [optional]

## Methods

### NewSuppressionEvent

`func NewSuppressionEvent(actorType string, canonicalEmail string, createdAt time.Time, eventType string, id int64, metadata map[string]interface{}) *SuppressionEvent`

NewSuppressionEvent instantiates a new SuppressionEvent object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSuppressionEventWithDefaults

`func NewSuppressionEventWithDefaults() *SuppressionEvent`

NewSuppressionEventWithDefaults instantiates a new SuppressionEvent object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetActorId

`func (o *SuppressionEvent) GetActorId() string`

GetActorId returns the ActorId field if non-nil, zero value otherwise.

### GetActorIdOk

`func (o *SuppressionEvent) GetActorIdOk() (*string, bool)`

GetActorIdOk returns a tuple with the ActorId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetActorId

`func (o *SuppressionEvent) SetActorId(v string)`

SetActorId sets ActorId field to given value.

### HasActorId

`func (o *SuppressionEvent) HasActorId() bool`

HasActorId returns a boolean if a field has been set.

### SetActorIdNil

`func (o *SuppressionEvent) SetActorIdNil()`

 SetActorIdNil sets the value for ActorId to be an explicit nil

### UnsetActorId
`func (o *SuppressionEvent) UnsetActorId()`

UnsetActorId ensures that no value is present for ActorId, not even an explicit nil

### GetActorType

`func (o *SuppressionEvent) GetActorType() string`

GetActorType returns the ActorType field if non-nil, zero value otherwise.

### GetActorTypeOk

`func (o *SuppressionEvent) GetActorTypeOk() (*string, bool)`

GetActorTypeOk returns a tuple with the ActorType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetActorType

`func (o *SuppressionEvent) SetActorType(v string)`

SetActorType sets ActorType field to given value.


### GetCanonicalEmail

`func (o *SuppressionEvent) GetCanonicalEmail() string`

GetCanonicalEmail returns the CanonicalEmail field if non-nil, zero value otherwise.

### GetCanonicalEmailOk

`func (o *SuppressionEvent) GetCanonicalEmailOk() (*string, bool)`

GetCanonicalEmailOk returns a tuple with the CanonicalEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCanonicalEmail

`func (o *SuppressionEvent) SetCanonicalEmail(v string)`

SetCanonicalEmail sets CanonicalEmail field to given value.


### GetCreatedAt

`func (o *SuppressionEvent) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *SuppressionEvent) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *SuppressionEvent) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetEntryId

`func (o *SuppressionEvent) GetEntryId() int32`

GetEntryId returns the EntryId field if non-nil, zero value otherwise.

### GetEntryIdOk

`func (o *SuppressionEvent) GetEntryIdOk() (*int32, bool)`

GetEntryIdOk returns a tuple with the EntryId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntryId

`func (o *SuppressionEvent) SetEntryId(v int32)`

SetEntryId sets EntryId field to given value.

### HasEntryId

`func (o *SuppressionEvent) HasEntryId() bool`

HasEntryId returns a boolean if a field has been set.

### SetEntryIdNil

`func (o *SuppressionEvent) SetEntryIdNil()`

 SetEntryIdNil sets the value for EntryId to be an explicit nil

### UnsetEntryId
`func (o *SuppressionEvent) UnsetEntryId()`

UnsetEntryId ensures that no value is present for EntryId, not even an explicit nil

### GetEventType

`func (o *SuppressionEvent) GetEventType() string`

GetEventType returns the EventType field if non-nil, zero value otherwise.

### GetEventTypeOk

`func (o *SuppressionEvent) GetEventTypeOk() (*string, bool)`

GetEventTypeOk returns a tuple with the EventType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEventType

`func (o *SuppressionEvent) SetEventType(v string)`

SetEventType sets EventType field to given value.


### GetFromStatus

`func (o *SuppressionEvent) GetFromStatus() string`

GetFromStatus returns the FromStatus field if non-nil, zero value otherwise.

### GetFromStatusOk

`func (o *SuppressionEvent) GetFromStatusOk() (*string, bool)`

GetFromStatusOk returns a tuple with the FromStatus field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFromStatus

`func (o *SuppressionEvent) SetFromStatus(v string)`

SetFromStatus sets FromStatus field to given value.

### HasFromStatus

`func (o *SuppressionEvent) HasFromStatus() bool`

HasFromStatus returns a boolean if a field has been set.

### SetFromStatusNil

`func (o *SuppressionEvent) SetFromStatusNil()`

 SetFromStatusNil sets the value for FromStatus to be an explicit nil

### UnsetFromStatus
`func (o *SuppressionEvent) UnsetFromStatus()`

UnsetFromStatus ensures that no value is present for FromStatus, not even an explicit nil

### GetId

`func (o *SuppressionEvent) GetId() int64`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *SuppressionEvent) GetIdOk() (*int64, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *SuppressionEvent) SetId(v int64)`

SetId sets Id field to given value.


### GetMetadata

`func (o *SuppressionEvent) GetMetadata() map[string]interface{}`

GetMetadata returns the Metadata field if non-nil, zero value otherwise.

### GetMetadataOk

`func (o *SuppressionEvent) GetMetadataOk() (map[string]interface{}, bool)`

GetMetadataOk returns a tuple with the Metadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMetadata

`func (o *SuppressionEvent) SetMetadata(v map[string]interface{})`

SetMetadata sets Metadata field to given value.


### GetReasonCode

`func (o *SuppressionEvent) GetReasonCode() string`

GetReasonCode returns the ReasonCode field if non-nil, zero value otherwise.

### GetReasonCodeOk

`func (o *SuppressionEvent) GetReasonCodeOk() (*string, bool)`

GetReasonCodeOk returns a tuple with the ReasonCode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReasonCode

`func (o *SuppressionEvent) SetReasonCode(v string)`

SetReasonCode sets ReasonCode field to given value.

### HasReasonCode

`func (o *SuppressionEvent) HasReasonCode() bool`

HasReasonCode returns a boolean if a field has been set.

### SetReasonCodeNil

`func (o *SuppressionEvent) SetReasonCodeNil()`

 SetReasonCodeNil sets the value for ReasonCode to be an explicit nil

### UnsetReasonCode
`func (o *SuppressionEvent) UnsetReasonCode()`

UnsetReasonCode ensures that no value is present for ReasonCode, not even an explicit nil

### GetReasonDetail

`func (o *SuppressionEvent) GetReasonDetail() string`

GetReasonDetail returns the ReasonDetail field if non-nil, zero value otherwise.

### GetReasonDetailOk

`func (o *SuppressionEvent) GetReasonDetailOk() (*string, bool)`

GetReasonDetailOk returns a tuple with the ReasonDetail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReasonDetail

`func (o *SuppressionEvent) SetReasonDetail(v string)`

SetReasonDetail sets ReasonDetail field to given value.

### HasReasonDetail

`func (o *SuppressionEvent) HasReasonDetail() bool`

HasReasonDetail returns a boolean if a field has been set.

### SetReasonDetailNil

`func (o *SuppressionEvent) SetReasonDetailNil()`

 SetReasonDetailNil sets the value for ReasonDetail to be an explicit nil

### UnsetReasonDetail
`func (o *SuppressionEvent) UnsetReasonDetail()`

UnsetReasonDetail ensures that no value is present for ReasonDetail, not even an explicit nil

### GetSourceRef

`func (o *SuppressionEvent) GetSourceRef() string`

GetSourceRef returns the SourceRef field if non-nil, zero value otherwise.

### GetSourceRefOk

`func (o *SuppressionEvent) GetSourceRefOk() (*string, bool)`

GetSourceRefOk returns a tuple with the SourceRef field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceRef

`func (o *SuppressionEvent) SetSourceRef(v string)`

SetSourceRef sets SourceRef field to given value.

### HasSourceRef

`func (o *SuppressionEvent) HasSourceRef() bool`

HasSourceRef returns a boolean if a field has been set.

### SetSourceRefNil

`func (o *SuppressionEvent) SetSourceRefNil()`

 SetSourceRefNil sets the value for SourceRef to be an explicit nil

### UnsetSourceRef
`func (o *SuppressionEvent) UnsetSourceRef()`

UnsetSourceRef ensures that no value is present for SourceRef, not even an explicit nil

### GetSourceType

`func (o *SuppressionEvent) GetSourceType() string`

GetSourceType returns the SourceType field if non-nil, zero value otherwise.

### GetSourceTypeOk

`func (o *SuppressionEvent) GetSourceTypeOk() (*string, bool)`

GetSourceTypeOk returns a tuple with the SourceType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceType

`func (o *SuppressionEvent) SetSourceType(v string)`

SetSourceType sets SourceType field to given value.

### HasSourceType

`func (o *SuppressionEvent) HasSourceType() bool`

HasSourceType returns a boolean if a field has been set.

### SetSourceTypeNil

`func (o *SuppressionEvent) SetSourceTypeNil()`

 SetSourceTypeNil sets the value for SourceType to be an explicit nil

### UnsetSourceType
`func (o *SuppressionEvent) UnsetSourceType()`

UnsetSourceType ensures that no value is present for SourceType, not even an explicit nil

### GetToStatus

`func (o *SuppressionEvent) GetToStatus() string`

GetToStatus returns the ToStatus field if non-nil, zero value otherwise.

### GetToStatusOk

`func (o *SuppressionEvent) GetToStatusOk() (*string, bool)`

GetToStatusOk returns a tuple with the ToStatus field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetToStatus

`func (o *SuppressionEvent) SetToStatus(v string)`

SetToStatus sets ToStatus field to given value.

### HasToStatus

`func (o *SuppressionEvent) HasToStatus() bool`

HasToStatus returns a boolean if a field has been set.

### SetToStatusNil

`func (o *SuppressionEvent) SetToStatusNil()`

 SetToStatusNil sets the value for ToStatus to be an explicit nil

### UnsetToStatus
`func (o *SuppressionEvent) UnsetToStatus()`

UnsetToStatus ensures that no value is present for ToStatus, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
