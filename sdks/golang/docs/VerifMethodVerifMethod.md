# VerifMethodVerifMethod

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**FromEmail** | Pointer to **string** |  | [optional]
**HelloName** | Pointer to **string** |  | [optional]
**Proxy** | Pointer to **NullableString** |  | [optional]
**Retries** | Pointer to **int32** |  | [optional]
**SmtpPort** | Pointer to **int32** |  | [optional]
**SmtpTimeout** | Pointer to [**NullableDuration**](Duration.md) |  | [optional]

## Methods

### NewVerifMethodVerifMethod

`func NewVerifMethodVerifMethod() *VerifMethodVerifMethod`

NewVerifMethodVerifMethod instantiates a new VerifMethodVerifMethod object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewVerifMethodVerifMethodWithDefaults

`func NewVerifMethodVerifMethodWithDefaults() *VerifMethodVerifMethod`

NewVerifMethodVerifMethodWithDefaults instantiates a new VerifMethodVerifMethod object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetFromEmail

`func (o *VerifMethodVerifMethod) GetFromEmail() string`

GetFromEmail returns the FromEmail field if non-nil, zero value otherwise.

### GetFromEmailOk

`func (o *VerifMethodVerifMethod) GetFromEmailOk() (*string, bool)`

GetFromEmailOk returns a tuple with the FromEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFromEmail

`func (o *VerifMethodVerifMethod) SetFromEmail(v string)`

SetFromEmail sets FromEmail field to given value.

### HasFromEmail

`func (o *VerifMethodVerifMethod) HasFromEmail() bool`

HasFromEmail returns a boolean if a field has been set.

### GetHelloName

`func (o *VerifMethodVerifMethod) GetHelloName() string`

GetHelloName returns the HelloName field if non-nil, zero value otherwise.

### GetHelloNameOk

`func (o *VerifMethodVerifMethod) GetHelloNameOk() (*string, bool)`

GetHelloNameOk returns a tuple with the HelloName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHelloName

`func (o *VerifMethodVerifMethod) SetHelloName(v string)`

SetHelloName sets HelloName field to given value.

### HasHelloName

`func (o *VerifMethodVerifMethod) HasHelloName() bool`

HasHelloName returns a boolean if a field has been set.

### GetProxy

`func (o *VerifMethodVerifMethod) GetProxy() string`

GetProxy returns the Proxy field if non-nil, zero value otherwise.

### GetProxyOk

`func (o *VerifMethodVerifMethod) GetProxyOk() (*string, bool)`

GetProxyOk returns a tuple with the Proxy field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProxy

`func (o *VerifMethodVerifMethod) SetProxy(v string)`

SetProxy sets Proxy field to given value.

### HasProxy

`func (o *VerifMethodVerifMethod) HasProxy() bool`

HasProxy returns a boolean if a field has been set.

### SetProxyNil

`func (o *VerifMethodVerifMethod) SetProxyNil()`

 SetProxyNil sets the value for Proxy to be an explicit nil

### UnsetProxy
`func (o *VerifMethodVerifMethod) UnsetProxy()`

UnsetProxy ensures that no value is present for Proxy, not even an explicit nil

### GetRetries

`func (o *VerifMethodVerifMethod) GetRetries() int32`

GetRetries returns the Retries field if non-nil, zero value otherwise.

### GetRetriesOk

`func (o *VerifMethodVerifMethod) GetRetriesOk() (*int32, bool)`

GetRetriesOk returns a tuple with the Retries field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRetries

`func (o *VerifMethodVerifMethod) SetRetries(v int32)`

SetRetries sets Retries field to given value.

### HasRetries

`func (o *VerifMethodVerifMethod) HasRetries() bool`

HasRetries returns a boolean if a field has been set.

### GetSmtpPort

`func (o *VerifMethodVerifMethod) GetSmtpPort() int32`

GetSmtpPort returns the SmtpPort field if non-nil, zero value otherwise.

### GetSmtpPortOk

`func (o *VerifMethodVerifMethod) GetSmtpPortOk() (*int32, bool)`

GetSmtpPortOk returns a tuple with the SmtpPort field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSmtpPort

`func (o *VerifMethodVerifMethod) SetSmtpPort(v int32)`

SetSmtpPort sets SmtpPort field to given value.

### HasSmtpPort

`func (o *VerifMethodVerifMethod) HasSmtpPort() bool`

HasSmtpPort returns a boolean if a field has been set.

### GetSmtpTimeout

`func (o *VerifMethodVerifMethod) GetSmtpTimeout() Duration`

GetSmtpTimeout returns the SmtpTimeout field if non-nil, zero value otherwise.

### GetSmtpTimeoutOk

`func (o *VerifMethodVerifMethod) GetSmtpTimeoutOk() (*Duration, bool)`

GetSmtpTimeoutOk returns a tuple with the SmtpTimeout field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSmtpTimeout

`func (o *VerifMethodVerifMethod) SetSmtpTimeout(v Duration)`

SetSmtpTimeout sets SmtpTimeout field to given value.

### HasSmtpTimeout

`func (o *VerifMethodVerifMethod) HasSmtpTimeout() bool`

HasSmtpTimeout returns a boolean if a field has been set.

### SetSmtpTimeoutNil

`func (o *VerifMethodVerifMethod) SetSmtpTimeoutNil()`

 SetSmtpTimeoutNil sets the value for SmtpTimeout to be an explicit nil

### UnsetSmtpTimeout
`func (o *VerifMethodVerifMethod) UnsetSmtpTimeout()`

UnsetSmtpTimeout ensures that no value is present for SmtpTimeout, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
