# CheckEmailRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CheckGravatar** | Pointer to **bool** | Whether to check if a Gravatar image exists for the given email. | [optional]
**FromEmail** | Pointer to **string** | In the SMTP connection, the FROM email address. | [optional]
**GmailVerifMethod** | Pointer to [**GmailVerifMethod**](GmailVerifMethod.md) |  | [optional]
**HelloName** | Pointer to **string** | In the SMTP connection, the EHLO hostname. | [optional]
**Hotmailb2bVerifMethod** | Pointer to [**HotmailB2BVerifMethod**](HotmailB2BVerifMethod.md) |  | [optional]
**Hotmailb2cVerifMethod** | Pointer to [**HotmailB2CVerifMethod**](HotmailB2CVerifMethod.md) |  | [optional]
**Proxy** | Pointer to [**CheckEmailInputProxy**](CheckEmailInputProxy.md) |  | [optional]
**Sandbox** | Pointer to **bool** | When true, returns deterministic mock results without consuming credits or making real SMTP connections. | [optional]
**SmtpPort** | Pointer to **float32** | SMTP port to use for email validation. Defaults to 25, but 465, 587, and 2525 are sometimes also used. | [optional]
**StrictProviderRules** | Pointer to **bool** | When false, skips provider-specific syntax validation even if the provider is recognized. | [optional]
**ToEmail** | **string** | The email address to check. | [required]
**YahooVerifMethod** | Pointer to [**YahooVerifMethod**](YahooVerifMethod.md) |  | [optional]

## Methods

### NewCheckEmailRequest

`func NewCheckEmailRequest(toEmail string) *CheckEmailRequest`

NewCheckEmailRequest instantiates a new CheckEmailRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCheckEmailRequestWithDefaults

`func NewCheckEmailRequestWithDefaults() *CheckEmailRequest`

NewCheckEmailRequestWithDefaults instantiates a new CheckEmailRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCheckGravatar

`func (o *CheckEmailRequest) GetCheckGravatar() bool`

GetCheckGravatar returns the CheckGravatar field if non-nil, zero value otherwise.

### GetCheckGravatarOk

`func (o *CheckEmailRequest) GetCheckGravatarOk() (*bool, bool)`

GetCheckGravatarOk returns a tuple with the CheckGravatar field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCheckGravatar

`func (o *CheckEmailRequest) SetCheckGravatar(v bool)`

SetCheckGravatar sets CheckGravatar field to given value.

### HasCheckGravatar

`func (o *CheckEmailRequest) HasCheckGravatar() bool`

HasCheckGravatar returns a boolean if a field has been set.

### GetFromEmail

`func (o *CheckEmailRequest) GetFromEmail() string`

GetFromEmail returns the FromEmail field if non-nil, zero value otherwise.

### GetFromEmailOk

`func (o *CheckEmailRequest) GetFromEmailOk() (*string, bool)`

GetFromEmailOk returns a tuple with the FromEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFromEmail

`func (o *CheckEmailRequest) SetFromEmail(v string)`

SetFromEmail sets FromEmail field to given value.

### HasFromEmail

`func (o *CheckEmailRequest) HasFromEmail() bool`

HasFromEmail returns a boolean if a field has been set.

### GetGmailVerifMethod

`func (o *CheckEmailRequest) GetGmailVerifMethod() GmailVerifMethod`

GetGmailVerifMethod returns the GmailVerifMethod field if non-nil, zero value otherwise.

### GetGmailVerifMethodOk

`func (o *CheckEmailRequest) GetGmailVerifMethodOk() (*GmailVerifMethod, bool)`

GetGmailVerifMethodOk returns a tuple with the GmailVerifMethod field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGmailVerifMethod

`func (o *CheckEmailRequest) SetGmailVerifMethod(v GmailVerifMethod)`

SetGmailVerifMethod sets GmailVerifMethod field to given value.

### HasGmailVerifMethod

`func (o *CheckEmailRequest) HasGmailVerifMethod() bool`

HasGmailVerifMethod returns a boolean if a field has been set.

### GetHelloName

`func (o *CheckEmailRequest) GetHelloName() string`

GetHelloName returns the HelloName field if non-nil, zero value otherwise.

### GetHelloNameOk

`func (o *CheckEmailRequest) GetHelloNameOk() (*string, bool)`

GetHelloNameOk returns a tuple with the HelloName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHelloName

`func (o *CheckEmailRequest) SetHelloName(v string)`

SetHelloName sets HelloName field to given value.

### HasHelloName

`func (o *CheckEmailRequest) HasHelloName() bool`

HasHelloName returns a boolean if a field has been set.

### GetHotmailb2bVerifMethod

`func (o *CheckEmailRequest) GetHotmailb2bVerifMethod() HotmailB2BVerifMethod`

GetHotmailb2bVerifMethod returns the Hotmailb2bVerifMethod field if non-nil, zero value otherwise.

### GetHotmailb2bVerifMethodOk

`func (o *CheckEmailRequest) GetHotmailb2bVerifMethodOk() (*HotmailB2BVerifMethod, bool)`

GetHotmailb2bVerifMethodOk returns a tuple with the Hotmailb2bVerifMethod field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHotmailb2bVerifMethod

`func (o *CheckEmailRequest) SetHotmailb2bVerifMethod(v HotmailB2BVerifMethod)`

SetHotmailb2bVerifMethod sets Hotmailb2bVerifMethod field to given value.

### HasHotmailb2bVerifMethod

`func (o *CheckEmailRequest) HasHotmailb2bVerifMethod() bool`

HasHotmailb2bVerifMethod returns a boolean if a field has been set.

### GetHotmailb2cVerifMethod

`func (o *CheckEmailRequest) GetHotmailb2cVerifMethod() HotmailB2CVerifMethod`

GetHotmailb2cVerifMethod returns the Hotmailb2cVerifMethod field if non-nil, zero value otherwise.

### GetHotmailb2cVerifMethodOk

`func (o *CheckEmailRequest) GetHotmailb2cVerifMethodOk() (*HotmailB2CVerifMethod, bool)`

GetHotmailb2cVerifMethodOk returns a tuple with the Hotmailb2cVerifMethod field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHotmailb2cVerifMethod

`func (o *CheckEmailRequest) SetHotmailb2cVerifMethod(v HotmailB2CVerifMethod)`

SetHotmailb2cVerifMethod sets Hotmailb2cVerifMethod field to given value.

### HasHotmailb2cVerifMethod

`func (o *CheckEmailRequest) HasHotmailb2cVerifMethod() bool`

HasHotmailb2cVerifMethod returns a boolean if a field has been set.

### GetProxy

`func (o *CheckEmailRequest) GetProxy() CheckEmailInputProxy`

GetProxy returns the Proxy field if non-nil, zero value otherwise.

### GetProxyOk

`func (o *CheckEmailRequest) GetProxyOk() (*CheckEmailInputProxy, bool)`

GetProxyOk returns a tuple with the Proxy field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProxy

`func (o *CheckEmailRequest) SetProxy(v CheckEmailInputProxy)`

SetProxy sets Proxy field to given value.

### HasProxy

`func (o *CheckEmailRequest) HasProxy() bool`

HasProxy returns a boolean if a field has been set.

### GetSandbox

`func (o *CheckEmailRequest) GetSandbox() bool`

GetSandbox returns the Sandbox field if non-nil, zero value otherwise.

### GetSandboxOk

`func (o *CheckEmailRequest) GetSandboxOk() (*bool, bool)`

GetSandboxOk returns a tuple with the Sandbox field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSandbox

`func (o *CheckEmailRequest) SetSandbox(v bool)`

SetSandbox sets Sandbox field to given value.

### HasSandbox

`func (o *CheckEmailRequest) HasSandbox() bool`

HasSandbox returns a boolean if a field has been set.

### GetSmtpPort

`func (o *CheckEmailRequest) GetSmtpPort() float32`

GetSmtpPort returns the SmtpPort field if non-nil, zero value otherwise.

### GetSmtpPortOk

`func (o *CheckEmailRequest) GetSmtpPortOk() (*float32, bool)`

GetSmtpPortOk returns a tuple with the SmtpPort field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSmtpPort

`func (o *CheckEmailRequest) SetSmtpPort(v float32)`

SetSmtpPort sets SmtpPort field to given value.

### HasSmtpPort

`func (o *CheckEmailRequest) HasSmtpPort() bool`

HasSmtpPort returns a boolean if a field has been set.

### GetStrictProviderRules

`func (o *CheckEmailRequest) GetStrictProviderRules() bool`

GetStrictProviderRules returns the StrictProviderRules field if non-nil, zero value otherwise.

### GetStrictProviderRulesOk

`func (o *CheckEmailRequest) GetStrictProviderRulesOk() (*bool, bool)`

GetStrictProviderRulesOk returns a tuple with the StrictProviderRules field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStrictProviderRules

`func (o *CheckEmailRequest) SetStrictProviderRules(v bool)`

SetStrictProviderRules sets StrictProviderRules field to given value.

### HasStrictProviderRules

`func (o *CheckEmailRequest) HasStrictProviderRules() bool`

HasStrictProviderRules returns a boolean if a field has been set.

### GetToEmail

`func (o *CheckEmailRequest) GetToEmail() string`

GetToEmail returns the ToEmail field if non-nil, zero value otherwise.

### GetToEmailOk

`func (o *CheckEmailRequest) GetToEmailOk() (*string, bool)`

GetToEmailOk returns a tuple with the ToEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetToEmail

`func (o *CheckEmailRequest) SetToEmail(v string)`

SetToEmail sets ToEmail field to given value.


### GetYahooVerifMethod

`func (o *CheckEmailRequest) GetYahooVerifMethod() YahooVerifMethod`

GetYahooVerifMethod returns the YahooVerifMethod field if non-nil, zero value otherwise.

### GetYahooVerifMethodOk

`func (o *CheckEmailRequest) GetYahooVerifMethodOk() (*YahooVerifMethod, bool)`

GetYahooVerifMethodOk returns a tuple with the YahooVerifMethod field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetYahooVerifMethod

`func (o *CheckEmailRequest) SetYahooVerifMethod(v YahooVerifMethod)`

SetYahooVerifMethod sets YahooVerifMethod field to given value.

### HasYahooVerifMethod

`func (o *CheckEmailRequest) HasYahooVerifMethod() bool`

HasYahooVerifMethod returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
