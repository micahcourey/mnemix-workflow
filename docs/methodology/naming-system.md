# Mnemix Workflow Naming System

## Purpose

This document defines the teaching vocabulary for `mnemix-workflow`.

The goal is to keep the methodology memorable without making the repository cryptic. Product language and file/folder language should reinforce each other instead of competing.

## Core Terms

### Workflow

`workflow` is the overall system and methodology.

It refers to:

- the planning method
- the CLI experience
- the relationship between specs, UX, plans, tasks, decisions, and optional standards-backed artifacts

### Workstream

A `workstream` is one unit of planned work.

In practice, a workstream is usually:

- one feature
- one meaningful capability
- one framework initiative
- one cohesive implementation narrative

Each workstream lives under `workflow/workstreams/`.

### Status

`STATUS.md` defines:

- the current lifecycle state
- a short machine-readable summary
- the last updated date
- optional linked PR numbers

This is the lightweight state artifact.

### Spec

`spec.md` defines:

- the problem
- the users
- the goals
- the scope
- the success criteria

This is the intent artifact.

### UX

`ux.md` defines:

- the user or developer experience
- the key journeys and states
- the interaction expectations
- the acceptance scenarios

This is the experience artifact.

### Plan

`plan.md` defines:

- the implementation strategy
- the standards involved
- the rollout and sequencing
- the main technical design for execution

This is the technical blueprint artifact.

### Tasks

`tasks.md` defines:

- the execution slices
- the order of work
- the validation checkpoints

This is the execution artifact.

### Decisions

`workflow/decisions/` stores durable decisions.

Use:

- `workflow/workstreams/<id>/decisions/` for workstream-local decisions
- `workflow/decisions/` for repo-wide framework decisions that constrain future work

## Folder Naming

### Top-Level Folders

- `workflow/`
  - chosen as the root artifact domain for the methodology
- `workflow/workstreams/`
  - chosen because each feature folder is a stream of work inside the overall workflow
- `workflow/decisions/`
  - chosen for durable repo-level framework decisions that should be separate from active workstreams
- `docs/`
  - chosen for methodology and planning documents that are not themselves active workstreams

### Why `workflow/`

`workflow` is the name of the methodology, and it works best as the root artifact domain rather than the name of each individual unit.

Using `workflow/` as the container while nesting `workstreams/` inside it keeps both levels clear:

- `workflow/` is the overall artifact space
- `workflow/workstreams/` is where individual streams of work live
- `workflow/decisions/` is where repo-level durable decisions live

That avoids mixing repo-level decisions directly into the repo root while still keeping the workstream concept explicit.

### Why Not `specs/`

`specs/` is clear, but too narrow for this methodology.

Each workstream contains more than specs:

- status metadata
- UX intent
- technical plan
- tasks
- decisions
- optional contracts and architecture artifacts

`workstreams/` better reflects the broader unit, and nesting it under `workflow/` keeps that broader unit inside the methodology's artifact root.

## Teaching Summary

The shortest teachable version of the methodology is:

> A workflow is made of workstreams. Each workstream moves from spec to UX to plan to tasks, with decisions recorded when they become durable.

## Initial Repository Convention

```text
workflow/
  workstreams/
    001-bootstrap-mnemix-workflow/
      STATUS.md
      spec.md
      ux.md
      plan.md
      tasks.md
      decisions/
  decisions/
```
