# SavedSegmentListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Segments** | [**[]SavedSegmentView**](SavedSegmentView.md) |  | [required]
**Total** | **int64** |  | [required]

## Methods

### NewSavedSegmentListResponse

`func NewSavedSegmentListResponse(segments []SavedSegmentView, total int64) *SavedSegmentListResponse`

NewSavedSegmentListResponse instantiates a new SavedSegmentListResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSavedSegmentListResponseWithDefaults

`func NewSavedSegmentListResponseWithDefaults() *SavedSegmentListResponse`

NewSavedSegmentListResponseWithDefaults instantiates a new SavedSegmentListResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetSegments

`func (o *SavedSegmentListResponse) GetSegments() []SavedSegmentView`

GetSegments returns the Segments field if non-nil, zero value otherwise.

### GetSegmentsOk

`func (o *SavedSegmentListResponse) GetSegmentsOk() ([]SavedSegmentView, bool)`

GetSegmentsOk returns a tuple with the Segments field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSegments

`func (o *SavedSegmentListResponse) SetSegments(v []SavedSegmentView)`

SetSegments sets Segments field to given value.


### GetTotal

`func (o *SavedSegmentListResponse) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *SavedSegmentListResponse) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *SavedSegmentListResponse) SetTotal(v int64)`

SetTotal sets Total field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
