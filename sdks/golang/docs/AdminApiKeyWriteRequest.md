# AdminApiKeyWriteRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExpiresAt** | Pointer to **NullableTime** |  | [optional]
**Name** | Pointer to **string** |  | [optional]
**Scopes** | Pointer to **[]string** |  | [optional]

## Methods

### NewAdminApiKeyWriteRequest

`func NewAdminApiKeyWriteRequest() *AdminApiKeyWriteRequest`

NewAdminApiKeyWriteRequest instantiates a new AdminApiKeyWriteRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAdminApiKeyWriteRequestWithDefaults

`func NewAdminApiKeyWriteRequestWithDefaults() *AdminApiKeyWriteRequest`

NewAdminApiKeyWriteRequestWithDefaults instantiates a new AdminApiKeyWriteRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExpiresAt

`func (o *AdminApiKeyWriteRequest) GetExpiresAt() time.Time`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *AdminApiKeyWriteRequest) GetExpiresAtOk() (*time.Time, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *AdminApiKeyWriteRequest) SetExpiresAt(v time.Time)`

SetExpiresAt sets ExpiresAt field to given value.

### HasExpiresAt

`func (o *AdminApiKeyWriteRequest) HasExpiresAt() bool`

HasExpiresAt returns a boolean if a field has been set.

### SetExpiresAtNil

`func (o *AdminApiKeyWriteRequest) SetExpiresAtNil()`

 SetExpiresAtNil sets the value for ExpiresAt to be an explicit nil

### UnsetExpiresAt
`func (o *AdminApiKeyWriteRequest) UnsetExpiresAt()`

UnsetExpiresAt ensures that no value is present for ExpiresAt, not even an explicit nil

### GetName

`func (o *AdminApiKeyWriteRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *AdminApiKeyWriteRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *AdminApiKeyWriteRequest) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *AdminApiKeyWriteRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### GetScopes

`func (o *AdminApiKeyWriteRequest) GetScopes() []string`

GetScopes returns the Scopes field if non-nil, zero value otherwise.

### GetScopesOk

`func (o *AdminApiKeyWriteRequest) GetScopesOk() ([]string, bool)`

GetScopesOk returns a tuple with the Scopes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScopes

`func (o *AdminApiKeyWriteRequest) SetScopes(v []string)`

SetScopes sets Scopes field to given value.

### HasScopes

`func (o *AdminApiKeyWriteRequest) HasScopes() bool`

HasScopes returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
