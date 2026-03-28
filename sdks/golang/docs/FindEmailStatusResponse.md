# FindEmailStatusResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BestMatch** | Pointer to [**NullableFinderBestMatch**](FinderBestMatch.md) |  | [optional]
**BulkJobId** | **int32** |  | [required]
**CandidatesChecked** | **int32** |  | [required]
**DomainHasMx** | **bool** |  | [required]
**DomainIsCatchAll** | **bool** |  | [required]
**JobId** | **int32** |  | [required]
**Results** | [**[]FinderCandidateResult**](FinderCandidateResult.md) |  | [required]
**Status** | **string** |  | [required]

## Methods

### NewFindEmailStatusResponse

`func NewFindEmailStatusResponse(bulkJobId int32, candidatesChecked int32, domainHasMx bool, domainIsCatchAll bool, jobId int32, results []FinderCandidateResult, status string) *FindEmailStatusResponse`

NewFindEmailStatusResponse instantiates a new FindEmailStatusResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewFindEmailStatusResponseWithDefaults

`func NewFindEmailStatusResponseWithDefaults() *FindEmailStatusResponse`

NewFindEmailStatusResponseWithDefaults instantiates a new FindEmailStatusResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBestMatch

`func (o *FindEmailStatusResponse) GetBestMatch() FinderBestMatch`

GetBestMatch returns the BestMatch field if non-nil, zero value otherwise.

### GetBestMatchOk

`func (o *FindEmailStatusResponse) GetBestMatchOk() (*FinderBestMatch, bool)`

GetBestMatchOk returns a tuple with the BestMatch field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBestMatch

`func (o *FindEmailStatusResponse) SetBestMatch(v FinderBestMatch)`

SetBestMatch sets BestMatch field to given value.

### HasBestMatch

`func (o *FindEmailStatusResponse) HasBestMatch() bool`

HasBestMatch returns a boolean if a field has been set.

### SetBestMatchNil

`func (o *FindEmailStatusResponse) SetBestMatchNil()`

 SetBestMatchNil sets the value for BestMatch to be an explicit nil

### UnsetBestMatch
`func (o *FindEmailStatusResponse) UnsetBestMatch()`

UnsetBestMatch ensures that no value is present for BestMatch, not even an explicit nil

### GetBulkJobId

`func (o *FindEmailStatusResponse) GetBulkJobId() int32`

GetBulkJobId returns the BulkJobId field if non-nil, zero value otherwise.

### GetBulkJobIdOk

`func (o *FindEmailStatusResponse) GetBulkJobIdOk() (*int32, bool)`

GetBulkJobIdOk returns a tuple with the BulkJobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBulkJobId

`func (o *FindEmailStatusResponse) SetBulkJobId(v int32)`

SetBulkJobId sets BulkJobId field to given value.


### GetCandidatesChecked

`func (o *FindEmailStatusResponse) GetCandidatesChecked() int32`

GetCandidatesChecked returns the CandidatesChecked field if non-nil, zero value otherwise.

### GetCandidatesCheckedOk

`func (o *FindEmailStatusResponse) GetCandidatesCheckedOk() (*int32, bool)`

GetCandidatesCheckedOk returns a tuple with the CandidatesChecked field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCandidatesChecked

`func (o *FindEmailStatusResponse) SetCandidatesChecked(v int32)`

SetCandidatesChecked sets CandidatesChecked field to given value.


### GetDomainHasMx

`func (o *FindEmailStatusResponse) GetDomainHasMx() bool`

GetDomainHasMx returns the DomainHasMx field if non-nil, zero value otherwise.

### GetDomainHasMxOk

`func (o *FindEmailStatusResponse) GetDomainHasMxOk() (*bool, bool)`

GetDomainHasMxOk returns a tuple with the DomainHasMx field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDomainHasMx

`func (o *FindEmailStatusResponse) SetDomainHasMx(v bool)`

SetDomainHasMx sets DomainHasMx field to given value.


### GetDomainIsCatchAll

`func (o *FindEmailStatusResponse) GetDomainIsCatchAll() bool`

GetDomainIsCatchAll returns the DomainIsCatchAll field if non-nil, zero value otherwise.

### GetDomainIsCatchAllOk

`func (o *FindEmailStatusResponse) GetDomainIsCatchAllOk() (*bool, bool)`

GetDomainIsCatchAllOk returns a tuple with the DomainIsCatchAll field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDomainIsCatchAll

`func (o *FindEmailStatusResponse) SetDomainIsCatchAll(v bool)`

SetDomainIsCatchAll sets DomainIsCatchAll field to given value.


### GetJobId

`func (o *FindEmailStatusResponse) GetJobId() int32`

GetJobId returns the JobId field if non-nil, zero value otherwise.

### GetJobIdOk

`func (o *FindEmailStatusResponse) GetJobIdOk() (*int32, bool)`

GetJobIdOk returns a tuple with the JobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobId

`func (o *FindEmailStatusResponse) SetJobId(v int32)`

SetJobId sets JobId field to given value.


### GetResults

`func (o *FindEmailStatusResponse) GetResults() []FinderCandidateResult`

GetResults returns the Results field if non-nil, zero value otherwise.

### GetResultsOk

`func (o *FindEmailStatusResponse) GetResultsOk() ([]FinderCandidateResult, bool)`

GetResultsOk returns a tuple with the Results field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResults

`func (o *FindEmailStatusResponse) SetResults(v []FinderCandidateResult)`

SetResults sets Results field to given value.


### GetStatus

`func (o *FindEmailStatusResponse) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *FindEmailStatusResponse) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *FindEmailStatusResponse) SetStatus(v string)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
