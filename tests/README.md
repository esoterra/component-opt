# Tests

The testing here is best-effort and done rapidly.
If you want more testing, please contribute it.

## `runtime`

Runtime tests perform the following steps.
1. Take components written in Claw.
2. Compile them to component binaries.
3. Load them into `wasmtime` which checks their validity.
4. Execute the corresponding Claw runtime tests to check behavior.
