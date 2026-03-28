# ApprovalChecklistResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Categories** | [**ApprovalCategoryBreakdown**](ApprovalCategoryBreakdown.md) |  | [required]
**JobId** | **int32** |  | [required]
**ReadyToSend** | **bool** |  | [required]
**Recommendation** | **string** |  | [required]
**RiskFlags** | [**ApprovalRiskFlags**](ApprovalRiskFlags.md) |  | [required]
**SafeToSendCount** | **int64** |  | [required]
**SafeToSendPct** | **float32** |  | [required]
**TotalRecords** | **int32** |  | [required]

## Methods

### NewApprovalChecklistResponse

`func NewApprovalChecklistResponse(categories ApprovalCategoryBreakdown, jobId int32, readyToSend bool, recommendation string, riskFlags ApprovalRiskFlags, safeToSendCount int64, safeToSendPct float32, totalRecords int32) *ApprovalChecklistResponse`

NewApprovalChecklistResponse instantiates a new ApprovalChecklistResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewApprovalChecklistResponseWithDefaults

`func NewApprovalChecklistResponseWithDefaults() *ApprovalChecklistResponse`

NewApprovalChecklistResponseWithDefaults instantiates a new ApprovalChecklistResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCategories

`func (o *ApprovalChecklistResponse) GetCategories() ApprovalCategoryBreakdown`

GetCategories returns the Categories field if non-nil, zero value otherwise.

### GetCategoriesOk

`func (o *ApprovalChecklistResponse) GetCategoriesOk() (*ApprovalCategoryBreakdown, bool)`

GetCategoriesOk returns a tuple with the Categories field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCategories

`func (o *ApprovalChecklistResponse) SetCategories(v ApprovalCategoryBreakdown)`

SetCategories sets Categories field to given value.


### GetJobId

`func (o *ApprovalChecklistResponse) GetJobId() int32`

GetJobId returns the JobId field if non-nil, zero value otherwise.

### GetJobIdOk

`func (o *ApprovalChecklistResponse) GetJobIdOk() (*int32, bool)`

GetJobIdOk returns a tuple with the JobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobId

`func (o *ApprovalChecklistResponse) SetJobId(v int32)`

SetJobId sets JobId field to given value.


### GetReadyToSend

`func (o *ApprovalChecklistResponse) GetReadyToSend() bool`

GetReadyToSend returns the ReadyToSend field if non-nil, zero value otherwise.

### GetReadyToSendOk

`func (o *ApprovalChecklistResponse) GetReadyToSendOk() (*bool, bool)`

GetReadyToSendOk returns a tuple with the ReadyToSend field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReadyToSend

`func (o *ApprovalChecklistResponse) SetReadyToSend(v bool)`

SetReadyToSend sets ReadyToSend field to given value.


### GetRecommendation

`func (o *ApprovalChecklistResponse) GetRecommendation() string`

GetRecommendation returns the Recommendation field if non-nil, zero value otherwise.

### GetRecommendationOk

`func (o *ApprovalChecklistResponse) GetRecommendationOk() (*string, bool)`

GetRecommendationOk returns a tuple with the Recommendation field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecommendation

`func (o *ApprovalChecklistResponse) SetRecommendation(v string)`

SetRecommendation sets Recommendation field to given value.


### GetRiskFlags

`func (o *ApprovalChecklistResponse) GetRiskFlags() ApprovalRiskFlags`

GetRiskFlags returns the RiskFlags field if non-nil, zero value otherwise.

### GetRiskFlagsOk

`func (o *ApprovalChecklistResponse) GetRiskFlagsOk() (*ApprovalRiskFlags, bool)`

GetRiskFlagsOk returns a tuple with the RiskFlags field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRiskFlags

`func (o *ApprovalChecklistResponse) SetRiskFlags(v ApprovalRiskFlags)`

SetRiskFlags sets RiskFlags field to given value.


### GetSafeToSendCount

`func (o *ApprovalChecklistResponse) GetSafeToSendCount() int64`

GetSafeToSendCount returns the SafeToSendCount field if non-nil, zero value otherwise.

### GetSafeToSendCountOk

`func (o *ApprovalChecklistResponse) GetSafeToSendCountOk() (*int64, bool)`

GetSafeToSendCountOk returns a tuple with the SafeToSendCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSafeToSendCount

`func (o *ApprovalChecklistResponse) SetSafeToSendCount(v int64)`

SetSafeToSendCount sets SafeToSendCount field to given value.


### GetSafeToSendPct

`func (o *ApprovalChecklistResponse) GetSafeToSendPct() float32`

GetSafeToSendPct returns the SafeToSendPct field if non-nil, zero value otherwise.

### GetSafeToSendPctOk

`func (o *ApprovalChecklistResponse) GetSafeToSendPctOk() (*float32, bool)`

GetSafeToSendPctOk returns a tuple with the SafeToSendPct field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSafeToSendPct

`func (o *ApprovalChecklistResponse) SetSafeToSendPct(v float32)`

SetSafeToSendPct sets SafeToSendPct field to given value.


### GetTotalRecords

`func (o *ApprovalChecklistResponse) GetTotalRecords() int32`

GetTotalRecords returns the TotalRecords field if non-nil, zero value otherwise.

### GetTotalRecordsOk

`func (o *ApprovalChecklistResponse) GetTotalRecordsOk() (*int32, bool)`

GetTotalRecordsOk returns a tuple with the TotalRecords field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalRecords

`func (o *ApprovalChecklistResponse) SetTotalRecords(v int32)`

SetTotalRecords sets TotalRecords field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
