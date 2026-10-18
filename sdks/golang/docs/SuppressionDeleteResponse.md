# SuppressionDeleteResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Deleted** | **bool** |  | [required]
**Status** | **string** |  | [required]

## Methods

### NewSuppressionDeleteResponse

`func NewSuppressionDeleteResponse(deleted bool, status string) *SuppressionDeleteResponse`

NewSuppressionDeleteResponse instantiates a new SuppressionDeleteResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSuppressionDeleteResponseWithDefaults

`func NewSuppressionDeleteResponseWithDefaults() *SuppressionDeleteResponse`

NewSuppressionDeleteResponseWithDefaults instantiates a new SuppressionDeleteResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDeleted

`func (o *SuppressionDeleteResponse) GetDeleted() bool`

GetDeleted returns the Deleted field if non-nil, zero value otherwise.

### GetDeletedOk

`func (o *SuppressionDeleteResponse) GetDeletedOk() (*bool, bool)`

GetDeletedOk returns a tuple with the Deleted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeleted

`func (o *SuppressionDeleteResponse) SetDeleted(v bool)`

SetDeleted sets Deleted field to given value.


### GetStatus

`func (o *SuppressionDeleteResponse) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *SuppressionDeleteResponse) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *SuppressionDeleteResponse) SetStatus(v string)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
