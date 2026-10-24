# HistoryRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Email** | **string** |  | [required]
**Limit** | Pointer to **NullableInt64** |  | [optional]

## Methods

### NewHistoryRequest

`func NewHistoryRequest(email string) *HistoryRequest`

NewHistoryRequest instantiates a new HistoryRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewHistoryRequestWithDefaults

`func NewHistoryRequestWithDefaults() *HistoryRequest`

NewHistoryRequestWithDefaults instantiates a new HistoryRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEmail

`func (o *HistoryRequest) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *HistoryRequest) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *HistoryRequest) SetEmail(v string)`

SetEmail sets Email field to given value.


### GetLimit

`func (o *HistoryRequest) GetLimit() int64`

GetLimit returns the Limit field if non-nil, zero value otherwise.

### GetLimitOk

`func (o *HistoryRequest) GetLimitOk() (*int64, bool)`

GetLimitOk returns a tuple with the Limit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimit

`func (o *HistoryRequest) SetLimit(v int64)`

SetLimit sets Limit field to given value.

### HasLimit

`func (o *HistoryRequest) HasLimit() bool`

HasLimit returns a boolean if a field has been set.

### SetLimitNil

`func (o *HistoryRequest) SetLimitNil()`

 SetLimitNil sets the value for Limit to be an explicit nil

### UnsetLimit
`func (o *HistoryRequest) UnsetLimit()`

UnsetLimit ensures that no value is present for Limit, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
