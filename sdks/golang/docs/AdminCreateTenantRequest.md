# AdminCreateTenantRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ContactEmail** | **string** |  | [required]
**DefaultWebhookUrl** | Pointer to **NullableString** |  | [optional]
**MaxRequestsPerDay** | Pointer to **int32** |  | [optional]
**MaxRequestsPerHour** | Pointer to **int32** |  | [optional]
**MaxRequestsPerMinute** | Pointer to **int32** |  | [optional]
**MaxRequestsPerSecond** | Pointer to **int32** |  | [optional]
**MonthlyEmailLimit** | Pointer to **int32** |  | [optional]
**Name** | **string** |  | [required]
**PlanTier** | Pointer to **string** |  | [optional]
**ResultRetentionDays** | Pointer to **int32** |  | [optional]
**Slug** | **string** |  | [required]
**WebhookSigningSecret** | Pointer to **NullableString** |  | [optional]

## Methods

### NewAdminCreateTenantRequest

`func NewAdminCreateTenantRequest(contactEmail string, name string, slug string) *AdminCreateTenantRequest`

NewAdminCreateTenantRequest instantiates a new AdminCreateTenantRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAdminCreateTenantRequestWithDefaults

`func NewAdminCreateTenantRequestWithDefaults() *AdminCreateTenantRequest`

NewAdminCreateTenantRequestWithDefaults instantiates a new AdminCreateTenantRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetContactEmail

`func (o *AdminCreateTenantRequest) GetContactEmail() string`

GetContactEmail returns the ContactEmail field if non-nil, zero value otherwise.

### GetContactEmailOk

`func (o *AdminCreateTenantRequest) GetContactEmailOk() (*string, bool)`

GetContactEmailOk returns a tuple with the ContactEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetContactEmail

`func (o *AdminCreateTenantRequest) SetContactEmail(v string)`

SetContactEmail sets ContactEmail field to given value.


### GetDefaultWebhookUrl

`func (o *AdminCreateTenantRequest) GetDefaultWebhookUrl() string`

GetDefaultWebhookUrl returns the DefaultWebhookUrl field if non-nil, zero value otherwise.

### GetDefaultWebhookUrlOk

`func (o *AdminCreateTenantRequest) GetDefaultWebhookUrlOk() (*string, bool)`

GetDefaultWebhookUrlOk returns a tuple with the DefaultWebhookUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultWebhookUrl

`func (o *AdminCreateTenantRequest) SetDefaultWebhookUrl(v string)`

SetDefaultWebhookUrl sets DefaultWebhookUrl field to given value.

### HasDefaultWebhookUrl

`func (o *AdminCreateTenantRequest) HasDefaultWebhookUrl() bool`

HasDefaultWebhookUrl returns a boolean if a field has been set.

### SetDefaultWebhookUrlNil

`func (o *AdminCreateTenantRequest) SetDefaultWebhookUrlNil()`

 SetDefaultWebhookUrlNil sets the value for DefaultWebhookUrl to be an explicit nil

### UnsetDefaultWebhookUrl
`func (o *AdminCreateTenantRequest) UnsetDefaultWebhookUrl()`

UnsetDefaultWebhookUrl ensures that no value is present for DefaultWebhookUrl, not even an explicit nil

### GetMaxRequestsPerDay

`func (o *AdminCreateTenantRequest) GetMaxRequestsPerDay() int32`

GetMaxRequestsPerDay returns the MaxRequestsPerDay field if non-nil, zero value otherwise.

### GetMaxRequestsPerDayOk

`func (o *AdminCreateTenantRequest) GetMaxRequestsPerDayOk() (*int32, bool)`

GetMaxRequestsPerDayOk returns a tuple with the MaxRequestsPerDay field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxRequestsPerDay

`func (o *AdminCreateTenantRequest) SetMaxRequestsPerDay(v int32)`

SetMaxRequestsPerDay sets MaxRequestsPerDay field to given value.

### HasMaxRequestsPerDay

`func (o *AdminCreateTenantRequest) HasMaxRequestsPerDay() bool`

HasMaxRequestsPerDay returns a boolean if a field has been set.

### GetMaxRequestsPerHour

`func (o *AdminCreateTenantRequest) GetMaxRequestsPerHour() int32`

GetMaxRequestsPerHour returns the MaxRequestsPerHour field if non-nil, zero value otherwise.

### GetMaxRequestsPerHourOk

`func (o *AdminCreateTenantRequest) GetMaxRequestsPerHourOk() (*int32, bool)`

GetMaxRequestsPerHourOk returns a tuple with the MaxRequestsPerHour field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxRequestsPerHour

`func (o *AdminCreateTenantRequest) SetMaxRequestsPerHour(v int32)`

SetMaxRequestsPerHour sets MaxRequestsPerHour field to given value.

### HasMaxRequestsPerHour

`func (o *AdminCreateTenantRequest) HasMaxRequestsPerHour() bool`

HasMaxRequestsPerHour returns a boolean if a field has been set.

### GetMaxRequestsPerMinute

`func (o *AdminCreateTenantRequest) GetMaxRequestsPerMinute() int32`

GetMaxRequestsPerMinute returns the MaxRequestsPerMinute field if non-nil, zero value otherwise.

### GetMaxRequestsPerMinuteOk

`func (o *AdminCreateTenantRequest) GetMaxRequestsPerMinuteOk() (*int32, bool)`

GetMaxRequestsPerMinuteOk returns a tuple with the MaxRequestsPerMinute field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxRequestsPerMinute

`func (o *AdminCreateTenantRequest) SetMaxRequestsPerMinute(v int32)`

SetMaxRequestsPerMinute sets MaxRequestsPerMinute field to given value.

### HasMaxRequestsPerMinute

`func (o *AdminCreateTenantRequest) HasMaxRequestsPerMinute() bool`

HasMaxRequestsPerMinute returns a boolean if a field has been set.

### GetMaxRequestsPerSecond

`func (o *AdminCreateTenantRequest) GetMaxRequestsPerSecond() int32`

GetMaxRequestsPerSecond returns the MaxRequestsPerSecond field if non-nil, zero value otherwise.

### GetMaxRequestsPerSecondOk

`func (o *AdminCreateTenantRequest) GetMaxRequestsPerSecondOk() (*int32, bool)`

GetMaxRequestsPerSecondOk returns a tuple with the MaxRequestsPerSecond field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxRequestsPerSecond

`func (o *AdminCreateTenantRequest) SetMaxRequestsPerSecond(v int32)`

SetMaxRequestsPerSecond sets MaxRequestsPerSecond field to given value.

### HasMaxRequestsPerSecond

`func (o *AdminCreateTenantRequest) HasMaxRequestsPerSecond() bool`

HasMaxRequestsPerSecond returns a boolean if a field has been set.

### GetMonthlyEmailLimit

`func (o *AdminCreateTenantRequest) GetMonthlyEmailLimit() int32`

GetMonthlyEmailLimit returns the MonthlyEmailLimit field if non-nil, zero value otherwise.

### GetMonthlyEmailLimitOk

`func (o *AdminCreateTenantRequest) GetMonthlyEmailLimitOk() (*int32, bool)`

GetMonthlyEmailLimitOk returns a tuple with the MonthlyEmailLimit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMonthlyEmailLimit

`func (o *AdminCreateTenantRequest) SetMonthlyEmailLimit(v int32)`

SetMonthlyEmailLimit sets MonthlyEmailLimit field to given value.

### HasMonthlyEmailLimit

`func (o *AdminCreateTenantRequest) HasMonthlyEmailLimit() bool`

HasMonthlyEmailLimit returns a boolean if a field has been set.

### GetName

`func (o *AdminCreateTenantRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *AdminCreateTenantRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *AdminCreateTenantRequest) SetName(v string)`

SetName sets Name field to given value.


### GetPlanTier

`func (o *AdminCreateTenantRequest) GetPlanTier() string`

GetPlanTier returns the PlanTier field if non-nil, zero value otherwise.

### GetPlanTierOk

`func (o *AdminCreateTenantRequest) GetPlanTierOk() (*string, bool)`

GetPlanTierOk returns a tuple with the PlanTier field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPlanTier

`func (o *AdminCreateTenantRequest) SetPlanTier(v string)`

SetPlanTier sets PlanTier field to given value.

### HasPlanTier

`func (o *AdminCreateTenantRequest) HasPlanTier() bool`

HasPlanTier returns a boolean if a field has been set.

### GetResultRetentionDays

`func (o *AdminCreateTenantRequest) GetResultRetentionDays() int32`

GetResultRetentionDays returns the ResultRetentionDays field if non-nil, zero value otherwise.

### GetResultRetentionDaysOk

`func (o *AdminCreateTenantRequest) GetResultRetentionDaysOk() (*int32, bool)`

GetResultRetentionDaysOk returns a tuple with the ResultRetentionDays field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResultRetentionDays

`func (o *AdminCreateTenantRequest) SetResultRetentionDays(v int32)`

SetResultRetentionDays sets ResultRetentionDays field to given value.

### HasResultRetentionDays

`func (o *AdminCreateTenantRequest) HasResultRetentionDays() bool`

HasResultRetentionDays returns a boolean if a field has been set.

### GetSlug

`func (o *AdminCreateTenantRequest) GetSlug() string`

GetSlug returns the Slug field if non-nil, zero value otherwise.

### GetSlugOk

`func (o *AdminCreateTenantRequest) GetSlugOk() (*string, bool)`

GetSlugOk returns a tuple with the Slug field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSlug

`func (o *AdminCreateTenantRequest) SetSlug(v string)`

SetSlug sets Slug field to given value.


### GetWebhookSigningSecret

`func (o *AdminCreateTenantRequest) GetWebhookSigningSecret() string`

GetWebhookSigningSecret returns the WebhookSigningSecret field if non-nil, zero value otherwise.

### GetWebhookSigningSecretOk

`func (o *AdminCreateTenantRequest) GetWebhookSigningSecretOk() (*string, bool)`

GetWebhookSigningSecretOk returns a tuple with the WebhookSigningSecret field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWebhookSigningSecret

`func (o *AdminCreateTenantRequest) SetWebhookSigningSecret(v string)`

SetWebhookSigningSecret sets WebhookSigningSecret field to given value.

### HasWebhookSigningSecret

`func (o *AdminCreateTenantRequest) HasWebhookSigningSecret() bool`

HasWebhookSigningSecret returns a boolean if a field has been set.

### SetWebhookSigningSecretNil

`func (o *AdminCreateTenantRequest) SetWebhookSigningSecretNil()`

 SetWebhookSigningSecretNil sets the value for WebhookSigningSecret to be an explicit nil

### UnsetWebhookSigningSecret
`func (o *AdminCreateTenantRequest) UnsetWebhookSigningSecret()`

UnsetWebhookSigningSecret ensures that no value is present for WebhookSigningSecret, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
