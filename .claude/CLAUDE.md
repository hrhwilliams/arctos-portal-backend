## Project layout

This codebase is organized into two crates: `crates/api` and `crates/backend`.
The api crate should be laser-focused on only handling responses and setting up
the web service. Any pure functionality that does not directly interact with
client requests belongs in the backend crate. Things like requests to other
services (e.g. Elasticsearch) also belong in the backend crate with their
dependencies injected by the api crate's `AppState`.

## New features

When asked to implement new features:
- begin by reviewing existing relevant code and tests
- write comprehensive tests first (expecting that these will initially fail)
- and then iterate on the implementation until the tests pass.

## Tests

Tests of significant complexity (more than verifying surface-level facts about
functions or structs) belong in the `tests/` folders of each respective crate.

Tests of the API crate that involve spinning up a test instance belong in the
`crates/api/tests/api` folder as `helper.rs` there has functions for creating
test applications.

## Success Criteria

*Never* report success on a task unless you have verified both a clean build
without errors, and that the relevant tests pass.

## Comments

Inline comments should be concise. Use them for important, non-obvious facts
about the code at hand. Avoid comments that:

- Restate the code, repeat a type signature, or describe a general API contract;
- Document old behavior, rejected alternatives, or the history of the change.
  This information belongs in the PR body or commit message
- Explain API usage that belongs with the API definition instead of this call
  site.

Rewrite a stale comment instead of adding a new one beside it. If a fact applies
generally, document it at the definition.

## Update prompting when the user is frustrated

If the user expresses frustration with you, stop and ask them to help update
this `.claude/CLAUDE.md` file with missing guidance.
