# AdminTenant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | Pointer to **string** |  | [optional]
**Name** | Pointer to **string** |  | [optional]
**Slug** | Pointer to **string** |  | [optional]
**ContactEmail** | Pointer to **string** |  | [optional]
**PlanTier** | Pointer to **string** |  | [optional]
**Status** | Pointer to **string** |  | [optional]
**MonthlyEmailLimit** | Pointer to **NullableInt32** |  | [optional]
**MaxRequestsPerSecond** | Pointer to **int32** |  | [optional]
**MaxRequestsPerMinute** | Pointer to **int32** |  | [optional]
**MaxRequestsPerHour** | Pointer to **int32** |  | [optional]
**MaxRequestsPerDay** | Pointer to **int32** |  | [optional]
**UsedThisPeriod** | Pointer to **int32** |  | [optional]
**DefaultWebhookUrl** | Pointer to **NullableString** |  | [optional]
**ResultRetentionDays** | Pointer to **int32** |  | [optional]
**CreatedAt** | Pointer to **time.Time** |  | [optional]
**UpdatedAt** | Pointer to **time.Time** |  | [optional]

## Methods

### NewAdminTenant

`func NewAdminTenant() *AdminTenant`

NewAdminTenant instantiates a new AdminTenant object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAdminTenantWithDefaults

`func NewAdminTenantWithDefaults() *AdminTenant`

NewAdminTenantWithDefaults instantiates a new AdminTenant object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *AdminTenant) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *AdminTenant) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *AdminTenant) SetId(v string)`

SetId sets Id field to given value.

### HasId

`func (o *AdminTenant) HasId() bool`

HasId returns a boolean if a field has been set.

### GetName

`func (o *AdminTenant) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *AdminTenant) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *AdminTenant) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *AdminTenant) HasName() bool`

HasName returns a boolean if a field has been set.

### GetSlug

`func (o *AdminTenant) GetSlug() string`

GetSlug returns the Slug field if non-nil, zero value otherwise.

### GetSlugOk

`func (o *AdminTenant) GetSlugOk() (*string, bool)`

GetSlugOk returns a tuple with the Slug field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSlug

`func (o *AdminTenant) SetSlug(v string)`

SetSlug sets Slug field to given value.

### HasSlug

`func (o *AdminTenant) HasSlug() bool`

HasSlug returns a boolean if a field has been set.

### GetContactEmail

`func (o *AdminTenant) GetContactEmail() string`

GetContactEmail returns the ContactEmail field if non-nil, zero value otherwise.

### GetContactEmailOk

`func (o *AdminTenant) GetContactEmailOk() (*string, bool)`

GetContactEmailOk returns a tuple with the ContactEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetContactEmail

`func (o *AdminTenant) SetContactEmail(v string)`

SetContactEmail sets ContactEmail field to given value.

### HasContactEmail

`func (o *AdminTenant) HasContactEmail() bool`

HasContactEmail returns a boolean if a field has been set.

### GetPlanTier

`func (o *AdminTenant) GetPlanTier() string`

GetPlanTier returns the PlanTier field if non-nil, zero value otherwise.

### GetPlanTierOk

`func (o *AdminTenant) GetPlanTierOk() (*string, bool)`

GetPlanTierOk returns a tuple with the PlanTier field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPlanTier

`func (o *AdminTenant) SetPlanTier(v string)`

SetPlanTier sets PlanTier field to given value.

### HasPlanTier

`func (o *AdminTenant) HasPlanTier() bool`

HasPlanTier returns a boolean if a field has been set.

### GetStatus

`func (o *AdminTenant) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *AdminTenant) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *AdminTenant) SetStatus(v string)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *AdminTenant) HasStatus() bool`

HasStatus returns a boolean if a field has been set.

### GetMonthlyEmailLimit

`func (o *AdminTenant) GetMonthlyEmailLimit() int32`

GetMonthlyEmailLimit returns the MonthlyEmailLimit field if non-nil, zero value otherwise.

### GetMonthlyEmailLimitOk

`func (o *AdminTenant) GetMonthlyEmailLimitOk() (*int32, bool)`

GetMonthlyEmailLimitOk returns a tuple with the MonthlyEmailLimit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMonthlyEmailLimit

`func (o *AdminTenant) SetMonthlyEmailLimit(v int32)`

SetMonthlyEmailLimit sets MonthlyEmailLimit field to given value.

### HasMonthlyEmailLimit

`func (o *AdminTenant) HasMonthlyEmailLimit() bool`

HasMonthlyEmailLimit returns a boolean if a field has been set.

### SetMonthlyEmailLimitNil

`func (o *AdminTenant) SetMonthlyEmailLimitNil()`

 SetMonthlyEmailLimitNil sets the value for MonthlyEmailLimit to be an explicit nil

### UnsetMonthlyEmailLimit
`func (o *AdminTenant) UnsetMonthlyEmailLimit()`

UnsetMonthlyEmailLimit ensures that no value is present for MonthlyEmailLimit, not even an explicit nil

### GetMaxRequestsPerSecond

`func (o *AdminTenant) GetMaxRequestsPerSecond() int32`

GetMaxRequestsPerSecond returns the MaxRequestsPerSecond field if non-nil, zero value otherwise.

### GetMaxRequestsPerSecondOk

`func (o *AdminTenant) GetMaxRequestsPerSecondOk() (*int32, bool)`

GetMaxRequestsPerSecondOk returns a tuple with the MaxRequestsPerSecond field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxRequestsPerSecond

`func (o *AdminTenant) SetMaxRequestsPerSecond(v int32)`

SetMaxRequestsPerSecond sets MaxRequestsPerSecond field to given value.

### HasMaxRequestsPerSecond

`func (o *AdminTenant) HasMaxRequestsPerSecond() bool`

HasMaxRequestsPerSecond returns a boolean if a field has been set.

### GetMaxRequestsPerMinute

`func (o *AdminTenant) GetMaxRequestsPerMinute() int32`

GetMaxRequestsPerMinute returns the MaxRequestsPerMinute field if non-nil, zero value otherwise.

### GetMaxRequestsPerMinuteOk

`func (o *AdminTenant) GetMaxRequestsPerMinuteOk() (*int32, bool)`

GetMaxRequestsPerMinuteOk returns a tuple with the MaxRequestsPerMinute field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxRequestsPerMinute

`func (o *AdminTenant) SetMaxRequestsPerMinute(v int32)`

SetMaxRequestsPerMinute sets MaxRequestsPerMinute field to given value.

### HasMaxRequestsPerMinute

`func (o *AdminTenant) HasMaxRequestsPerMinute() bool`

HasMaxRequestsPerMinute returns a boolean if a field has been set.

### GetMaxRequestsPerHour

`func (o *AdminTenant) GetMaxRequestsPerHour() int32`

GetMaxRequestsPerHour returns the MaxRequestsPerHour field if non-nil, zero value otherwise.

### GetMaxRequestsPerHourOk

`func (o *AdminTenant) GetMaxRequestsPerHourOk() (*int32, bool)`

GetMaxRequestsPerHourOk returns a tuple with the MaxRequestsPerHour field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxRequestsPerHour

`func (o *AdminTenant) SetMaxRequestsPerHour(v int32)`

SetMaxRequestsPerHour sets MaxRequestsPerHour field to given value.

### HasMaxRequestsPerHour

`func (o *AdminTenant) HasMaxRequestsPerHour() bool`

HasMaxRequestsPerHour returns a boolean if a field has been set.

### GetMaxRequestsPerDay

`func (o *AdminTenant) GetMaxRequestsPerDay() int32`

GetMaxRequestsPerDay returns the MaxRequestsPerDay field if non-nil, zero value otherwise.

### GetMaxRequestsPerDayOk

`func (o *AdminTenant) GetMaxRequestsPerDayOk() (*int32, bool)`

GetMaxRequestsPerDayOk returns a tuple with the MaxRequestsPerDay field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxRequestsPerDay

`func (o *AdminTenant) SetMaxRequestsPerDay(v int32)`

SetMaxRequestsPerDay sets MaxRequestsPerDay field to given value.

### HasMaxRequestsPerDay

`func (o *AdminTenant) HasMaxRequestsPerDay() bool`

HasMaxRequestsPerDay returns a boolean if a field has been set.

### GetUsedThisPeriod

`func (o *AdminTenant) GetUsedThisPeriod() int32`

GetUsedThisPeriod returns the UsedThisPeriod field if non-nil, zero value otherwise.

### GetUsedThisPeriodOk

`func (o *AdminTenant) GetUsedThisPeriodOk() (*int32, bool)`

GetUsedThisPeriodOk returns a tuple with the UsedThisPeriod field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUsedThisPeriod

`func (o *AdminTenant) SetUsedThisPeriod(v int32)`

SetUsedThisPeriod sets UsedThisPeriod field to given value.

### HasUsedThisPeriod

`func (o *AdminTenant) HasUsedThisPeriod() bool`

HasUsedThisPeriod returns a boolean if a field has been set.

### GetDefaultWebhookUrl

`func (o *AdminTenant) GetDefaultWebhookUrl() string`

GetDefaultWebhookUrl returns the DefaultWebhookUrl field if non-nil, zero value otherwise.

### GetDefaultWebhookUrlOk

`func (o *AdminTenant) GetDefaultWebhookUrlOk() (*string, bool)`

GetDefaultWebhookUrlOk returns a tuple with the DefaultWebhookUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultWebhookUrl

`func (o *AdminTenant) SetDefaultWebhookUrl(v string)`

SetDefaultWebhookUrl sets DefaultWebhookUrl field to given value.

### HasDefaultWebhookUrl

`func (o *AdminTenant) HasDefaultWebhookUrl() bool`

HasDefaultWebhookUrl returns a boolean if a field has been set.

### SetDefaultWebhookUrlNil

`func (o *AdminTenant) SetDefaultWebhookUrlNil()`

 SetDefaultWebhookUrlNil sets the value for DefaultWebhookUrl to be an explicit nil

### UnsetDefaultWebhookUrl
`func (o *AdminTenant) UnsetDefaultWebhookUrl()`

UnsetDefaultWebhookUrl ensures that no value is present for DefaultWebhookUrl, not even an explicit nil

### GetResultRetentionDays

`func (o *AdminTenant) GetResultRetentionDays() int32`

GetResultRetentionDays returns the ResultRetentionDays field if non-nil, zero value otherwise.

### GetResultRetentionDaysOk

`func (o *AdminTenant) GetResultRetentionDaysOk() (*int32, bool)`

GetResultRetentionDaysOk returns a tuple with the ResultRetentionDays field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResultRetentionDays

`func (o *AdminTenant) SetResultRetentionDays(v int32)`

SetResultRetentionDays sets ResultRetentionDays field to given value.

### HasResultRetentionDays

`func (o *AdminTenant) HasResultRetentionDays() bool`

HasResultRetentionDays returns a boolean if a field has been set.

### GetCreatedAt

`func (o *AdminTenant) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *AdminTenant) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *AdminTenant) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *AdminTenant) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### GetUpdatedAt

`func (o *AdminTenant) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *AdminTenant) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *AdminTenant) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *AdminTenant) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
