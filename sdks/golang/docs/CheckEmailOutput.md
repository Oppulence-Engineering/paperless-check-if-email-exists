# CheckEmailOutput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BounceRisk** | Pointer to [**NullableBounceRiskAssessment**](BounceRiskAssessment.md) | Additive bounce-risk assessment. May be null when enrichment is disabled or unavailable. | [optional]
**Debug** | Pointer to [**DebugDetails**](DebugDetails.md) |  | [optional]
**Input** | **string** | The email address that was verified. | [required]
**IsReachable** | [**Reachable**](Reachable.md) |  | [required]
**Misc** | [**CheckEmailOutputMisc**](CheckEmailOutputMisc.md) |  | [required]
**Mx** | [**CheckEmailOutputMx**](CheckEmailOutputMx.md) |  | [required]
**Provider** | Pointer to [**Provider**](Provider.md) |  | [optional]
**ProviderConfidence** | Pointer to [**ProviderConfidence**](ProviderConfidence.md) |  | [optional]
**ProviderRejectionReason** | Pointer to [**ProviderRejectionReason**](ProviderRejectionReason.md) |  | [optional]
**ProviderRulesApplied** | **bool** |  | [required]
**Score** | [**EmailScore**](EmailScore.md) |  | [required]
**Smtp** | [**CheckEmailOutputSmtp**](CheckEmailOutputSmtp.md) |  | [required]
**Syntax** | [**SyntaxDetails**](SyntaxDetails.md) |  | [required]

## Methods

### NewCheckEmailOutput

`func NewCheckEmailOutput(input string, isReachable Reachable, misc CheckEmailOutputMisc, mx CheckEmailOutputMx, providerRulesApplied bool, score EmailScore, smtp CheckEmailOutputSmtp, syntax SyntaxDetails) *CheckEmailOutput`

NewCheckEmailOutput instantiates a new CheckEmailOutput object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCheckEmailOutputWithDefaults

`func NewCheckEmailOutputWithDefaults() *CheckEmailOutput`

NewCheckEmailOutputWithDefaults instantiates a new CheckEmailOutput object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBounceRisk

`func (o *CheckEmailOutput) GetBounceRisk() BounceRiskAssessment`

GetBounceRisk returns the BounceRisk field if non-nil, zero value otherwise.

### GetBounceRiskOk

`func (o *CheckEmailOutput) GetBounceRiskOk() (*BounceRiskAssessment, bool)`

GetBounceRiskOk returns a tuple with the BounceRisk field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBounceRisk

`func (o *CheckEmailOutput) SetBounceRisk(v BounceRiskAssessment)`

SetBounceRisk sets BounceRisk field to given value.

### HasBounceRisk

`func (o *CheckEmailOutput) HasBounceRisk() bool`

HasBounceRisk returns a boolean if a field has been set.

### SetBounceRiskNil

`func (o *CheckEmailOutput) SetBounceRiskNil()`

 SetBounceRiskNil sets the value for BounceRisk to be an explicit nil

### UnsetBounceRisk
`func (o *CheckEmailOutput) UnsetBounceRisk()`

UnsetBounceRisk ensures that no value is present for BounceRisk, not even an explicit nil

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


### GetProvider

`func (o *CheckEmailOutput) GetProvider() Provider`

GetProvider returns the Provider field if non-nil, zero value otherwise.

### GetProviderOk

`func (o *CheckEmailOutput) GetProviderOk() (*Provider, bool)`

GetProviderOk returns a tuple with the Provider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProvider

`func (o *CheckEmailOutput) SetProvider(v Provider)`

SetProvider sets Provider field to given value.

### HasProvider

`func (o *CheckEmailOutput) HasProvider() bool`

HasProvider returns a boolean if a field has been set.

### GetProviderConfidence

`func (o *CheckEmailOutput) GetProviderConfidence() ProviderConfidence`

GetProviderConfidence returns the ProviderConfidence field if non-nil, zero value otherwise.

### GetProviderConfidenceOk

`func (o *CheckEmailOutput) GetProviderConfidenceOk() (*ProviderConfidence, bool)`

GetProviderConfidenceOk returns a tuple with the ProviderConfidence field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderConfidence

`func (o *CheckEmailOutput) SetProviderConfidence(v ProviderConfidence)`

SetProviderConfidence sets ProviderConfidence field to given value.

### HasProviderConfidence

`func (o *CheckEmailOutput) HasProviderConfidence() bool`

HasProviderConfidence returns a boolean if a field has been set.

### GetProviderRejectionReason

`func (o *CheckEmailOutput) GetProviderRejectionReason() ProviderRejectionReason`

GetProviderRejectionReason returns the ProviderRejectionReason field if non-nil, zero value otherwise.

### GetProviderRejectionReasonOk

`func (o *CheckEmailOutput) GetProviderRejectionReasonOk() (*ProviderRejectionReason, bool)`

GetProviderRejectionReasonOk returns a tuple with the ProviderRejectionReason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderRejectionReason

`func (o *CheckEmailOutput) SetProviderRejectionReason(v ProviderRejectionReason)`

SetProviderRejectionReason sets ProviderRejectionReason field to given value.

### HasProviderRejectionReason

`func (o *CheckEmailOutput) HasProviderRejectionReason() bool`

HasProviderRejectionReason returns a boolean if a field has been set.

### GetProviderRulesApplied

`func (o *CheckEmailOutput) GetProviderRulesApplied() bool`

GetProviderRulesApplied returns the ProviderRulesApplied field if non-nil, zero value otherwise.

### GetProviderRulesAppliedOk

`func (o *CheckEmailOutput) GetProviderRulesAppliedOk() (*bool, bool)`

GetProviderRulesAppliedOk returns a tuple with the ProviderRulesApplied field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderRulesApplied

`func (o *CheckEmailOutput) SetProviderRulesApplied(v bool)`

SetProviderRulesApplied sets ProviderRulesApplied field to given value.


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
