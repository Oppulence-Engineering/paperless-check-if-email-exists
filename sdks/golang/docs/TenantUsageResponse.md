# TenantUsageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**MonthlyEmailLimit** | Pointer to **NullableInt32** |  | [optional]
**PeriodResetAt** | **string** |  | [required]
**PlanTier** | **string** |  | [required]
**QuotaRemaining** | Pointer to **NullableInt32** |  | [optional]
**QuotaUnlimited** | **bool** |  | [required]
**TenantId** | **string** |  | [required]
**TenantName** | **string** |  | [required]
**UsedThisPeriod** | **int32** |  | [required]

## Methods

### NewTenantUsageResponse

`func NewTenantUsageResponse(periodResetAt string, planTier string, quotaUnlimited bool, tenantId string, tenantName string, usedThisPeriod int32) *TenantUsageResponse`

NewTenantUsageResponse instantiates a new TenantUsageResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewTenantUsageResponseWithDefaults

`func NewTenantUsageResponseWithDefaults() *TenantUsageResponse`

NewTenantUsageResponseWithDefaults instantiates a new TenantUsageResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetMonthlyEmailLimit

`func (o *TenantUsageResponse) GetMonthlyEmailLimit() int32`

GetMonthlyEmailLimit returns the MonthlyEmailLimit field if non-nil, zero value otherwise.

### GetMonthlyEmailLimitOk

`func (o *TenantUsageResponse) GetMonthlyEmailLimitOk() (*int32, bool)`

GetMonthlyEmailLimitOk returns a tuple with the MonthlyEmailLimit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMonthlyEmailLimit

`func (o *TenantUsageResponse) SetMonthlyEmailLimit(v int32)`

SetMonthlyEmailLimit sets MonthlyEmailLimit field to given value.

### HasMonthlyEmailLimit

`func (o *TenantUsageResponse) HasMonthlyEmailLimit() bool`

HasMonthlyEmailLimit returns a boolean if a field has been set.

### SetMonthlyEmailLimitNil

`func (o *TenantUsageResponse) SetMonthlyEmailLimitNil()`

 SetMonthlyEmailLimitNil sets the value for MonthlyEmailLimit to be an explicit nil

### UnsetMonthlyEmailLimit
`func (o *TenantUsageResponse) UnsetMonthlyEmailLimit()`

UnsetMonthlyEmailLimit ensures that no value is present for MonthlyEmailLimit, not even an explicit nil

### GetPeriodResetAt

`func (o *TenantUsageResponse) GetPeriodResetAt() string`

GetPeriodResetAt returns the PeriodResetAt field if non-nil, zero value otherwise.

### GetPeriodResetAtOk

`func (o *TenantUsageResponse) GetPeriodResetAtOk() (*string, bool)`

GetPeriodResetAtOk returns a tuple with the PeriodResetAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPeriodResetAt

`func (o *TenantUsageResponse) SetPeriodResetAt(v string)`

SetPeriodResetAt sets PeriodResetAt field to given value.


### GetPlanTier

`func (o *TenantUsageResponse) GetPlanTier() string`

GetPlanTier returns the PlanTier field if non-nil, zero value otherwise.

### GetPlanTierOk

`func (o *TenantUsageResponse) GetPlanTierOk() (*string, bool)`

GetPlanTierOk returns a tuple with the PlanTier field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPlanTier

`func (o *TenantUsageResponse) SetPlanTier(v string)`

SetPlanTier sets PlanTier field to given value.


### GetQuotaRemaining

`func (o *TenantUsageResponse) GetQuotaRemaining() int32`

GetQuotaRemaining returns the QuotaRemaining field if non-nil, zero value otherwise.

### GetQuotaRemainingOk

`func (o *TenantUsageResponse) GetQuotaRemainingOk() (*int32, bool)`

GetQuotaRemainingOk returns a tuple with the QuotaRemaining field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetQuotaRemaining

`func (o *TenantUsageResponse) SetQuotaRemaining(v int32)`

SetQuotaRemaining sets QuotaRemaining field to given value.

### HasQuotaRemaining

`func (o *TenantUsageResponse) HasQuotaRemaining() bool`

HasQuotaRemaining returns a boolean if a field has been set.

### SetQuotaRemainingNil

`func (o *TenantUsageResponse) SetQuotaRemainingNil()`

 SetQuotaRemainingNil sets the value for QuotaRemaining to be an explicit nil

### UnsetQuotaRemaining
`func (o *TenantUsageResponse) UnsetQuotaRemaining()`

UnsetQuotaRemaining ensures that no value is present for QuotaRemaining, not even an explicit nil

### GetQuotaUnlimited

`func (o *TenantUsageResponse) GetQuotaUnlimited() bool`

GetQuotaUnlimited returns the QuotaUnlimited field if non-nil, zero value otherwise.

### GetQuotaUnlimitedOk

`func (o *TenantUsageResponse) GetQuotaUnlimitedOk() (*bool, bool)`

GetQuotaUnlimitedOk returns a tuple with the QuotaUnlimited field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetQuotaUnlimited

`func (o *TenantUsageResponse) SetQuotaUnlimited(v bool)`

SetQuotaUnlimited sets QuotaUnlimited field to given value.


### GetTenantId

`func (o *TenantUsageResponse) GetTenantId() string`

GetTenantId returns the TenantId field if non-nil, zero value otherwise.

### GetTenantIdOk

`func (o *TenantUsageResponse) GetTenantIdOk() (*string, bool)`

GetTenantIdOk returns a tuple with the TenantId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantId

`func (o *TenantUsageResponse) SetTenantId(v string)`

SetTenantId sets TenantId field to given value.


### GetTenantName

`func (o *TenantUsageResponse) GetTenantName() string`

GetTenantName returns the TenantName field if non-nil, zero value otherwise.

### GetTenantNameOk

`func (o *TenantUsageResponse) GetTenantNameOk() (*string, bool)`

GetTenantNameOk returns a tuple with the TenantName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantName

`func (o *TenantUsageResponse) SetTenantName(v string)`

SetTenantName sets TenantName field to given value.


### GetUsedThisPeriod

`func (o *TenantUsageResponse) GetUsedThisPeriod() int32`

GetUsedThisPeriod returns the UsedThisPeriod field if non-nil, zero value otherwise.

### GetUsedThisPeriodOk

`func (o *TenantUsageResponse) GetUsedThisPeriodOk() (*int32, bool)`

GetUsedThisPeriodOk returns a tuple with the UsedThisPeriod field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUsedThisPeriod

`func (o *TenantUsageResponse) SetUsedThisPeriod(v int32)`

SetUsedThisPeriod sets UsedThisPeriod field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
