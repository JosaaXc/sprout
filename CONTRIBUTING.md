# 🌱 Contributing to Sprout

Hi, and thanks for thinking about contributing to Sprout. 💚

Sprout is intentionally small and opinionated, and the best contributions are the ones that **keep it that way while making it more useful**. This document explains how we work, what we expect from a PR, and — most importantly — **how the codebase is shaped so you can add features without fighting it**.

If anything here is unclear, open a [Discussion](https://github.com/JosaaXc/sprout/discussions) before writing code. We'd rather chat for ten minutes than have you spend an afternoon on a PR we can't merge.

---

## 🚦 The TL;DR

```bash
# 1. Fork on GitHub, then clone your fork
git clone https://github.com/<your-username>/sprout.git
cd sprout

# 2. Install the pre-push hook so you never push a red build
./scripts/install-hooks.sh

# 3. Branch off main
git checkout -b feat/short-descriptive-name

# 4. Make your change, then run the preflight (or rely on the hook)
./scripts/preflight.sh

# 5. Commit, push, open a PR against `main`
git commit -m "feat(schematics): add R2DBC repository templates"
git push origin feat/short-descriptive-name
```

The pre-push hook runs `cargo fmt --check`, `cargo clippy -D warnings` and `cargo test --all`, which are the exact gates CI enforces. Skip it once with `git push --no-verify` if you absolutely need to (e.g., to push WIP for review).

A pull request is welcome when:
- ✅ `cargo test` passes
- ✅ `cargo clippy --all-targets -- -D warnings` is silent
- ✅ `cargo fmt --check` is silent
- ✅ The change is described in plain English in the PR body
- ✅ Any new public function has a doc comment **only if** the *why* is non-obvious

---

## 🧭 The Workflow

We use the classic **Fork → Branch → PR** flow. Direct pushes to `main` are not allowed.

### 1. Fork

Click "Fork" on the GitHub UI. Clone *your fork*, not the upstream:

```bash
git clone https://github.com/<your-username>/sprout.git
cd sprout
git remote add upstream https://github.com/JosaaXc/sprout.git
```

### 2. Branch

Always branch from an up-to-date `main`:

```bash
git fetch upstream
git checkout main
git merge upstream/main
git checkout -b <type>/<short-slug>
```

**Branch naming.** Use one of: `feat/…`, `fix/…`, `docs/…`, `refactor/…`, `test/…`, `chore/…`.

### 3. Commit messages

We follow **Conventional Commits**. Examples:

```
feat(schematics): add R2DBC repository templates
fix(workspace): skip <dependencyManagement> when injecting into pom.xml
docs(readme): clarify Hexagonal layout example
refactor(context): inline PackageMap construction
```

The first line stays under **72 characters**. Body wraps at 100. Body is optional but encouraged when the *why* isn't obvious from the diff.

### 4. Pull Request

Open the PR against `main`. The PR title should be a single conventional-commit line. The body should answer three questions:

1. **What is the change?** One paragraph.
2. **Why now?** Link issues, cite use cases.
3. **How was it verified?** Tests added, manual smoke runs, screenshots of generated Java.

CI will run `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test --all`. **All three must be green** before a maintainer reviews.

---

## 🏗️ Architecture: The 5-Minute Tour

Sprout is small on purpose. Reading the code top-to-bottom takes less than an hour. Here are the load-bearing ideas — internalize these and most contributions become obvious.

### The folder map

```
src/
├── cli/             # clap definitions; nothing else
├── context/         # the data that drives Tera (architecture, dto_style, persistence, GenerationContext)
├── naming/          # NameSet — case conversions + pluralization
├── prompts/         # dialoguer + InteractivePrompter trait (so tests can inject a fake)
├── rendering/       # rust-embed loader + TeraEngine wrapper
├── schematics/      # one file per artifact: entity_schematic.rs, repository_schematic.rs, …
│                    # plus the registry (factory) and resource_schematic (composite)
├── workspace/       # ProjectContext discovery + DiskFileWriter + build_tool/
└── error.rs

templates/           # embedded into the binary with rust-embed
├── entity/{jpa,mongo}.java.tera
├── repository/{jpa,mongo}.java.tera
├── dto/{request_record,request_class,response_record,response_class}.java.tera
├── mapper/mapstruct.java.tera
├── service/{interface,implementation}.java.tera
└── controller/controller.java.tera
```

### Three principles

#### 1. **Open / Closed: schematics are a registry, not a switch**

Every artifact implements the `Schematic` trait. `SchematicRegistry::resolve(&SchematicKind)` is the **only** central dispatch point. Adding a new artifact never edits existing schematics — you add a struct, one match arm in the registry, and one `.tera` file.

#### 2. **Separation of physical path and logical package**

This is the single most important pattern in Sprout.

- The **physical anchor** lives in [`ProjectContext::base_path`](src/workspace/project_detector.rs) — the on-disk directory of the class annotated with `@SpringBootApplication`. Found once at startup by walking `src/main/java` and parsing `package …;` with a regex.
- The **logical anchor** lives in [`PackageMap`](src/context/generation_context.rs) — a serializable struct that tells templates what to put in `package …;` and `import …;` lines, *for the current architecture and feature*.

Both are derived from the same `Architecture::path_for(feature, artifact)` function. That keeps Modular, Layered, and Hexagonal in lockstep: changing one updates both ends.

> 💡 **If you ever feel like building paths inside a `.tera` template, stop and put it in `PackageMap` instead.**

#### 3. **No `{% if %}` inside templates — one file per concrete case**

We deliberately split templates per combination:

```
templates/entity/jpa.java.tera
templates/entity/mongo.java.tera
```

…instead of a single `entity.java.tera` with `{% if is_jpa %}` branches.

**Why?**

- **Diff clarity in PRs.** Adding R2DBC support shouldn't touch the JPA template. Reviewers see only new files.
- **Lower merge-conflict surface.** Two contributors adding two database dialects in parallel never collide.
- **Each template is grep-friendly.** Search for `MongoRepository` and you land on exactly one place.
- **Templates stay readable.** Java + Jinja is hard enough; Java + Jinja + nested conditionals is hostile to new contributors.

The Rust side picks the right template via methods on enums:

```rust
// src/context/persistence.rs
pub fn entity_template(&self) -> &'static str {
    match self {
        Self::JpaRelational => "entity/jpa.java.tera",
        Self::MongoDb       => "entity/mongo.java.tera",
    }
}
```

So **the decision lives in the language, not in the template**.

---

## 🧪 Example: Add a new database dialect (3 steps)

Suppose we want R2DBC support — reactive SQL repositories. Here's the entire diff shape, by file count and intent:

### Step 1 — Add the variant to `Persistence`

```rust
// src/context/persistence.rs
pub enum Persistence {
    JpaRelational,
    MongoDb,
    R2dbc, // ← new
}

impl Persistence {
    pub fn all() -> &'static [Persistence] {
        &[Self::JpaRelational, Self::MongoDb, Self::R2dbc]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::JpaRelational => "JPA (SQL)",
            Self::MongoDb       => "MongoDB (NoSQL)",
            Self::R2dbc         => "R2DBC (Reactive SQL)",
        }
    }

    pub fn entity_template(&self) -> &'static str {
        match self {
            Self::JpaRelational => "entity/jpa.java.tera",
            Self::MongoDb       => "entity/mongo.java.tera",
            Self::R2dbc         => "entity/r2dbc.java.tera",
        }
    }

    pub fn repository_template(&self) -> &'static str {
        match self {
            Self::JpaRelational => "repository/jpa.java.tera",
            Self::MongoDb       => "repository/mongo.java.tera",
            Self::R2dbc         => "repository/r2dbc.java.tera",
        }
    }

    pub fn id_type(&self) -> &'static str {
        match self {
            Self::JpaRelational | Self::R2dbc => "Long",
            Self::MongoDb                     => "String",
        }
    }
}
```

### Step 2 — Add the two template files

```
templates/entity/r2dbc.java.tera
templates/repository/r2dbc.java.tera
```

`rust-embed` discovers them automatically. **You do not register them anywhere.**

```jinja
{# templates/repository/r2dbc.java.tera #}
package {{ packages.repository }};

import org.springframework.data.repository.reactive.ReactiveCrudRepository;
import org.springframework.stereotype.Repository;

import {{ packages.entity }}.{{ name.pascal }};

@Repository
public interface {{ name.pascal }}Repository
        extends ReactiveCrudRepository<{{ name.pascal }}, {{ id_type }}> {
}
```

### Step 3 — Teach the dependency auditor about the new runtime dep

```rust
// src/workspace/build_tool/mod.rs
const DATA_R2DBC: DependencyCoord = DependencyCoord {
    group_id:    "org.springframework.boot",
    artifact_id: "spring-boot-starter-data-r2dbc",
    version:     None,
    purpose:     "R2DBC runtime required by ReactiveCrudRepository<Entity, Long>",
};

pub fn required_dependencies_for(ctx: &GenerationContext) -> Vec<DependencyCoord> {
    let mut deps = vec![MAPSTRUCT, VALIDATION];
    match ctx.persistence {
        Persistence::JpaRelational => deps.push(DATA_JPA),
        Persistence::MongoDb       => deps.push(DATA_MONGO),
        Persistence::R2dbc         => deps.push(DATA_R2DBC),
    }
    deps
}
```

**That's it.** No edits to schematics, controllers, services, DTOs, the registry, or any existing template. That's the OCP guarantee in action.

> ⚠️ **Don't add a `match` on `Persistence` outside of `src/context/persistence.rs` or `src/workspace/build_tool/mod.rs`.** Those are the two places that *should* be exhaustive. Everywhere else, route through methods on the enum.

---

## 🧩 Example: Add a brand-new schematic (e.g., `sprout g event`)

Three places, always the same three:

1. **Variant on `SchematicKind`** ([src/cli/schematic_kind.rs](src/cli/schematic_kind.rs)) — clap generates parsing for free.
2. **Struct implementing `Schematic`** in `src/schematics/event_schematic.rs`.
3. **One arm** in `SchematicRegistry::resolve` ([src/schematics/registry.rs](src/schematics/registry.rs)).

Plus your `.tera` file under `templates/event/`. Done.

If your schematic needs a new prompt, add it to `prompts/`, expose the result on `GenerationContext`, and gate it behind a `needs_*()` helper on `SchematicKind` so unrelated commands don't ask it. The pattern is established — copy `needs_persistence()`.

---

## ✅ Quality bar

### Required for every PR

- **`cargo fmt --all`** — no exceptions.
- **`cargo clippy --all-targets --all-features -- -D warnings`** — warnings break the build.
- **`cargo test --all`** — including the doctests in `src/`.

CI runs all three on every push and on every PR. Locally, just run the preflight:

```bash
# macOS / Linux (and Windows via git-bash or WSL)
./scripts/preflight.sh

# Windows PowerShell
.\scripts\preflight.ps1
```

If you ran `./scripts/install-hooks.sh` once after cloning (Unix only), the bash version runs automatically on every `git push`.

### Encouraged

- **Tests for new templates.** Add a unit test that renders the template with a fixture `GenerationContext` and asserts a few key strings appear (`@Entity`, the package line, the expected ID type). See `src/workspace/build_tool/maven.rs` for the testing style.
- **Smoke-test the Java.** Generate a resource into a real Spring Boot project and run `./mvnw compile`. **The single most valuable contract test for a code generator.**
- **Comments only when the *why* is non-obvious.** Don't narrate code; cite constraints, invariants, or workarounds.

### Discouraged

- ❌ Adding runtime dependencies. Sprout's `Cargo.toml` is intentionally lean.
- ❌ Hidden `match` statements on `Persistence`, `DtoStyle`, `Architecture` outside their owning modules.
- ❌ Logic in templates. If you reach for `{% if %}` to switch behavior, consider whether the decision should live in Rust instead.
- ❌ `unwrap()` / `expect()` in non-test code (use `anyhow::Context` and `?`).

---

## 📜 Code of Conduct

Be kind. Be patient. Be precise. This project follows the [Contributor Covenant v2.1](https://www.contributor-covenant.org/version/2/1/code_of_conduct/) — see [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

Disrespectful, exclusionary, or hostile behavior in issues, PRs, or discussions will be moderated.

---

## 🙏 Thank You

Every star, every issue, every typo-fix PR makes Sprout better. Whether you're submitting a one-line clippy fix or a whole new schematic, you're keeping this project alive and useful.

**Welcome aboard.** 🌱

— *The Sprout maintainers*
