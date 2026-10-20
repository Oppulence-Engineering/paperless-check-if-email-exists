# ResearchCandidate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Email** | **string** |  | [required]
**EmailSource** | **string** |  | [required]
**EmploymentStatus** | **string** |  | [required]
**EvidenceUrl** | Pointer to **NullableString** |  | [optional]
**Mailbox** | Pointer to [**NullableResearchMailbox**](ResearchMailbox.md) |  | [optional]
**Name** | **string** |  | [required]
**ObservedAt** | Pointer to **NullableString** |  | [optional]
**PersonStatus** | **string** |  | [required]
**RoleStatus** | **string** |  | [required]
**Title** | **string** |  | [required]
**VerificationError** | Pointer to **NullableString** |  | [optional]

## Methods

### NewResearchCandidate

`func NewResearchCandidate(email string, emailSource string, employmentStatus string, name string, personStatus string, roleStatus string, title string) *ResearchCandidate`

NewResearchCandidate instantiates a new ResearchCandidate object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewResearchCandidateWithDefaults

`func NewResearchCandidateWithDefaults() *ResearchCandidate`

NewResearchCandidateWithDefaults instantiates a new ResearchCandidate object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEmail

`func (o *ResearchCandidate) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *ResearchCandidate) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *ResearchCandidate) SetEmail(v string)`

SetEmail sets Email field to given value.


### GetEmailSource

`func (o *ResearchCandidate) GetEmailSource() string`

GetEmailSource returns the EmailSource field if non-nil, zero value otherwise.

### GetEmailSourceOk

`func (o *ResearchCandidate) GetEmailSourceOk() (*string, bool)`

GetEmailSourceOk returns a tuple with the EmailSource field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmailSource

`func (o *ResearchCandidate) SetEmailSource(v string)`

SetEmailSource sets EmailSource field to given value.


### GetEmploymentStatus

`func (o *ResearchCandidate) GetEmploymentStatus() string`

GetEmploymentStatus returns the EmploymentStatus field if non-nil, zero value otherwise.

### GetEmploymentStatusOk

`func (o *ResearchCandidate) GetEmploymentStatusOk() (*string, bool)`

GetEmploymentStatusOk returns a tuple with the EmploymentStatus field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmploymentStatus

`func (o *ResearchCandidate) SetEmploymentStatus(v string)`

SetEmploymentStatus sets EmploymentStatus field to given value.


### GetEvidenceUrl

`func (o *ResearchCandidate) GetEvidenceUrl() string`

GetEvidenceUrl returns the EvidenceUrl field if non-nil, zero value otherwise.

### GetEvidenceUrlOk

`func (o *ResearchCandidate) GetEvidenceUrlOk() (*string, bool)`

GetEvidenceUrlOk returns a tuple with the EvidenceUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEvidenceUrl

`func (o *ResearchCandidate) SetEvidenceUrl(v string)`

SetEvidenceUrl sets EvidenceUrl field to given value.

### HasEvidenceUrl

`func (o *ResearchCandidate) HasEvidenceUrl() bool`

HasEvidenceUrl returns a boolean if a field has been set.

### SetEvidenceUrlNil

`func (o *ResearchCandidate) SetEvidenceUrlNil()`

 SetEvidenceUrlNil sets the value for EvidenceUrl to be an explicit nil

### UnsetEvidenceUrl
`func (o *ResearchCandidate) UnsetEvidenceUrl()`

UnsetEvidenceUrl ensures that no value is present for EvidenceUrl, not even an explicit nil

### GetMailbox

`func (o *ResearchCandidate) GetMailbox() ResearchMailbox`

GetMailbox returns the Mailbox field if non-nil, zero value otherwise.

### GetMailboxOk

`func (o *ResearchCandidate) GetMailboxOk() (*ResearchMailbox, bool)`

GetMailboxOk returns a tuple with the Mailbox field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMailbox

`func (o *ResearchCandidate) SetMailbox(v ResearchMailbox)`

SetMailbox sets Mailbox field to given value.

### HasMailbox

`func (o *ResearchCandidate) HasMailbox() bool`

HasMailbox returns a boolean if a field has been set.

### SetMailboxNil

`func (o *ResearchCandidate) SetMailboxNil()`

 SetMailboxNil sets the value for Mailbox to be an explicit nil

### UnsetMailbox
`func (o *ResearchCandidate) UnsetMailbox()`

UnsetMailbox ensures that no value is present for Mailbox, not even an explicit nil

### GetName

`func (o *ResearchCandidate) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ResearchCandidate) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ResearchCandidate) SetName(v string)`

SetName sets Name field to given value.


### GetObservedAt

`func (o *ResearchCandidate) GetObservedAt() string`

GetObservedAt returns the ObservedAt field if non-nil, zero value otherwise.

### GetObservedAtOk

`func (o *ResearchCandidate) GetObservedAtOk() (*string, bool)`

GetObservedAtOk returns a tuple with the ObservedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetObservedAt

`func (o *ResearchCandidate) SetObservedAt(v string)`

SetObservedAt sets ObservedAt field to given value.

### HasObservedAt

`func (o *ResearchCandidate) HasObservedAt() bool`

HasObservedAt returns a boolean if a field has been set.

### SetObservedAtNil

`func (o *ResearchCandidate) SetObservedAtNil()`

 SetObservedAtNil sets the value for ObservedAt to be an explicit nil

### UnsetObservedAt
`func (o *ResearchCandidate) UnsetObservedAt()`

UnsetObservedAt ensures that no value is present for ObservedAt, not even an explicit nil

### GetPersonStatus

`func (o *ResearchCandidate) GetPersonStatus() string`

GetPersonStatus returns the PersonStatus field if non-nil, zero value otherwise.

### GetPersonStatusOk

`func (o *ResearchCandidate) GetPersonStatusOk() (*string, bool)`

GetPersonStatusOk returns a tuple with the PersonStatus field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPersonStatus

`func (o *ResearchCandidate) SetPersonStatus(v string)`

SetPersonStatus sets PersonStatus field to given value.


### GetRoleStatus

`func (o *ResearchCandidate) GetRoleStatus() string`

GetRoleStatus returns the RoleStatus field if non-nil, zero value otherwise.

### GetRoleStatusOk

`func (o *ResearchCandidate) GetRoleStatusOk() (*string, bool)`

GetRoleStatusOk returns a tuple with the RoleStatus field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRoleStatus

`func (o *ResearchCandidate) SetRoleStatus(v string)`

SetRoleStatus sets RoleStatus field to given value.


### GetTitle

`func (o *ResearchCandidate) GetTitle() string`

GetTitle returns the Title field if non-nil, zero value otherwise.

### GetTitleOk

`func (o *ResearchCandidate) GetTitleOk() (*string, bool)`

GetTitleOk returns a tuple with the Title field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTitle

`func (o *ResearchCandidate) SetTitle(v string)`

SetTitle sets Title field to given value.


### GetVerificationError

`func (o *ResearchCandidate) GetVerificationError() string`

GetVerificationError returns the VerificationError field if non-nil, zero value otherwise.

### GetVerificationErrorOk

`func (o *ResearchCandidate) GetVerificationErrorOk() (*string, bool)`

GetVerificationErrorOk returns a tuple with the VerificationError field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVerificationError

`func (o *ResearchCandidate) SetVerificationError(v string)`

SetVerificationError sets VerificationError field to given value.

### HasVerificationError

`func (o *ResearchCandidate) HasVerificationError() bool`

HasVerificationError returns a boolean if a field has been set.

### SetVerificationErrorNil

`func (o *ResearchCandidate) SetVerificationErrorNil()`

 SetVerificationErrorNil sets the value for VerificationError to be an explicit nil

### UnsetVerificationError
`func (o *ResearchCandidate) UnsetVerificationError()`

UnsetVerificationError ensures that no value is present for VerificationError, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
