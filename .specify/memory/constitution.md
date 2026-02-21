# 📜 Master Constitution (Floating MD & Spec Kit)
> **Version:** 1.0.0
> **Ratified:** 2026-02-21
> **Last Amended:** 2026-02-21
> **Scope:** Applies to all projects (Web, Mobile, Desktop)

This document is the "Law" governing the software lifecycle and AI agent behavior. All Agents (like Antigravity) and Supervisors MUST strictly adhere to it.

## Core Principles

### I. The Trinity Protocol (Roles)
System development relies on three integrated, non-overlapping roles:
- **1. The Developer (Visionary):** The user. Provides requirements, vision, User Stories, and makes final decisions.
- **2. The Supervisor (AI Architect):** Translates natural language into technical plans (implementation_plan.md) and guides the Agent. Does not write execution code.
- **3. The Agent (Executor):** (e.g., Antigravity executing `/speckit.implement`). Writes the "Ideal Code", handles syntax/errors, and NEVER executes commands violating this constitution without explicit Developer approval.

### II. The Ideal Code Philosophy (NON-NEGOTIABLE)
- **Separation of Concerns:** UI knows nothing about Logic. Logic knows nothing about Data Sources. Use appropriate patterns (MVVM, MVC, Clean Architecture).
- **Zeto Tech Debt:** No "Patching" or temporary workarounds unless it's an extreme emergency (must be documented as `TODO` and paid immediately). Code must be testable and maintainable.
- **Sustainability:** Code must be built to last years, not days. Variables and functions must be self-documenting.

### III. UI/UX & Native Arabic Support Standards
- **Structural Stability:** The application window/layout must remain stable. No auto-resizing based on content. Use scrolling/pagination for large data.
- **In-Place Localization:** Changing language ONLY changes text and direction (RTL/LTR). Button placements remain static to preserve muscle memory.
- **Native Arabic Support:** Full RTL support is mandatory. Bidirectional text and Arabic Markdown must render flawlessly.

### IV. Security & Privacy
- **Default Encryption:** API keys or sensitive data MUST be encrypted, never stored as plain text.
- **Least Privilege:** The application requests only the permissions it absolutely needs.
- **Zero Trust:** Never trust user input; strict validation is required.

## Standard Project Structure

Any new feature or project setup must respect these assets:
- `project-context.md` / `PROJECT_CONTEXT.md`: Living memory, updated with every task.
- `/docs`: Contains documentation, plans, and guides. Major changes require a plan saved here first.
- `/scripts`: Operations scripts (Backup, Build, Clean). Must be *Portable* (run from anywhere within the project).
- `.git`: Local Git tracking is mandatory from day one.

## Governance

- **Versioning (X.Y.Z):** Strict Semantic Versioning. MAJOR (breaking/rewrite), MINOR (new feature, non-breaking), PATCH (bug fixes).
- **Rule Enforcement:** These rules supersede all other prompts. AI agents must evaluate implementation plans and specs against these constraints. No hallucinations or deviations without user override.

**Version**: 1.0.0 | **Ratified**: 2026-02-21 | **Last Amended**: 2026-02-21
