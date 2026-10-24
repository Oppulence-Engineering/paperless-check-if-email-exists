# V1PreviewList200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Duplicates** | **int32** |  | [required]
**EmailColumn** | **string** |  | [required]
**EstimatedCredits** | **int32** |  | [required]
**Rejected** | [**[]RejectedRow**](RejectedRow.md) |  | [required]
**TotalRows** | **int32** |  | [required]
**UniqueEmails** | **int32** |  | [required]

## Methods

### NewV1PreviewList200Response

`func NewV1PreviewList200Response(duplicates int32, emailColumn string, estimatedCredits int32, rejected []RejectedRow, totalRows int32, uniqueEmails int32) *V1PreviewList200Response`

NewV1PreviewList200Response instantiates a new V1PreviewList200Response object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewV1PreviewList200ResponseWithDefaults

`func NewV1PreviewList200ResponseWithDefaults() *V1PreviewList200Response`

NewV1PreviewList200ResponseWithDefaults instantiates a new V1PreviewList200Response object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDuplicates

`func (o *V1PreviewList200Response) GetDuplicates() int32`

GetDuplicates returns the Duplicates field if non-nil, zero value otherwise.

### GetDuplicatesOk

`func (o *V1PreviewList200Response) GetDuplicatesOk() (*int32, bool)`

GetDuplicatesOk returns a tuple with the Duplicates field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDuplicates

`func (o *V1PreviewList200Response) SetDuplicates(v int32)`

SetDuplicates sets Duplicates field to given value.


### GetEmailColumn

`func (o *V1PreviewList200Response) GetEmailColumn() string`

GetEmailColumn returns the EmailColumn field if non-nil, zero value otherwise.

### GetEmailColumnOk

`func (o *V1PreviewList200Response) GetEmailColumnOk() (*string, bool)`

GetEmailColumnOk returns a tuple with the EmailColumn field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmailColumn

`func (o *V1PreviewList200Response) SetEmailColumn(v string)`

SetEmailColumn sets EmailColumn field to given value.


### GetEstimatedCredits

`func (o *V1PreviewList200Response) GetEstimatedCredits() int32`

GetEstimatedCredits returns the EstimatedCredits field if non-nil, zero value otherwise.

### GetEstimatedCreditsOk

`func (o *V1PreviewList200Response) GetEstimatedCreditsOk() (*int32, bool)`

GetEstimatedCreditsOk returns a tuple with the EstimatedCredits field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEstimatedCredits

`func (o *V1PreviewList200Response) SetEstimatedCredits(v int32)`

SetEstimatedCredits sets EstimatedCredits field to given value.


### GetRejected

`func (o *V1PreviewList200Response) GetRejected() []RejectedRow`

GetRejected returns the Rejected field if non-nil, zero value otherwise.

### GetRejectedOk

`func (o *V1PreviewList200Response) GetRejectedOk() ([]RejectedRow, bool)`

GetRejectedOk returns a tuple with the Rejected field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRejected

`func (o *V1PreviewList200Response) SetRejected(v []RejectedRow)`

SetRejected sets Rejected field to given value.


### GetTotalRows

`func (o *V1PreviewList200Response) GetTotalRows() int32`

GetTotalRows returns the TotalRows field if non-nil, zero value otherwise.

### GetTotalRowsOk

`func (o *V1PreviewList200Response) GetTotalRowsOk() (*int32, bool)`

GetTotalRowsOk returns a tuple with the TotalRows field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalRows

`func (o *V1PreviewList200Response) SetTotalRows(v int32)`

SetTotalRows sets TotalRows field to given value.


### GetUniqueEmails

`func (o *V1PreviewList200Response) GetUniqueEmails() int32`

GetUniqueEmails returns the UniqueEmails field if non-nil, zero value otherwise.

### GetUniqueEmailsOk

`func (o *V1PreviewList200Response) GetUniqueEmailsOk() (*int32, bool)`

GetUniqueEmailsOk returns a tuple with the UniqueEmails field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUniqueEmails

`func (o *V1PreviewList200Response) SetUniqueEmails(v int32)`

SetUniqueEmails sets UniqueEmails field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
