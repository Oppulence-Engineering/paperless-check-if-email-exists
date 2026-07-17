# OutcomeView

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CampaignId** | Pointer to **NullableString** |  | [optional]
**CanonicalEmail** | **string** |  | [required]
**CorrelationStatus** | **string** |  | [required]
**CreatedAt** | **time.Time** |  | [required]
**Email** | **string** |  | [required]
**EndpointId** | Pointer to **NullableString** |  | [optional]
**EventFamily** | Pointer to **NullableString** |  | [optional]
**EventType** | **string** |  | [required]
**Id** | **int64** |  | [required]
**Metadata** | **interface{}** |  | [required]
**OccurredAt** | **time.Time** |  | [required]
**Provider** | **string** |  | [required]
**ProviderEventId** | Pointer to **NullableString** |  | [optional]
**ProviderMessageId** | Pointer to **NullableString** |  | [optional]
**ReceiptId** | Pointer to **NullableString** |  | [optional]
**SourceKey** | Pointer to **NullableString** |  | [optional]

## Methods

### NewOutcomeView

`func NewOutcomeView(canonicalEmail string, correlationStatus string, createdAt time.Time, email string, eventType string, id int64, metadata interface{}, occurredAt time.Time, provider string) *OutcomeView`

NewOutcomeView instantiates a new OutcomeView object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOutcomeViewWithDefaults

`func NewOutcomeViewWithDefaults() *OutcomeView`

NewOutcomeViewWithDefaults instantiates a new OutcomeView object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCampaignId

`func (o *OutcomeView) GetCampaignId() string`

GetCampaignId returns the CampaignId field if non-nil, zero value otherwise.

### GetCampaignIdOk

`func (o *OutcomeView) GetCampaignIdOk() (*string, bool)`

GetCampaignIdOk returns a tuple with the CampaignId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCampaignId

`func (o *OutcomeView) SetCampaignId(v string)`

SetCampaignId sets CampaignId field to given value.

### HasCampaignId

`func (o *OutcomeView) HasCampaignId() bool`

HasCampaignId returns a boolean if a field has been set.

### SetCampaignIdNil

`func (o *OutcomeView) SetCampaignIdNil()`

 SetCampaignIdNil sets the value for CampaignId to be an explicit nil

### UnsetCampaignId
`func (o *OutcomeView) UnsetCampaignId()`

UnsetCampaignId ensures that no value is present for CampaignId, not even an explicit nil

### GetCanonicalEmail

`func (o *OutcomeView) GetCanonicalEmail() string`

GetCanonicalEmail returns the CanonicalEmail field if non-nil, zero value otherwise.

### GetCanonicalEmailOk

`func (o *OutcomeView) GetCanonicalEmailOk() (*string, bool)`

GetCanonicalEmailOk returns a tuple with the CanonicalEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCanonicalEmail

`func (o *OutcomeView) SetCanonicalEmail(v string)`

SetCanonicalEmail sets CanonicalEmail field to given value.


### GetCorrelationStatus

`func (o *OutcomeView) GetCorrelationStatus() string`

GetCorrelationStatus returns the CorrelationStatus field if non-nil, zero value otherwise.

### GetCorrelationStatusOk

`func (o *OutcomeView) GetCorrelationStatusOk() (*string, bool)`

GetCorrelationStatusOk returns a tuple with the CorrelationStatus field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCorrelationStatus

`func (o *OutcomeView) SetCorrelationStatus(v string)`

SetCorrelationStatus sets CorrelationStatus field to given value.


### GetCreatedAt

`func (o *OutcomeView) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *OutcomeView) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *OutcomeView) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetEmail

`func (o *OutcomeView) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *OutcomeView) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *OutcomeView) SetEmail(v string)`

SetEmail sets Email field to given value.


### GetEndpointId

`func (o *OutcomeView) GetEndpointId() string`

GetEndpointId returns the EndpointId field if non-nil, zero value otherwise.

### GetEndpointIdOk

`func (o *OutcomeView) GetEndpointIdOk() (*string, bool)`

GetEndpointIdOk returns a tuple with the EndpointId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEndpointId

`func (o *OutcomeView) SetEndpointId(v string)`

SetEndpointId sets EndpointId field to given value.

### HasEndpointId

`func (o *OutcomeView) HasEndpointId() bool`

HasEndpointId returns a boolean if a field has been set.

### SetEndpointIdNil

`func (o *OutcomeView) SetEndpointIdNil()`

 SetEndpointIdNil sets the value for EndpointId to be an explicit nil

### UnsetEndpointId
`func (o *OutcomeView) UnsetEndpointId()`

UnsetEndpointId ensures that no value is present for EndpointId, not even an explicit nil

### GetEventFamily

`func (o *OutcomeView) GetEventFamily() string`

GetEventFamily returns the EventFamily field if non-nil, zero value otherwise.

### GetEventFamilyOk

`func (o *OutcomeView) GetEventFamilyOk() (*string, bool)`

GetEventFamilyOk returns a tuple with the EventFamily field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEventFamily

`func (o *OutcomeView) SetEventFamily(v string)`

SetEventFamily sets EventFamily field to given value.

### HasEventFamily

`func (o *OutcomeView) HasEventFamily() bool`

HasEventFamily returns a boolean if a field has been set.

### SetEventFamilyNil

`func (o *OutcomeView) SetEventFamilyNil()`

 SetEventFamilyNil sets the value for EventFamily to be an explicit nil

### UnsetEventFamily
`func (o *OutcomeView) UnsetEventFamily()`

UnsetEventFamily ensures that no value is present for EventFamily, not even an explicit nil

### GetEventType

`func (o *OutcomeView) GetEventType() string`

GetEventType returns the EventType field if non-nil, zero value otherwise.

### GetEventTypeOk

`func (o *OutcomeView) GetEventTypeOk() (*string, bool)`

GetEventTypeOk returns a tuple with the EventType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEventType

`func (o *OutcomeView) SetEventType(v string)`

SetEventType sets EventType field to given value.


### GetId

`func (o *OutcomeView) GetId() int64`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *OutcomeView) GetIdOk() (*int64, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *OutcomeView) SetId(v int64)`

SetId sets Id field to given value.


### GetMetadata

`func (o *OutcomeView) GetMetadata() interface{}`

GetMetadata returns the Metadata field if non-nil, zero value otherwise.

### GetMetadataOk

`func (o *OutcomeView) GetMetadataOk() (*interface{}, bool)`

GetMetadataOk returns a tuple with the Metadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMetadata

`func (o *OutcomeView) SetMetadata(v interface{})`

SetMetadata sets Metadata field to given value.


### GetOccurredAt

`func (o *OutcomeView) GetOccurredAt() time.Time`

GetOccurredAt returns the OccurredAt field if non-nil, zero value otherwise.

### GetOccurredAtOk

`func (o *OutcomeView) GetOccurredAtOk() (*time.Time, bool)`

GetOccurredAtOk returns a tuple with the OccurredAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOccurredAt

`func (o *OutcomeView) SetOccurredAt(v time.Time)`

SetOccurredAt sets OccurredAt field to given value.


### GetProvider

`func (o *OutcomeView) GetProvider() string`

GetProvider returns the Provider field if non-nil, zero value otherwise.

### GetProviderOk

`func (o *OutcomeView) GetProviderOk() (*string, bool)`

GetProviderOk returns a tuple with the Provider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProvider

`func (o *OutcomeView) SetProvider(v string)`

SetProvider sets Provider field to given value.


### GetProviderEventId

`func (o *OutcomeView) GetProviderEventId() string`

GetProviderEventId returns the ProviderEventId field if non-nil, zero value otherwise.

### GetProviderEventIdOk

`func (o *OutcomeView) GetProviderEventIdOk() (*string, bool)`

GetProviderEventIdOk returns a tuple with the ProviderEventId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderEventId

`func (o *OutcomeView) SetProviderEventId(v string)`

SetProviderEventId sets ProviderEventId field to given value.

### HasProviderEventId

`func (o *OutcomeView) HasProviderEventId() bool`

HasProviderEventId returns a boolean if a field has been set.

### SetProviderEventIdNil

`func (o *OutcomeView) SetProviderEventIdNil()`

 SetProviderEventIdNil sets the value for ProviderEventId to be an explicit nil

### UnsetProviderEventId
`func (o *OutcomeView) UnsetProviderEventId()`

UnsetProviderEventId ensures that no value is present for ProviderEventId, not even an explicit nil

### GetProviderMessageId

`func (o *OutcomeView) GetProviderMessageId() string`

GetProviderMessageId returns the ProviderMessageId field if non-nil, zero value otherwise.

### GetProviderMessageIdOk

`func (o *OutcomeView) GetProviderMessageIdOk() (*string, bool)`

GetProviderMessageIdOk returns a tuple with the ProviderMessageId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderMessageId

`func (o *OutcomeView) SetProviderMessageId(v string)`

SetProviderMessageId sets ProviderMessageId field to given value.

### HasProviderMessageId

`func (o *OutcomeView) HasProviderMessageId() bool`

HasProviderMessageId returns a boolean if a field has been set.

### SetProviderMessageIdNil

`func (o *OutcomeView) SetProviderMessageIdNil()`

 SetProviderMessageIdNil sets the value for ProviderMessageId to be an explicit nil

### UnsetProviderMessageId
`func (o *OutcomeView) UnsetProviderMessageId()`

UnsetProviderMessageId ensures that no value is present for ProviderMessageId, not even an explicit nil

### GetReceiptId

`func (o *OutcomeView) GetReceiptId() string`

GetReceiptId returns the ReceiptId field if non-nil, zero value otherwise.

### GetReceiptIdOk

`func (o *OutcomeView) GetReceiptIdOk() (*string, bool)`

GetReceiptIdOk returns a tuple with the ReceiptId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReceiptId

`func (o *OutcomeView) SetReceiptId(v string)`

SetReceiptId sets ReceiptId field to given value.

### HasReceiptId

`func (o *OutcomeView) HasReceiptId() bool`

HasReceiptId returns a boolean if a field has been set.

### SetReceiptIdNil

`func (o *OutcomeView) SetReceiptIdNil()`

 SetReceiptIdNil sets the value for ReceiptId to be an explicit nil

### UnsetReceiptId
`func (o *OutcomeView) UnsetReceiptId()`

UnsetReceiptId ensures that no value is present for ReceiptId, not even an explicit nil

### GetSourceKey

`func (o *OutcomeView) GetSourceKey() string`

GetSourceKey returns the SourceKey field if non-nil, zero value otherwise.

### GetSourceKeyOk

`func (o *OutcomeView) GetSourceKeyOk() (*string, bool)`

GetSourceKeyOk returns a tuple with the SourceKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceKey

`func (o *OutcomeView) SetSourceKey(v string)`

SetSourceKey sets SourceKey field to given value.

### HasSourceKey

`func (o *OutcomeView) HasSourceKey() bool`

HasSourceKey returns a boolean if a field has been set.

### SetSourceKeyNil

`func (o *OutcomeView) SetSourceKeyNil()`

 SetSourceKeyNil sets the value for SourceKey to be an explicit nil

### UnsetSourceKey
`func (o *OutcomeView) UnsetSourceKey()`

UnsetSourceKey ensures that no value is present for SourceKey, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
