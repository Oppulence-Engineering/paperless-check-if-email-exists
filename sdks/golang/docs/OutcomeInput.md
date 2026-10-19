# OutcomeInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CampaignId** | Pointer to **NullableString** |  | [optional]
**Email** | **string** |  | [required]
**EventType** | **string** |  | [required]
**Metadata** | Pointer to **interface{}** |  | [optional]
**OccurredAt** | Pointer to **NullableTime** |  | [optional]
**ProviderEventId** | Pointer to **NullableString** |  | [optional]
**ProviderMessageId** | Pointer to **NullableString** |  | [optional]
**SourceKey** | Pointer to **NullableString** |  | [optional]

## Methods

### NewOutcomeInput

`func NewOutcomeInput(email string, eventType string) *OutcomeInput`

NewOutcomeInput instantiates a new OutcomeInput object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOutcomeInputWithDefaults

`func NewOutcomeInputWithDefaults() *OutcomeInput`

NewOutcomeInputWithDefaults instantiates a new OutcomeInput object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCampaignId

`func (o *OutcomeInput) GetCampaignId() string`

GetCampaignId returns the CampaignId field if non-nil, zero value otherwise.

### GetCampaignIdOk

`func (o *OutcomeInput) GetCampaignIdOk() (*string, bool)`

GetCampaignIdOk returns a tuple with the CampaignId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCampaignId

`func (o *OutcomeInput) SetCampaignId(v string)`

SetCampaignId sets CampaignId field to given value.

### HasCampaignId

`func (o *OutcomeInput) HasCampaignId() bool`

HasCampaignId returns a boolean if a field has been set.

### SetCampaignIdNil

`func (o *OutcomeInput) SetCampaignIdNil()`

 SetCampaignIdNil sets the value for CampaignId to be an explicit nil

### UnsetCampaignId
`func (o *OutcomeInput) UnsetCampaignId()`

UnsetCampaignId ensures that no value is present for CampaignId, not even an explicit nil

### GetEmail

`func (o *OutcomeInput) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *OutcomeInput) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *OutcomeInput) SetEmail(v string)`

SetEmail sets Email field to given value.


### GetEventType

`func (o *OutcomeInput) GetEventType() string`

GetEventType returns the EventType field if non-nil, zero value otherwise.

### GetEventTypeOk

`func (o *OutcomeInput) GetEventTypeOk() (*string, bool)`

GetEventTypeOk returns a tuple with the EventType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEventType

`func (o *OutcomeInput) SetEventType(v string)`

SetEventType sets EventType field to given value.


### GetMetadata

`func (o *OutcomeInput) GetMetadata() interface{}`

GetMetadata returns the Metadata field if non-nil, zero value otherwise.

### GetMetadataOk

`func (o *OutcomeInput) GetMetadataOk() (*interface{}, bool)`

GetMetadataOk returns a tuple with the Metadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMetadata

`func (o *OutcomeInput) SetMetadata(v interface{})`

SetMetadata sets Metadata field to given value.

### HasMetadata

`func (o *OutcomeInput) HasMetadata() bool`

HasMetadata returns a boolean if a field has been set.

### GetOccurredAt

`func (o *OutcomeInput) GetOccurredAt() time.Time`

GetOccurredAt returns the OccurredAt field if non-nil, zero value otherwise.

### GetOccurredAtOk

`func (o *OutcomeInput) GetOccurredAtOk() (*time.Time, bool)`

GetOccurredAtOk returns a tuple with the OccurredAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOccurredAt

`func (o *OutcomeInput) SetOccurredAt(v time.Time)`

SetOccurredAt sets OccurredAt field to given value.

### HasOccurredAt

`func (o *OutcomeInput) HasOccurredAt() bool`

HasOccurredAt returns a boolean if a field has been set.

### SetOccurredAtNil

`func (o *OutcomeInput) SetOccurredAtNil()`

 SetOccurredAtNil sets the value for OccurredAt to be an explicit nil

### UnsetOccurredAt
`func (o *OutcomeInput) UnsetOccurredAt()`

UnsetOccurredAt ensures that no value is present for OccurredAt, not even an explicit nil

### GetProviderEventId

`func (o *OutcomeInput) GetProviderEventId() string`

GetProviderEventId returns the ProviderEventId field if non-nil, zero value otherwise.

### GetProviderEventIdOk

`func (o *OutcomeInput) GetProviderEventIdOk() (*string, bool)`

GetProviderEventIdOk returns a tuple with the ProviderEventId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderEventId

`func (o *OutcomeInput) SetProviderEventId(v string)`

SetProviderEventId sets ProviderEventId field to given value.

### HasProviderEventId

`func (o *OutcomeInput) HasProviderEventId() bool`

HasProviderEventId returns a boolean if a field has been set.

### SetProviderEventIdNil

`func (o *OutcomeInput) SetProviderEventIdNil()`

 SetProviderEventIdNil sets the value for ProviderEventId to be an explicit nil

### UnsetProviderEventId
`func (o *OutcomeInput) UnsetProviderEventId()`

UnsetProviderEventId ensures that no value is present for ProviderEventId, not even an explicit nil

### GetProviderMessageId

`func (o *OutcomeInput) GetProviderMessageId() string`

GetProviderMessageId returns the ProviderMessageId field if non-nil, zero value otherwise.

### GetProviderMessageIdOk

`func (o *OutcomeInput) GetProviderMessageIdOk() (*string, bool)`

GetProviderMessageIdOk returns a tuple with the ProviderMessageId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderMessageId

`func (o *OutcomeInput) SetProviderMessageId(v string)`

SetProviderMessageId sets ProviderMessageId field to given value.

### HasProviderMessageId

`func (o *OutcomeInput) HasProviderMessageId() bool`

HasProviderMessageId returns a boolean if a field has been set.

### SetProviderMessageIdNil

`func (o *OutcomeInput) SetProviderMessageIdNil()`

 SetProviderMessageIdNil sets the value for ProviderMessageId to be an explicit nil

### UnsetProviderMessageId
`func (o *OutcomeInput) UnsetProviderMessageId()`

UnsetProviderMessageId ensures that no value is present for ProviderMessageId, not even an explicit nil

### GetSourceKey

`func (o *OutcomeInput) GetSourceKey() string`

GetSourceKey returns the SourceKey field if non-nil, zero value otherwise.

### GetSourceKeyOk

`func (o *OutcomeInput) GetSourceKeyOk() (*string, bool)`

GetSourceKeyOk returns a tuple with the SourceKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceKey

`func (o *OutcomeInput) SetSourceKey(v string)`

SetSourceKey sets SourceKey field to given value.

### HasSourceKey

`func (o *OutcomeInput) HasSourceKey() bool`

HasSourceKey returns a boolean if a field has been set.

### SetSourceKeyNil

`func (o *OutcomeInput) SetSourceKeyNil()`

 SetSourceKeyNil sets the value for SourceKey to be an explicit nil

### UnsetSourceKey
`func (o *OutcomeInput) UnsetSourceKey()`

UnsetSourceKey ensures that no value is present for SourceKey, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
