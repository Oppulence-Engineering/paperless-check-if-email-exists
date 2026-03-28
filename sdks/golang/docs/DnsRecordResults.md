# DnsRecordResults

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DmarcPolicy** | Pointer to **NullableString** |  | [optional]
**HasDkim** | **bool** |  | [required]
**HasDmarc** | **bool** |  | [required]
**HasMx** | **bool** |  | [required]
**HasSpf** | **bool** |  | [required]
**MxRecords** | **[]string** |  | [required]
**SpfValid** | **bool** |  | [required]

## Methods

### NewDnsRecordResults

`func NewDnsRecordResults(hasDkim bool, hasDmarc bool, hasMx bool, hasSpf bool, mxRecords []string, spfValid bool) *DnsRecordResults`

NewDnsRecordResults instantiates a new DnsRecordResults object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDnsRecordResultsWithDefaults

`func NewDnsRecordResultsWithDefaults() *DnsRecordResults`

NewDnsRecordResultsWithDefaults instantiates a new DnsRecordResults object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDmarcPolicy

`func (o *DnsRecordResults) GetDmarcPolicy() string`

GetDmarcPolicy returns the DmarcPolicy field if non-nil, zero value otherwise.

### GetDmarcPolicyOk

`func (o *DnsRecordResults) GetDmarcPolicyOk() (*string, bool)`

GetDmarcPolicyOk returns a tuple with the DmarcPolicy field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDmarcPolicy

`func (o *DnsRecordResults) SetDmarcPolicy(v string)`

SetDmarcPolicy sets DmarcPolicy field to given value.

### HasDmarcPolicy

`func (o *DnsRecordResults) HasDmarcPolicy() bool`

HasDmarcPolicy returns a boolean if a field has been set.

### SetDmarcPolicyNil

`func (o *DnsRecordResults) SetDmarcPolicyNil(b bool)`

 SetDmarcPolicyNil sets the value for DmarcPolicy to be an explicit nil

### UnsetDmarcPolicy
`func (o *DnsRecordResults) UnsetDmarcPolicy()`

UnsetDmarcPolicy ensures that no value is present for DmarcPolicy, not even an explicit nil
### GetHasDkim

`func (o *DnsRecordResults) GetHasDkim() bool`

GetHasDkim returns the HasDkim field if non-nil, zero value otherwise.

### GetHasDkimOk

`func (o *DnsRecordResults) GetHasDkimOk() (*bool, bool)`

GetHasDkimOk returns a tuple with the HasDkim field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasDkim

`func (o *DnsRecordResults) SetHasDkim(v bool)`

SetHasDkim sets HasDkim field to given value.


### GetHasDmarc

`func (o *DnsRecordResults) GetHasDmarc() bool`

GetHasDmarc returns the HasDmarc field if non-nil, zero value otherwise.

### GetHasDmarcOk

`func (o *DnsRecordResults) GetHasDmarcOk() (*bool, bool)`

GetHasDmarcOk returns a tuple with the HasDmarc field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasDmarc

`func (o *DnsRecordResults) SetHasDmarc(v bool)`

SetHasDmarc sets HasDmarc field to given value.


### GetHasMx

`func (o *DnsRecordResults) GetHasMx() bool`

GetHasMx returns the HasMx field if non-nil, zero value otherwise.

### GetHasMxOk

`func (o *DnsRecordResults) GetHasMxOk() (*bool, bool)`

GetHasMxOk returns a tuple with the HasMx field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasMx

`func (o *DnsRecordResults) SetHasMx(v bool)`

SetHasMx sets HasMx field to given value.


### GetHasSpf

`func (o *DnsRecordResults) GetHasSpf() bool`

GetHasSpf returns the HasSpf field if non-nil, zero value otherwise.

### GetHasSpfOk

`func (o *DnsRecordResults) GetHasSpfOk() (*bool, bool)`

GetHasSpfOk returns a tuple with the HasSpf field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasSpf

`func (o *DnsRecordResults) SetHasSpf(v bool)`

SetHasSpf sets HasSpf field to given value.


### GetMxRecords

`func (o *DnsRecordResults) GetMxRecords() []string`

GetMxRecords returns the MxRecords field if non-nil, zero value otherwise.

### GetMxRecordsOk

`func (o *DnsRecordResults) GetMxRecordsOk() ([]string, bool)`

GetMxRecordsOk returns a tuple with the MxRecords field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMxRecords

`func (o *DnsRecordResults) SetMxRecords(v []string)`

SetMxRecords sets MxRecords field to given value.


### GetSpfValid

`func (o *DnsRecordResults) GetSpfValid() bool`

GetSpfValid returns the SpfValid field if non-nil, zero value otherwise.

### GetSpfValidOk

`func (o *DnsRecordResults) GetSpfValidOk() (*bool, bool)`

GetSpfValidOk returns a tuple with the SpfValid field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSpfValid

`func (o *DnsRecordResults) SetSpfValid(v bool)`

SetSpfValid sets SpfValid field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


