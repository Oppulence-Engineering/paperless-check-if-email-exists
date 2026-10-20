# CreateResearchRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IdempotencyKey** | **string** |  | [required]
**ImportContent** | **string** |  | [required]
**ImportFormat** | [**ResearchImportFormat**](ResearchImportFormat.md) |  | [required]
**MaxCandidates** | **int32** |  | [required]
**MaxVerifications** | **int32** |  | [required]
**Name** | **string** |  | [required]
**TargetRole** | **string** |  | [required]

## Methods

### NewCreateResearchRequest

`func NewCreateResearchRequest(idempotencyKey string, importContent string, importFormat ResearchImportFormat, maxCandidates int32, maxVerifications int32, name string, targetRole string) *CreateResearchRequest`

NewCreateResearchRequest instantiates a new CreateResearchRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateResearchRequestWithDefaults

`func NewCreateResearchRequestWithDefaults() *CreateResearchRequest`

NewCreateResearchRequestWithDefaults instantiates a new CreateResearchRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIdempotencyKey

`func (o *CreateResearchRequest) GetIdempotencyKey() string`

GetIdempotencyKey returns the IdempotencyKey field if non-nil, zero value otherwise.

### GetIdempotencyKeyOk

`func (o *CreateResearchRequest) GetIdempotencyKeyOk() (*string, bool)`

GetIdempotencyKeyOk returns a tuple with the IdempotencyKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIdempotencyKey

`func (o *CreateResearchRequest) SetIdempotencyKey(v string)`

SetIdempotencyKey sets IdempotencyKey field to given value.


### GetImportContent

`func (o *CreateResearchRequest) GetImportContent() string`

GetImportContent returns the ImportContent field if non-nil, zero value otherwise.

### GetImportContentOk

`func (o *CreateResearchRequest) GetImportContentOk() (*string, bool)`

GetImportContentOk returns a tuple with the ImportContent field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImportContent

`func (o *CreateResearchRequest) SetImportContent(v string)`

SetImportContent sets ImportContent field to given value.


### GetImportFormat

`func (o *CreateResearchRequest) GetImportFormat() ResearchImportFormat`

GetImportFormat returns the ImportFormat field if non-nil, zero value otherwise.

### GetImportFormatOk

`func (o *CreateResearchRequest) GetImportFormatOk() (*ResearchImportFormat, bool)`

GetImportFormatOk returns a tuple with the ImportFormat field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImportFormat

`func (o *CreateResearchRequest) SetImportFormat(v ResearchImportFormat)`

SetImportFormat sets ImportFormat field to given value.


### GetMaxCandidates

`func (o *CreateResearchRequest) GetMaxCandidates() int32`

GetMaxCandidates returns the MaxCandidates field if non-nil, zero value otherwise.

### GetMaxCandidatesOk

`func (o *CreateResearchRequest) GetMaxCandidatesOk() (*int32, bool)`

GetMaxCandidatesOk returns a tuple with the MaxCandidates field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxCandidates

`func (o *CreateResearchRequest) SetMaxCandidates(v int32)`

SetMaxCandidates sets MaxCandidates field to given value.


### GetMaxVerifications

`func (o *CreateResearchRequest) GetMaxVerifications() int32`

GetMaxVerifications returns the MaxVerifications field if non-nil, zero value otherwise.

### GetMaxVerificationsOk

`func (o *CreateResearchRequest) GetMaxVerificationsOk() (*int32, bool)`

GetMaxVerificationsOk returns a tuple with the MaxVerifications field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxVerifications

`func (o *CreateResearchRequest) SetMaxVerifications(v int32)`

SetMaxVerifications sets MaxVerifications field to given value.


### GetName

`func (o *CreateResearchRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateResearchRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateResearchRequest) SetName(v string)`

SetName sets Name field to given value.


### GetTargetRole

`func (o *CreateResearchRequest) GetTargetRole() string`

GetTargetRole returns the TargetRole field if non-nil, zero value otherwise.

### GetTargetRoleOk

`func (o *CreateResearchRequest) GetTargetRoleOk() (*string, bool)`

GetTargetRoleOk returns a tuple with the TargetRole field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTargetRole

`func (o *CreateResearchRequest) SetTargetRole(v string)`

SetTargetRole sets TargetRole field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
