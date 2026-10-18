# OutcomeListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Limit** | **int64** |  | [required]
**Offset** | **int64** |  | [required]
**Outcomes** | [**[]OutcomeView**](OutcomeView.md) |  | [required]

## Methods

### NewOutcomeListResponse

`func NewOutcomeListResponse(limit int64, offset int64, outcomes []OutcomeView) *OutcomeListResponse`

NewOutcomeListResponse instantiates a new OutcomeListResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOutcomeListResponseWithDefaults

`func NewOutcomeListResponseWithDefaults() *OutcomeListResponse`

NewOutcomeListResponseWithDefaults instantiates a new OutcomeListResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetLimit

`func (o *OutcomeListResponse) GetLimit() int64`

GetLimit returns the Limit field if non-nil, zero value otherwise.

### GetLimitOk

`func (o *OutcomeListResponse) GetLimitOk() (*int64, bool)`

GetLimitOk returns a tuple with the Limit field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimit

`func (o *OutcomeListResponse) SetLimit(v int64)`

SetLimit sets Limit field to given value.


### GetOffset

`func (o *OutcomeListResponse) GetOffset() int64`

GetOffset returns the Offset field if non-nil, zero value otherwise.

### GetOffsetOk

`func (o *OutcomeListResponse) GetOffsetOk() (*int64, bool)`

GetOffsetOk returns a tuple with the Offset field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOffset

`func (o *OutcomeListResponse) SetOffset(v int64)`

SetOffset sets Offset field to given value.


### GetOutcomes

`func (o *OutcomeListResponse) GetOutcomes() []OutcomeView`

GetOutcomes returns the Outcomes field if non-nil, zero value otherwise.

### GetOutcomesOk

`func (o *OutcomeListResponse) GetOutcomesOk() ([]OutcomeView, bool)`

GetOutcomesOk returns a tuple with the Outcomes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutcomes

`func (o *OutcomeListResponse) SetOutcomes(v []OutcomeView)`

SetOutcomes sets Outcomes field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
