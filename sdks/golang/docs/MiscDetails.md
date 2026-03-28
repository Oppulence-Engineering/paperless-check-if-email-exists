# MiscDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**GravatarUrl** | Pointer to **string** | URL to the Gravatar profile picture associated with the email, if available and requested. | [optional]
**IsB2c** | **bool** | Is this a B2C email address? | [required]
**IsDisposable** | **bool** | Indicates if the email address is from a known disposable email provider. | [required]
**IsRoleAccount** | **bool** | Indicates if the email address is a role-based account. | [required]

## Methods

### NewMiscDetails

`func NewMiscDetails(isB2c bool, isDisposable bool, isRoleAccount bool) *MiscDetails`

NewMiscDetails instantiates a new MiscDetails object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMiscDetailsWithDefaults

`func NewMiscDetailsWithDefaults() *MiscDetails`

NewMiscDetailsWithDefaults instantiates a new MiscDetails object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetGravatarUrl

`func (o *MiscDetails) GetGravatarUrl() string`

GetGravatarUrl returns the GravatarUrl field if non-nil, zero value otherwise.

### GetGravatarUrlOk

`func (o *MiscDetails) GetGravatarUrlOk() (*string, bool)`

GetGravatarUrlOk returns a tuple with the GravatarUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGravatarUrl

`func (o *MiscDetails) SetGravatarUrl(v string)`

SetGravatarUrl sets GravatarUrl field to given value.

### HasGravatarUrl

`func (o *MiscDetails) HasGravatarUrl() bool`

HasGravatarUrl returns a boolean if a field has been set.

### GetIsB2c

`func (o *MiscDetails) GetIsB2c() bool`

GetIsB2c returns the IsB2c field if non-nil, zero value otherwise.

### GetIsB2cOk

`func (o *MiscDetails) GetIsB2cOk() (*bool, bool)`

GetIsB2cOk returns a tuple with the IsB2c field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsB2c

`func (o *MiscDetails) SetIsB2c(v bool)`

SetIsB2c sets IsB2c field to given value.


### GetIsDisposable

`func (o *MiscDetails) GetIsDisposable() bool`

GetIsDisposable returns the IsDisposable field if non-nil, zero value otherwise.

### GetIsDisposableOk

`func (o *MiscDetails) GetIsDisposableOk() (*bool, bool)`

GetIsDisposableOk returns a tuple with the IsDisposable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsDisposable

`func (o *MiscDetails) SetIsDisposable(v bool)`

SetIsDisposable sets IsDisposable field to given value.


### GetIsRoleAccount

`func (o *MiscDetails) GetIsRoleAccount() bool`

GetIsRoleAccount returns the IsRoleAccount field if non-nil, zero value otherwise.

### GetIsRoleAccountOk

`func (o *MiscDetails) GetIsRoleAccountOk() (*bool, bool)`

GetIsRoleAccountOk returns a tuple with the IsRoleAccount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsRoleAccount

`func (o *MiscDetails) SetIsRoleAccount(v bool)`

SetIsRoleAccount sets IsRoleAccount field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
