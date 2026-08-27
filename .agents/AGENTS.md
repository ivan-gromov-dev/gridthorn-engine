# Repository Skill Instructions

- Keep each skill focused on one repeatable repository workflow.
- Use concise, discriminating frontmatter descriptions so implicit activation
  does not capture unrelated requests.
- Refer to repository `AGENTS.md` files as the live source of coding and
  verification conventions instead of duplicating their full contents.
- Prefer instruction-only skills. Add scripts or references only when they
  remove repeated deterministic work or keep conditional detail out of the
  entrypoint.
- Preserve authorization boundaries: a workflow may prepare release metadata
  without implicitly authorizing commits, tags, pushes, or hosted releases.
- Validate every new or changed skill with the system skill validator.

