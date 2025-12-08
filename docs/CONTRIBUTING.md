# Contributing to Project Daneel

Thank you for your interest in contributing.  
This project is early-stage but follows strict architectural principles.  
Contributions must preserve **Tier-0 immutability**, **structural ethics**, and the **lattice-based decision model**.

---

## 🔥 Core Rules (Non-Negotiable)

1. **Tier-0 is immutable.**  
   - No edits, no extensions, no conditional logic added later.  
   - If you believe a Tier-0 principle is flawed, open an issue — do *not* alter code.

2. **No executable logic without tests.**  
   - Every change in `src/` requires a matching test in `tests/`.

3. **No pseudo-code in core modules.**  
   - All merged code must compile and pass `cargo test`.

4. **Architecture > Features.**  
   - New features must fit the 3-Tier model:  
     - **Tier-0:** Immutable principles  
     - **Tier-1:** Stable structure  
     - **Tier-2:** Adaptive behavior

5. **Security assumptions must be explicit.**  
   - If your PR touches identity, memory, or integrity, list threat models and failure cases.

---

## 🚀 How to Contribute

### 1. Fork and Clone
```bash
git clone https://github.com/YOUR_USERNAME/project-daneel.git
cd project-daneel
```

### 2. Create a Branch
```bash
git checkout -b feature/my-change
```

### 3. Follow the Repo Structure
- `src/core.rs` — root lattice, core structs  
- `src/tier.rs` — Tier-0 system  
- `src/lib.rs` — public crate interface  
- `docs/` — specifications, evolution notes, whitepapers  
- `tests/` — required for all PRs  

Do **not** create new modules lightly.

---

## 🧪 Testing

Run full test suite:

```bash
cargo test
```

All PRs must:
- Add tests for new logic
- Not break existing tests
- Not introduce nondeterminism unless justified

---

## 📝 Documentation

Every merged change must include:
- Inline Rustdoc comments
- Updates to `docs/` if the architecture is affected

Minimalism preferred. Precision required.

---

## 🔍 Review Expectations

PRs will be evaluated on:

- **Correctness**
- **Alignment with architecture**
- **Clarity of reasoning**
- **Security implications**
- **Test coverage**

Vague or speculative PRs will be closed.

---

## 🛡 Philosophy

Project Daneel adheres to the principle:

> *"Bones, not chains."*  
> Structure should enable ethical behavior — not constrain it through hacks.

Contributions must strengthen the structure, not bolt logic onto the side.

---

## 🆘 Questions?

Open a GitHub Issue with:
- The problem
- Expected behavior
- Architectural impact analysis

Do **not** open PRs without prior discussion for structural changes.

---

Thank you for contributing with rigor and respect for the system’s foundations.
