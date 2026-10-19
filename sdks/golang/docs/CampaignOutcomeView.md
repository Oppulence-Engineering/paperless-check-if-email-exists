# CampaignOutcomeView

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

### NewCampaignOutcomeView

`func NewCampaignOutcomeView(createdAt time.Time, email string, id int64, occurredAt time.Time, type_ string) *CampaignOutcomeView`

NewCampaignOutcomeView instantiates a new CampaignOutcomeView object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCampaignOutcomeViewWithDefaults

`func NewCampaignOutcomeViewWithDefaults() *CampaignOutcomeView`

NewCampaignOutcomeViewWithDefaults instantiates a new CampaignOutcomeView object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCampaignId

`func (o *CampaignOutcomeView) GetCampaignId() string`

GetCampaignId returns the CampaignId field if non-nil, zero value otherwise.

### GetCampaignIdOk

`func (o *CampaignOutcomeView) GetCampaignIdOk() (*string, bool)`

GetCampaignIdOk returns a tuple with the CampaignId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCampaignId

`func (o *CampaignOutcomeView) SetCampaignId(v string)`

SetCampaignId sets CampaignId field to given value.

### HasCampaignId

`func (o *CampaignOutcomeView) HasCampaignId() bool`

HasCampaignId returns a boolean if a field has been set.

### SetCampaignIdNil

`func (o *CampaignOutcomeView) SetCampaignIdNil()`

 SetCampaignIdNil sets the value for CampaignId to be an explicit nil

### UnsetCampaignId
`func (o *CampaignOutcomeView) UnsetCampaignId()`

UnsetCampaignId ensures that no value is present for CampaignId, not even an explicit nil

### GetCreatedAt

`func (o *CampaignOutcomeView) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *CampaignOutcomeView) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *CampaignOutcomeView) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetEmail

`func (o *CampaignOutcomeView) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *CampaignOutcomeView) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *CampaignOutcomeView) SetEmail(v string)`

SetEmail sets Email field to given value.


### GetId

`func (o *CampaignOutcomeView) GetId() int64`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *CampaignOutcomeView) GetIdOk() (*int64, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *CampaignOutcomeView) SetId(v int64)`

SetId sets Id field to given value.


### GetOccurredAt

`func (o *CampaignOutcomeView) GetOccurredAt() time.Time`

GetOccurredAt returns the OccurredAt field if non-nil, zero value otherwise.

### GetOccurredAtOk

`func (o *CampaignOutcomeView) GetOccurredAtOk() (*time.Time, bool)`

GetOccurredAtOk returns a tuple with the OccurredAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOccurredAt

`func (o *CampaignOutcomeView) SetOccurredAt(v time.Time)`

SetOccurredAt sets OccurredAt field to given value.


### GetPolicyAction

`func (o *CampaignOutcomeView) GetPolicyAction() string`

GetPolicyAction returns the PolicyAction field if non-nil, zero value otherwise.

### GetPolicyActionOk

`func (o *CampaignOutcomeView) GetPolicyActionOk() (*string, bool)`

GetPolicyActionOk returns a tuple with the PolicyAction field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicyAction

`func (o *CampaignOutcomeView) SetPolicyAction(v string)`

SetPolicyAction sets PolicyAction field to given value.

### HasPolicyAction

`func (o *CampaignOutcomeView) HasPolicyAction() bool`

HasPolicyAction returns a boolean if a field has been set.

### SetPolicyActionNil

`func (o *CampaignOutcomeView) SetPolicyActionNil()`

 SetPolicyActionNil sets the value for PolicyAction to be an explicit nil

### UnsetPolicyAction
`func (o *CampaignOutcomeView) UnsetPolicyAction()`

UnsetPolicyAction ensures that no value is present for PolicyAction, not even an explicit nil

### GetSource

`func (o *CampaignOutcomeView) GetSource() string`

GetSource returns the Source field if non-nil, zero value otherwise.

### GetSourceOk

`func (o *CampaignOutcomeView) GetSourceOk() (*string, bool)`

GetSourceOk returns a tuple with the Source field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSource

`func (o *CampaignOutcomeView) SetSource(v string)`

SetSource sets Source field to given value.

### HasSource

`func (o *CampaignOutcomeView) HasSource() bool`

HasSource returns a boolean if a field has been set.

### SetSourceNil

`func (o *CampaignOutcomeView) SetSourceNil()`

 SetSourceNil sets the value for Source to be an explicit nil

### UnsetSource
`func (o *CampaignOutcomeView) UnsetSource()`

UnsetSource ensures that no value is present for Source, not even an explicit nil

### GetType

`func (o *CampaignOutcomeView) GetType() string`

GetType returns the Type field if non-nil, zero value otherwise.

### GetTypeOk

`func (o *CampaignOutcomeView) GetTypeOk() (*string, bool)`

GetTypeOk returns a tuple with the Type field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetType

`func (o *CampaignOutcomeView) SetType(v string)`

SetType sets Type field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
