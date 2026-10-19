# VerifMethod

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Host** | Pointer to **string** | The mail server host the check connected to. Only for Smtp. | [optional]
**Type** | **string** | The method used for the email verification. | [required]
**VerifMethod** | Pointer to [**VerifMethodVerifMethod**](VerifMethodVerifMethod.md) |  | [optional]

## Methods

### NewVerifMethod

`func NewVerifMethod(type_ string) *VerifMethod`

NewVerifMethod instantiates a new VerifMethod object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewVerifMethodWithDefaults

`func NewVerifMethodWithDefaults() *VerifMethod`

NewVerifMethodWithDefaults instantiates a new VerifMethod object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetHost

`func (o *VerifMethod) GetHost() string`

GetHost returns the Host field if non-nil, zero value otherwise.

### GetHostOk

`func (o *VerifMethod) GetHostOk() (*string, bool)`

GetHostOk returns a tuple with the Host field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHost

`func (o *VerifMethod) SetHost(v string)`

SetHost sets Host field to given value.

### HasHost

`func (o *VerifMethod) HasHost() bool`

HasHost returns a boolean if a field has been set.

### GetType

`func (o *VerifMethod) GetType() string`

GetType returns the Type field if non-nil, zero value otherwise.

### GetTypeOk

`func (o *VerifMethod) GetTypeOk() (*string, bool)`

GetTypeOk returns a tuple with the Type field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetType

`func (o *VerifMethod) SetType(v string)`

SetType sets Type field to given value.


### GetVerifMethod

`func (o *VerifMethod) GetVerifMethod() VerifMethodVerifMethod`

GetVerifMethod returns the VerifMethod field if non-nil, zero value otherwise.

### GetVerifMethodOk

`func (o *VerifMethod) GetVerifMethodOk() (*VerifMethodVerifMethod, bool)`

GetVerifMethodOk returns a tuple with the VerifMethod field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVerifMethod

`func (o *VerifMethod) SetVerifMethod(v VerifMethodVerifMethod)`

SetVerifMethod sets VerifMethod field to given value.

### HasVerifMethod

`func (o *VerifMethod) HasVerifMethod() bool`

HasVerifMethod returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
