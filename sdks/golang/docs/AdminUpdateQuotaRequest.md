# AdminUpdateQuotaRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**MonthlyEmailLimit** | **NullableInt32** |  | [required]

## Methods

### NewAdminUpdateQuotaRequest

`func NewAdminUpdateQuotaRequest(monthlyEmailLimit NullableInt32) *AdminUpdateQuotaRequest`

NewAdminUpdateQuotaRequest instantiates a new AdminUpdateQuotaRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAdminUpdateQuotaRequestWithDefaults

`func NewAdminUpdateQuotaRequestWithDefaults() *AdminUpdateQuotaRequest`

NewAdminUpdateQuotaRequestWithDefaults instantiates a new AdminUpdateQuotaRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetMonthlyEmailLimit

`func (o *AdminUpdateQuotaRequest) GetMonthlyEmailLimit() int32`

GetMonthlyEmailLimit returns the MonthlyEmailLimit field if non-nil, zero value otherwise.

### GetMonthlyEmailLimitOk

`func (o *AdminUpdateQuotaRequest) GetMonthlyEmailLimitOk() (*int32, bool)`

GetMonthlyEmailLimitOk returns a tuple with the MonthlyEmailLimit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMonthlyEmailLimit

`func (o *AdminUpdateQuotaRequest) SetMonthlyEmailLimit(v int32)`

SetMonthlyEmailLimit sets MonthlyEmailLimit field to given value.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
