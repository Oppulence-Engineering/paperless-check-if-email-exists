# SegmentFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Category** | Pointer to [**SegmentFilterCategory**](SegmentFilterCategory.md) |  | [optional]
**ChangeType** | Pointer to [**SegmentFilterCategory**](SegmentFilterCategory.md) |  | [optional]
**FreshnessDaysMax** | Pointer to **int32** |  | [optional]
**IsCatchAll** | Pointer to **bool** |  | [optional]
**IsDisposable** | Pointer to **bool** |  | [optional]
**IsRoleAccount** | Pointer to **bool** |  | [optional]
**PolicyDecision** | Pointer to **string** |  | [optional]
**ReasonCodesAny** | Pointer to **[]string** |  | [optional]
**SafeToSend** | Pointer to **bool** |  | [optional]
**ScoreMax** | Pointer to **int32** |  | [optional]
**ScoreMin** | Pointer to **int32** |  | [optional]

## Methods

### NewSegmentFilter

`func NewSegmentFilter() *SegmentFilter`

NewSegmentFilter instantiates a new SegmentFilter object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSegmentFilterWithDefaults

`func NewSegmentFilterWithDefaults() *SegmentFilter`

NewSegmentFilterWithDefaults instantiates a new SegmentFilter object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCategory

`func (o *SegmentFilter) GetCategory() SegmentFilterCategory`

GetCategory returns the Category field if non-nil, zero value otherwise.

### GetCategoryOk

`func (o *SegmentFilter) GetCategoryOk() (*SegmentFilterCategory, bool)`

GetCategoryOk returns a tuple with the Category field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCategory

`func (o *SegmentFilter) SetCategory(v SegmentFilterCategory)`

SetCategory sets Category field to given value.

### HasCategory

`func (o *SegmentFilter) HasCategory() bool`

HasCategory returns a boolean if a field has been set.

### GetChangeType

`func (o *SegmentFilter) GetChangeType() SegmentFilterCategory`

GetChangeType returns the ChangeType field if non-nil, zero value otherwise.

### GetChangeTypeOk

`func (o *SegmentFilter) GetChangeTypeOk() (*SegmentFilterCategory, bool)`

GetChangeTypeOk returns a tuple with the ChangeType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetChangeType

`func (o *SegmentFilter) SetChangeType(v SegmentFilterCategory)`

SetChangeType sets ChangeType field to given value.

### HasChangeType

`func (o *SegmentFilter) HasChangeType() bool`

HasChangeType returns a boolean if a field has been set.

### GetFreshnessDaysMax

`func (o *SegmentFilter) GetFreshnessDaysMax() int32`

GetFreshnessDaysMax returns the FreshnessDaysMax field if non-nil, zero value otherwise.

### GetFreshnessDaysMaxOk

`func (o *SegmentFilter) GetFreshnessDaysMaxOk() (*int32, bool)`

GetFreshnessDaysMaxOk returns a tuple with the FreshnessDaysMax field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFreshnessDaysMax

`func (o *SegmentFilter) SetFreshnessDaysMax(v int32)`

SetFreshnessDaysMax sets FreshnessDaysMax field to given value.

### HasFreshnessDaysMax

`func (o *SegmentFilter) HasFreshnessDaysMax() bool`

HasFreshnessDaysMax returns a boolean if a field has been set.

### GetIsCatchAll

`func (o *SegmentFilter) GetIsCatchAll() bool`

GetIsCatchAll returns the IsCatchAll field if non-nil, zero value otherwise.

### GetIsCatchAllOk

`func (o *SegmentFilter) GetIsCatchAllOk() (*bool, bool)`

GetIsCatchAllOk returns a tuple with the IsCatchAll field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsCatchAll

`func (o *SegmentFilter) SetIsCatchAll(v bool)`

SetIsCatchAll sets IsCatchAll field to given value.

### HasIsCatchAll

`func (o *SegmentFilter) HasIsCatchAll() bool`

HasIsCatchAll returns a boolean if a field has been set.

### GetIsDisposable

`func (o *SegmentFilter) GetIsDisposable() bool`

GetIsDisposable returns the IsDisposable field if non-nil, zero value otherwise.

### GetIsDisposableOk

`func (o *SegmentFilter) GetIsDisposableOk() (*bool, bool)`

GetIsDisposableOk returns a tuple with the IsDisposable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsDisposable

`func (o *SegmentFilter) SetIsDisposable(v bool)`

SetIsDisposable sets IsDisposable field to given value.

### HasIsDisposable

`func (o *SegmentFilter) HasIsDisposable() bool`

HasIsDisposable returns a boolean if a field has been set.

### GetIsRoleAccount

`func (o *SegmentFilter) GetIsRoleAccount() bool`

GetIsRoleAccount returns the IsRoleAccount field if non-nil, zero value otherwise.

### GetIsRoleAccountOk

`func (o *SegmentFilter) GetIsRoleAccountOk() (*bool, bool)`

GetIsRoleAccountOk returns a tuple with the IsRoleAccount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsRoleAccount

`func (o *SegmentFilter) SetIsRoleAccount(v bool)`

SetIsRoleAccount sets IsRoleAccount field to given value.

### HasIsRoleAccount

`func (o *SegmentFilter) HasIsRoleAccount() bool`

HasIsRoleAccount returns a boolean if a field has been set.

### GetPolicyDecision

`func (o *SegmentFilter) GetPolicyDecision() string`

GetPolicyDecision returns the PolicyDecision field if non-nil, zero value otherwise.

### GetPolicyDecisionOk

`func (o *SegmentFilter) GetPolicyDecisionOk() (*string, bool)`

GetPolicyDecisionOk returns a tuple with the PolicyDecision field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicyDecision

`func (o *SegmentFilter) SetPolicyDecision(v string)`

SetPolicyDecision sets PolicyDecision field to given value.

### HasPolicyDecision

`func (o *SegmentFilter) HasPolicyDecision() bool`

HasPolicyDecision returns a boolean if a field has been set.

### GetReasonCodesAny

`func (o *SegmentFilter) GetReasonCodesAny() []string`

GetReasonCodesAny returns the ReasonCodesAny field if non-nil, zero value otherwise.

### GetReasonCodesAnyOk

`func (o *SegmentFilter) GetReasonCodesAnyOk() ([]string, bool)`

GetReasonCodesAnyOk returns a tuple with the ReasonCodesAny field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReasonCodesAny

`func (o *SegmentFilter) SetReasonCodesAny(v []string)`

SetReasonCodesAny sets ReasonCodesAny field to given value.

### HasReasonCodesAny

`func (o *SegmentFilter) HasReasonCodesAny() bool`

HasReasonCodesAny returns a boolean if a field has been set.

### GetSafeToSend

`func (o *SegmentFilter) GetSafeToSend() bool`

GetSafeToSend returns the SafeToSend field if non-nil, zero value otherwise.

### GetSafeToSendOk

`func (o *SegmentFilter) GetSafeToSendOk() (*bool, bool)`

GetSafeToSendOk returns a tuple with the SafeToSend field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSafeToSend

`func (o *SegmentFilter) SetSafeToSend(v bool)`

SetSafeToSend sets SafeToSend field to given value.

### HasSafeToSend

`func (o *SegmentFilter) HasSafeToSend() bool`

HasSafeToSend returns a boolean if a field has been set.

### GetScoreMax

`func (o *SegmentFilter) GetScoreMax() int32`

GetScoreMax returns the ScoreMax field if non-nil, zero value otherwise.

### GetScoreMaxOk

`func (o *SegmentFilter) GetScoreMaxOk() (*int32, bool)`

GetScoreMaxOk returns a tuple with the ScoreMax field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScoreMax

`func (o *SegmentFilter) SetScoreMax(v int32)`

SetScoreMax sets ScoreMax field to given value.

### HasScoreMax

`func (o *SegmentFilter) HasScoreMax() bool`

HasScoreMax returns a boolean if a field has been set.

### GetScoreMin

`func (o *SegmentFilter) GetScoreMin() int32`

GetScoreMin returns the ScoreMin field if non-nil, zero value otherwise.

### GetScoreMinOk

`func (o *SegmentFilter) GetScoreMinOk() (*int32, bool)`

GetScoreMinOk returns a tuple with the ScoreMin field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScoreMin

`func (o *SegmentFilter) SetScoreMin(v int32)`

SetScoreMin sets ScoreMin field to given value.

### HasScoreMin

`func (o *SegmentFilter) HasScoreMin() bool`

HasScoreMin returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
