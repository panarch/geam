# Release Notes

Each release PR supplies one reviewed `docs/releases/<version>.md` file.
Use the workspace version in both its filename and first heading:

```markdown
# Geam <version>

Summarize the purpose and scope of the release in a short paragraph.

## Changes

- Explain a change and its effect. ([#number](PR_URL), @author)

## Compatibility

- Explain any migration steps or changed behavior, when applicable.
```

Replace the example prose with actual release content; omit sections that do
not apply. Notes are written and reviewed by people, not generated from commit
subjects. Distinguish shipped behavior from future work.

## Content And Attribution

- Review the full comparison with the previous release so substantive PRs are
  not missed. Do not present mechanical version or lock updates as new features.
- Explain what changed and why it matters, rather than copying commit titles.
  Identify maintenance-only releases without implying runtime improvements.
- Link the relevant PRs at the end of each change item. Group related PRs when
  they deliver one change; a PR does not have to become a separate item.
- Credit verified human contributors with their GitHub handles, including
  relevant co-authors. Use the same convention for maintainers and external
  contributors; do not credit automated bump bots.

## First-Time Contributors

When a release includes first-time contributors, add a `## New Contributors`
section after the release content. Welcome each person by their GitHub handle
and link their first merged PR. Verify that it is their first contribution to
the repository, not merely their first in this release.

Omit this section when there are no new contributors. Ordinary attribution stays
with each change item; do not duplicate it in a separate contributors list.

## Publication Checks

Publishing rejects a missing file, mismatched heading, heading-only content,
or `TODO`, `TBD`, `FIXME`, and HTML comment placeholders. Mechanical checks do
not replace editorial review. The reviewed file becomes the GitHub Release body
without a second manual copy or changelog update.
