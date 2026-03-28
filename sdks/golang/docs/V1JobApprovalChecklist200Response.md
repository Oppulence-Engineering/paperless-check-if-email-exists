# V1JobApprovalChecklist200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Categories** | Pointer to **map[string]interface{}** |  | [optional]
**JobId** | Pointer to **int32** |  | [optional]
**ReadyToSend** | Pointer to **bool** |  | [optional]
**Recommendation** | Pointer to **string** |  | [optional]
**RiskFlags** | Pointer to **map[string]interface{}** |  | [optional]
**SafeToSendCount** | Pointer to **int64** |  | [optional]
**SafeToSendPct** | Pointer to **float32** |  | [optional]
**TotalRecords** | Pointer to **int32** |  | [optional]

## Methods

### NewV1JobApprovalChecklist200Response

`func NewV1JobApprovalChecklist200Response() *V1JobApprovalChecklist200Response`

NewV1JobApprovalChecklist200Response instantiates a new V1JobApprovalChecklist200Response object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewV1JobApprovalChecklist200ResponseWithDefaults

`func NewV1JobApprovalChecklist200ResponseWithDefaults() *V1JobApprovalChecklist200Response`

NewV1JobApprovalChecklist200ResponseWithDefaults instantiates a new V1JobApprovalChecklist200Response object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCategories

`func (o *V1JobApprovalChecklist200Response) GetCategories() map[string]interface{}`

GetCategories returns the Categories field if non-nil, zero value otherwise.

### GetCategoriesOk

`func (o *V1JobApprovalChecklist200Response) GetCategoriesOk() (*map[string]interface{}, bool)`

GetCategoriesOk returns a tuple with the Categories field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCategories

`func (o *V1JobApprovalChecklist200Response) SetCategories(v map[string]interface{})`

SetCategories sets Categories field to given value.

### HasCategories

`func (o *V1JobApprovalChecklist200Response) HasCategories() bool`

HasCategories returns a boolean if a field has been set.

### GetJobId

`func (o *V1JobApprovalChecklist200Response) GetJobId() int32`

GetJobId returns the JobId field if non-nil, zero value otherwise.

### GetJobIdOk

`func (o *V1JobApprovalChecklist200Response) GetJobIdOk() (*int32, bool)`

GetJobIdOk returns a tuple with the JobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobId

`func (o *V1JobApprovalChecklist200Response) SetJobId(v int32)`

SetJobId sets JobId field to given value.

### HasJobId

`func (o *V1JobApprovalChecklist200Response) HasJobId() bool`

HasJobId returns a boolean if a field has been set.

### GetReadyToSend

`func (o *V1JobApprovalChecklist200Response) GetReadyToSend() bool`

GetReadyToSend returns the ReadyToSend field if non-nil, zero value otherwise.

### GetReadyToSendOk

`func (o *V1JobApprovalChecklist200Response) GetReadyToSendOk() (*bool, bool)`

GetReadyToSendOk returns a tuple with the ReadyToSend field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReadyToSend

`func (o *V1JobApprovalChecklist200Response) SetReadyToSend(v bool)`

SetReadyToSend sets ReadyToSend field to given value.

### HasReadyToSend

`func (o *V1JobApprovalChecklist200Response) HasReadyToSend() bool`

HasReadyToSend returns a boolean if a field has been set.

### GetRecommendation

`func (o *V1JobApprovalChecklist200Response) GetRecommendation() string`

GetRecommendation returns the Recommendation field if non-nil, zero value otherwise.

### GetRecommendationOk

`func (o *V1JobApprovalChecklist200Response) GetRecommendationOk() (*string, bool)`

GetRecommendationOk returns a tuple with the Recommendation field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecommendation

`func (o *V1JobApprovalChecklist200Response) SetRecommendation(v string)`

SetRecommendation sets Recommendation field to given value.

### HasRecommendation

`func (o *V1JobApprovalChecklist200Response) HasRecommendation() bool`

HasRecommendation returns a boolean if a field has been set.

### GetRiskFlags

`func (o *V1JobApprovalChecklist200Response) GetRiskFlags() map[string]interface{}`

GetRiskFlags returns the RiskFlags field if non-nil, zero value otherwise.

### GetRiskFlagsOk

`func (o *V1JobApprovalChecklist200Response) GetRiskFlagsOk() (*map[string]interface{}, bool)`

GetRiskFlagsOk returns a tuple with the RiskFlags field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRiskFlags

`func (o *V1JobApprovalChecklist200Response) SetRiskFlags(v map[string]interface{})`

SetRiskFlags sets RiskFlags field to given value.

### HasRiskFlags

`func (o *V1JobApprovalChecklist200Response) HasRiskFlags() bool`

HasRiskFlags returns a boolean if a field has been set.

### GetSafeToSendCount

`func (o *V1JobApprovalChecklist200Response) GetSafeToSendCount() int64`

GetSafeToSendCount returns the SafeToSendCount field if non-nil, zero value otherwise.

### GetSafeToSendCountOk

`func (o *V1JobApprovalChecklist200Response) GetSafeToSendCountOk() (*int64, bool)`

GetSafeToSendCountOk returns a tuple with the SafeToSendCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSafeToSendCount

`func (o *V1JobApprovalChecklist200Response) SetSafeToSendCount(v int64)`

SetSafeToSendCount sets SafeToSendCount field to given value.

### HasSafeToSendCount

`func (o *V1JobApprovalChecklist200Response) HasSafeToSendCount() bool`

HasSafeToSendCount returns a boolean if a field has been set.

### GetSafeToSendPct

`func (o *V1JobApprovalChecklist200Response) GetSafeToSendPct() float32`

GetSafeToSendPct returns the SafeToSendPct field if non-nil, zero value otherwise.

### GetSafeToSendPctOk

`func (o *V1JobApprovalChecklist200Response) GetSafeToSendPctOk() (*float32, bool)`

GetSafeToSendPctOk returns a tuple with the SafeToSendPct field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSafeToSendPct

`func (o *V1JobApprovalChecklist200Response) SetSafeToSendPct(v float32)`

SetSafeToSendPct sets SafeToSendPct field to given value.

### HasSafeToSendPct

`func (o *V1JobApprovalChecklist200Response) HasSafeToSendPct() bool`

HasSafeToSendPct returns a boolean if a field has been set.

### GetTotalRecords

`func (o *V1JobApprovalChecklist200Response) GetTotalRecords() int32`

GetTotalRecords returns the TotalRecords field if non-nil, zero value otherwise.

### GetTotalRecordsOk

`func (o *V1JobApprovalChecklist200Response) GetTotalRecordsOk() (*int32, bool)`

GetTotalRecordsOk returns a tuple with the TotalRecords field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalRecords

`func (o *V1JobApprovalChecklist200Response) SetTotalRecords(v int32)`

SetTotalRecords sets TotalRecords field to given value.

### HasTotalRecords

`func (o *V1JobApprovalChecklist200Response) HasTotalRecords() bool`

HasTotalRecords returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


