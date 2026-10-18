# OutcomeView

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CampaignId** | Pointer to **NullableString** |  | [optional]
**CreatedAt** | **time.Time** |  | [required]
**Email** | **string** |  | [required]
**Id** | **int64** |  | [required]
**OccurredAt** | **time.Time** |  | [required]
**PolicyAction** | Pointer to **NullableString** |  | [optional]
**Source** | Pointer to **NullableString** |  | [optional]
**Type** | **string** |  | [required]

## Methods

### NewOutcomeView

`func NewOutcomeView(createdAt time.Time, email string, id int64, occurredAt time.Time, type_ string) *OutcomeView`

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


### GetPolicyAction

`func (o *OutcomeView) GetPolicyAction() string`

GetPolicyAction returns the PolicyAction field if non-nil, zero value otherwise.

### GetPolicyActionOk

`func (o *OutcomeView) GetPolicyActionOk() (*string, bool)`

GetPolicyActionOk returns a tuple with the PolicyAction field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicyAction

`func (o *OutcomeView) SetPolicyAction(v string)`

SetPolicyAction sets PolicyAction field to given value.

### HasPolicyAction

`func (o *OutcomeView) HasPolicyAction() bool`

HasPolicyAction returns a boolean if a field has been set.

### SetPolicyActionNil

`func (o *OutcomeView) SetPolicyActionNil()`

 SetPolicyActionNil sets the value for PolicyAction to be an explicit nil

### UnsetPolicyAction
`func (o *OutcomeView) UnsetPolicyAction()`

UnsetPolicyAction ensures that no value is present for PolicyAction, not even an explicit nil

### GetSource

`func (o *OutcomeView) GetSource() string`

GetSource returns the Source field if non-nil, zero value otherwise.

### GetSourceOk

`func (o *OutcomeView) GetSourceOk() (*string, bool)`

GetSourceOk returns a tuple with the Source field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSource

`func (o *OutcomeView) SetSource(v string)`

SetSource sets Source field to given value.

### HasSource

`func (o *OutcomeView) HasSource() bool`

HasSource returns a boolean if a field has been set.

### SetSourceNil

`func (o *OutcomeView) SetSourceNil()`

 SetSourceNil sets the value for Source to be an explicit nil

### UnsetSource
`func (o *OutcomeView) UnsetSource()`

UnsetSource ensures that no value is present for Source, not even an explicit nil

### GetType

`func (o *OutcomeView) GetType() string`

GetType returns the Type field if non-nil, zero value otherwise.

### GetTypeOk

`func (o *OutcomeView) GetTypeOk() (*string, bool)`

GetTypeOk returns a tuple with the Type field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetType

`func (o *OutcomeView) SetType(v string)`

SetType sets Type field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
