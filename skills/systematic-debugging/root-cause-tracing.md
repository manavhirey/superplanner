# Root Cause Tracing

Use backward tracing when a failure appears far from the state that caused it.

1. Record the exact symptom and the operation that directly emitted it.
2. Inspect that operation's inputs and assumptions.
3. Find its caller and record the values passed at that boundary.
4. Repeat through callers, events, queues, transformations, or persisted state until reaching the first point where actual and expected state diverge.
5. Prove the origin with a focused trace, assertion, breakpoint, or minimal reproducer.
6. Fix the origin, then verify the downstream symptom disappears.

When static reading is insufficient, add temporary diagnostics immediately before the dangerous operation and at each upstream boundary. Include identifiers, relevant values, environment, timestamp or sequence, and a stack or causal ID. Avoid secrets and personal data. Capture one clean run and one failing run, then compare the first divergence.

Do not stop at the first invalid value. Ask where it came from, why it was accepted, and which earlier contract was violated. The root cause is the earliest actionable violation that explains all observed evidence, not merely the deepest stack frame.
