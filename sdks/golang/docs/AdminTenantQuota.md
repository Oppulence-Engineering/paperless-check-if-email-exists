# AdminTenantQuota

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**MonthlyEmailLimit** | Pointer to **NullableInt32** |  | [optional]
**Name** | Pointer to **string** |  | [optional]
**PeriodResetAt** | Pointer to **time.Time** |  | [optional]
**QuotaUnlimited** | Pointer to **bool** |  | [optional]
**RemainingQuota** | Pointer to **NullableInt32** |  | [optional]
**TenantId** | Pointer to **string** |  | [optional]
**UsedThisPeriod** | Pointer to **int32** |  | [optional]

## Methods

### NewAdminTenantQuota

`func NewAdminTenantQuota() *AdminTenantQuota`

NewAdminTenantQuota instantiates a new AdminTenantQuota object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAdminTenantQuotaWithDefaults

`func NewAdminTenantQuotaWithDefaults() *AdminTenantQuota`

NewAdminTenantQuotaWithDefaults instantiates a new AdminTenantQuota object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetMonthlyEmailLimit

`func (o *AdminTenantQuota) GetMonthlyEmailLimit() int32`

GetMonthlyEmailLimit returns the MonthlyEmailLimit field if non-nil, zero value otherwise.

### GetMonthlyEmailLimitOk

`func (o *AdminTenantQuota) GetMonthlyEmailLimitOk() (*int32, bool)`

GetMonthlyEmailLimitOk returns a tuple with the MonthlyEmailLimit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMonthlyEmailLimit

`func (o *AdminTenantQuota) SetMonthlyEmailLimit(v int32)`

SetMonthlyEmailLimit sets MonthlyEmailLimit field to given value.

### HasMonthlyEmailLimit

`func (o *AdminTenantQuota) HasMonthlyEmailLimit() bool`

HasMonthlyEmailLimit returns a boolean if a field has been set.

### SetMonthlyEmailLimitNil

`func (o *AdminTenantQuota) SetMonthlyEmailLimitNil()`

 SetMonthlyEmailLimitNil sets the value for MonthlyEmailLimit to be an explicit nil

### UnsetMonthlyEmailLimit
`func (o *AdminTenantQuota) UnsetMonthlyEmailLimit()`

UnsetMonthlyEmailLimit ensures that no value is present for MonthlyEmailLimit, not even an explicit nil

### GetName

`func (o *AdminTenantQuota) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *AdminTenantQuota) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *AdminTenantQuota) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *AdminTenantQuota) HasName() bool`

HasName returns a boolean if a field has been set.

### GetPeriodResetAt

`func (o *AdminTenantQuota) GetPeriodResetAt() time.Time`

GetPeriodResetAt returns the PeriodResetAt field if non-nil, zero value otherwise.

### GetPeriodResetAtOk

`func (o *AdminTenantQuota) GetPeriodResetAtOk() (*time.Time, bool)`

GetPeriodResetAtOk returns a tuple with the PeriodResetAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPeriodResetAt

`func (o *AdminTenantQuota) SetPeriodResetAt(v time.Time)`

SetPeriodResetAt sets PeriodResetAt field to given value.

### HasPeriodResetAt

`func (o *AdminTenantQuota) HasPeriodResetAt() bool`

HasPeriodResetAt returns a boolean if a field has been set.

### GetQuotaUnlimited

`func (o *AdminTenantQuota) GetQuotaUnlimited() bool`

GetQuotaUnlimited returns the QuotaUnlimited field if non-nil, zero value otherwise.

### GetQuotaUnlimitedOk

`func (o *AdminTenantQuota) GetQuotaUnlimitedOk() (*bool, bool)`

GetQuotaUnlimitedOk returns a tuple with the QuotaUnlimited field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetQuotaUnlimited

`func (o *AdminTenantQuota) SetQuotaUnlimited(v bool)`

SetQuotaUnlimited sets QuotaUnlimited field to given value.

### HasQuotaUnlimited

`func (o *AdminTenantQuota) HasQuotaUnlimited() bool`

HasQuotaUnlimited returns a boolean if a field has been set.

### GetRemainingQuota

`func (o *AdminTenantQuota) GetRemainingQuota() int32`

GetRemainingQuota returns the RemainingQuota field if non-nil, zero value otherwise.

### GetRemainingQuotaOk

`func (o *AdminTenantQuota) GetRemainingQuotaOk() (*int32, bool)`

GetRemainingQuotaOk returns a tuple with the RemainingQuota field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRemainingQuota

`func (o *AdminTenantQuota) SetRemainingQuota(v int32)`

SetRemainingQuota sets RemainingQuota field to given value.

### HasRemainingQuota

`func (o *AdminTenantQuota) HasRemainingQuota() bool`

HasRemainingQuota returns a boolean if a field has been set.

### SetRemainingQuotaNil

`func (o *AdminTenantQuota) SetRemainingQuotaNil()`

 SetRemainingQuotaNil sets the value for RemainingQuota to be an explicit nil

### UnsetRemainingQuota
`func (o *AdminTenantQuota) UnsetRemainingQuota()`

UnsetRemainingQuota ensures that no value is present for RemainingQuota, not even an explicit nil

### GetTenantId

`func (o *AdminTenantQuota) GetTenantId() string`

GetTenantId returns the TenantId field if non-nil, zero value otherwise.

### GetTenantIdOk

`func (o *AdminTenantQuota) GetTenantIdOk() (*string, bool)`

GetTenantIdOk returns a tuple with the TenantId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantId

`func (o *AdminTenantQuota) SetTenantId(v string)`

SetTenantId sets TenantId field to given value.

### HasTenantId

`func (o *AdminTenantQuota) HasTenantId() bool`

HasTenantId returns a boolean if a field has been set.

### GetUsedThisPeriod

`func (o *AdminTenantQuota) GetUsedThisPeriod() int32`

GetUsedThisPeriod returns the UsedThisPeriod field if non-nil, zero value otherwise.

### GetUsedThisPeriodOk

`func (o *AdminTenantQuota) GetUsedThisPeriodOk() (*int32, bool)`

GetUsedThisPeriodOk returns a tuple with the UsedThisPeriod field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUsedThisPeriod

`func (o *AdminTenantQuota) SetUsedThisPeriod(v int32)`

SetUsedThisPeriod sets UsedThisPeriod field to given value.

### HasUsedThisPeriod

`func (o *AdminTenantQuota) HasUsedThisPeriod() bool`

HasUsedThisPeriod returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
