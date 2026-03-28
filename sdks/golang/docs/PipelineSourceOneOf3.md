# PipelineSourceOneOf3

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Bucket** | **string** |  | [required]
**PathPattern** | Pointer to **NullableString** |  | [optional]
**Prefix** | Pointer to **NullableString** |  | [optional]
**Provider** | **string** |  | [required]
**Region** | Pointer to **NullableString** |  | [optional]
**Type** | **string** |  | [required]

## Methods

### NewPipelineSourceOneOf3

`func NewPipelineSourceOneOf3(bucket string, provider string, type_ string) *PipelineSourceOneOf3`

NewPipelineSourceOneOf3 instantiates a new PipelineSourceOneOf3 object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPipelineSourceOneOf3WithDefaults

`func NewPipelineSourceOneOf3WithDefaults() *PipelineSourceOneOf3`

NewPipelineSourceOneOf3WithDefaults instantiates a new PipelineSourceOneOf3 object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBucket

`func (o *PipelineSourceOneOf3) GetBucket() string`

GetBucket returns the Bucket field if non-nil, zero value otherwise.

### GetBucketOk

`func (o *PipelineSourceOneOf3) GetBucketOk() (*string, bool)`

GetBucketOk returns a tuple with the Bucket field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBucket

`func (o *PipelineSourceOneOf3) SetBucket(v string)`

SetBucket sets Bucket field to given value.


### GetPathPattern

`func (o *PipelineSourceOneOf3) GetPathPattern() string`

GetPathPattern returns the PathPattern field if non-nil, zero value otherwise.

### GetPathPatternOk

`func (o *PipelineSourceOneOf3) GetPathPatternOk() (*string, bool)`

GetPathPatternOk returns a tuple with the PathPattern field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPathPattern

`func (o *PipelineSourceOneOf3) SetPathPattern(v string)`

SetPathPattern sets PathPattern field to given value.

### HasPathPattern

`func (o *PipelineSourceOneOf3) HasPathPattern() bool`

HasPathPattern returns a boolean if a field has been set.

### SetPathPatternNil

`func (o *PipelineSourceOneOf3) SetPathPatternNil(b bool)`

 SetPathPatternNil sets the value for PathPattern to be an explicit nil

### UnsetPathPattern
`func (o *PipelineSourceOneOf3) UnsetPathPattern()`

UnsetPathPattern ensures that no value is present for PathPattern, not even an explicit nil
### GetPrefix

`func (o *PipelineSourceOneOf3) GetPrefix() string`

GetPrefix returns the Prefix field if non-nil, zero value otherwise.

### GetPrefixOk

`func (o *PipelineSourceOneOf3) GetPrefixOk() (*string, bool)`

GetPrefixOk returns a tuple with the Prefix field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPrefix

`func (o *PipelineSourceOneOf3) SetPrefix(v string)`

SetPrefix sets Prefix field to given value.

### HasPrefix

`func (o *PipelineSourceOneOf3) HasPrefix() bool`

HasPrefix returns a boolean if a field has been set.

### SetPrefixNil

`func (o *PipelineSourceOneOf3) SetPrefixNil(b bool)`

 SetPrefixNil sets the value for Prefix to be an explicit nil

### UnsetPrefix
`func (o *PipelineSourceOneOf3) UnsetPrefix()`

UnsetPrefix ensures that no value is present for Prefix, not even an explicit nil
### GetProvider

`func (o *PipelineSourceOneOf3) GetProvider() string`

GetProvider returns the Provider field if non-nil, zero value otherwise.

### GetProviderOk

`func (o *PipelineSourceOneOf3) GetProviderOk() (*string, bool)`

GetProviderOk returns a tuple with the Provider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProvider

`func (o *PipelineSourceOneOf3) SetProvider(v string)`

SetProvider sets Provider field to given value.


### GetRegion

`func (o *PipelineSourceOneOf3) GetRegion() string`

GetRegion returns the Region field if non-nil, zero value otherwise.

### GetRegionOk

`func (o *PipelineSourceOneOf3) GetRegionOk() (*string, bool)`

GetRegionOk returns a tuple with the Region field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRegion

`func (o *PipelineSourceOneOf3) SetRegion(v string)`

SetRegion sets Region field to given value.

### HasRegion

`func (o *PipelineSourceOneOf3) HasRegion() bool`

HasRegion returns a boolean if a field has been set.

### SetRegionNil

`func (o *PipelineSourceOneOf3) SetRegionNil(b bool)`

 SetRegionNil sets the value for Region to be an explicit nil

### UnsetRegion
`func (o *PipelineSourceOneOf3) UnsetRegion()`

UnsetRegion ensures that no value is present for Region, not even an explicit nil
### GetType

`func (o *PipelineSourceOneOf3) GetType() string`

GetType returns the Type field if non-nil, zero value otherwise.

### GetTypeOk

`func (o *PipelineSourceOneOf3) GetTypeOk() (*string, bool)`

GetTypeOk returns a tuple with the Type field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetType

`func (o *PipelineSourceOneOf3) SetType(v string)`

SetType sets Type field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


