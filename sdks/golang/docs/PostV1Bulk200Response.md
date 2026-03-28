# PostV1Bulk200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**JobId** | **int32** | The unique ID generated for this bulk verification job. Use this &#x60;job_id&#x60; to query the progress or results of the bulk verification. | [optional]

## Methods

### NewPostV1Bulk200Response

`func NewPostV1Bulk200Response(jobId int32) *PostV1Bulk200Response`

NewPostV1Bulk200Response instantiates a new PostV1Bulk200Response object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPostV1Bulk200ResponseWithDefaults

`func NewPostV1Bulk200ResponseWithDefaults() *PostV1Bulk200Response`

NewPostV1Bulk200ResponseWithDefaults instantiates a new PostV1Bulk200Response object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetJobId

`func (o *PostV1Bulk200Response) GetJobId() int32`

GetJobId returns the JobId field if non-nil, zero value otherwise.

### GetJobIdOk

`func (o *PostV1Bulk200Response) GetJobIdOk() (*int32, bool)`

GetJobIdOk returns a tuple with the JobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobId

`func (o *PostV1Bulk200Response) SetJobId(v int32)`

SetJobId sets JobId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


