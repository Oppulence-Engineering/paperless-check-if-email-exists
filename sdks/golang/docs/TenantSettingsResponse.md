# TenantSettingsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DefaultPolicyMode** | **string** |  | [required]
**DefaultWebhookUrl** | Pointer to **NullableString** |  | [optional]
**MonthlyEmailLimit** | Pointer to **NullableInt32** |  | [optional]
**Name** | **string** |  | [required]
**PeriodResetAt** | **string** |  | [required]
**ResultRetentionDays** | **int32** |  | [required]
**Slug** | **string** |  | [required]
**TenantId** | **string** |  | [required]
**UsedThisPeriod** | **int32** |  | [required]

## Methods

### NewTenantSettingsResponse

`func NewTenantSettingsResponse(defaultPolicyMode string, name string, periodResetAt string, resultRetentionDays int32, slug string, tenantId string, usedThisPeriod int32) *TenantSettingsResponse`

NewTenantSettingsResponse instantiates a new TenantSettingsResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewTenantSettingsResponseWithDefaults

`func NewTenantSettingsResponseWithDefaults() *TenantSettingsResponse`

NewTenantSettingsResponseWithDefaults instantiates a new TenantSettingsResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDefaultPolicyMode

`func (o *TenantSettingsResponse) GetDefaultPolicyMode() string`

GetDefaultPolicyMode returns the DefaultPolicyMode field if non-nil, zero value otherwise.

### GetDefaultPolicyModeOk

`func (o *TenantSettingsResponse) GetDefaultPolicyModeOk() (*string, bool)`

GetDefaultPolicyModeOk returns a tuple with the DefaultPolicyMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultPolicyMode

`func (o *TenantSettingsResponse) SetDefaultPolicyMode(v string)`

SetDefaultPolicyMode sets DefaultPolicyMode field to given value.


### GetDefaultWebhookUrl

`func (o *TenantSettingsResponse) GetDefaultWebhookUrl() string`

GetDefaultWebhookUrl returns the DefaultWebhookUrl field if non-nil, zero value otherwise.

### GetDefaultWebhookUrlOk

`func (o *TenantSettingsResponse) GetDefaultWebhookUrlOk() (*string, bool)`

GetDefaultWebhookUrlOk returns a tuple with the DefaultWebhookUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultWebhookUrl

`func (o *TenantSettingsResponse) SetDefaultWebhookUrl(v string)`

SetDefaultWebhookUrl sets DefaultWebhookUrl field to given value.

### HasDefaultWebhookUrl

`func (o *TenantSettingsResponse) HasDefaultWebhookUrl() bool`

HasDefaultWebhookUrl returns a boolean if a field has been set.

### SetDefaultWebhookUrlNil

`func (o *TenantSettingsResponse) SetDefaultWebhookUrlNil()`

 SetDefaultWebhookUrlNil sets the value for DefaultWebhookUrl to be an explicit nil

### UnsetDefaultWebhookUrl
`func (o *TenantSettingsResponse) UnsetDefaultWebhookUrl()`

UnsetDefaultWebhookUrl ensures that no value is present for DefaultWebhookUrl, not even an explicit nil

### GetMonthlyEmailLimit

`func (o *TenantSettingsResponse) GetMonthlyEmailLimit() int32`

GetMonthlyEmailLimit returns the MonthlyEmailLimit field if non-nil, zero value otherwise.

### GetMonthlyEmailLimitOk

`func (o *TenantSettingsResponse) GetMonthlyEmailLimitOk() (*int32, bool)`

GetMonthlyEmailLimitOk returns a tuple with the MonthlyEmailLimit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMonthlyEmailLimit

`func (o *TenantSettingsResponse) SetMonthlyEmailLimit(v int32)`

SetMonthlyEmailLimit sets MonthlyEmailLimit field to given value.

### HasMonthlyEmailLimit

`func (o *TenantSettingsResponse) HasMonthlyEmailLimit() bool`

HasMonthlyEmailLimit returns a boolean if a field has been set.

### SetMonthlyEmailLimitNil

`func (o *TenantSettingsResponse) SetMonthlyEmailLimitNil()`

 SetMonthlyEmailLimitNil sets the value for MonthlyEmailLimit to be an explicit nil

### UnsetMonthlyEmailLimit
`func (o *TenantSettingsResponse) UnsetMonthlyEmailLimit()`

UnsetMonthlyEmailLimit ensures that no value is present for MonthlyEmailLimit, not even an explicit nil

### GetName

`func (o *TenantSettingsResponse) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *TenantSettingsResponse) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *TenantSettingsResponse) SetName(v string)`

SetName sets Name field to given value.


### GetPeriodResetAt

`func (o *TenantSettingsResponse) GetPeriodResetAt() string`

GetPeriodResetAt returns the PeriodResetAt field if non-nil, zero value otherwise.

### GetPeriodResetAtOk

`func (o *TenantSettingsResponse) GetPeriodResetAtOk() (*string, bool)`

GetPeriodResetAtOk returns a tuple with the PeriodResetAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPeriodResetAt

`func (o *TenantSettingsResponse) SetPeriodResetAt(v string)`

SetPeriodResetAt sets PeriodResetAt field to given value.


### GetResultRetentionDays

`func (o *TenantSettingsResponse) GetResultRetentionDays() int32`

GetResultRetentionDays returns the ResultRetentionDays field if non-nil, zero value otherwise.

### GetResultRetentionDaysOk

`func (o *TenantSettingsResponse) GetResultRetentionDaysOk() (*int32, bool)`

GetResultRetentionDaysOk returns a tuple with the ResultRetentionDays field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResultRetentionDays

`func (o *TenantSettingsResponse) SetResultRetentionDays(v int32)`

SetResultRetentionDays sets ResultRetentionDays field to given value.


### GetSlug

`func (o *TenantSettingsResponse) GetSlug() string`

GetSlug returns the Slug field if non-nil, zero value otherwise.

### GetSlugOk

`func (o *TenantSettingsResponse) GetSlugOk() (*string, bool)`

GetSlugOk returns a tuple with the Slug field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSlug

`func (o *TenantSettingsResponse) SetSlug(v string)`

SetSlug sets Slug field to given value.


### GetTenantId

`func (o *TenantSettingsResponse) GetTenantId() string`

GetTenantId returns the TenantId field if non-nil, zero value otherwise.

### GetTenantIdOk

`func (o *TenantSettingsResponse) GetTenantIdOk() (*string, bool)`

GetTenantIdOk returns a tuple with the TenantId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantId

`func (o *TenantSettingsResponse) SetTenantId(v string)`

SetTenantId sets TenantId field to given value.


### GetUsedThisPeriod

`func (o *TenantSettingsResponse) GetUsedThisPeriod() int32`

GetUsedThisPeriod returns the UsedThisPeriod field if non-nil, zero value otherwise.

### GetUsedThisPeriodOk

`func (o *TenantSettingsResponse) GetUsedThisPeriodOk() (*int32, bool)`

GetUsedThisPeriodOk returns a tuple with the UsedThisPeriod field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUsedThisPeriod

`func (o *TenantSettingsResponse) SetUsedThisPeriod(v int32)`

SetUsedThisPeriod sets UsedThisPeriod field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
