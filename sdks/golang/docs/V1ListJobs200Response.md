# V1ListJobs200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Jobs** | [**[]Job**](Job.md) |  | [required]
**Total** | **int64** |  | [required]

## Methods

### NewV1ListJobs200Response

`func NewV1ListJobs200Response(jobs []Job, total int64) *V1ListJobs200Response`

NewV1ListJobs200Response instantiates a new V1ListJobs200Response object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewV1ListJobs200ResponseWithDefaults

`func NewV1ListJobs200ResponseWithDefaults() *V1ListJobs200Response`

NewV1ListJobs200ResponseWithDefaults instantiates a new V1ListJobs200Response object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetJobs

`func (o *V1ListJobs200Response) GetJobs() []Job`

GetJobs returns the Jobs field if non-nil, zero value otherwise.

### GetJobsOk

`func (o *V1ListJobs200Response) GetJobsOk() ([]Job, bool)`

GetJobsOk returns a tuple with the Jobs field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobs

`func (o *V1ListJobs200Response) SetJobs(v []Job)`

SetJobs sets Jobs field to given value.


### GetTotal

`func (o *V1ListJobs200Response) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *V1ListJobs200Response) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *V1ListJobs200Response) SetTotal(v int64)`

SetTotal sets Total field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
