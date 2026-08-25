# Architecture Decision Records

Architecture Decision Records (ADRs) capture decisions that are expensive to
reverse or that constrain multiple subsystems. Design documents describe the
current direction; ADRs explain why a particular direction was accepted.

## When an ADR is required

Write an ADR before treating any of the following as stable:

- a public API or crate boundary that is difficult to migrate;
- selection or replacement of a foundational dependency;
- determinism, serialization, migration, or compatibility guarantees;
- process, protocol, threading, or plugin lifecycle models;
- an intentional exception to an architecture invariant.

Small local implementation choices do not need an ADR unless they accumulate
into one of these boundaries.

## Lifecycle

An ADR has one of these statuses:

- **Proposed:** under discussion or awaiting a spike.
- **Accepted:** the decision is the current architecture contract.
- **Rejected:** evaluated but not adopted.
- **Superseded:** replaced by a later ADR that links back to it.

Accepted ADR text is immutable except for typo fixes and additional links. A
changed decision receives a new ADR, and the previous record becomes
superseded. Implementation status belongs in the roadmap, not in ADR status.

## Workflow

1. Copy [0000-template.md](0000-template.md).
2. Allocate the next four-digit number and a short kebab-case filename.
3. Describe context, decision drivers, considered options, and consequences.
4. Keep the ADR `Proposed` while its required spike or review is incomplete.
5. Link an accepted ADR from the affected design document and update any
   superseded record.

The decision section must be specific enough to test. If the selected option
has not passed its stated validation, it remains provisional even when it is the
preferred direction.

## Index

No project decisions have been accepted yet. The current technology baseline is
provisional design input for the Milestone 0 spikes.
