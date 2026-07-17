# OutcomeIngestRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Outcomes** | [**[]OutcomeInput**](OutcomeInput.md) |  | [required]
**Provider** | **string** |  | [required]
**SourceKey** | Pointer to **NullableString** |  | [optional]

## Methods

### NewOutcomeIngestRequest

`func NewOutcomeIngestRequest(outcomes []OutcomeInput, provider string) *OutcomeIngestRequest`

NewOutcomeIngestRequest instantiates a new OutcomeIngestRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOutcomeIngestRequestWithDefaults

`func NewOutcomeIngestRequestWithDefaults() *OutcomeIngestRequest`

NewOutcomeIngestRequestWithDefaults instantiates a new OutcomeIngestRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetOutcomes

`func (o *OutcomeIngestRequest) GetOutcomes() []OutcomeInput`

GetOutcomes returns the Outcomes field if non-nil, zero value otherwise.

### GetOutcomesOk

`func (o *OutcomeIngestRequest) GetOutcomesOk() ([]OutcomeInput, bool)`

GetOutcomesOk returns a tuple with the Outcomes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutcomes

`func (o *OutcomeIngestRequest) SetOutcomes(v []OutcomeInput)`

SetOutcomes sets Outcomes field to given value.


### GetProvider

`func (o *OutcomeIngestRequest) GetProvider() string`

GetProvider returns the Provider field if non-nil, zero value otherwise.

### GetProviderOk

`func (o *OutcomeIngestRequest) GetProviderOk() (*string, bool)`

GetProviderOk returns a tuple with the Provider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProvider

`func (o *OutcomeIngestRequest) SetProvider(v string)`

SetProvider sets Provider field to given value.


### GetSourceKey

`func (o *OutcomeIngestRequest) GetSourceKey() string`

GetSourceKey returns the SourceKey field if non-nil, zero value otherwise.

### GetSourceKeyOk

`func (o *OutcomeIngestRequest) GetSourceKeyOk() (*string, bool)`

GetSourceKeyOk returns a tuple with the SourceKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceKey

`func (o *OutcomeIngestRequest) SetSourceKey(v string)`

SetSourceKey sets SourceKey field to given value.

### HasSourceKey

`func (o *OutcomeIngestRequest) HasSourceKey() bool`

HasSourceKey returns a boolean if a field has been set.

### SetSourceKeyNil

`func (o *OutcomeIngestRequest) SetSourceKeyNil()`

 SetSourceKeyNil sets the value for SourceKey to be an explicit nil

### UnsetSourceKey
`func (o *OutcomeIngestRequest) UnsetSourceKey()`

UnsetSourceKey ensures that no value is present for SourceKey, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
