# Defense In Depth

After proving and fixing the root cause, map where the invalid state enters, crosses trust boundaries, becomes dangerous, and can be diagnosed. Add only protections that catch a distinct realistic path:

- validate shape and invariants at external entry points;
- enforce business invariants where the operation is performed;
- guard destructive or environment-specific operations at the final boundary;
- retain safe diagnostics that identify future violations without exposing secrets.

Tests should prove the source fix and each added boundary by attempting the path that boundary owns. Keep validation messages specific and preserve one authoritative rule when possible; duplicated business logic that can drift is not defense in depth.

These layers are not a substitute for tracing. A downstream guard may limit damage, but the task is incomplete until the original producer of invalid state is corrected or the external cause is explicitly documented and handled.
