# Git Workflow Notes

## Commit Message Execution

When proposing a commit message and then creating the commit, the actual commit
message must exactly match the proposed shape.

- Do not split a bullet body across multiple `git commit -m` arguments.
- Do not use chained `git commit -m` calls for multi-line messages.
- Always write the proposed message to a temporary file and commit with
  `git commit -F`.
- When the body is a bullet list, keep the bullets adjacent. Do not insert blank
  lines between bullet items.
- After committing, verify the real message with `git log -1 --format=%B`.
- After amend or history rewrite, verify the rewritten range with `git log`.

This is an execution checklist, not a code review policy. The commit is not done
until the actual Git message has been checked.

## GitHub CLI Execution

`gh` commands that need credentials or network access should be run outside the
sandbox. Sandboxed `gh auth status` can report an invalid token even when the
host keyring session is valid.

- Use escalated execution for `gh auth status`, `gh pr create`, and similar
  GitHub CLI operations.
- If sandboxed `gh` reports an auth failure, re-check with escalated execution
  before concluding that the user must re-authenticate.
