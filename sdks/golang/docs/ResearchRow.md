# ResearchRow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Candidates** | [**[]ResearchCandidate**](ResearchCandidate.md) |  | [required]
**CheckedPages** | **[]string** |  | [required]
**CompanyDomain** | **string** |  | [required]
**CompanyName** | **string** |  | [required]
**Reason** | **string** |  | [required]
**RowKey** | **string** |  | [required]
**Status** | **string** |  | [required]

## Methods

### NewResearchRow

`func NewResearchRow(candidates []ResearchCandidate, checkedPages []string, companyDomain string, companyName string, reason string, rowKey string, status string) *ResearchRow`

NewResearchRow instantiates a new ResearchRow object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewResearchRowWithDefaults

`func NewResearchRowWithDefaults() *ResearchRow`

NewResearchRowWithDefaults instantiates a new ResearchRow object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCandidates

`func (o *ResearchRow) GetCandidates() []ResearchCandidate`

GetCandidates returns the Candidates field if non-nil, zero value otherwise.

### GetCandidatesOk

`func (o *ResearchRow) GetCandidatesOk() ([]ResearchCandidate, bool)`

GetCandidatesOk returns a tuple with the Candidates field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCandidates

`func (o *ResearchRow) SetCandidates(v []ResearchCandidate)`

SetCandidates sets Candidates field to given value.


### GetCheckedPages

`func (o *ResearchRow) GetCheckedPages() []string`

GetCheckedPages returns the CheckedPages field if non-nil, zero value otherwise.

### GetCheckedPagesOk

`func (o *ResearchRow) GetCheckedPagesOk() ([]string, bool)`

GetCheckedPagesOk returns a tuple with the CheckedPages field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCheckedPages

`func (o *ResearchRow) SetCheckedPages(v []string)`

SetCheckedPages sets CheckedPages field to given value.


### GetCompanyDomain

`func (o *ResearchRow) GetCompanyDomain() string`

GetCompanyDomain returns the CompanyDomain field if non-nil, zero value otherwise.

### GetCompanyDomainOk

`func (o *ResearchRow) GetCompanyDomainOk() (*string, bool)`

GetCompanyDomainOk returns a tuple with the CompanyDomain field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCompanyDomain

`func (o *ResearchRow) SetCompanyDomain(v string)`

SetCompanyDomain sets CompanyDomain field to given value.


### GetCompanyName

`func (o *ResearchRow) GetCompanyName() string`

GetCompanyName returns the CompanyName field if non-nil, zero value otherwise.

### GetCompanyNameOk

`func (o *ResearchRow) GetCompanyNameOk() (*string, bool)`

GetCompanyNameOk returns a tuple with the CompanyName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCompanyName

`func (o *ResearchRow) SetCompanyName(v string)`

SetCompanyName sets CompanyName field to given value.


### GetReason

`func (o *ResearchRow) GetReason() string`

GetReason returns the Reason field if non-nil, zero value otherwise.

### GetReasonOk

`func (o *ResearchRow) GetReasonOk() (*string, bool)`

GetReasonOk returns a tuple with the Reason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReason

`func (o *ResearchRow) SetReason(v string)`

SetReason sets Reason field to given value.


### GetRowKey

`func (o *ResearchRow) GetRowKey() string`

GetRowKey returns the RowKey field if non-nil, zero value otherwise.

### GetRowKeyOk

`func (o *ResearchRow) GetRowKeyOk() (*string, bool)`

GetRowKeyOk returns a tuple with the RowKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRowKey

`func (o *ResearchRow) SetRowKey(v string)`

SetRowKey sets RowKey field to given value.


### GetStatus

`func (o *ResearchRow) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *ResearchRow) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *ResearchRow) SetStatus(v string)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
