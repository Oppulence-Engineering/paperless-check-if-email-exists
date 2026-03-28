# PipelineDeliveryConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Dashboard** | Pointer to **bool** |  | [optional]
**MaxAttempts** | Pointer to **int32** |  | [optional]
**RetryBackoffSeconds** | Pointer to **int32** |  | [optional]
**Webhook** | Pointer to [**NullablePipelineDeliveryWebhook**](PipelineDeliveryWebhook.md) |  | [optional]

## Methods

### NewPipelineDeliveryConfig

`func NewPipelineDeliveryConfig() *PipelineDeliveryConfig`

NewPipelineDeliveryConfig instantiates a new PipelineDeliveryConfig object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPipelineDeliveryConfigWithDefaults

`func NewPipelineDeliveryConfigWithDefaults() *PipelineDeliveryConfig`

NewPipelineDeliveryConfigWithDefaults instantiates a new PipelineDeliveryConfig object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDashboard

`func (o *PipelineDeliveryConfig) GetDashboard() bool`

GetDashboard returns the Dashboard field if non-nil, zero value otherwise.

### GetDashboardOk

`func (o *PipelineDeliveryConfig) GetDashboardOk() (*bool, bool)`

GetDashboardOk returns a tuple with the Dashboard field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDashboard

`func (o *PipelineDeliveryConfig) SetDashboard(v bool)`

SetDashboard sets Dashboard field to given value.

### HasDashboard

`func (o *PipelineDeliveryConfig) HasDashboard() bool`

HasDashboard returns a boolean if a field has been set.

### GetMaxAttempts

`func (o *PipelineDeliveryConfig) GetMaxAttempts() int32`

GetMaxAttempts returns the MaxAttempts field if non-nil, zero value otherwise.

### GetMaxAttemptsOk

`func (o *PipelineDeliveryConfig) GetMaxAttemptsOk() (*int32, bool)`

GetMaxAttemptsOk returns a tuple with the MaxAttempts field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMaxAttempts

`func (o *PipelineDeliveryConfig) SetMaxAttempts(v int32)`

SetMaxAttempts sets MaxAttempts field to given value.

### HasMaxAttempts

`func (o *PipelineDeliveryConfig) HasMaxAttempts() bool`

HasMaxAttempts returns a boolean if a field has been set.

### GetRetryBackoffSeconds

`func (o *PipelineDeliveryConfig) GetRetryBackoffSeconds() int32`

GetRetryBackoffSeconds returns the RetryBackoffSeconds field if non-nil, zero value otherwise.

### GetRetryBackoffSecondsOk

`func (o *PipelineDeliveryConfig) GetRetryBackoffSecondsOk() (*int32, bool)`

GetRetryBackoffSecondsOk returns a tuple with the RetryBackoffSeconds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRetryBackoffSeconds

`func (o *PipelineDeliveryConfig) SetRetryBackoffSeconds(v int32)`

SetRetryBackoffSeconds sets RetryBackoffSeconds field to given value.

### HasRetryBackoffSeconds

`func (o *PipelineDeliveryConfig) HasRetryBackoffSeconds() bool`

HasRetryBackoffSeconds returns a boolean if a field has been set.

### GetWebhook

`func (o *PipelineDeliveryConfig) GetWebhook() PipelineDeliveryWebhook`

GetWebhook returns the Webhook field if non-nil, zero value otherwise.

### GetWebhookOk

`func (o *PipelineDeliveryConfig) GetWebhookOk() (*PipelineDeliveryWebhook, bool)`

GetWebhookOk returns a tuple with the Webhook field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWebhook

`func (o *PipelineDeliveryConfig) SetWebhook(v PipelineDeliveryWebhook)`

SetWebhook sets Webhook field to given value.

### HasWebhook

`func (o *PipelineDeliveryConfig) HasWebhook() bool`

HasWebhook returns a boolean if a field has been set.

### SetWebhookNil

`func (o *PipelineDeliveryConfig) SetWebhookNil()`

 SetWebhookNil sets the value for Webhook to be an explicit nil

### UnsetWebhook
`func (o *PipelineDeliveryConfig) UnsetWebhook()`

UnsetWebhook ensures that no value is present for Webhook, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
