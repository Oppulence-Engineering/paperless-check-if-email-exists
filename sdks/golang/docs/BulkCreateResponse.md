# BulkCreateResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**JobId** | **int32** |  | [required]
**SourceKey** | Pointer to **NullableString** |  | [optional]

## Methods

### NewBulkCreateResponse

`func NewBulkCreateResponse(jobId int32) *BulkCreateResponse`

NewBulkCreateResponse instantiates a new BulkCreateResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewBulkCreateResponseWithDefaults

`func NewBulkCreateResponseWithDefaults() *BulkCreateResponse`

NewBulkCreateResponseWithDefaults instantiates a new BulkCreateResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetJobId

`func (o *BulkCreateResponse) GetJobId() int32`

GetJobId returns the JobId field if non-nil, zero value otherwise.

### GetJobIdOk

`func (o *BulkCreateResponse) GetJobIdOk() (*int32, bool)`

GetJobIdOk returns a tuple with the JobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobId

`func (o *BulkCreateResponse) SetJobId(v int32)`

SetJobId sets JobId field to given value.


### GetSourceKey

`func (o *BulkCreateResponse) GetSourceKey() string`

GetSourceKey returns the SourceKey field if non-nil, zero value otherwise.

### GetSourceKeyOk

`func (o *BulkCreateResponse) GetSourceKeyOk() (*string, bool)`

GetSourceKeyOk returns a tuple with the SourceKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceKey

`func (o *BulkCreateResponse) SetSourceKey(v string)`

SetSourceKey sets SourceKey field to given value.

### HasSourceKey

`func (o *BulkCreateResponse) HasSourceKey() bool`

HasSourceKey returns a boolean if a field has been set.

### SetSourceKeyNil

`func (o *BulkCreateResponse) SetSourceKeyNil()`

 SetSourceKeyNil sets the value for SourceKey to be an explicit nil

### UnsetSourceKey
`func (o *BulkCreateResponse) UnsetSourceKey()`

UnsetSourceKey ensures that no value is present for SourceKey, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
