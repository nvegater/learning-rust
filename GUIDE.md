# Rust Learning Path — From TypeScript to Production Rust

## Phase 1: Foundations

### Step 1 — Project Setup

```bash
# Initialize a new Rust project (like `npm init`)
cargo init --name learning_rust

# Verify it worked — should print "Hello, world!"
cargo run

# Run the built-in test (should pass)
cargo test
```

**What just happened:**
- `Cargo.toml` = your `package.json` (dependencies, metadata)
- `src/main.rs` = entry point (like `index.ts`)
- `cargo run` = compile + execute (like `ts-node index.ts` but produces a native binary)
- `cargo test` = runs all `#[test]` functions (built-in, no jest/vitest needed)

**Verify:** You should see `Hello, world!` from `cargo run` and `0 passed` from `cargo test` (no tests yet).

---

### Step 2 — Ownership & Borrowing

> This is THE concept. TypeScript has a garbage collector — Rust has ownership.
> Once you get this, everything else clicks.

We'll write this code together. After `cargo init`, tell me and we'll write
the first module with tests that prove ownership rules at compile time.

---

### Step 3 — Structs, Enums, Traits

> Think: `interface` → `trait`, `type Union =` → `enum`, `class` → `struct + impl`

We'll build a domain model with tests.

---

### Step 4 — Error Handling

> No try/catch. `Result<T, E>` is like a discriminated union that the compiler forces you to handle.

We'll build error types with tests that verify both happy and error paths.

---

## Phase 2: REST API

### Step 5 — Axum Web Framework

```bash
# Add dependencies (like `npm install`)
cargo add axum tokio --features tokio/full
```

We'll build routes together with integration tests that hit real endpoints.

---

### Step 6 — Async/Await with Tokio

> Same `async/await` keywords as TS, but Rust doesn't have a built-in runtime.
> Tokio is the runtime (like Node's event loop, but you choose it explicitly).

---

### Step 7 — Serde (Serialization)

```bash
cargo add serde --features derive
cargo add serde_json
```

> Like `zod` + `JSON.parse` but enforced at compile time.

---

### Step 8 — SQLx + Postgres

```bash
cargo add sqlx --features runtime-tokio,postgres,macros
```

> Compile-time checked SQL queries. No ORM, no runtime surprises.

---

### Step 9 — Middleware, Auth, Error Responses

Production patterns for a real API.

---

## Phase 3: Professional Infra

### Step 10 — Testing

```bash
# Run all tests
cargo test

# Run tests with output visible
cargo test -- --nocapture

# Run a specific test
cargo test test_name
```

Unit tests live next to the code (in the same file). Integration tests go in `tests/`.

---

### Step 11 — CI/CD (GitHub Actions)

```bash
git init
gh repo create learning-rust --private --source=. --remote=origin
```

We'll create `.github/workflows/ci.yml` with:
- `cargo fmt --check` (formatting)
- `cargo clippy` (linting — like ESLint but stricter)
- `cargo test` (tests)
- Dependency caching

---

### Step 12 — Docker

```bash
docker build -t learning-rust .
docker run -p 3000:3000 learning-rust
```

Multi-stage build: compile in a fat image, copy binary to a ~10MB scratch image.

---

### Step 13 — Deployment

Options: Fly.io, Railway, or plain Docker on any cloud.

```bash
# Example: Fly.io
fly launch
fly deploy
```

---

### Step 14 — Observability

```bash
cargo add tracing tracing-subscriber
```

Structured logging like pino/winston, but with compile-time span tracking.

---

## Quick Reference: TS → Rust

| TypeScript | Rust |
|---|---|
| `npm init` | `cargo init` |
| `npm install foo` | `cargo add foo` |
| `npx ts-node index.ts` | `cargo run` |
| `npm test` | `cargo test` |
| `eslint` | `cargo clippy` |
| `prettier` | `cargo fmt` |
| `interface` | `trait` |
| `type A \| B` | `enum` |
| `class` | `struct + impl` |
| `try/catch` | `Result<T, E>` + `?` |
| `null/undefined` | `Option<T>` |
| `async/await` | `async/await` + Tokio runtime |
| `JSON.parse` | `serde_json::from_str` |
| `package.json` | `Cargo.toml` |
