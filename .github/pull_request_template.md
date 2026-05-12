<!--
Thanks for contributing to Sprout! 🌱
Please make sure your PR title follows Conventional Commits:
  feat(schematics): ...
  fix(workspace): ...
  docs(readme): ...
-->

## Summary

<!-- One paragraph: what does this PR change and why? -->

## What changed

<!-- Bullet list of concrete changes. Mention new files, new options, new schematics. -->
- 

## How was this verified?

<!-- Tick what applies. Add details under each box if useful. -->
- [ ] `cargo fmt --all -- --check` is clean
- [ ] `cargo clippy --all-targets --all-features --locked -- -D warnings` is clean
- [ ] `cargo test --all --locked` is green
- [ ] Manually generated code into a real Spring Boot project and ran `./mvnw compile` / `./gradlew compileJava`
- [ ] Updated relevant docs (README / CONTRIBUTING / templates)

## Related issues

<!-- Closes #123, refs #456 -->

## Notes for reviewers

<!-- Anything that's worth flagging: tradeoffs, follow-ups, areas you're unsure about. -->
