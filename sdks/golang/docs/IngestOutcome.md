# IngestOutcome

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CampaignId** | Pointer to **NullableString** |  | [optional]
**Email** | **string** |  | [required]
**Metadata** | Pointer to **interface{}** |  | [optional]
**OccurredAt** | **time.Time** |  | [required]
**Source** | Pointer to **NullableString** |  | [optional]
**Type** | [**OutcomeType**](OutcomeType.md) |  | [required]

## Methods

### NewIngestOutcome

`func NewIngestOutcome(email string, occurredAt time.Time, type_ OutcomeType) *IngestOutcome`

NewIngestOutcome instantiates a new IngestOutcome object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewIngestOutcomeWithDefaults

`func NewIngestOutcomeWithDefaults() *IngestOutcome`

NewIngestOutcomeWithDefaults instantiates a new IngestOutcome object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCampaignId

`func (o *IngestOutcome) GetCampaignId() string`

GetCampaignId returns the CampaignId field if non-nil, zero value otherwise.

### GetCampaignIdOk

`func (o *IngestOutcome) GetCampaignIdOk() (*string, bool)`

GetCampaignIdOk returns a tuple with the CampaignId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCampaignId

`func (o *IngestOutcome) SetCampaignId(v string)`

SetCampaignId sets CampaignId field to given value.

### HasCampaignId

`func (o *IngestOutcome) HasCampaignId() bool`

HasCampaignId returns a boolean if a field has been set.

### SetCampaignIdNil

`func (o *IngestOutcome) SetCampaignIdNil()`

 SetCampaignIdNil sets the value for CampaignId to be an explicit nil

### UnsetCampaignId
`func (o *IngestOutcome) UnsetCampaignId()`

UnsetCampaignId ensures that no value is present for CampaignId, not even an explicit nil

### GetEmail

`func (o *IngestOutcome) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *IngestOutcome) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *IngestOutcome) SetEmail(v string)`

SetEmail sets Email field to given value.


### GetMetadata

`func (o *IngestOutcome) GetMetadata() interface{}`

GetMetadata returns the Metadata field if non-nil, zero value otherwise.

### GetMetadataOk

`func (o *IngestOutcome) GetMetadataOk() (*interface{}, bool)`

GetMetadataOk returns a tuple with the Metadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMetadata

`func (o *IngestOutcome) SetMetadata(v interface{})`

SetMetadata sets Metadata field to given value.

### HasMetadata

`func (o *IngestOutcome) HasMetadata() bool`

HasMetadata returns a boolean if a field has been set.

### GetOccurredAt

`func (o *IngestOutcome) GetOccurredAt() time.Time`

GetOccurredAt returns the OccurredAt field if non-nil, zero value otherwise.

### GetOccurredAtOk

`func (o *IngestOutcome) GetOccurredAtOk() (*time.Time, bool)`

GetOccurredAtOk returns a tuple with the OccurredAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOccurredAt

`func (o *IngestOutcome) SetOccurredAt(v time.Time)`

SetOccurredAt sets OccurredAt field to given value.


### GetSource

`func (o *IngestOutcome) GetSource() string`

GetSource returns the Source field if non-nil, zero value otherwise.

### GetSourceOk

`func (o *IngestOutcome) GetSourceOk() (*string, bool)`

GetSourceOk returns a tuple with the Source field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSource

`func (o *IngestOutcome) SetSource(v string)`

SetSource sets Source field to given value.

### HasSource

`func (o *IngestOutcome) HasSource() bool`

HasSource returns a boolean if a field has been set.

### SetSourceNil

`func (o *IngestOutcome) SetSourceNil()`

 SetSourceNil sets the value for Source to be an explicit nil

### UnsetSource
`func (o *IngestOutcome) UnsetSource()`

UnsetSource ensures that no value is present for Source, not even an explicit nil

### GetType

`func (o *IngestOutcome) GetType() OutcomeType`

GetType returns the Type field if non-nil, zero value otherwise.

### GetTypeOk

`func (o *IngestOutcome) GetTypeOk() (*OutcomeType, bool)`

GetTypeOk returns a tuple with the Type field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetType

`func (o *IngestOutcome) SetType(v OutcomeType)`

SetType sets Type field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
