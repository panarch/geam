# Release Notes

Each release PR supplies one reviewed `docs/releases/<version>.md` file.
Use the workspace version in both its filename and first heading:

```markdown
# Geam <version>

## Changes

- Describe the user-visible changes and fixes in this release.

## Compatibility

- Explain any migration steps or changed behavior, when applicable.
```

Replace the example prose with actual release content; omit sections that do
not apply. Notes are written and reviewed by people, not generated from commit
subjects. Distinguish shipped behavior from future work.

Publishing rejects a missing file, mismatched heading, heading-only content,
or `TODO`, `TBD`, `FIXME`, and HTML comment placeholders. Mechanical checks do
not replace editorial review. The reviewed file becomes the GitHub Release body
without a second manual copy or changelog update.
