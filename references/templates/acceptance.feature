# Approval
# superplanner-approval:start
# status: pending
# approver: none
# approved_at: none
# approval_evidence: none
# content_id: sha256:<64 lowercase hex>
# superplanner-approval:end
# Approval updates change only bytes inside the markers. Any outside-byte change
# changes content_id and invalidates approval.
# Approved status requires a non-none approver, valid ISO-8601 time, and explicit
# user-message evidence bound to this exact content_id.

Feature: <observable capability>
  As a <actor>
  I want <capability>
  So that <delivered value>

  Rule: <accepted business or domain rule>

    Scenario: <single observable outcome>
      Given <required precondition>
      And <additional relevant precondition>
      When <one business or domain action>
      Then <observable outcome>
      And <additional observable evidence>
