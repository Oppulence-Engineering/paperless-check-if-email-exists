# TaskWebhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**OnEachEmail** | Pointer to [**Webhook**](Webhook.md) |  | [optional]

## Methods

### NewTaskWebhook

`func NewTaskWebhook() *TaskWebhook`

NewTaskWebhook instantiates a new TaskWebhook object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewTaskWebhookWithDefaults

`func NewTaskWebhookWithDefaults() *TaskWebhook`

NewTaskWebhookWithDefaults instantiates a new TaskWebhook object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetOnEachEmail

`func (o *TaskWebhook) GetOnEachEmail() Webhook`

GetOnEachEmail returns the OnEachEmail field if non-nil, zero value otherwise.

### GetOnEachEmailOk

`func (o *TaskWebhook) GetOnEachEmailOk() (*Webhook, bool)`

GetOnEachEmailOk returns a tuple with the OnEachEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOnEachEmail

`func (o *TaskWebhook) SetOnEachEmail(v Webhook)`

SetOnEachEmail sets OnEachEmail field to given value.

### HasOnEachEmail

`func (o *TaskWebhook) HasOnEachEmail() bool`

HasOnEachEmail returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
