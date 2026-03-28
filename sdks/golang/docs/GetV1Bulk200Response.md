# GetV1Bulk200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**JobId** | **int32** |  | [required]
**CreatedAt** | **time.Time** | The date and time when the bulk verification job was created. | [required]
**FinishedAt** | Pointer to **time.Time** | If the bulk verification job is completed, the date and time when it was finished. | [optional]
**TotalRecords** | **int32** | The number of emails to verify in the bulk verification job. | [required]
**TotalProcessed** | **int32** | The number of emails that have been verified at the time of the query. | [required]
**Summary** | [**GetV1Bulk200ResponseSummary**](GetV1Bulk200ResponseSummary.md) |  | [required]
**JobStatus** | **string** | The status of the job, either \&quot;Running\&quot; or \&quot;Completed\&quot;. | [required]

## Methods

### NewGetV1Bulk200Response

`func NewGetV1Bulk200Response(jobId int32, createdAt time.Time, totalRecords int32, totalProcessed int32, summary GetV1Bulk200ResponseSummary, jobStatus string) *GetV1Bulk200Response`

NewGetV1Bulk200Response instantiates a new GetV1Bulk200Response object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewGetV1Bulk200ResponseWithDefaults

`func NewGetV1Bulk200ResponseWithDefaults() *GetV1Bulk200Response`

NewGetV1Bulk200ResponseWithDefaults instantiates a new GetV1Bulk200Response object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetJobId

`func (o *GetV1Bulk200Response) GetJobId() int32`

GetJobId returns the JobId field if non-nil, zero value otherwise.

### GetJobIdOk

`func (o *GetV1Bulk200Response) GetJobIdOk() (*int32, bool)`

GetJobIdOk returns a tuple with the JobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobId

`func (o *GetV1Bulk200Response) SetJobId(v int32)`

SetJobId sets JobId field to given value.


### GetCreatedAt

`func (o *GetV1Bulk200Response) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *GetV1Bulk200Response) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *GetV1Bulk200Response) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetFinishedAt

`func (o *GetV1Bulk200Response) GetFinishedAt() time.Time`

GetFinishedAt returns the FinishedAt field if non-nil, zero value otherwise.

### GetFinishedAtOk

`func (o *GetV1Bulk200Response) GetFinishedAtOk() (*time.Time, bool)`

GetFinishedAtOk returns a tuple with the FinishedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFinishedAt

`func (o *GetV1Bulk200Response) SetFinishedAt(v time.Time)`

SetFinishedAt sets FinishedAt field to given value.

### HasFinishedAt

`func (o *GetV1Bulk200Response) HasFinishedAt() bool`

HasFinishedAt returns a boolean if a field has been set.

### GetTotalRecords

`func (o *GetV1Bulk200Response) GetTotalRecords() int32`

GetTotalRecords returns the TotalRecords field if non-nil, zero value otherwise.

### GetTotalRecordsOk

`func (o *GetV1Bulk200Response) GetTotalRecordsOk() (*int32, bool)`

GetTotalRecordsOk returns a tuple with the TotalRecords field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalRecords

`func (o *GetV1Bulk200Response) SetTotalRecords(v int32)`

SetTotalRecords sets TotalRecords field to given value.


### GetTotalProcessed

`func (o *GetV1Bulk200Response) GetTotalProcessed() int32`

GetTotalProcessed returns the TotalProcessed field if non-nil, zero value otherwise.

### GetTotalProcessedOk

`func (o *GetV1Bulk200Response) GetTotalProcessedOk() (*int32, bool)`

GetTotalProcessedOk returns a tuple with the TotalProcessed field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalProcessed

`func (o *GetV1Bulk200Response) SetTotalProcessed(v int32)`

SetTotalProcessed sets TotalProcessed field to given value.


### GetSummary

`func (o *GetV1Bulk200Response) GetSummary() GetV1Bulk200ResponseSummary`

GetSummary returns the Summary field if non-nil, zero value otherwise.

### GetSummaryOk

`func (o *GetV1Bulk200Response) GetSummaryOk() (*GetV1Bulk200ResponseSummary, bool)`

GetSummaryOk returns a tuple with the Summary field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSummary

`func (o *GetV1Bulk200Response) SetSummary(v GetV1Bulk200ResponseSummary)`

SetSummary sets Summary field to given value.


### GetJobStatus

`func (o *GetV1Bulk200Response) GetJobStatus() string`

GetJobStatus returns the JobStatus field if non-nil, zero value otherwise.

### GetJobStatusOk

`func (o *GetV1Bulk200Response) GetJobStatusOk() (*string, bool)`

GetJobStatusOk returns a tuple with the JobStatus field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobStatus

`func (o *GetV1Bulk200Response) SetJobStatus(v string)`

SetJobStatus sets JobStatus field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
