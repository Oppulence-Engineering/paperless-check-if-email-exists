# CreateCommentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Author** | Pointer to **NullableString** |  | [optional]
**Body** | **string** |  | [required]
**JobId** | Pointer to **NullableInt32** |  | [optional]
**ListId** | Pointer to **NullableInt32** |  | [optional]

## Methods

### NewCreateCommentRequest

`func NewCreateCommentRequest(body string) *CreateCommentRequest`

NewCreateCommentRequest instantiates a new CreateCommentRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateCommentRequestWithDefaults

`func NewCreateCommentRequestWithDefaults() *CreateCommentRequest`

NewCreateCommentRequestWithDefaults instantiates a new CreateCommentRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAuthor

`func (o *CreateCommentRequest) GetAuthor() string`

GetAuthor returns the Author field if non-nil, zero value otherwise.

### GetAuthorOk

`func (o *CreateCommentRequest) GetAuthorOk() (*string, bool)`

GetAuthorOk returns a tuple with the Author field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAuthor

`func (o *CreateCommentRequest) SetAuthor(v string)`

SetAuthor sets Author field to given value.

### HasAuthor

`func (o *CreateCommentRequest) HasAuthor() bool`

HasAuthor returns a boolean if a field has been set.

### SetAuthorNil

`func (o *CreateCommentRequest) SetAuthorNil()`

 SetAuthorNil sets the value for Author to be an explicit nil

### UnsetAuthor
`func (o *CreateCommentRequest) UnsetAuthor()`

UnsetAuthor ensures that no value is present for Author, not even an explicit nil

### GetBody

`func (o *CreateCommentRequest) GetBody() string`

GetBody returns the Body field if non-nil, zero value otherwise.

### GetBodyOk

`func (o *CreateCommentRequest) GetBodyOk() (*string, bool)`

GetBodyOk returns a tuple with the Body field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBody

`func (o *CreateCommentRequest) SetBody(v string)`

SetBody sets Body field to given value.


### GetJobId

`func (o *CreateCommentRequest) GetJobId() int32`

GetJobId returns the JobId field if non-nil, zero value otherwise.

### GetJobIdOk

`func (o *CreateCommentRequest) GetJobIdOk() (*int32, bool)`

GetJobIdOk returns a tuple with the JobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobId

`func (o *CreateCommentRequest) SetJobId(v int32)`

SetJobId sets JobId field to given value.

### HasJobId

`func (o *CreateCommentRequest) HasJobId() bool`

HasJobId returns a boolean if a field has been set.

### SetJobIdNil

`func (o *CreateCommentRequest) SetJobIdNil()`

 SetJobIdNil sets the value for JobId to be an explicit nil

### UnsetJobId
`func (o *CreateCommentRequest) UnsetJobId()`

UnsetJobId ensures that no value is present for JobId, not even an explicit nil

### GetListId

`func (o *CreateCommentRequest) GetListId() int32`

GetListId returns the ListId field if non-nil, zero value otherwise.

### GetListIdOk

`func (o *CreateCommentRequest) GetListIdOk() (*int32, bool)`

GetListIdOk returns a tuple with the ListId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListId

`func (o *CreateCommentRequest) SetListId(v int32)`

SetListId sets ListId field to given value.

### HasListId

`func (o *CreateCommentRequest) HasListId() bool`

HasListId returns a boolean if a field has been set.

### SetListIdNil

`func (o *CreateCommentRequest) SetListIdNil()`

 SetListIdNil sets the value for ListId to be an explicit nil

### UnsetListId
`func (o *CreateCommentRequest) UnsetListId()`

UnsetListId ensures that no value is present for ListId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
