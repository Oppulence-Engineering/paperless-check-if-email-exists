# IngestRowError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Email** | **string** |  | [required]
**Index** | **int32** |  | [required]
**Message** | **string** |  | [required]

## Methods

### NewIngestRowError

`func NewIngestRowError(email string, index int32, message string) *IngestRowError`

NewIngestRowError instantiates a new IngestRowError object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewIngestRowErrorWithDefaults

`func NewIngestRowErrorWithDefaults() *IngestRowError`

NewIngestRowErrorWithDefaults instantiates a new IngestRowError object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEmail

`func (o *IngestRowError) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *IngestRowError) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *IngestRowError) SetEmail(v string)`

SetEmail sets Email field to given value.


### GetIndex

`func (o *IngestRowError) GetIndex() int32`

GetIndex returns the Index field if non-nil, zero value otherwise.

### GetIndexOk

`func (o *IngestRowError) GetIndexOk() (*int32, bool)`

GetIndexOk returns a tuple with the Index field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIndex

`func (o *IngestRowError) SetIndex(v int32)`

SetIndex sets Index field to given value.


### GetMessage

`func (o *IngestRowError) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *IngestRowError) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *IngestRowError) SetMessage(v string)`

SetMessage sets Message field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
