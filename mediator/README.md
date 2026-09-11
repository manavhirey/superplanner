# Mediator

This directory holds the `safe_git` mediator **client**, which the contracts
require to be a bash script started through the exact
`/usr/bin/env -i ... /bin/bash --noprofile --norc` boundary
(`references/integration-protocol.md:617-621`) and to remain bash-3.2-safe.

The client lands with issue #9 together with the Rust `sp-safegit` service
that implements the positional `plain`, `objects`, `external`, and `authorized`
modes. Until then this directory intentionally contains no executable code;
nothing here is functional.
