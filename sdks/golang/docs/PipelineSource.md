# PipelineSource

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ListId** | **int32** |  | [optional]
**Type** | **string** |  | [optional]
**AudienceId** | **string** |  | [optional]
**ConnectionId** | **string** |  | [optional]
**FieldMapping** | Pointer to **interface{}** |  | [optional]
**Provider** | **string** |  | [optional]
**AcceptedFormat** | **string** |  | [optional]
**TokenId** | **string** |  | [optional]
**Bucket** | **string** |  | [optional]
**PathPattern** | Pointer to **NullableString** |  | [optional]
**Prefix** | Pointer to **NullableString** |  | [optional]
**Region** | Pointer to **NullableString** |  | [optional]

## Methods

### NewPipelineSource

`func NewPipelineSource(listId int32, type_ string, audienceId string, connectionId string, provider string, acceptedFormat string, tokenId string, bucket string) *PipelineSource`

NewPipelineSource instantiates a new PipelineSource object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPipelineSourceWithDefaults

`func NewPipelineSourceWithDefaults() *PipelineSource`

NewPipelineSourceWithDefaults instantiates a new PipelineSource object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetListId

`func (o *PipelineSource) GetListId() int32`

GetListId returns the ListId field if non-nil, zero value otherwise.

### GetListIdOk

`func (o *PipelineSource) GetListIdOk() (*int32, bool)`

GetListIdOk returns a tuple with the ListId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListId

`func (o *PipelineSource) SetListId(v int32)`

SetListId sets ListId field to given value.


### GetType

`func (o *PipelineSource) GetType() string`

GetType returns the Type field if non-nil, zero value otherwise.

### GetTypeOk

`func (o *PipelineSource) GetTypeOk() (*string, bool)`

GetTypeOk returns a tuple with the Type field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetType

`func (o *PipelineSource) SetType(v string)`

SetType sets Type field to given value.


### GetAudienceId

`func (o *PipelineSource) GetAudienceId() string`

GetAudienceId returns the AudienceId field if non-nil, zero value otherwise.

### GetAudienceIdOk

`func (o *PipelineSource) GetAudienceIdOk() (*string, bool)`

GetAudienceIdOk returns a tuple with the AudienceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAudienceId

`func (o *PipelineSource) SetAudienceId(v string)`

SetAudienceId sets AudienceId field to given value.


### GetConnectionId

`func (o *PipelineSource) GetConnectionId() string`

GetConnectionId returns the ConnectionId field if non-nil, zero value otherwise.

### GetConnectionIdOk

`func (o *PipelineSource) GetConnectionIdOk() (*string, bool)`

GetConnectionIdOk returns a tuple with the ConnectionId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConnectionId

`func (o *PipelineSource) SetConnectionId(v string)`

SetConnectionId sets ConnectionId field to given value.


### GetFieldMapping

`func (o *PipelineSource) GetFieldMapping() interface{}`

GetFieldMapping returns the FieldMapping field if non-nil, zero value otherwise.

### GetFieldMappingOk

`func (o *PipelineSource) GetFieldMappingOk() (*interface{}, bool)`

GetFieldMappingOk returns a tuple with the FieldMapping field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFieldMapping

`func (o *PipelineSource) SetFieldMapping(v interface{})`

SetFieldMapping sets FieldMapping field to given value.

### HasFieldMapping

`func (o *PipelineSource) HasFieldMapping() bool`

HasFieldMapping returns a boolean if a field has been set.

### SetFieldMappingNil

`func (o *PipelineSource) SetFieldMappingNil(b bool)`

 SetFieldMappingNil sets the value for FieldMapping to be an explicit nil

### UnsetFieldMapping
`func (o *PipelineSource) UnsetFieldMapping()`

UnsetFieldMapping ensures that no value is present for FieldMapping, not even an explicit nil
### GetProvider

`func (o *PipelineSource) GetProvider() string`

GetProvider returns the Provider field if non-nil, zero value otherwise.

### GetProviderOk

`func (o *PipelineSource) GetProviderOk() (*string, bool)`

GetProviderOk returns a tuple with the Provider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProvider

`func (o *PipelineSource) SetProvider(v string)`

SetProvider sets Provider field to given value.


### GetAcceptedFormat

`func (o *PipelineSource) GetAcceptedFormat() string`

GetAcceptedFormat returns the AcceptedFormat field if non-nil, zero value otherwise.

### GetAcceptedFormatOk

`func (o *PipelineSource) GetAcceptedFormatOk() (*string, bool)`

GetAcceptedFormatOk returns a tuple with the AcceptedFormat field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAcceptedFormat

`func (o *PipelineSource) SetAcceptedFormat(v string)`

SetAcceptedFormat sets AcceptedFormat field to given value.


### GetTokenId

`func (o *PipelineSource) GetTokenId() string`

GetTokenId returns the TokenId field if non-nil, zero value otherwise.

### GetTokenIdOk

`func (o *PipelineSource) GetTokenIdOk() (*string, bool)`

GetTokenIdOk returns a tuple with the TokenId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTokenId

`func (o *PipelineSource) SetTokenId(v string)`

SetTokenId sets TokenId field to given value.


### GetBucket

`func (o *PipelineSource) GetBucket() string`

GetBucket returns the Bucket field if non-nil, zero value otherwise.

### GetBucketOk

`func (o *PipelineSource) GetBucketOk() (*string, bool)`

GetBucketOk returns a tuple with the Bucket field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBucket

`func (o *PipelineSource) SetBucket(v string)`

SetBucket sets Bucket field to given value.


### GetPathPattern

`func (o *PipelineSource) GetPathPattern() string`

GetPathPattern returns the PathPattern field if non-nil, zero value otherwise.

### GetPathPatternOk

`func (o *PipelineSource) GetPathPatternOk() (*string, bool)`

GetPathPatternOk returns a tuple with the PathPattern field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPathPattern

`func (o *PipelineSource) SetPathPattern(v string)`

SetPathPattern sets PathPattern field to given value.

### HasPathPattern

`func (o *PipelineSource) HasPathPattern() bool`

HasPathPattern returns a boolean if a field has been set.

### SetPathPatternNil

`func (o *PipelineSource) SetPathPatternNil(b bool)`

 SetPathPatternNil sets the value for PathPattern to be an explicit nil

### UnsetPathPattern
`func (o *PipelineSource) UnsetPathPattern()`

UnsetPathPattern ensures that no value is present for PathPattern, not even an explicit nil
### GetPrefix

`func (o *PipelineSource) GetPrefix() string`

GetPrefix returns the Prefix field if non-nil, zero value otherwise.

### GetPrefixOk

`func (o *PipelineSource) GetPrefixOk() (*string, bool)`

GetPrefixOk returns a tuple with the Prefix field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPrefix

`func (o *PipelineSource) SetPrefix(v string)`

SetPrefix sets Prefix field to given value.

### HasPrefix

`func (o *PipelineSource) HasPrefix() bool`

HasPrefix returns a boolean if a field has been set.

### SetPrefixNil

`func (o *PipelineSource) SetPrefixNil(b bool)`

 SetPrefixNil sets the value for Prefix to be an explicit nil

### UnsetPrefix
`func (o *PipelineSource) UnsetPrefix()`

UnsetPrefix ensures that no value is present for Prefix, not even an explicit nil
### GetRegion

`func (o *PipelineSource) GetRegion() string`

GetRegion returns the Region field if non-nil, zero value otherwise.

### GetRegionOk

`func (o *PipelineSource) GetRegionOk() (*string, bool)`

GetRegionOk returns a tuple with the Region field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRegion

`func (o *PipelineSource) SetRegion(v string)`

SetRegion sets Region field to given value.

### HasRegion

`func (o *PipelineSource) HasRegion() bool`

HasRegion returns a boolean if a field has been set.

### SetRegionNil

`func (o *PipelineSource) SetRegionNil(b bool)`

 SetRegionNil sets the value for Region to be an explicit nil

### UnsetRegion
`func (o *PipelineSource) UnsetRegion()`

UnsetRegion ensures that no value is present for Region, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


