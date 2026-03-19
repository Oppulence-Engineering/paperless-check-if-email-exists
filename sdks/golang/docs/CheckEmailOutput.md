# CheckEmailOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Debug** | Pointer to [**DebugDetails**](DebugDetails.md) |  | [optional] 
**Input** | **string** | The email address that was verified. | 
**IsReachable** | [**Reachable**](Reachable.md) |  | 
**Misc** | [**CheckEmailOutputMisc**](CheckEmailOutputMisc.md) |  | 
**Mx** | [**CheckEmailOutputMx**](CheckEmailOutputMx.md) |  | 
**Score** | [**EmailScore**](EmailScore.md) |  | 
**Smtp** | [**CheckEmailOutputSmtp**](CheckEmailOutputSmtp.md) |  | 
**Syntax** | [**SyntaxDetails**](SyntaxDetails.md) |  | 

## Methods

### NewCheckEmailOutput

`func NewCheckEmailOutput(input string, isReachable Reachable, misc CheckEmailOutputMisc, mx CheckEmailOutputMx, score EmailScore, smtp CheckEmailOutputSmtp, syntax SyntaxDetails, ) *CheckEmailOutput`

NewCheckEmailOutput instantiates a new CheckEmailOutput object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCheckEmailOutputWithDefaults

`func NewCheckEmailOutputWithDefaults() *CheckEmailOutput`

NewCheckEmailOutputWithDefaults instantiates a new CheckEmailOutput object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDebug

`func (o *CheckEmailOutput) GetDebug() DebugDetails`

GetDebug returns the Debug field if non-nil, zero value otherwise.

### GetDebugOk

`func (o *CheckEmailOutput) GetDebugOk() (*DebugDetails, bool)`

GetDebugOk returns a tuple with the Debug field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDebug

`func (o *CheckEmailOutput) SetDebug(v DebugDetails)`

SetDebug sets Debug field to given value.

### HasDebug

`func (o *CheckEmailOutput) HasDebug() bool`

HasDebug returns a boolean if a field has been set.

### GetInput

`func (o *CheckEmailOutput) GetInput() string`

GetInput returns the Input field if non-nil, zero value otherwise.

### GetInputOk

`func (o *CheckEmailOutput) GetInputOk() (*string, bool)`

GetInputOk returns a tuple with the Input field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInput

`func (o *CheckEmailOutput) SetInput(v string)`

SetInput sets Input field to given value.


### GetIsReachable

`func (o *CheckEmailOutput) GetIsReachable() Reachable`

GetIsReachable returns the IsReachable field if non-nil, zero value otherwise.

### GetIsReachableOk

`func (o *CheckEmailOutput) GetIsReachableOk() (*Reachable, bool)`

GetIsReachableOk returns a tuple with the IsReachable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsReachable

`func (o *CheckEmailOutput) SetIsReachable(v Reachable)`

SetIsReachable sets IsReachable field to given value.


### GetMisc

`func (o *CheckEmailOutput) GetMisc() CheckEmailOutputMisc`

GetMisc returns the Misc field if non-nil, zero value otherwise.

### GetMiscOk

`func (o *CheckEmailOutput) GetMiscOk() (*CheckEmailOutputMisc, bool)`

GetMiscOk returns a tuple with the Misc field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMisc

`func (o *CheckEmailOutput) SetMisc(v CheckEmailOutputMisc)`

SetMisc sets Misc field to given value.


### GetMx

`func (o *CheckEmailOutput) GetMx() CheckEmailOutputMx`

GetMx returns the Mx field if non-nil, zero value otherwise.

### GetMxOk

`func (o *CheckEmailOutput) GetMxOk() (*CheckEmailOutputMx, bool)`

GetMxOk returns a tuple with the Mx field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMx

`func (o *CheckEmailOutput) SetMx(v CheckEmailOutputMx)`

SetMx sets Mx field to given value.


### GetScore

`func (o *CheckEmailOutput) GetScore() EmailScore`

GetScore returns the Score field if non-nil, zero value otherwise.

### GetScoreOk

`func (o *CheckEmailOutput) GetScoreOk() (*EmailScore, bool)`

GetScoreOk returns a tuple with the Score field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScore

`func (o *CheckEmailOutput) SetScore(v EmailScore)`

SetScore sets Score field to given value.


### GetSmtp

`func (o *CheckEmailOutput) GetSmtp() CheckEmailOutputSmtp`

GetSmtp returns the Smtp field if non-nil, zero value otherwise.

### GetSmtpOk

`func (o *CheckEmailOutput) GetSmtpOk() (*CheckEmailOutputSmtp, bool)`

GetSmtpOk returns a tuple with the Smtp field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSmtp

`func (o *CheckEmailOutput) SetSmtp(v CheckEmailOutputSmtp)`

SetSmtp sets Smtp field to given value.


### GetSyntax

`func (o *CheckEmailOutput) GetSyntax() SyntaxDetails`

GetSyntax returns the Syntax field if non-nil, zero value otherwise.

### GetSyntaxOk

`func (o *CheckEmailOutput) GetSyntaxOk() (*SyntaxDetails, bool)`

GetSyntaxOk returns a tuple with the Syntax field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSyntax

`func (o *CheckEmailOutput) SetSyntax(v SyntaxDetails)`

SetSyntax sets Syntax field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


