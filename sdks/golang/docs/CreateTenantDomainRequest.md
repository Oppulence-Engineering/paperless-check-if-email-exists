# CreateTenantDomainRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Domain** | **string** |  | [required]
**IsActive** | Pointer to **NullableBool** |  | [optional]
**Notes** | Pointer to **NullableString** |  | [optional]

## Methods

### NewCreateTenantDomainRequest

`func NewCreateTenantDomainRequest(domain string) *CreateTenantDomainRequest`

NewCreateTenantDomainRequest instantiates a new CreateTenantDomainRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateTenantDomainRequestWithDefaults

`func NewCreateTenantDomainRequestWithDefaults() *CreateTenantDomainRequest`

NewCreateTenantDomainRequestWithDefaults instantiates a new CreateTenantDomainRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDomain

`func (o *CreateTenantDomainRequest) GetDomain() string`

GetDomain returns the Domain field if non-nil, zero value otherwise.

### GetDomainOk

`func (o *CreateTenantDomainRequest) GetDomainOk() (*string, bool)`

GetDomainOk returns a tuple with the Domain field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDomain

`func (o *CreateTenantDomainRequest) SetDomain(v string)`

SetDomain sets Domain field to given value.


### GetIsActive

`func (o *CreateTenantDomainRequest) GetIsActive() bool`

GetIsActive returns the IsActive field if non-nil, zero value otherwise.

### GetIsActiveOk

`func (o *CreateTenantDomainRequest) GetIsActiveOk() (*bool, bool)`

GetIsActiveOk returns a tuple with the IsActive field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsActive

`func (o *CreateTenantDomainRequest) SetIsActive(v bool)`

SetIsActive sets IsActive field to given value.

### HasIsActive

`func (o *CreateTenantDomainRequest) HasIsActive() bool`

HasIsActive returns a boolean if a field has been set.

### SetIsActiveNil

`func (o *CreateTenantDomainRequest) SetIsActiveNil()`

 SetIsActiveNil sets the value for IsActive to be an explicit nil

### UnsetIsActive
`func (o *CreateTenantDomainRequest) UnsetIsActive()`

UnsetIsActive ensures that no value is present for IsActive, not even an explicit nil

### GetNotes

`func (o *CreateTenantDomainRequest) GetNotes() string`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *CreateTenantDomainRequest) GetNotesOk() (*string, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *CreateTenantDomainRequest) SetNotes(v string)`

SetNotes sets Notes field to given value.

### HasNotes

`func (o *CreateTenantDomainRequest) HasNotes() bool`

HasNotes returns a boolean if a field has been set.

### SetNotesNil

`func (o *CreateTenantDomainRequest) SetNotesNil()`

 SetNotesNil sets the value for Notes to be an explicit nil

### UnsetNotes
`func (o *CreateTenantDomainRequest) UnsetNotes()`

UnsetNotes ensures that no value is present for Notes, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
