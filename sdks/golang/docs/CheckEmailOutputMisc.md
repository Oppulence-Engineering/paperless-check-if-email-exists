# CheckEmailOutputMisc

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**GravatarUrl** | Pointer to **string** | URL to the Gravatar profile picture associated with the email, if available and requested. | [optional] 
**IsB2c** | **bool** | Is this a B2C email address? | 
**IsDisposable** | **bool** | Indicates if the email address is from a known disposable email provider. | 
**IsRoleAccount** | **bool** | Indicates if the email address is a role-based account. | 
**Message** | **string** | A human-readable description of the error. | 
**Type** | **string** | The type of error. | 

## Methods

### NewCheckEmailOutputMisc

`func NewCheckEmailOutputMisc(isB2c bool, isDisposable bool, isRoleAccount bool, message string, type_ string, ) *CheckEmailOutputMisc`

NewCheckEmailOutputMisc instantiates a new CheckEmailOutputMisc object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCheckEmailOutputMiscWithDefaults

`func NewCheckEmailOutputMiscWithDefaults() *CheckEmailOutputMisc`

NewCheckEmailOutputMiscWithDefaults instantiates a new CheckEmailOutputMisc object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetGravatarUrl

`func (o *CheckEmailOutputMisc) GetGravatarUrl() string`

GetGravatarUrl returns the GravatarUrl field if non-nil, zero value otherwise.

### GetGravatarUrlOk

`func (o *CheckEmailOutputMisc) GetGravatarUrlOk() (*string, bool)`

GetGravatarUrlOk returns a tuple with the GravatarUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGravatarUrl

`func (o *CheckEmailOutputMisc) SetGravatarUrl(v string)`

SetGravatarUrl sets GravatarUrl field to given value.

### HasGravatarUrl

`func (o *CheckEmailOutputMisc) HasGravatarUrl() bool`

HasGravatarUrl returns a boolean if a field has been set.

### GetIsB2c

`func (o *CheckEmailOutputMisc) GetIsB2c() bool`

GetIsB2c returns the IsB2c field if non-nil, zero value otherwise.

### GetIsB2cOk

`func (o *CheckEmailOutputMisc) GetIsB2cOk() (*bool, bool)`

GetIsB2cOk returns a tuple with the IsB2c field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsB2c

`func (o *CheckEmailOutputMisc) SetIsB2c(v bool)`

SetIsB2c sets IsB2c field to given value.


### GetIsDisposable

`func (o *CheckEmailOutputMisc) GetIsDisposable() bool`

GetIsDisposable returns the IsDisposable field if non-nil, zero value otherwise.

### GetIsDisposableOk

`func (o *CheckEmailOutputMisc) GetIsDisposableOk() (*bool, bool)`

GetIsDisposableOk returns a tuple with the IsDisposable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsDisposable

`func (o *CheckEmailOutputMisc) SetIsDisposable(v bool)`

SetIsDisposable sets IsDisposable field to given value.


### GetIsRoleAccount

`func (o *CheckEmailOutputMisc) GetIsRoleAccount() bool`

GetIsRoleAccount returns the IsRoleAccount field if non-nil, zero value otherwise.

### GetIsRoleAccountOk

`func (o *CheckEmailOutputMisc) GetIsRoleAccountOk() (*bool, bool)`

GetIsRoleAccountOk returns a tuple with the IsRoleAccount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsRoleAccount

`func (o *CheckEmailOutputMisc) SetIsRoleAccount(v bool)`

SetIsRoleAccount sets IsRoleAccount field to given value.


### GetMessage

`func (o *CheckEmailOutputMisc) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *CheckEmailOutputMisc) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *CheckEmailOutputMisc) SetMessage(v string)`

SetMessage sets Message field to given value.


### GetType

`func (o *CheckEmailOutputMisc) GetType() string`

GetType returns the Type field if non-nil, zero value otherwise.

### GetTypeOk

`func (o *CheckEmailOutputMisc) GetTypeOk() (*string, bool)`

GetTypeOk returns a tuple with the Type field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetType

`func (o *CheckEmailOutputMisc) SetType(v string)`

SetType sets Type field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


