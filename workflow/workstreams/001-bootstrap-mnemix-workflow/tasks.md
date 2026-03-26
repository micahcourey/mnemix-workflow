# Tasks: Bootstrap Mnemix Workflow

## Workstream Goal

Create the minimum viable repository structure and planning artifacts needed to start building `mnemix-workflow` by using the workflow on itself.

## Execution Slices

### Slice 1: Establish Core Repository Shape

- [x] Add a root `README.md`
- [x] Add `docs/methodology/`
- [x] Add `docs/prd.md` as the living product document
- [x] Add repo-level `workflow/decisions/`
- [x] Add `workflow/workstreams/001-bootstrap-mnemix-workflow/`

### Slice 2: Define The Methodology

- [x] Write the one-line project description
- [x] Define the vocabulary for workflow, workstream, spec, UX, plan, tasks, and decisions
- [x] Document why `workstreams/` is the unit-of-work folder

### Slice 3: Dogfood The First Workstream

- [x] Create `spec.md`
- [x] Create `ux.md`
- [x] Create `plan.md`
- [x] Create `tasks.md`

### Slice 4: Prepare The Next Workstream

- [x] Decide which repo-level ADRs should be written first
- [x] Define the scope of the next workstream for CLI bootstrap
- [x] Decide whether to add initial template files in the next workstream or a later one

## Validation Checklist

- [x] The repository has a clear root entrypoint
- [x] The methodology vocabulary lives in the repo
- [x] The first workstream is self-contained
- [x] `ux.md` includes Gherkin scenarios
- [x] The next implementation-focused workstream is defined

## Notes

- This workstream intentionally stops short of CLI implementation.
- The goal is to make the repo legible and actionable before building code.
