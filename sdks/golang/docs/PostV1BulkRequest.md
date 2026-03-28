# PostV1BulkRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Input** | **[]string** | A list of emails to verify. | [required]
**Webhook** | Pointer to [**TaskWebhook**](TaskWebhook.md) |  | [optional]

## Methods

### NewPostV1BulkRequest

`func NewPostV1BulkRequest(input []string) *PostV1BulkRequest`

NewPostV1BulkRequest instantiates a new PostV1BulkRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPostV1BulkRequestWithDefaults

`func NewPostV1BulkRequestWithDefaults() *PostV1BulkRequest`

NewPostV1BulkRequestWithDefaults instantiates a new PostV1BulkRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetInput

`func (o *PostV1BulkRequest) GetInput() []string`

GetInput returns the Input field if non-nil, zero value otherwise.

### GetInputOk

`func (o *PostV1BulkRequest) GetInputOk() ([]string, bool)`

GetInputOk returns a tuple with the Input field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInput

`func (o *PostV1BulkRequest) SetInput(v []string)`

SetInput sets Input field to given value.


### GetWebhook

`func (o *PostV1BulkRequest) GetWebhook() TaskWebhook`

GetWebhook returns the Webhook field if non-nil, zero value otherwise.

### GetWebhookOk

`func (o *PostV1BulkRequest) GetWebhookOk() (*TaskWebhook, bool)`

GetWebhookOk returns a tuple with the Webhook field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWebhook

`func (o *PostV1BulkRequest) SetWebhook(v TaskWebhook)`

SetWebhook sets Webhook field to given value.

### HasWebhook

`func (o *PostV1BulkRequest) HasWebhook() bool`

HasWebhook returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
