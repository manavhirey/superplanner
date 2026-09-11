# Condition-Based Waiting

Arbitrary sleeps guess when asynchronous work will finish. Wait for the observable condition instead.

```text
wait until condition:
  read fresh state
  return when the expected event, state, count, or resource exists
  fail after a bounded deadline with the last observed state
  pause briefly or subscribe to an event before checking again
```

Prefer an event, promise, callback, notification, or test-framework wait primitive over polling. If polling is necessary, use a reasonable interval, read fresh state on every iteration, enforce a timeout, and make the timeout error describe the condition and last observation.

An elapsed-time delay is valid only when timing itself is under test, such as debounce or lease expiry. First synchronize on the triggering condition, then wait the contractually defined duration and document why that duration proves the behavior. Increasing a timeout without evidence is not a root-cause fix.
