# EmailScore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AgeDays** | Pointer to **int64** |  | [optional] 
**Category** | [**EmailCategory**](EmailCategory.md) |  | 
**Freshness** | Pointer to [**Freshness**](Freshness.md) |  | [optional] 
**ReasonCodes** | [**[]ReasonCode**](ReasonCode.md) |  | 
**SafeToSend** | **bool** |  | 
**Score** | **int32** |  | 
**Signals** | [**ScoringSignals**](ScoringSignals.md) |  | 
**SubReason** | [**SubReason**](SubReason.md) |  | 
**VerifiedAt** | Pointer to **time.Time** |  | [optional] 

## Methods

### NewEmailScore

`func NewEmailScore(category EmailCategory, reasonCodes []ReasonCode, safeToSend bool, score int32, signals ScoringSignals, subReason SubReason, ) *EmailScore`

NewEmailScore instantiates a new EmailScore object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewEmailScoreWithDefaults

`func NewEmailScoreWithDefaults() *EmailScore`

NewEmailScoreWithDefaults instantiates a new EmailScore object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAgeDays

`func (o *EmailScore) GetAgeDays() int64`

GetAgeDays returns the AgeDays field if non-nil, zero value otherwise.

### GetAgeDaysOk

`func (o *EmailScore) GetAgeDaysOk() (*int64, bool)`

GetAgeDaysOk returns a tuple with the AgeDays field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAgeDays

`func (o *EmailScore) SetAgeDays(v int64)`

SetAgeDays sets AgeDays field to given value.

### HasAgeDays

`func (o *EmailScore) HasAgeDays() bool`

HasAgeDays returns a boolean if a field has been set.

### GetCategory

`func (o *EmailScore) GetCategory() EmailCategory`

GetCategory returns the Category field if non-nil, zero value otherwise.

### GetCategoryOk

`func (o *EmailScore) GetCategoryOk() (*EmailCategory, bool)`

GetCategoryOk returns a tuple with the Category field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCategory

`func (o *EmailScore) SetCategory(v EmailCategory)`

SetCategory sets Category field to given value.


### GetFreshness

`func (o *EmailScore) GetFreshness() Freshness`

GetFreshness returns the Freshness field if non-nil, zero value otherwise.

### GetFreshnessOk

`func (o *EmailScore) GetFreshnessOk() (*Freshness, bool)`

GetFreshnessOk returns a tuple with the Freshness field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFreshness

`func (o *EmailScore) SetFreshness(v Freshness)`

SetFreshness sets Freshness field to given value.

### HasFreshness

`func (o *EmailScore) HasFreshness() bool`

HasFreshness returns a boolean if a field has been set.

### GetReasonCodes

`func (o *EmailScore) GetReasonCodes() []ReasonCode`

GetReasonCodes returns the ReasonCodes field if non-nil, zero value otherwise.

### GetReasonCodesOk

`func (o *EmailScore) GetReasonCodesOk() (*[]ReasonCode, bool)`

GetReasonCodesOk returns a tuple with the ReasonCodes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReasonCodes

`func (o *EmailScore) SetReasonCodes(v []ReasonCode)`

SetReasonCodes sets ReasonCodes field to given value.


### GetSafeToSend

`func (o *EmailScore) GetSafeToSend() bool`

GetSafeToSend returns the SafeToSend field if non-nil, zero value otherwise.

### GetSafeToSendOk

`func (o *EmailScore) GetSafeToSendOk() (*bool, bool)`

GetSafeToSendOk returns a tuple with the SafeToSend field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSafeToSend

`func (o *EmailScore) SetSafeToSend(v bool)`

SetSafeToSend sets SafeToSend field to given value.


### GetScore

`func (o *EmailScore) GetScore() int32`

GetScore returns the Score field if non-nil, zero value otherwise.

### GetScoreOk

`func (o *EmailScore) GetScoreOk() (*int32, bool)`

GetScoreOk returns a tuple with the Score field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScore

`func (o *EmailScore) SetScore(v int32)`

SetScore sets Score field to given value.


### GetSignals

`func (o *EmailScore) GetSignals() ScoringSignals`

GetSignals returns the Signals field if non-nil, zero value otherwise.

### GetSignalsOk

`func (o *EmailScore) GetSignalsOk() (*ScoringSignals, bool)`

GetSignalsOk returns a tuple with the Signals field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSignals

`func (o *EmailScore) SetSignals(v ScoringSignals)`

SetSignals sets Signals field to given value.


### GetSubReason

`func (o *EmailScore) GetSubReason() SubReason`

GetSubReason returns the SubReason field if non-nil, zero value otherwise.

### GetSubReasonOk

`func (o *EmailScore) GetSubReasonOk() (*SubReason, bool)`

GetSubReasonOk returns a tuple with the SubReason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSubReason

`func (o *EmailScore) SetSubReason(v SubReason)`

SetSubReason sets SubReason field to given value.


### GetVerifiedAt

`func (o *EmailScore) GetVerifiedAt() time.Time`

GetVerifiedAt returns the VerifiedAt field if non-nil, zero value otherwise.

### GetVerifiedAtOk

`func (o *EmailScore) GetVerifiedAtOk() (*time.Time, bool)`

GetVerifiedAtOk returns a tuple with the VerifiedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVerifiedAt

`func (o *EmailScore) SetVerifiedAt(v time.Time)`

SetVerifiedAt sets VerifiedAt field to given value.

### HasVerifiedAt

`func (o *EmailScore) HasVerifiedAt() bool`

HasVerifiedAt returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


