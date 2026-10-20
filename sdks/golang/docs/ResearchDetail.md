# ResearchDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**MaxCandidates** | **int32** |  | [required]
**MaxVerifications** | **int32** |  | [required]
**Rows** | [**[]ResearchRow**](ResearchRow.md) |  | [required]
**Session** | [**ResearchSummary**](ResearchSummary.md) |  | [required]
**Status** | **string** |  | [required]
**TargetRole** | **string** |  | [required]
**VerificationRequests** | **int32** |  | [required]

## Methods

### NewResearchDetail

`func NewResearchDetail(maxCandidates int32, maxVerifications int32, rows []ResearchRow, session ResearchSummary, status string, targetRole string, verificationRequests int32) *ResearchDetail`

NewResearchDetail instantiates a new ResearchDetail object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewResearchDetailWithDefaults

`func NewResearchDetailWithDefaults() *ResearchDetail`

NewResearchDetailWithDefaults instantiates a new ResearchDetail object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetMaxCandidates

`func (o *ResearchDetail) GetMaxCandidates() int32`

GetMaxCandidates returns the MaxCandidates field if non-nil, zero value otherwise.

### GetMaxCandidatesOk

`func (o *ResearchDetail) GetMaxCandidatesOk() (*int32, bool)`

GetMaxCandidatesOk returns a tuple with the MaxCandidates field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxCandidates

`func (o *ResearchDetail) SetMaxCandidates(v int32)`

SetMaxCandidates sets MaxCandidates field to given value.


### GetMaxVerifications

`func (o *ResearchDetail) GetMaxVerifications() int32`

GetMaxVerifications returns the MaxVerifications field if non-nil, zero value otherwise.

### GetMaxVerificationsOk

`func (o *ResearchDetail) GetMaxVerificationsOk() (*int32, bool)`

GetMaxVerificationsOk returns a tuple with the MaxVerifications field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxVerifications

`func (o *ResearchDetail) SetMaxVerifications(v int32)`

SetMaxVerifications sets MaxVerifications field to given value.


### GetRows

`func (o *ResearchDetail) GetRows() []ResearchRow`

GetRows returns the Rows field if non-nil, zero value otherwise.

### GetRowsOk

`func (o *ResearchDetail) GetRowsOk() ([]ResearchRow, bool)`

GetRowsOk returns a tuple with the Rows field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRows

`func (o *ResearchDetail) SetRows(v []ResearchRow)`

SetRows sets Rows field to given value.


### GetSession

`func (o *ResearchDetail) GetSession() ResearchSummary`

GetSession returns the Session field if non-nil, zero value otherwise.

### GetSessionOk

`func (o *ResearchDetail) GetSessionOk() (*ResearchSummary, bool)`

GetSessionOk returns a tuple with the Session field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSession

`func (o *ResearchDetail) SetSession(v ResearchSummary)`

SetSession sets Session field to given value.


### GetStatus

`func (o *ResearchDetail) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *ResearchDetail) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *ResearchDetail) SetStatus(v string)`

SetStatus sets Status field to given value.


### GetTargetRole

`func (o *ResearchDetail) GetTargetRole() string`

GetTargetRole returns the TargetRole field if non-nil, zero value otherwise.

### GetTargetRoleOk

`func (o *ResearchDetail) GetTargetRoleOk() (*string, bool)`

GetTargetRoleOk returns a tuple with the TargetRole field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTargetRole

`func (o *ResearchDetail) SetTargetRole(v string)`

SetTargetRole sets TargetRole field to given value.


### GetVerificationRequests

`func (o *ResearchDetail) GetVerificationRequests() int32`

GetVerificationRequests returns the VerificationRequests field if non-nil, zero value otherwise.

### GetVerificationRequestsOk

`func (o *ResearchDetail) GetVerificationRequestsOk() (*int32, bool)`

GetVerificationRequestsOk returns a tuple with the VerificationRequests field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVerificationRequests

`func (o *ResearchDetail) SetVerificationRequests(v int32)`

SetVerificationRequests sets VerificationRequests field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
