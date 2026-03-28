# CheckEmailOutputMx

Details obtained from querying the mail server\'s MX records.

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accepts_mail** | **boolean** | Indicates if the mail server accepts emails. | [optional]
**records** | **Array&lt;string&gt;** | List of Fully Qualified Domain Names (FQDN) of the mail server. | [optional]
**message** | **string** | A human-readable description of the error. | [optional]
**type** | **string** | The type of error. | [optional]

## Example

```typescript
import { CheckEmailOutputMx } from '@oppulence/reacher-sdk';

const instance: CheckEmailOutputMx = {
    accepts_mail: true,
    records: [],
    message: 'example',
    type: 'example',
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
