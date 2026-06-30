# BulkCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Input** | **[]string** |  | [required]
**Source** | Pointer to **NullableString** | Alias for source_key. | [optional]
**SourceKey** | Pointer to **NullableString** | Optional source key used for source quality analytics. | [optional]
**Webhook** | Pointer to **map[string]interface{}** |  | [optional]

## Methods

### NewBulkCreateRequest

`func NewBulkCreateRequest(input []string) *BulkCreateRequest`

NewBulkCreateRequest instantiates a new BulkCreateRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewBulkCreateRequestWithDefaults

`func NewBulkCreateRequestWithDefaults() *BulkCreateRequest`

NewBulkCreateRequestWithDefaults instantiates a new BulkCreateRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetInput

`func (o *BulkCreateRequest) GetInput() []string`

GetInput returns the Input field if non-nil, zero value otherwise.

### GetInputOk

`func (o *BulkCreateRequest) GetInputOk() ([]string, bool)`

GetInputOk returns a tuple with the Input field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInput

`func (o *BulkCreateRequest) SetInput(v []string)`

SetInput sets Input field to given value.


### GetSource

`func (o *BulkCreateRequest) GetSource() string`

GetSource returns the Source field if non-nil, zero value otherwise.

### GetSourceOk

`func (o *BulkCreateRequest) GetSourceOk() (*string, bool)`

GetSourceOk returns a tuple with the Source field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSource

`func (o *BulkCreateRequest) SetSource(v string)`

SetSource sets Source field to given value.

### HasSource

`func (o *BulkCreateRequest) HasSource() bool`

HasSource returns a boolean if a field has been set.

### SetSourceNil

`func (o *BulkCreateRequest) SetSourceNil()`

 SetSourceNil sets the value for Source to be an explicit nil

### UnsetSource
`func (o *BulkCreateRequest) UnsetSource()`

UnsetSource ensures that no value is present for Source, not even an explicit nil

### GetSourceKey

`func (o *BulkCreateRequest) GetSourceKey() string`

GetSourceKey returns the SourceKey field if non-nil, zero value otherwise.

### GetSourceKeyOk

`func (o *BulkCreateRequest) GetSourceKeyOk() (*string, bool)`

GetSourceKeyOk returns a tuple with the SourceKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceKey

`func (o *BulkCreateRequest) SetSourceKey(v string)`

SetSourceKey sets SourceKey field to given value.

### HasSourceKey

`func (o *BulkCreateRequest) HasSourceKey() bool`

HasSourceKey returns a boolean if a field has been set.

### SetSourceKeyNil

`func (o *BulkCreateRequest) SetSourceKeyNil()`

 SetSourceKeyNil sets the value for SourceKey to be an explicit nil

### UnsetSourceKey
`func (o *BulkCreateRequest) UnsetSourceKey()`

UnsetSourceKey ensures that no value is present for SourceKey, not even an explicit nil

### GetWebhook

`func (o *BulkCreateRequest) GetWebhook() map[string]interface{}`

GetWebhook returns the Webhook field if non-nil, zero value otherwise.

### GetWebhookOk

`func (o *BulkCreateRequest) GetWebhookOk() (map[string]interface{}, bool)`

GetWebhookOk returns a tuple with the Webhook field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWebhook

`func (o *BulkCreateRequest) SetWebhook(v map[string]interface{})`

SetWebhook sets Webhook field to given value.

### HasWebhook

`func (o *BulkCreateRequest) HasWebhook() bool`

HasWebhook returns a boolean if a field has been set.

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
