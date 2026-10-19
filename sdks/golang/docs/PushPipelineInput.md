# PushPipelineInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**EmailColumn** | Pointer to **string** |  | [optional]
**Rows** | **[]map[string]interface{}** |  | [required]
**SourceKey** | Pointer to **NullableString** |  | [optional]

## Methods

### NewPushPipelineInput

`func NewPushPipelineInput(rows []map[string]interface{}) *PushPipelineInput`

NewPushPipelineInput instantiates a new PushPipelineInput object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPushPipelineInputWithDefaults

`func NewPushPipelineInputWithDefaults() *PushPipelineInput`

NewPushPipelineInputWithDefaults instantiates a new PushPipelineInput object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEmailColumn

`func (o *PushPipelineInput) GetEmailColumn() string`

GetEmailColumn returns the EmailColumn field if non-nil, zero value otherwise.

### GetEmailColumnOk

`func (o *PushPipelineInput) GetEmailColumnOk() (*string, bool)`

GetEmailColumnOk returns a tuple with the EmailColumn field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmailColumn

`func (o *PushPipelineInput) SetEmailColumn(v string)`

SetEmailColumn sets EmailColumn field to given value.

### HasEmailColumn

`func (o *PushPipelineInput) HasEmailColumn() bool`

HasEmailColumn returns a boolean if a field has been set.

### GetRows

`func (o *PushPipelineInput) GetRows() []map[string]interface{}`

GetRows returns the Rows field if non-nil, zero value otherwise.

### GetRowsOk

`func (o *PushPipelineInput) GetRowsOk() ([]map[string]interface{}, bool)`

GetRowsOk returns a tuple with the Rows field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRows

`func (o *PushPipelineInput) SetRows(v []map[string]interface{})`

SetRows sets Rows field to given value.


### GetSourceKey

`func (o *PushPipelineInput) GetSourceKey() string`

GetSourceKey returns the SourceKey field if non-nil, zero value otherwise.

### GetSourceKeyOk

`func (o *PushPipelineInput) GetSourceKeyOk() (*string, bool)`

GetSourceKeyOk returns a tuple with the SourceKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceKey

`func (o *PushPipelineInput) SetSourceKey(v string)`

SetSourceKey sets SourceKey field to given value.

### HasSourceKey

`func (o *PushPipelineInput) HasSourceKey() bool`

HasSourceKey returns a boolean if a field has been set.

### SetSourceKeyNil

`func (o *PushPipelineInput) SetSourceKeyNil()`

 SetSourceKeyNil sets the value for SourceKey to be an explicit nil

### UnsetSourceKey
`func (o *PushPipelineInput) UnsetSourceKey()`

UnsetSourceKey ensures that no value is present for SourceKey, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
