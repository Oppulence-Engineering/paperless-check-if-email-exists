# AdminUpdateTenantRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Name** | Pointer to **string** |  | [optional]
**ContactEmail** | Pointer to **string** |  | [optional]
**PlanTier** | Pointer to **string** |  | [optional]
**Status** | Pointer to **string** |  | [optional]
**MonthlyEmailLimit** | Pointer to **NullableInt32** |  | [optional]
**MaxRequestsPerSecond** | Pointer to **int32** |  | [optional]
**MaxRequestsPerMinute** | Pointer to **int32** |  | [optional]
**MaxRequestsPerHour** | Pointer to **int32** |  | [optional]
**MaxRequestsPerDay** | Pointer to **int32** |  | [optional]
**DefaultWebhookUrl** | Pointer to **NullableString** |  | [optional]
**WebhookSigningSecret** | Pointer to **NullableString** |  | [optional]
**ResultRetentionDays** | Pointer to **int32** |  | [optional]

## Methods

### NewAdminUpdateTenantRequest

`func NewAdminUpdateTenantRequest() *AdminUpdateTenantRequest`

NewAdminUpdateTenantRequest instantiates a new AdminUpdateTenantRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAdminUpdateTenantRequestWithDefaults

`func NewAdminUpdateTenantRequestWithDefaults() *AdminUpdateTenantRequest`

NewAdminUpdateTenantRequestWithDefaults instantiates a new AdminUpdateTenantRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetName

`func (o *AdminUpdateTenantRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *AdminUpdateTenantRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *AdminUpdateTenantRequest) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *AdminUpdateTenantRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### GetContactEmail

`func (o *AdminUpdateTenantRequest) GetContactEmail() string`

GetContactEmail returns the ContactEmail field if non-nil, zero value otherwise.

### GetContactEmailOk

`func (o *AdminUpdateTenantRequest) GetContactEmailOk() (*string, bool)`

GetContactEmailOk returns a tuple with the ContactEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetContactEmail

`func (o *AdminUpdateTenantRequest) SetContactEmail(v string)`

SetContactEmail sets ContactEmail field to given value.

### HasContactEmail

`func (o *AdminUpdateTenantRequest) HasContactEmail() bool`

HasContactEmail returns a boolean if a field has been set.

### GetPlanTier

`func (o *AdminUpdateTenantRequest) GetPlanTier() string`

GetPlanTier returns the PlanTier field if non-nil, zero value otherwise.

### GetPlanTierOk

`func (o *AdminUpdateTenantRequest) GetPlanTierOk() (*string, bool)`

GetPlanTierOk returns a tuple with the PlanTier field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPlanTier

`func (o *AdminUpdateTenantRequest) SetPlanTier(v string)`

SetPlanTier sets PlanTier field to given value.

### HasPlanTier

`func (o *AdminUpdateTenantRequest) HasPlanTier() bool`

HasPlanTier returns a boolean if a field has been set.

### GetStatus

`func (o *AdminUpdateTenantRequest) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *AdminUpdateTenantRequest) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *AdminUpdateTenantRequest) SetStatus(v string)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *AdminUpdateTenantRequest) HasStatus() bool`

HasStatus returns a boolean if a field has been set.

### GetMonthlyEmailLimit

`func (o *AdminUpdateTenantRequest) GetMonthlyEmailLimit() int32`

GetMonthlyEmailLimit returns the MonthlyEmailLimit field if non-nil, zero value otherwise.

### GetMonthlyEmailLimitOk

`func (o *AdminUpdateTenantRequest) GetMonthlyEmailLimitOk() (*int32, bool)`

GetMonthlyEmailLimitOk returns a tuple with the MonthlyEmailLimit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMonthlyEmailLimit

`func (o *AdminUpdateTenantRequest) SetMonthlyEmailLimit(v int32)`

SetMonthlyEmailLimit sets MonthlyEmailLimit field to given value.

### HasMonthlyEmailLimit

`func (o *AdminUpdateTenantRequest) HasMonthlyEmailLimit() bool`

HasMonthlyEmailLimit returns a boolean if a field has been set.

### SetMonthlyEmailLimitNil

`func (o *AdminUpdateTenantRequest) SetMonthlyEmailLimitNil()`

 SetMonthlyEmailLimitNil sets the value for MonthlyEmailLimit to be an explicit nil

### UnsetMonthlyEmailLimit
`func (o *AdminUpdateTenantRequest) UnsetMonthlyEmailLimit()`

UnsetMonthlyEmailLimit ensures that no value is present for MonthlyEmailLimit, not even an explicit nil

### GetMaxRequestsPerSecond

`func (o *AdminUpdateTenantRequest) GetMaxRequestsPerSecond() int32`

GetMaxRequestsPerSecond returns the MaxRequestsPerSecond field if non-nil, zero value otherwise.

### GetMaxRequestsPerSecondOk

`func (o *AdminUpdateTenantRequest) GetMaxRequestsPerSecondOk() (*int32, bool)`

GetMaxRequestsPerSecondOk returns a tuple with the MaxRequestsPerSecond field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxRequestsPerSecond

`func (o *AdminUpdateTenantRequest) SetMaxRequestsPerSecond(v int32)`

SetMaxRequestsPerSecond sets MaxRequestsPerSecond field to given value.

### HasMaxRequestsPerSecond

`func (o *AdminUpdateTenantRequest) HasMaxRequestsPerSecond() bool`

HasMaxRequestsPerSecond returns a boolean if a field has been set.

### GetMaxRequestsPerMinute

`func (o *AdminUpdateTenantRequest) GetMaxRequestsPerMinute() int32`

GetMaxRequestsPerMinute returns the MaxRequestsPerMinute field if non-nil, zero value otherwise.

### GetMaxRequestsPerMinuteOk

`func (o *AdminUpdateTenantRequest) GetMaxRequestsPerMinuteOk() (*int32, bool)`

GetMaxRequestsPerMinuteOk returns a tuple with the MaxRequestsPerMinute field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxRequestsPerMinute

`func (o *AdminUpdateTenantRequest) SetMaxRequestsPerMinute(v int32)`

SetMaxRequestsPerMinute sets MaxRequestsPerMinute field to given value.

### HasMaxRequestsPerMinute

`func (o *AdminUpdateTenantRequest) HasMaxRequestsPerMinute() bool`

HasMaxRequestsPerMinute returns a boolean if a field has been set.

### GetMaxRequestsPerHour

`func (o *AdminUpdateTenantRequest) GetMaxRequestsPerHour() int32`

GetMaxRequestsPerHour returns the MaxRequestsPerHour field if non-nil, zero value otherwise.

### GetMaxRequestsPerHourOk

`func (o *AdminUpdateTenantRequest) GetMaxRequestsPerHourOk() (*int32, bool)`

GetMaxRequestsPerHourOk returns a tuple with the MaxRequestsPerHour field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxRequestsPerHour

`func (o *AdminUpdateTenantRequest) SetMaxRequestsPerHour(v int32)`

SetMaxRequestsPerHour sets MaxRequestsPerHour field to given value.

### HasMaxRequestsPerHour

`func (o *AdminUpdateTenantRequest) HasMaxRequestsPerHour() bool`

HasMaxRequestsPerHour returns a boolean if a field has been set.

### GetMaxRequestsPerDay

`func (o *AdminUpdateTenantRequest) GetMaxRequestsPerDay() int32`

GetMaxRequestsPerDay returns the MaxRequestsPerDay field if non-nil, zero value otherwise.

### GetMaxRequestsPerDayOk

`func (o *AdminUpdateTenantRequest) GetMaxRequestsPerDayOk() (*int32, bool)`

GetMaxRequestsPerDayOk returns a tuple with the MaxRequestsPerDay field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxRequestsPerDay

`func (o *AdminUpdateTenantRequest) SetMaxRequestsPerDay(v int32)`

SetMaxRequestsPerDay sets MaxRequestsPerDay field to given value.

### HasMaxRequestsPerDay

`func (o *AdminUpdateTenantRequest) HasMaxRequestsPerDay() bool`

HasMaxRequestsPerDay returns a boolean if a field has been set.

### GetDefaultWebhookUrl

`func (o *AdminUpdateTenantRequest) GetDefaultWebhookUrl() string`

GetDefaultWebhookUrl returns the DefaultWebhookUrl field if non-nil, zero value otherwise.

### GetDefaultWebhookUrlOk

`func (o *AdminUpdateTenantRequest) GetDefaultWebhookUrlOk() (*string, bool)`

GetDefaultWebhookUrlOk returns a tuple with the DefaultWebhookUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultWebhookUrl

`func (o *AdminUpdateTenantRequest) SetDefaultWebhookUrl(v string)`

SetDefaultWebhookUrl sets DefaultWebhookUrl field to given value.

### HasDefaultWebhookUrl

`func (o *AdminUpdateTenantRequest) HasDefaultWebhookUrl() bool`

HasDefaultWebhookUrl returns a boolean if a field has been set.

### SetDefaultWebhookUrlNil

`func (o *AdminUpdateTenantRequest) SetDefaultWebhookUrlNil()`

 SetDefaultWebhookUrlNil sets the value for DefaultWebhookUrl to be an explicit nil

### UnsetDefaultWebhookUrl
`func (o *AdminUpdateTenantRequest) UnsetDefaultWebhookUrl()`

UnsetDefaultWebhookUrl ensures that no value is present for DefaultWebhookUrl, not even an explicit nil

### GetWebhookSigningSecret

`func (o *AdminUpdateTenantRequest) GetWebhookSigningSecret() string`

GetWebhookSigningSecret returns the WebhookSigningSecret field if non-nil, zero value otherwise.

### GetWebhookSigningSecretOk

`func (o *AdminUpdateTenantRequest) GetWebhookSigningSecretOk() (*string, bool)`

GetWebhookSigningSecretOk returns a tuple with the WebhookSigningSecret field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWebhookSigningSecret

`func (o *AdminUpdateTenantRequest) SetWebhookSigningSecret(v string)`

SetWebhookSigningSecret sets WebhookSigningSecret field to given value.

### HasWebhookSigningSecret

`func (o *AdminUpdateTenantRequest) HasWebhookSigningSecret() bool`

HasWebhookSigningSecret returns a boolean if a field has been set.

### SetWebhookSigningSecretNil

`func (o *AdminUpdateTenantRequest) SetWebhookSigningSecretNil()`

 SetWebhookSigningSecretNil sets the value for WebhookSigningSecret to be an explicit nil

### UnsetWebhookSigningSecret
`func (o *AdminUpdateTenantRequest) UnsetWebhookSigningSecret()`

UnsetWebhookSigningSecret ensures that no value is present for WebhookSigningSecret, not even an explicit nil

### GetResultRetentionDays

`func (o *AdminUpdateTenantRequest) GetResultRetentionDays() int32`

GetResultRetentionDays returns the ResultRetentionDays field if non-nil, zero value otherwise.

### GetResultRetentionDaysOk

`func (o *AdminUpdateTenantRequest) GetResultRetentionDaysOk() (*int32, bool)`

GetResultRetentionDaysOk returns a tuple with the ResultRetentionDays field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResultRetentionDays

`func (o *AdminUpdateTenantRequest) SetResultRetentionDays(v int32)`

SetResultRetentionDays sets ResultRetentionDays field to given value.

### HasResultRetentionDays

`func (o *AdminUpdateTenantRequest) HasResultRetentionDays() bool`

HasResultRetentionDays returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
