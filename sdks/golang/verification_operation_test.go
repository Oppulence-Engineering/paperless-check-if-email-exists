package reacher

import (
	"context"
	"fmt"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestVerificationOperationPendingAndCompletedResponses(t *testing.T) {
	const operationID = "123e4567-e89b-42d3-a456-426614174000"
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		if r.Method == "POST" {
			if r.Header.Get("Idempotency-Key") != "one-operation" {
				t.Error("missing idempotency key")
			}
			w.WriteHeader(http.StatusAccepted)
			fmt.Fprintf(w, `{"operation_id":%q,"status":"pending","status_url":"/v1/operations/%s"}`, operationID, operationID)
			return
		}
		if r.URL.Path != "/v1/operations/"+operationID {
			t.Errorf("unexpected retrieval path: %s", r.URL.Path)
		}
		fmt.Fprint(w, `{
			"input":"invalid-syntax","is_reachable":"invalid","provider_rules_applied":false,
			"misc":{"error":{"type":"Skipped"}},"mx":{"error":{"type":"Skipped"}},"smtp":{"error":{"type":"Skipped"}},
			"syntax":{"domain":"","username":"","is_valid_syntax":false,"address":null,"normalized_email":null,"suggestion":null},
			"score":{"score":0,"category":"invalid","sub_reason":"invalid_syntax","safe_to_send":false,"reason_codes":[],
				"signals":{"valid_syntax":false,"reachable":"invalid","has_mx_records":false,"smtp_error":true,"smtp_can_connect":false,
					"smtp_is_deliverable":false,"smtp_is_disabled":false,"smtp_is_catch_all":false,"smtp_has_full_inbox":false,
					"is_disposable":false,"is_role_account":false,"is_spam_trap_domain":false,"is_free_provider":false,"has_domain_suggestion":false}}
		}`)
	}))
	defer server.Close()
	config := NewConfiguration()
	config.Servers = ServerConfigurations{{URL: server.URL}}
	client := NewAPIClient(config)
	pending, response, err := client.V1API.V1CheckEmail(context.Background()).
		CheckEmailRequest(*NewCheckEmailRequest("invalid-syntax")).IdempotencyKey("one-operation").Execute()
	if err != nil {
		t.Fatal(err)
	}
	if response.StatusCode != http.StatusAccepted || pending.PendingVerification == nil || pending.CheckEmailOutput != nil {
		t.Fatalf("wrong pending variant: %#v", pending)
	}
	completed, response, err := client.V1API.V1GetVerificationOperation(context.Background(), pending.PendingVerification.OperationId).Execute()
	if err != nil {
		t.Fatal(err)
	}
	if response.StatusCode != http.StatusOK || completed.CheckEmailOutput == nil || completed.PendingVerification != nil {
		t.Fatalf("wrong completed variant: %#v", completed)
	}
}
