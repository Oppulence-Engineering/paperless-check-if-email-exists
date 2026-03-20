# SuppressionListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Entries** | [**[]SuppressionEntry**](SuppressionEntry.md) |  | 
**Total** | **int64** |  | 

## Methods

### NewSuppressionListResponse

`func NewSuppressionListResponse(entries []SuppressionEntry, total int64, ) *SuppressionListResponse`

NewSuppressionListResponse instantiates a new SuppressionListResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSuppressionListResponseWithDefaults

`func NewSuppressionListResponseWithDefaults() *SuppressionListResponse`

NewSuppressionListResponseWithDefaults instantiates a new SuppressionListResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEntries

`func (o *SuppressionListResponse) GetEntries() []SuppressionEntry`

GetEntries returns the Entries field if non-nil, zero value otherwise.

### GetEntriesOk

`func (o *SuppressionListResponse) GetEntriesOk() (*[]SuppressionEntry, bool)`

GetEntriesOk returns a tuple with the Entries field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEntries

`func (o *SuppressionListResponse) SetEntries(v []SuppressionEntry)`

SetEntries sets Entries field to given value.


### GetTotal

`func (o *SuppressionListResponse) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *SuppressionListResponse) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *SuppressionListResponse) SetTotal(v int64)`

SetTotal sets Total field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


