# ListDetailResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DeduplicatedCount** | Pointer to **NullableInt32** |  | [optional] 
**EmailColumn** | **string** |  | 
**Id** | **int32** |  | 
**JobId** | **int32** |  | 
**Name** | **string** |  | 
**Status** | **string** |  | 
**Summary** | [**ListSummary**](ListSummary.md) |  | 
**TotalRows** | **int32** |  | 
**UniqueEmails** | Pointer to **NullableInt32** |  | [optional] 

## Methods

### NewListDetailResponse

`func NewListDetailResponse(emailColumn string, id int32, jobId int32, name string, status string, summary ListSummary, totalRows int32, ) *ListDetailResponse`

NewListDetailResponse instantiates a new ListDetailResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewListDetailResponseWithDefaults

`func NewListDetailResponseWithDefaults() *ListDetailResponse`

NewListDetailResponseWithDefaults instantiates a new ListDetailResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDeduplicatedCount

`func (o *ListDetailResponse) GetDeduplicatedCount() int32`

GetDeduplicatedCount returns the DeduplicatedCount field if non-nil, zero value otherwise.

### GetDeduplicatedCountOk

`func (o *ListDetailResponse) GetDeduplicatedCountOk() (*int32, bool)`

GetDeduplicatedCountOk returns a tuple with the DeduplicatedCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeduplicatedCount

`func (o *ListDetailResponse) SetDeduplicatedCount(v int32)`

SetDeduplicatedCount sets DeduplicatedCount field to given value.

### HasDeduplicatedCount

`func (o *ListDetailResponse) HasDeduplicatedCount() bool`

HasDeduplicatedCount returns a boolean if a field has been set.

### SetDeduplicatedCountNil

`func (o *ListDetailResponse) SetDeduplicatedCountNil(b bool)`

 SetDeduplicatedCountNil sets the value for DeduplicatedCount to be an explicit nil

### UnsetDeduplicatedCount
`func (o *ListDetailResponse) UnsetDeduplicatedCount()`

UnsetDeduplicatedCount ensures that no value is present for DeduplicatedCount, not even an explicit nil
### GetEmailColumn

`func (o *ListDetailResponse) GetEmailColumn() string`

GetEmailColumn returns the EmailColumn field if non-nil, zero value otherwise.

### GetEmailColumnOk

`func (o *ListDetailResponse) GetEmailColumnOk() (*string, bool)`

GetEmailColumnOk returns a tuple with the EmailColumn field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmailColumn

`func (o *ListDetailResponse) SetEmailColumn(v string)`

SetEmailColumn sets EmailColumn field to given value.


### GetId

`func (o *ListDetailResponse) GetId() int32`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ListDetailResponse) GetIdOk() (*int32, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ListDetailResponse) SetId(v int32)`

SetId sets Id field to given value.


### GetJobId

`func (o *ListDetailResponse) GetJobId() int32`

GetJobId returns the JobId field if non-nil, zero value otherwise.

### GetJobIdOk

`func (o *ListDetailResponse) GetJobIdOk() (*int32, bool)`

GetJobIdOk returns a tuple with the JobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobId

`func (o *ListDetailResponse) SetJobId(v int32)`

SetJobId sets JobId field to given value.


### GetName

`func (o *ListDetailResponse) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ListDetailResponse) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ListDetailResponse) SetName(v string)`

SetName sets Name field to given value.


### GetStatus

`func (o *ListDetailResponse) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *ListDetailResponse) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *ListDetailResponse) SetStatus(v string)`

SetStatus sets Status field to given value.


### GetSummary

`func (o *ListDetailResponse) GetSummary() ListSummary`

GetSummary returns the Summary field if non-nil, zero value otherwise.

### GetSummaryOk

`func (o *ListDetailResponse) GetSummaryOk() (*ListSummary, bool)`

GetSummaryOk returns a tuple with the Summary field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSummary

`func (o *ListDetailResponse) SetSummary(v ListSummary)`

SetSummary sets Summary field to given value.


### GetTotalRows

`func (o *ListDetailResponse) GetTotalRows() int32`

GetTotalRows returns the TotalRows field if non-nil, zero value otherwise.

### GetTotalRowsOk

`func (o *ListDetailResponse) GetTotalRowsOk() (*int32, bool)`

GetTotalRowsOk returns a tuple with the TotalRows field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalRows

`func (o *ListDetailResponse) SetTotalRows(v int32)`

SetTotalRows sets TotalRows field to given value.


### GetUniqueEmails

`func (o *ListDetailResponse) GetUniqueEmails() int32`

GetUniqueEmails returns the UniqueEmails field if non-nil, zero value otherwise.

### GetUniqueEmailsOk

`func (o *ListDetailResponse) GetUniqueEmailsOk() (*int32, bool)`

GetUniqueEmailsOk returns a tuple with the UniqueEmails field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUniqueEmails

`func (o *ListDetailResponse) SetUniqueEmails(v int32)`

SetUniqueEmails sets UniqueEmails field to given value.

### HasUniqueEmails

`func (o *ListDetailResponse) HasUniqueEmails() bool`

HasUniqueEmails returns a boolean if a field has been set.

### SetUniqueEmailsNil

`func (o *ListDetailResponse) SetUniqueEmailsNil(b bool)`

 SetUniqueEmailsNil sets the value for UniqueEmails to be an explicit nil

### UnsetUniqueEmails
`func (o *ListDetailResponse) UnsetUniqueEmails()`

UnsetUniqueEmails ensures that no value is present for UniqueEmails, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


