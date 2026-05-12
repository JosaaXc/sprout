<div align="center">

# 🌱 Sprout

**The missing scaffolding CLI for Spring Boot.**
**Blazing fast, NestJS-like experience — written in Rust.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.1.0-orange.svg)](Cargo.toml)
[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)](#)
[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-CE422B?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Spring Boot 3.x](https://img.shields.io/badge/Spring%20Boot-3.x-6DB33F?logo=springboot&logoColor=white)](https://spring.io/projects/spring-boot)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

```bash
sprout g resource user
```

</div>

---

## 🎯 Why Sprout?

You love Spring Boot. You love how productive NestJS developers feel when they type `nest g resource users` and watch a full CRUD slice appear in seconds. **You want that for Java.**

JHipster solves a different problem: it scaffolds *an entire application* — full-stack, opinionated end-to-end, with its own configurator and project layout. That's amazing on day zero, but heavyweight when **you already have a Spring Boot project and just want one more `User` slice in your existing package structure**.

> **Sprout drops into your existing Spring Boot project and generates incremental artifacts** — entity, repository, DTOs, mapper, service (interface + impl) and controller — following your conventions, in your packages, with your build tool. No new project. No new framework. No re-onboarding.

|                            | 🌱 **Sprout**                       | 🐘 **JHipster**                      |
| -------------------------- | ----------------------------------- | ------------------------------------ |
| **Scope**                  | Incremental artifact generation     | Full-application scaffolding          |
| **Runtime**                | Native Rust binary (sub-100ms cold) | Node + JVM toolchain                  |
| **Where does it run?**     | Inside your existing project        | Bootstraps a new project              |
| **Output coupling**        | Plain Spring Boot, zero runtime     | Opinionated stack (Liquibase, etc.)   |
| **Templates**              | Editable `.tera` files, one per case | Yeoman generators                     |
| **Onboarding**             | `cargo install` + `sprout g resource` | Read the docs first                 |

Sprout is **not a JHipster replacement**. It's the missing nimble tool for the 90% of days when you already have a project and just want to add a slice.

---

## 🚀 Features

### 🔮 Workspace Intelligence
- 🧭 **Auto-discovers your base package** by scanning `src/main/java` for the `@SpringBootApplication` class — never hard-code a path again.
- 🛠️ **Detects Maven or Gradle** automatically (`pom.xml`, `build.gradle`, `build.gradle.kts`).
- 📦 **Smart dependency injection.** Missing `mapstruct`, `spring-boot-starter-validation`, `spring-boot-starter-data-jpa` or `spring-boot-starter-data-mongodb`? Sprout *offers to add them* to your `pom.xml` / `build.gradle` before generating files, so the output compiles immediately.
- 🛡️ **Anti-overwrite protection.** If a target file already exists, Sprout prompts:
  ```
  ⚠️  File UserService.java already exists. Overwrite? (y/N)
  ```
  Decline once → that file is skipped; the rest of the slice still generates.

### 🏛️ Multi-Architecture Support
Pick the layout that matches your codebase:

| Architecture     | Layout                                                            |
| ---------------- | ----------------------------------------------------------------- |
| **🧩 Modular**   | `user/{entity,dto,mapper,repository,service,controller}` (feature-first) |
| **📚 Layered**   | `{entity,dto,mapper,repository,service,controller}/...`           |
| **🎯 Hexagonal** | `user/{domain/model, application, application/dto, infrastructure/{web,persistence,mapper}}` |

Sprout writes files to the right physical location **and** emits the matching `package` declaration — handled by the `PackageMap` abstraction so templates stay flat.

### 🧬 Persistence
- **JPA / SQL** — `@Entity`, `@Table`, `@GeneratedValue(strategy = IDENTITY)`, `JpaRepository<T, Long>`.
- **MongoDB / NoSQL** — `@Document`, `JpaRepository`'s sibling `MongoRepository<T, String>`.

### 📦 DTO Styles
- **Java Records** (immutable, modern, default).
- **Classic Classes** with Lombok `@Getter @Setter @Builder @NoArgsConstructor @AllArgsConstructor`.

Both ship with **Bean Validation** out of the box (`@NotBlank`, `@Valid`).

### 🎨 Generated Code Style (the Sprout Standard)
- Services are split into **interface + `@Service` impl** with `@RequiredArgsConstructor`.
- Controllers use `@RestController`, `@RequestMapping`, `@ResponseStatus` — **no `ResponseEntity` ceremony**.
- Mappers use **MapStruct** with `@Mapper(componentModel = "spring")`.
- Entities are decorated with `@Getter @Setter @Builder @NoArgsConstructor @AllArgsConstructor`.

---

## 📦 Installation

**No Rust toolchain required.** Sprout ships as a native binary for Linux, macOS and Windows. Pick the line that matches your shell:

### 🐧 Linux / 🍎 macOS

```bash
curl -fsSL https://raw.githubusercontent.com/JosaaXc/sprout/main/install.sh | sh
```

The script auto-detects your OS and CPU (`x86_64`, `aarch64`), downloads the latest release from GitHub, and drops the binary in `~/.local/bin/sprout`. To install a specific version:

```bash
curl -fsSL https://raw.githubusercontent.com/JosaaXc/sprout/main/install.sh | sh -s -- --version v0.1.0
```

### 🪟 Windows (PowerShell 5+)

```powershell
irm https://raw.githubusercontent.com/JosaaXc/sprout/main/install.ps1 | iex
```

Installs to `%LOCALAPPDATA%\Programs\sprout\sprout.exe` and adds it to your user `PATH`. Restart the terminal afterwards.

### 📥 Manual download

Grab the binary for your platform from the [latest release](https://github.com/JosaaXc/sprout/releases/latest), extract, and place it anywhere on your `PATH`. Available archives:

| Platform               | Asset                                          |
| ---------------------- | ---------------------------------------------- |
| Linux x86_64           | `sprout-x86_64-unknown-linux-gnu.tar.gz`       |
| Linux ARM64            | `sprout-aarch64-unknown-linux-gnu.tar.gz`      |
| macOS Intel            | `sprout-x86_64-apple-darwin.tar.gz`            |
| macOS Apple Silicon    | `sprout-aarch64-apple-darwin.tar.gz`           |
| Windows x86_64         | `sprout-x86_64-pc-windows-msvc.zip`            |

Each archive ships with a `.sha256` companion you can verify with `shasum -a 256` / `Get-FileHash`.

### 👩‍💻 From source (contributors only)

If you want to hack on Sprout itself, you need a Rust toolchain (1.75+):

```bash
git clone https://github.com/JosaaXc/sprout.git
cd sprout
cargo install --path .
```

> Once installed, Sprout has **zero runtime dependencies** — no JVM, no Node, no Python.

---

## ⚡ Quick Start

From the root of your Spring Boot project:

```bash
sprout g resource user
```

You'll get three prompts:

```
✓ Detected Spring Boot project at /home/you/store (base package: com.acme.store)

? Which architecture do you prefer?
  ❯ Modular (feature-first)
    Layered (by responsibility)
    Hexagonal (ports & adapters)

? How would you like to generate the DTOs?
  ❯ Java 14+ Records
    Classic Classes (Lombok)

? What type of database will you use for this resource?
  ❯ JPA (SQL)
    MongoDB (NoSQL)

⚠️  Missing dependencies in /home/you/store/pom.xml
   Sprout's templates rely on these and Maven is the active build tool:
     • org.mapstruct:mapstruct:1.5.5.Final           — DTO ↔ entity mapping used by generated @Mapper interfaces
     • org.springframework.boot:spring-boot-starter-validation  — Bean Validation (@NotBlank, @Valid) used by generated DTOs and controllers

? Would you like Sprout to add the missing dependencies now? (Y/n) y
  INSTALL mapstruct (org.mapstruct:mapstruct:1.5.5.Final)
  INSTALL spring-boot-starter-validation (org.springframework.boot:spring-boot-starter-validation)
  ✓ Re-run your build to refresh the dependency graph.

  CREATE /home/you/store/src/main/java/com/acme/store/user/entity/User.java
  CREATE /home/you/store/src/main/java/com/acme/store/user/repository/UserRepository.java
  CREATE /home/you/store/src/main/java/com/acme/store/user/dto/UserRequest.java
  CREATE /home/you/store/src/main/java/com/acme/store/user/dto/UserResponse.java
  CREATE /home/you/store/src/main/java/com/acme/store/user/mapper/UserMapper.java
  CREATE /home/you/store/src/main/java/com/acme/store/user/service/UserService.java
  CREATE /home/you/store/src/main/java/com/acme/store/user/service/UserServiceImpl.java
  CREATE /home/you/store/src/main/java/com/acme/store/user/controller/UserController.java

✓ Done.
```

That's it. Run `./mvnw spring-boot:run`, hit `POST /api/users`, and you have a working CRUD.

---

## 📖 Commands

`sprout g` is an alias for `sprout generate`.

| Command                        | What it generates                                                        |
| ------------------------------ | ------------------------------------------------------------------------ |
| `sprout g resource <name>`     | **Full CRUD slice** — entity + repository + DTOs + mapper + service + controller |
| `sprout g entity <name>`       | JPA or Mongo entity (Lombok-decorated)                                   |
| `sprout g repository <name>`   | `JpaRepository` or `MongoRepository`                                     |
| `sprout g dto <name>`          | `Request` + `Response` (Records or classic classes)                      |
| `sprout g mapper <name>`       | MapStruct `@Mapper(componentModel = "spring")`                           |
| `sprout g service <name>`      | `XxxService` interface + `@Service XxxServiceImpl` with `@RequiredArgsConstructor` |
| `sprout g controller <name>`   | `@RestController` with `@ResponseStatus` endpoints                       |

`<name>` is case-flexible — `user`, `User`, `user-account`, `UserAccount` all produce the same `User` / `UserAccount` Pascal identifier internally.

---

## 🗺️ Roadmap

The 0.1 release nails the core CRUD slice. Here's where we're headed — **PRs welcome**, see [CONTRIBUTING.md](CONTRIBUTING.md).

- [ ] 🔐 **`sprout g security`** — Spring Security scaffolding (JWT, OAuth2 Resource Server, method-level `@PreAuthorize` patterns)
- [ ] 🌊 **R2DBC + WebFlux** — reactive equivalents of the JPA path (`ReactiveCrudRepository`, `Mono` / `Flux` controllers)
- [ ] 🟪 **Kotlin templates** — same schematics, idiomatic Kotlin output (data classes, coroutines)
- [ ] 🔭 **GraphQL** — `sprout g resolver` with Spring for GraphQL
- [ ] 🧪 **Test scaffolding** — auto-generated `@SpringBootTest` + Testcontainers slice for each resource
- [ ] 📜 **OpenAPI annotations** — emit `@Operation`, `@ApiResponse` on generated controllers
- [ ] 🪝 **Custom template overrides** — `.sprout/templates/` in the project root takes precedence over embedded ones
- [ ] 🌍 **`sprout init`** — bootstrap an empty Spring Boot project with the layout already wired
- [ ] 🧩 **Plugin system** — third-party schematics installable via `cargo install`
- [ ] 📋 **`sprout list`** — show available schematics and the architecture currently in use
- [ ] 🔧 **`sprout doctor`** — health-check the project against Sprout's expectations

---

## 🏗️ Architecture (the 30-second version)

Sprout is built around three small ideas:

1. **`ProjectContext`** — auto-discovered once. Holds `base_path` (physical) and `base_package` (logical).
2. **`PackageMap`** — computed per resource. Resolves where each artifact's `package` declaration should point, given the chosen architecture. **This decouples "where the file goes on disk" from "what the package line says".**
3. **One `.tera` template per concrete combination** — no `{% if %}` branches inside templates. The Rust side picks `entity/jpa.java.tera` *or* `entity/mongo.java.tera`. Adding a new database dialect is a flat, additive change.

For the full design, the trait hierarchy, and how to extend Sprout, see **[CONTRIBUTING.md](CONTRIBUTING.md)**.

---

## 💬 Community

- 🐛 **Bug reports & feature requests:** [open an issue](https://github.com/JosaaXc/sprout/issues)
- 💡 **Ideas & discussion:** [GitHub Issues](https://github.com/JosaaXc/sprout/issues)
- 🤝 **Contributing:** [CONTRIBUTING.md](CONTRIBUTING.md)
- 📜 **Code of Conduct:** [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)

---

## 📄 License

Sprout is licensed under the **MIT License**. See [LICENSE](LICENSE).

---

<div align="center">

**Built with 🦀 Rust. Made for ☕ Java developers.**

If Sprout saved you ten minutes today, ⭐ the repo. If it saved you an hour, [drop a comment in our issues](https://github.com/JosaaXc/sprout/issues).

</div>
