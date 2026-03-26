# ADR 001: Use `workflow/` As The Artifact Root

## Status

Accepted

## Context

`mnemix-workflow` needs a repository structure that distinguishes:

- explanatory documents
- reusable operational assets
- active planning artifacts
- durable framework decisions

Early iterations considered top-level `workstreams/` and `decisions/` folders directly in the repo root, but that made repo-level decisions feel disconnected from the workflow domain itself.

## Decision

Use `workflow/` as the root artifact domain for active planning artifacts.

Inside it:

- `workflow/workstreams/` stores active workstreams
- `workflow/decisions/` stores repo-level durable decisions

## Consequences

- The repository root stays cleaner
- Repo-level decisions are clearly part of the workflow system
- The distinction between the overall methodology and each individual workstream remains explicit
