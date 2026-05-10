# IngestOutcomesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Outcomes** | [**[]IngestOutcome**](IngestOutcome.md) |  | [required]

## Methods

### NewIngestOutcomesRequest

`func NewIngestOutcomesRequest(outcomes []IngestOutcome) *IngestOutcomesRequest`

NewIngestOutcomesRequest instantiates a new IngestOutcomesRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewIngestOutcomesRequestWithDefaults

`func NewIngestOutcomesRequestWithDefaults() *IngestOutcomesRequest`

NewIngestOutcomesRequestWithDefaults instantiates a new IngestOutcomesRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetOutcomes

`func (o *IngestOutcomesRequest) GetOutcomes() []IngestOutcome`

GetOutcomes returns the Outcomes field if non-nil, zero value otherwise.

### GetOutcomesOk

`func (o *IngestOutcomesRequest) GetOutcomesOk() ([]IngestOutcome, bool)`

GetOutcomesOk returns a tuple with the Outcomes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutcomes

`func (o *IngestOutcomesRequest) SetOutcomes(v []IngestOutcome)`

SetOutcomes sets Outcomes field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
