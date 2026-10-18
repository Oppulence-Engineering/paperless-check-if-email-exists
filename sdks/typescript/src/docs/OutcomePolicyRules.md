# OutcomePolicyRules


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**click** | [**OutcomeRuleAction**](OutcomeRuleAction.md) |  | [optional]
**complaint** | [**OutcomeRuleAction**](OutcomeRuleAction.md) |  | [optional]
**delivered** | [**OutcomeRuleAction**](OutcomeRuleAction.md) |  | [optional]
**hard_bounce** | [**OutcomeRuleAction**](OutcomeRuleAction.md) |  | [optional]
**open** | [**OutcomeRuleAction**](OutcomeRuleAction.md) |  | [optional]
**outcome_ttl_days** | **number** |  | [optional]
**soft_bounce** | [**OutcomeRuleAction**](OutcomeRuleAction.md) |  | [optional]
**unsubscribe** | [**OutcomeRuleAction**](OutcomeRuleAction.md) |  | [optional]

## Example

```typescript
import { OutcomePolicyRules } from '@oppulence/reacher-sdk';

const instance: OutcomePolicyRules = {
    click: {} as any,
    complaint: {} as any,
    delivered: {} as any,
    hard_bounce: {} as any,
    open: {} as any,
    outcome_ttl_days: 0,
    soft_bounce: {} as any,
    unsubscribe: {} as any,
};
```

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
