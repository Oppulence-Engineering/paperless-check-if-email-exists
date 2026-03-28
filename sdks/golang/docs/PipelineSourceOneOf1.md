# PipelineSourceOneOf1

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AudienceId** | **string** |  | [required]
**ConnectionId** | **string** |  | [required]
**FieldMapping** | Pointer to **interface{}** |  | [optional]
**Provider** | **string** |  | [required]
**Type** | **string** |  | [required]

## Methods

### NewPipelineSourceOneOf1

`func NewPipelineSourceOneOf1(audienceId string, connectionId string, provider string, type_ string) *PipelineSourceOneOf1`

NewPipelineSourceOneOf1 instantiates a new PipelineSourceOneOf1 object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPipelineSourceOneOf1WithDefaults

`func NewPipelineSourceOneOf1WithDefaults() *PipelineSourceOneOf1`

NewPipelineSourceOneOf1WithDefaults instantiates a new PipelineSourceOneOf1 object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAudienceId

`func (o *PipelineSourceOneOf1) GetAudienceId() string`

GetAudienceId returns the AudienceId field if non-nil, zero value otherwise.

### GetAudienceIdOk

`func (o *PipelineSourceOneOf1) GetAudienceIdOk() (*string, bool)`

GetAudienceIdOk returns a tuple with the AudienceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAudienceId

`func (o *PipelineSourceOneOf1) SetAudienceId(v string)`

SetAudienceId sets AudienceId field to given value.


### GetConnectionId

`func (o *PipelineSourceOneOf1) GetConnectionId() string`

GetConnectionId returns the ConnectionId field if non-nil, zero value otherwise.

### GetConnectionIdOk

`func (o *PipelineSourceOneOf1) GetConnectionIdOk() (*string, bool)`

GetConnectionIdOk returns a tuple with the ConnectionId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConnectionId

`func (o *PipelineSourceOneOf1) SetConnectionId(v string)`

SetConnectionId sets ConnectionId field to given value.


### GetFieldMapping

`func (o *PipelineSourceOneOf1) GetFieldMapping() interface{}`

GetFieldMapping returns the FieldMapping field if non-nil, zero value otherwise.

### GetFieldMappingOk

`func (o *PipelineSourceOneOf1) GetFieldMappingOk() (*interface{}, bool)`

GetFieldMappingOk returns a tuple with the FieldMapping field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFieldMapping

`func (o *PipelineSourceOneOf1) SetFieldMapping(v interface{})`

SetFieldMapping sets FieldMapping field to given value.

### HasFieldMapping

`func (o *PipelineSourceOneOf1) HasFieldMapping() bool`

HasFieldMapping returns a boolean if a field has been set.

### SetFieldMappingNil

`func (o *PipelineSourceOneOf1) SetFieldMappingNil(b bool)`

 SetFieldMappingNil sets the value for FieldMapping to be an explicit nil

### UnsetFieldMapping
`func (o *PipelineSourceOneOf1) UnsetFieldMapping()`

UnsetFieldMapping ensures that no value is present for FieldMapping, not even an explicit nil
### GetProvider

`func (o *PipelineSourceOneOf1) GetProvider() string`

GetProvider returns the Provider field if non-nil, zero value otherwise.

### GetProviderOk

`func (o *PipelineSourceOneOf1) GetProviderOk() (*string, bool)`

GetProviderOk returns a tuple with the Provider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProvider

`func (o *PipelineSourceOneOf1) SetProvider(v string)`

SetProvider sets Provider field to given value.


### GetType

`func (o *PipelineSourceOneOf1) GetType() string`

GetType returns the Type field if non-nil, zero value otherwise.

### GetTypeOk

`func (o *PipelineSourceOneOf1) GetTypeOk() (*string, bool)`

GetTypeOk returns a tuple with the Type field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetType

`func (o *PipelineSourceOneOf1) SetType(v string)`

SetType sets Type field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


