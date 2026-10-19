# UpdateTenantSettingsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ApprovalExcellentThreshold** | Pointer to **NullableFloat64** | Safe-to-send percentage for an excellent list. Set it with &#x60;approval_good_threshold&#x60;. | [optional]
**ApprovalGoodThreshold** | Pointer to **NullableFloat64** | Safe-to-send percentage for a good list. Must be below the excellent threshold. | [optional]
**DefaultPolicyMode** | Pointer to **NullableString** |  | [optional]
**DefaultWebhookUrl** | Pointer to **NullableString** |  | [optional]
**ResultRetentionDays** | Pointer to **NullableInt32** |  | [optional]
**WebhookSigningSecret** | Pointer to **NullableString** |  | [optional]

## Methods

### NewUpdateTenantSettingsRequest

`func NewUpdateTenantSettingsRequest() *UpdateTenantSettingsRequest`

NewUpdateTenantSettingsRequest instantiates a new UpdateTenantSettingsRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateTenantSettingsRequestWithDefaults

`func NewUpdateTenantSettingsRequestWithDefaults() *UpdateTenantSettingsRequest`

NewUpdateTenantSettingsRequestWithDefaults instantiates a new UpdateTenantSettingsRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetApprovalExcellentThreshold

`func (o *UpdateTenantSettingsRequest) GetApprovalExcellentThreshold() float64`

GetApprovalExcellentThreshold returns the ApprovalExcellentThreshold field if non-nil, zero value otherwise.

### GetApprovalExcellentThresholdOk

`func (o *UpdateTenantSettingsRequest) GetApprovalExcellentThresholdOk() (*float64, bool)`

GetApprovalExcellentThresholdOk returns a tuple with the ApprovalExcellentThreshold field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetApprovalExcellentThreshold

`func (o *UpdateTenantSettingsRequest) SetApprovalExcellentThreshold(v float64)`

SetApprovalExcellentThreshold sets ApprovalExcellentThreshold field to given value.

### HasApprovalExcellentThreshold

`func (o *UpdateTenantSettingsRequest) HasApprovalExcellentThreshold() bool`

HasApprovalExcellentThreshold returns a boolean if a field has been set.

### SetApprovalExcellentThresholdNil

`func (o *UpdateTenantSettingsRequest) SetApprovalExcellentThresholdNil()`

 SetApprovalExcellentThresholdNil sets the value for ApprovalExcellentThreshold to be an explicit nil

### UnsetApprovalExcellentThreshold
`func (o *UpdateTenantSettingsRequest) UnsetApprovalExcellentThreshold()`

UnsetApprovalExcellentThreshold ensures that no value is present for ApprovalExcellentThreshold, not even an explicit nil

### GetApprovalGoodThreshold

`func (o *UpdateTenantSettingsRequest) GetApprovalGoodThreshold() float64`

GetApprovalGoodThreshold returns the ApprovalGoodThreshold field if non-nil, zero value otherwise.

### GetApprovalGoodThresholdOk

`func (o *UpdateTenantSettingsRequest) GetApprovalGoodThresholdOk() (*float64, bool)`

GetApprovalGoodThresholdOk returns a tuple with the ApprovalGoodThreshold field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetApprovalGoodThreshold

`func (o *UpdateTenantSettingsRequest) SetApprovalGoodThreshold(v float64)`

SetApprovalGoodThreshold sets ApprovalGoodThreshold field to given value.

### HasApprovalGoodThreshold

`func (o *UpdateTenantSettingsRequest) HasApprovalGoodThreshold() bool`

HasApprovalGoodThreshold returns a boolean if a field has been set.

### SetApprovalGoodThresholdNil

`func (o *UpdateTenantSettingsRequest) SetApprovalGoodThresholdNil()`

 SetApprovalGoodThresholdNil sets the value for ApprovalGoodThreshold to be an explicit nil

### UnsetApprovalGoodThreshold
`func (o *UpdateTenantSettingsRequest) UnsetApprovalGoodThreshold()`

UnsetApprovalGoodThreshold ensures that no value is present for ApprovalGoodThreshold, not even an explicit nil

### GetDefaultPolicyMode

`func (o *UpdateTenantSettingsRequest) GetDefaultPolicyMode() string`

GetDefaultPolicyMode returns the DefaultPolicyMode field if non-nil, zero value otherwise.

### GetDefaultPolicyModeOk

`func (o *UpdateTenantSettingsRequest) GetDefaultPolicyModeOk() (*string, bool)`

GetDefaultPolicyModeOk returns a tuple with the DefaultPolicyMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultPolicyMode

`func (o *UpdateTenantSettingsRequest) SetDefaultPolicyMode(v string)`

SetDefaultPolicyMode sets DefaultPolicyMode field to given value.

### HasDefaultPolicyMode

`func (o *UpdateTenantSettingsRequest) HasDefaultPolicyMode() bool`

HasDefaultPolicyMode returns a boolean if a field has been set.

### SetDefaultPolicyModeNil

`func (o *UpdateTenantSettingsRequest) SetDefaultPolicyModeNil()`

 SetDefaultPolicyModeNil sets the value for DefaultPolicyMode to be an explicit nil

### UnsetDefaultPolicyMode
`func (o *UpdateTenantSettingsRequest) UnsetDefaultPolicyMode()`

UnsetDefaultPolicyMode ensures that no value is present for DefaultPolicyMode, not even an explicit nil

### GetDefaultWebhookUrl

`func (o *UpdateTenantSettingsRequest) GetDefaultWebhookUrl() string`

GetDefaultWebhookUrl returns the DefaultWebhookUrl field if non-nil, zero value otherwise.

### GetDefaultWebhookUrlOk

`func (o *UpdateTenantSettingsRequest) GetDefaultWebhookUrlOk() (*string, bool)`

GetDefaultWebhookUrlOk returns a tuple with the DefaultWebhookUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultWebhookUrl

`func (o *UpdateTenantSettingsRequest) SetDefaultWebhookUrl(v string)`

SetDefaultWebhookUrl sets DefaultWebhookUrl field to given value.

### HasDefaultWebhookUrl

`func (o *UpdateTenantSettingsRequest) HasDefaultWebhookUrl() bool`

HasDefaultWebhookUrl returns a boolean if a field has been set.

### SetDefaultWebhookUrlNil

`func (o *UpdateTenantSettingsRequest) SetDefaultWebhookUrlNil()`

 SetDefaultWebhookUrlNil sets the value for DefaultWebhookUrl to be an explicit nil

### UnsetDefaultWebhookUrl
`func (o *UpdateTenantSettingsRequest) UnsetDefaultWebhookUrl()`

UnsetDefaultWebhookUrl ensures that no value is present for DefaultWebhookUrl, not even an explicit nil

### GetResultRetentionDays

`func (o *UpdateTenantSettingsRequest) GetResultRetentionDays() int32`

GetResultRetentionDays returns the ResultRetentionDays field if non-nil, zero value otherwise.

### GetResultRetentionDaysOk

`func (o *UpdateTenantSettingsRequest) GetResultRetentionDaysOk() (*int32, bool)`

GetResultRetentionDaysOk returns a tuple with the ResultRetentionDays field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResultRetentionDays

`func (o *UpdateTenantSettingsRequest) SetResultRetentionDays(v int32)`

SetResultRetentionDays sets ResultRetentionDays field to given value.

### HasResultRetentionDays

`func (o *UpdateTenantSettingsRequest) HasResultRetentionDays() bool`

HasResultRetentionDays returns a boolean if a field has been set.

### SetResultRetentionDaysNil

`func (o *UpdateTenantSettingsRequest) SetResultRetentionDaysNil()`

 SetResultRetentionDaysNil sets the value for ResultRetentionDays to be an explicit nil

### UnsetResultRetentionDays
`func (o *UpdateTenantSettingsRequest) UnsetResultRetentionDays()`

UnsetResultRetentionDays ensures that no value is present for ResultRetentionDays, not even an explicit nil

### GetWebhookSigningSecret

`func (o *UpdateTenantSettingsRequest) GetWebhookSigningSecret() string`

GetWebhookSigningSecret returns the WebhookSigningSecret field if non-nil, zero value otherwise.

### GetWebhookSigningSecretOk

`func (o *UpdateTenantSettingsRequest) GetWebhookSigningSecretOk() (*string, bool)`

GetWebhookSigningSecretOk returns a tuple with the WebhookSigningSecret field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWebhookSigningSecret

`func (o *UpdateTenantSettingsRequest) SetWebhookSigningSecret(v string)`

SetWebhookSigningSecret sets WebhookSigningSecret field to given value.

### HasWebhookSigningSecret

`func (o *UpdateTenantSettingsRequest) HasWebhookSigningSecret() bool`

HasWebhookSigningSecret returns a boolean if a field has been set.

### SetWebhookSigningSecretNil

`func (o *UpdateTenantSettingsRequest) SetWebhookSigningSecretNil()`

 SetWebhookSigningSecretNil sets the value for WebhookSigningSecret to be an explicit nil

### UnsetWebhookSigningSecret
`func (o *UpdateTenantSettingsRequest) UnsetWebhookSigningSecret()`

UnsetWebhookSigningSecret ensures that no value is present for WebhookSigningSecret, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
