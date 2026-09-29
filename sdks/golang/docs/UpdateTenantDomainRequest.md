# UpdateTenantDomainRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Domain** | Pointer to **NullableString** |  | [optional]
**IsActive** | Pointer to **NullableBool** |  | [optional]
**IsVerified** | Pointer to **NullableBool** |  | [optional]
**Notes** | Pointer to **NullableString** |  | [optional]

## Methods

### NewUpdateTenantDomainRequest

`func NewUpdateTenantDomainRequest() *UpdateTenantDomainRequest`

NewUpdateTenantDomainRequest instantiates a new UpdateTenantDomainRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateTenantDomainRequestWithDefaults

`func NewUpdateTenantDomainRequestWithDefaults() *UpdateTenantDomainRequest`

NewUpdateTenantDomainRequestWithDefaults instantiates a new UpdateTenantDomainRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDomain

`func (o *UpdateTenantDomainRequest) GetDomain() string`

GetDomain returns the Domain field if non-nil, zero value otherwise.

### GetDomainOk

`func (o *UpdateTenantDomainRequest) GetDomainOk() (*string, bool)`

GetDomainOk returns a tuple with the Domain field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDomain

`func (o *UpdateTenantDomainRequest) SetDomain(v string)`

SetDomain sets Domain field to given value.

### HasDomain

`func (o *UpdateTenantDomainRequest) HasDomain() bool`

HasDomain returns a boolean if a field has been set.

### SetDomainNil

`func (o *UpdateTenantDomainRequest) SetDomainNil()`

 SetDomainNil sets the value for Domain to be an explicit nil

### UnsetDomain
`func (o *UpdateTenantDomainRequest) UnsetDomain()`

UnsetDomain ensures that no value is present for Domain, not even an explicit nil

### GetIsActive

`func (o *UpdateTenantDomainRequest) GetIsActive() bool`

GetIsActive returns the IsActive field if non-nil, zero value otherwise.

### GetIsActiveOk

`func (o *UpdateTenantDomainRequest) GetIsActiveOk() (*bool, bool)`

GetIsActiveOk returns a tuple with the IsActive field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsActive

`func (o *UpdateTenantDomainRequest) SetIsActive(v bool)`

SetIsActive sets IsActive field to given value.

### HasIsActive

`func (o *UpdateTenantDomainRequest) HasIsActive() bool`

HasIsActive returns a boolean if a field has been set.

### SetIsActiveNil

`func (o *UpdateTenantDomainRequest) SetIsActiveNil()`

 SetIsActiveNil sets the value for IsActive to be an explicit nil

### UnsetIsActive
`func (o *UpdateTenantDomainRequest) UnsetIsActive()`

UnsetIsActive ensures that no value is present for IsActive, not even an explicit nil

### GetIsVerified

`func (o *UpdateTenantDomainRequest) GetIsVerified() bool`

GetIsVerified returns the IsVerified field if non-nil, zero value otherwise.

### GetIsVerifiedOk

`func (o *UpdateTenantDomainRequest) GetIsVerifiedOk() (*bool, bool)`

GetIsVerifiedOk returns a tuple with the IsVerified field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsVerified

`func (o *UpdateTenantDomainRequest) SetIsVerified(v bool)`

SetIsVerified sets IsVerified field to given value.

### HasIsVerified

`func (o *UpdateTenantDomainRequest) HasIsVerified() bool`

HasIsVerified returns a boolean if a field has been set.

### SetIsVerifiedNil

`func (o *UpdateTenantDomainRequest) SetIsVerifiedNil()`

 SetIsVerifiedNil sets the value for IsVerified to be an explicit nil

### UnsetIsVerified
`func (o *UpdateTenantDomainRequest) UnsetIsVerified()`

UnsetIsVerified ensures that no value is present for IsVerified, not even an explicit nil

### GetNotes

`func (o *UpdateTenantDomainRequest) GetNotes() string`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *UpdateTenantDomainRequest) GetNotesOk() (*string, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *UpdateTenantDomainRequest) SetNotes(v string)`

SetNotes sets Notes field to given value.

### HasNotes

`func (o *UpdateTenantDomainRequest) HasNotes() bool`

HasNotes returns a boolean if a field has been set.

### SetNotesNil

`func (o *UpdateTenantDomainRequest) SetNotesNil()`

 SetNotesNil sets the value for Notes to be an explicit nil

### UnsetNotes
`func (o *UpdateTenantDomainRequest) UnsetNotes()`

UnsetNotes ensures that no value is present for Notes, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
