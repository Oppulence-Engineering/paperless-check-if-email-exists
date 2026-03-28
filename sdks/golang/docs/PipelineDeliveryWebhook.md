# PipelineDeliveryWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Headers** | Pointer to **map[string]string** |  | [optional]
**Url** | **string** |  | [required]

## Methods

### NewPipelineDeliveryWebhook

`func NewPipelineDeliveryWebhook(url string) *PipelineDeliveryWebhook`

NewPipelineDeliveryWebhook instantiates a new PipelineDeliveryWebhook object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPipelineDeliveryWebhookWithDefaults

`func NewPipelineDeliveryWebhookWithDefaults() *PipelineDeliveryWebhook`

NewPipelineDeliveryWebhookWithDefaults instantiates a new PipelineDeliveryWebhook object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetHeaders

`func (o *PipelineDeliveryWebhook) GetHeaders() map[string]string`

GetHeaders returns the Headers field if non-nil, zero value otherwise.

### GetHeadersOk

`func (o *PipelineDeliveryWebhook) GetHeadersOk() (*map[string]string, bool)`

GetHeadersOk returns a tuple with the Headers field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHeaders

`func (o *PipelineDeliveryWebhook) SetHeaders(v map[string]string)`

SetHeaders sets Headers field to given value.

### HasHeaders

`func (o *PipelineDeliveryWebhook) HasHeaders() bool`

HasHeaders returns a boolean if a field has been set.

### GetUrl

`func (o *PipelineDeliveryWebhook) GetUrl() string`

GetUrl returns the Url field if non-nil, zero value otherwise.

### GetUrlOk

`func (o *PipelineDeliveryWebhook) GetUrlOk() (*string, bool)`

GetUrlOk returns a tuple with the Url field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUrl

`func (o *PipelineDeliveryWebhook) SetUrl(v string)`

SetUrl sets Url field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


