# CreateApiKeyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExpiresAt** | Pointer to **NullableString** |  | [optional]
**Name** | Pointer to **NullableString** |  | [optional]
**Scopes** | Pointer to **[]string** |  | [optional]

## Methods

### NewCreateApiKeyRequest

`func NewCreateApiKeyRequest() *CreateApiKeyRequest`

NewCreateApiKeyRequest instantiates a new CreateApiKeyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateApiKeyRequestWithDefaults

`func NewCreateApiKeyRequestWithDefaults() *CreateApiKeyRequest`

NewCreateApiKeyRequestWithDefaults instantiates a new CreateApiKeyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExpiresAt

`func (o *CreateApiKeyRequest) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *CreateApiKeyRequest) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *CreateApiKeyRequest) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.

### HasExpiresAt

`func (o *CreateApiKeyRequest) HasExpiresAt() bool`

HasExpiresAt returns a boolean if a field has been set.

### SetExpiresAtNil

`func (o *CreateApiKeyRequest) SetExpiresAtNil()`

 SetExpiresAtNil sets the value for ExpiresAt to be an explicit nil

### UnsetExpiresAt
`func (o *CreateApiKeyRequest) UnsetExpiresAt()`

UnsetExpiresAt ensures that no value is present for ExpiresAt, not even an explicit nil

### GetName

`func (o *CreateApiKeyRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateApiKeyRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateApiKeyRequest) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *CreateApiKeyRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *CreateApiKeyRequest) SetNameNil()`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *CreateApiKeyRequest) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil

### GetScopes

`func (o *CreateApiKeyRequest) GetScopes() []string`

GetScopes returns the Scopes field if non-nil, zero value otherwise.

### GetScopesOk

`func (o *CreateApiKeyRequest) GetScopesOk() ([]string, bool)`

GetScopesOk returns a tuple with the Scopes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScopes

`func (o *CreateApiKeyRequest) SetScopes(v []string)`

SetScopes sets Scopes field to given value.

### HasScopes

`func (o *CreateApiKeyRequest) HasScopes() bool`

HasScopes returns a boolean if a field has been set.

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
