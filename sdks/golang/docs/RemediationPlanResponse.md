# RemediationPlanResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CompletedAt** | Pointer to **NullableString** |  | [optional]
**CreatedAt** | **string** |  | [required]
**EffectiveJobId** | Pointer to **NullableInt32** |  | [optional]
**ListId** | **int32** |  | [required]
**Options** | [**RemediationOptions**](RemediationOptions.md) |  | [required]
**PlanId** | **int64** |  | [required]
**ResultStateDigest** | **string** |  | [required]
**RuleVersion** | **string** |  | [required]
**Status** | **string** |  | [required]
**SummaryCounts** | [**RemediationSummaryCounts**](RemediationSummaryCounts.md) |  | [required]

## Methods

### NewRemediationPlanResponse

`func NewRemediationPlanResponse(createdAt string, listId int32, options RemediationOptions, planId int64, resultStateDigest string, ruleVersion string, status string, summaryCounts RemediationSummaryCounts) *RemediationPlanResponse`

NewRemediationPlanResponse instantiates a new RemediationPlanResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewRemediationPlanResponseWithDefaults

`func NewRemediationPlanResponseWithDefaults() *RemediationPlanResponse`

NewRemediationPlanResponseWithDefaults instantiates a new RemediationPlanResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCompletedAt

`func (o *RemediationPlanResponse) GetCompletedAt() string`

GetCompletedAt returns the CompletedAt field if non-nil, zero value otherwise.

### GetCompletedAtOk

`func (o *RemediationPlanResponse) GetCompletedAtOk() (*string, bool)`

GetCompletedAtOk returns a tuple with the CompletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCompletedAt

`func (o *RemediationPlanResponse) SetCompletedAt(v string)`

SetCompletedAt sets CompletedAt field to given value.

### HasCompletedAt

`func (o *RemediationPlanResponse) HasCompletedAt() bool`

HasCompletedAt returns a boolean if a field has been set.

### SetCompletedAtNil

`func (o *RemediationPlanResponse) SetCompletedAtNil()`

 SetCompletedAtNil sets the value for CompletedAt to be an explicit nil

### UnsetCompletedAt
`func (o *RemediationPlanResponse) UnsetCompletedAt()`

UnsetCompletedAt ensures that no value is present for CompletedAt, not even an explicit nil

### GetCreatedAt

`func (o *RemediationPlanResponse) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *RemediationPlanResponse) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *RemediationPlanResponse) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.


### GetEffectiveJobId

`func (o *RemediationPlanResponse) GetEffectiveJobId() int32`

GetEffectiveJobId returns the EffectiveJobId field if non-nil, zero value otherwise.

### GetEffectiveJobIdOk

`func (o *RemediationPlanResponse) GetEffectiveJobIdOk() (*int32, bool)`

GetEffectiveJobIdOk returns a tuple with the EffectiveJobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEffectiveJobId

`func (o *RemediationPlanResponse) SetEffectiveJobId(v int32)`

SetEffectiveJobId sets EffectiveJobId field to given value.

### HasEffectiveJobId

`func (o *RemediationPlanResponse) HasEffectiveJobId() bool`

HasEffectiveJobId returns a boolean if a field has been set.

### SetEffectiveJobIdNil

`func (o *RemediationPlanResponse) SetEffectiveJobIdNil()`

 SetEffectiveJobIdNil sets the value for EffectiveJobId to be an explicit nil

### UnsetEffectiveJobId
`func (o *RemediationPlanResponse) UnsetEffectiveJobId()`

UnsetEffectiveJobId ensures that no value is present for EffectiveJobId, not even an explicit nil

### GetListId

`func (o *RemediationPlanResponse) GetListId() int32`

GetListId returns the ListId field if non-nil, zero value otherwise.

### GetListIdOk

`func (o *RemediationPlanResponse) GetListIdOk() (*int32, bool)`

GetListIdOk returns a tuple with the ListId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListId

`func (o *RemediationPlanResponse) SetListId(v int32)`

SetListId sets ListId field to given value.


### GetOptions

`func (o *RemediationPlanResponse) GetOptions() RemediationOptions`

GetOptions returns the Options field if non-nil, zero value otherwise.

### GetOptionsOk

`func (o *RemediationPlanResponse) GetOptionsOk() (*RemediationOptions, bool)`

GetOptionsOk returns a tuple with the Options field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOptions

`func (o *RemediationPlanResponse) SetOptions(v RemediationOptions)`

SetOptions sets Options field to given value.


### GetPlanId

`func (o *RemediationPlanResponse) GetPlanId() int64`

GetPlanId returns the PlanId field if non-nil, zero value otherwise.

### GetPlanIdOk

`func (o *RemediationPlanResponse) GetPlanIdOk() (*int64, bool)`

GetPlanIdOk returns a tuple with the PlanId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPlanId

`func (o *RemediationPlanResponse) SetPlanId(v int64)`

SetPlanId sets PlanId field to given value.


### GetResultStateDigest

`func (o *RemediationPlanResponse) GetResultStateDigest() string`

GetResultStateDigest returns the ResultStateDigest field if non-nil, zero value otherwise.

### GetResultStateDigestOk

`func (o *RemediationPlanResponse) GetResultStateDigestOk() (*string, bool)`

GetResultStateDigestOk returns a tuple with the ResultStateDigest field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResultStateDigest

`func (o *RemediationPlanResponse) SetResultStateDigest(v string)`

SetResultStateDigest sets ResultStateDigest field to given value.


### GetRuleVersion

`func (o *RemediationPlanResponse) GetRuleVersion() string`

GetRuleVersion returns the RuleVersion field if non-nil, zero value otherwise.

### GetRuleVersionOk

`func (o *RemediationPlanResponse) GetRuleVersionOk() (*string, bool)`

GetRuleVersionOk returns a tuple with the RuleVersion field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRuleVersion

`func (o *RemediationPlanResponse) SetRuleVersion(v string)`

SetRuleVersion sets RuleVersion field to given value.


### GetStatus

`func (o *RemediationPlanResponse) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *RemediationPlanResponse) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *RemediationPlanResponse) SetStatus(v string)`

SetStatus sets Status field to given value.


### GetSummaryCounts

`func (o *RemediationPlanResponse) GetSummaryCounts() RemediationSummaryCounts`

GetSummaryCounts returns the SummaryCounts field if non-nil, zero value otherwise.

### GetSummaryCountsOk

`func (o *RemediationPlanResponse) GetSummaryCountsOk() (*RemediationSummaryCounts, bool)`

GetSummaryCountsOk returns a tuple with the SummaryCounts field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSummaryCounts

`func (o *RemediationPlanResponse) SetSummaryCounts(v RemediationSummaryCounts)`

SetSummaryCounts sets SummaryCounts field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
