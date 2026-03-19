# ScoringSignals

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**HasDomainSuggestion** | **bool** |  | 
**HasMxRecords** | **bool** |  | 
**IsDisposable** | **bool** |  | 
**IsFreeProvider** | **bool** |  | 
**IsRoleAccount** | **bool** |  | 
**IsSpamTrapDomain** | **bool** |  | 
**Reachable** | [**Reachable**](Reachable.md) |  | 
**SmtpCanConnect** | **bool** |  | 
**SmtpError** | **bool** |  | 
**SmtpHasFullInbox** | **bool** |  | 
**SmtpIsCatchAll** | **bool** |  | 
**SmtpIsDeliverable** | **bool** |  | 
**SmtpIsDisabled** | **bool** |  | 
**ValidSyntax** | **bool** |  | 

## Methods

### NewScoringSignals

`func NewScoringSignals(hasDomainSuggestion bool, hasMxRecords bool, isDisposable bool, isFreeProvider bool, isRoleAccount bool, isSpamTrapDomain bool, reachable Reachable, smtpCanConnect bool, smtpError bool, smtpHasFullInbox bool, smtpIsCatchAll bool, smtpIsDeliverable bool, smtpIsDisabled bool, validSyntax bool, ) *ScoringSignals`

NewScoringSignals instantiates a new ScoringSignals object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewScoringSignalsWithDefaults

`func NewScoringSignalsWithDefaults() *ScoringSignals`

NewScoringSignalsWithDefaults instantiates a new ScoringSignals object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetHasDomainSuggestion

`func (o *ScoringSignals) GetHasDomainSuggestion() bool`

GetHasDomainSuggestion returns the HasDomainSuggestion field if non-nil, zero value otherwise.

### GetHasDomainSuggestionOk

`func (o *ScoringSignals) GetHasDomainSuggestionOk() (*bool, bool)`

GetHasDomainSuggestionOk returns a tuple with the HasDomainSuggestion field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasDomainSuggestion

`func (o *ScoringSignals) SetHasDomainSuggestion(v bool)`

SetHasDomainSuggestion sets HasDomainSuggestion field to given value.


### GetHasMxRecords

`func (o *ScoringSignals) GetHasMxRecords() bool`

GetHasMxRecords returns the HasMxRecords field if non-nil, zero value otherwise.

### GetHasMxRecordsOk

`func (o *ScoringSignals) GetHasMxRecordsOk() (*bool, bool)`

GetHasMxRecordsOk returns a tuple with the HasMxRecords field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasMxRecords

`func (o *ScoringSignals) SetHasMxRecords(v bool)`

SetHasMxRecords sets HasMxRecords field to given value.


### GetIsDisposable

`func (o *ScoringSignals) GetIsDisposable() bool`

GetIsDisposable returns the IsDisposable field if non-nil, zero value otherwise.

### GetIsDisposableOk

`func (o *ScoringSignals) GetIsDisposableOk() (*bool, bool)`

GetIsDisposableOk returns a tuple with the IsDisposable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsDisposable

`func (o *ScoringSignals) SetIsDisposable(v bool)`

SetIsDisposable sets IsDisposable field to given value.


### GetIsFreeProvider

`func (o *ScoringSignals) GetIsFreeProvider() bool`

GetIsFreeProvider returns the IsFreeProvider field if non-nil, zero value otherwise.

### GetIsFreeProviderOk

`func (o *ScoringSignals) GetIsFreeProviderOk() (*bool, bool)`

GetIsFreeProviderOk returns a tuple with the IsFreeProvider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsFreeProvider

`func (o *ScoringSignals) SetIsFreeProvider(v bool)`

SetIsFreeProvider sets IsFreeProvider field to given value.


### GetIsRoleAccount

`func (o *ScoringSignals) GetIsRoleAccount() bool`

GetIsRoleAccount returns the IsRoleAccount field if non-nil, zero value otherwise.

### GetIsRoleAccountOk

`func (o *ScoringSignals) GetIsRoleAccountOk() (*bool, bool)`

GetIsRoleAccountOk returns a tuple with the IsRoleAccount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsRoleAccount

`func (o *ScoringSignals) SetIsRoleAccount(v bool)`

SetIsRoleAccount sets IsRoleAccount field to given value.


### GetIsSpamTrapDomain

`func (o *ScoringSignals) GetIsSpamTrapDomain() bool`

GetIsSpamTrapDomain returns the IsSpamTrapDomain field if non-nil, zero value otherwise.

### GetIsSpamTrapDomainOk

`func (o *ScoringSignals) GetIsSpamTrapDomainOk() (*bool, bool)`

GetIsSpamTrapDomainOk returns a tuple with the IsSpamTrapDomain field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsSpamTrapDomain

`func (o *ScoringSignals) SetIsSpamTrapDomain(v bool)`

SetIsSpamTrapDomain sets IsSpamTrapDomain field to given value.


### GetReachable

`func (o *ScoringSignals) GetReachable() Reachable`

GetReachable returns the Reachable field if non-nil, zero value otherwise.

### GetReachableOk

`func (o *ScoringSignals) GetReachableOk() (*Reachable, bool)`

GetReachableOk returns a tuple with the Reachable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReachable

`func (o *ScoringSignals) SetReachable(v Reachable)`

SetReachable sets Reachable field to given value.


### GetSmtpCanConnect

`func (o *ScoringSignals) GetSmtpCanConnect() bool`

GetSmtpCanConnect returns the SmtpCanConnect field if non-nil, zero value otherwise.

### GetSmtpCanConnectOk

`func (o *ScoringSignals) GetSmtpCanConnectOk() (*bool, bool)`

GetSmtpCanConnectOk returns a tuple with the SmtpCanConnect field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSmtpCanConnect

`func (o *ScoringSignals) SetSmtpCanConnect(v bool)`

SetSmtpCanConnect sets SmtpCanConnect field to given value.


### GetSmtpError

`func (o *ScoringSignals) GetSmtpError() bool`

GetSmtpError returns the SmtpError field if non-nil, zero value otherwise.

### GetSmtpErrorOk

`func (o *ScoringSignals) GetSmtpErrorOk() (*bool, bool)`

GetSmtpErrorOk returns a tuple with the SmtpError field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSmtpError

`func (o *ScoringSignals) SetSmtpError(v bool)`

SetSmtpError sets SmtpError field to given value.


### GetSmtpHasFullInbox

`func (o *ScoringSignals) GetSmtpHasFullInbox() bool`

GetSmtpHasFullInbox returns the SmtpHasFullInbox field if non-nil, zero value otherwise.

### GetSmtpHasFullInboxOk

`func (o *ScoringSignals) GetSmtpHasFullInboxOk() (*bool, bool)`

GetSmtpHasFullInboxOk returns a tuple with the SmtpHasFullInbox field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSmtpHasFullInbox

`func (o *ScoringSignals) SetSmtpHasFullInbox(v bool)`

SetSmtpHasFullInbox sets SmtpHasFullInbox field to given value.


### GetSmtpIsCatchAll

`func (o *ScoringSignals) GetSmtpIsCatchAll() bool`

GetSmtpIsCatchAll returns the SmtpIsCatchAll field if non-nil, zero value otherwise.

### GetSmtpIsCatchAllOk

`func (o *ScoringSignals) GetSmtpIsCatchAllOk() (*bool, bool)`

GetSmtpIsCatchAllOk returns a tuple with the SmtpIsCatchAll field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSmtpIsCatchAll

`func (o *ScoringSignals) SetSmtpIsCatchAll(v bool)`

SetSmtpIsCatchAll sets SmtpIsCatchAll field to given value.


### GetSmtpIsDeliverable

`func (o *ScoringSignals) GetSmtpIsDeliverable() bool`

GetSmtpIsDeliverable returns the SmtpIsDeliverable field if non-nil, zero value otherwise.

### GetSmtpIsDeliverableOk

`func (o *ScoringSignals) GetSmtpIsDeliverableOk() (*bool, bool)`

GetSmtpIsDeliverableOk returns a tuple with the SmtpIsDeliverable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSmtpIsDeliverable

`func (o *ScoringSignals) SetSmtpIsDeliverable(v bool)`

SetSmtpIsDeliverable sets SmtpIsDeliverable field to given value.


### GetSmtpIsDisabled

`func (o *ScoringSignals) GetSmtpIsDisabled() bool`

GetSmtpIsDisabled returns the SmtpIsDisabled field if non-nil, zero value otherwise.

### GetSmtpIsDisabledOk

`func (o *ScoringSignals) GetSmtpIsDisabledOk() (*bool, bool)`

GetSmtpIsDisabledOk returns a tuple with the SmtpIsDisabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSmtpIsDisabled

`func (o *ScoringSignals) SetSmtpIsDisabled(v bool)`

SetSmtpIsDisabled sets SmtpIsDisabled field to given value.


### GetValidSyntax

`func (o *ScoringSignals) GetValidSyntax() bool`

GetValidSyntax returns the ValidSyntax field if non-nil, zero value otherwise.

### GetValidSyntaxOk

`func (o *ScoringSignals) GetValidSyntaxOk() (*bool, bool)`

GetValidSyntaxOk returns a tuple with the ValidSyntax field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidSyntax

`func (o *ScoringSignals) SetValidSyntax(v bool)`

SetValidSyntax sets ValidSyntax field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


