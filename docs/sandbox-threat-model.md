# Sandbox threat model (baseline)

## Objective

Reduce risk from untrusted learner code execution while preserving useful feedback.

## Baseline controls implemented

- Executor container runs as non-root user.
- Outbound network disabled (`network_mode: none`).
- Process count limited (`pids_limit`).
- Memory limit configured (`mem_limit`).
- Read-only root filesystem + tmpfs scratch space.
- Execution timeout enforced for `cargo check` and `cargo test`.
- Output-size truncation to prevent oversized response payloads.

## Known limitations

- Strong syscall filtering and namespace isolation are not yet enforced.
- CPU quotas and disk quotas need explicit runtime policy coverage in production deployment.
- Timeout currently wraps command execution but does not yet guarantee process-tree kill semantics for all cases.

## Next hardening steps

1. Add explicit seccomp/apparmor profiles.
2. Add strict CPU quotas and I/O quotas.
3. Add egress audit telemetry and sandbox timeout metrics.
4. Add adversarial submission test suite (fork bombs, memory pressure, infinite loops).
