# DomainInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**DomainAgeDays** | Pointer to **NullableInt64** |  | [optional]
**Registrar** | Pointer to **NullableString** |  | [optional]

## Methods

### NewDomainInfo

`func NewDomainInfo() *DomainInfo`

NewDomainInfo instantiates a new DomainInfo object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDomainInfoWithDefaults

`func NewDomainInfoWithDefaults() *DomainInfo`

NewDomainInfoWithDefaults instantiates a new DomainInfo object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *DomainInfo) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *DomainInfo) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *DomainInfo) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *DomainInfo) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *DomainInfo) SetCreatedAtNil()`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *DomainInfo) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil

### GetDomainAgeDays

`func (o *DomainInfo) GetDomainAgeDays() int64`

GetDomainAgeDays returns the DomainAgeDays field if non-nil, zero value otherwise.

### GetDomainAgeDaysOk

`func (o *DomainInfo) GetDomainAgeDaysOk() (*int64, bool)`

GetDomainAgeDaysOk returns a tuple with the DomainAgeDays field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDomainAgeDays

`func (o *DomainInfo) SetDomainAgeDays(v int64)`

SetDomainAgeDays sets DomainAgeDays field to given value.

### HasDomainAgeDays

`func (o *DomainInfo) HasDomainAgeDays() bool`

HasDomainAgeDays returns a boolean if a field has been set.

### SetDomainAgeDaysNil

`func (o *DomainInfo) SetDomainAgeDaysNil()`

 SetDomainAgeDaysNil sets the value for DomainAgeDays to be an explicit nil

### UnsetDomainAgeDays
`func (o *DomainInfo) UnsetDomainAgeDays()`

UnsetDomainAgeDays ensures that no value is present for DomainAgeDays, not even an explicit nil

### GetRegistrar

`func (o *DomainInfo) GetRegistrar() string`

GetRegistrar returns the Registrar field if non-nil, zero value otherwise.

### GetRegistrarOk

`func (o *DomainInfo) GetRegistrarOk() (*string, bool)`

GetRegistrarOk returns a tuple with the Registrar field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRegistrar

`func (o *DomainInfo) SetRegistrar(v string)`

SetRegistrar sets Registrar field to given value.

### HasRegistrar

`func (o *DomainInfo) HasRegistrar() bool`

HasRegistrar returns a boolean if a field has been set.

### SetRegistrarNil

`func (o *DomainInfo) SetRegistrarNil()`

 SetRegistrarNil sets the value for Registrar to be an explicit nil

### UnsetRegistrar
`func (o *DomainInfo) UnsetRegistrar()`

UnsetRegistrar ensures that no value is present for Registrar, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
