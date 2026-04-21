Cargo is **much more than pip**—that’s the first thing to fix in your mental model.

👉 **pip (Python)** = installs packages only
👉 **Cargo (Rust)** = package manager + build system + project manager + dependency resolver

So Cargo is closer to **npm (Node.js)** or even **npm + build tool + virtualenv combined**, not just pip.

---

## 🧰 What is Cargo?

Cargo is Rust’s official tool that helps you:

* Create new projects
* Build and compile code
* Manage dependencies (libraries)
* Run your program
* Run tests
* Generate docs

---

## 🚀 1. Create a new Rust project

```bash
cargo new my_project
```

This creates:

```
my_project/
├── Cargo.toml   ← config file (like package.json / requirements.txt)
└── src/
    └── main.rs  ← entry point
```

---

## ▶️ 2. Go into the project

```bash
cd my_project
```

---

## ▶️ 3. Run your project

```bash
cargo run
```

This does:

1. Compile code
2. Run executable

---

## 🔨 4. Build only (no run)

```bash
cargo build
```

For optimized release build:

```bash
cargo build --release
```

---

## 📦 5. Add dependencies (like pip install)

Instead of `pip install requests`, you:

### Option A (modern way)

```bash
cargo add serde
```

### Option B (manual)

Edit `Cargo.toml`:

```toml
[dependencies]
serde = "1.0"
```

Then:

```bash
cargo build
```

Cargo downloads + compiles automatically.

---

## 📄 Example `Cargo.toml`

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
```

---

## 🧪 6. Run tests

```bash
cargo test
```

---

## 📚 7. Generate documentation

```bash
cargo doc --open
```

---

## 🧹 8. Clean build files

```bash
cargo clean
```

---

## 🧠 Quick Comparison

| Feature          | pip     | cargo ✅          |
| ---------------- | ------- | ---------------- |
| Install packages | ✅       | ✅                |
| Build system     | ❌       | ✅                |
| Project creation | ❌       | ✅                |
| Run code         | ❌       | ✅                |
| Dependency lock  | Limited | ✅ (`Cargo.lock`) |
| Testing support  | ❌       | ✅                |

---

## ⚠️ Important Concept

Cargo manages:

* `Cargo.toml` → your dependencies
* `Cargo.lock` → exact versions (don’t edit manually)

---

## 🏁 Minimal Workflow

```bash
cargo new hello
cd hello
cargo run
```

---
