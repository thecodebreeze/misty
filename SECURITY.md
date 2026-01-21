# Security Policy

## Supported Versions

This project is under active development. Security fixes are provided for:

- `main` (latest commit)
- The latest published release (if/when releases are published)

If you are using an older revision, please try to reproduce the issue on `main` before reporting.

## Reporting a Vulnerability

Please **do not** open a public GitHub issue for security vulnerabilities.

Instead, report security issues via one of the following private channels:

- **GitHub Security Advisories (preferred):**  
  Go to the repository page → **Security** → **Advisories** → **New draft security advisory**.
- **Email:** `<security@asagi.fi>`  
  (Replace this placeholder with a real monitored address.)

If you are unsure whether an issue is security-related, it’s fine to report it through the security channel anyway.

### What to Include

To help us investigate quickly, please include:

- A clear description of the vulnerability and its impact
- Affected component(s) and crate(s) (e.g., `misty-net`, `misty-parser`, `misty-compiler`)
- Steps to reproduce (proof-of-concept code is welcome)
- Expected vs actual behavior
- Your environment (OS, Rust version, relevant flags)
- Any known mitigations or workarounds

If the report is about dependency vulnerabilities, please include:

- The dependency name and version
- The advisory reference (e.g., RustSec ID) if available
- How it is reached/used (direct vs transitive)

### Coordinated Disclosure

We ask that you practice **coordinated vulnerability disclosure**:

- Please keep details private until we have investigated and released a fix (or confirmed it is not a vulnerability).
- We will coordinate a disclosure timeline with you when possible.
- We will credit reporters in release notes/advisories if they want (optional).

## Response and Fix Timeline (Best Effort)

This project is maintained on a best-effort basis. Typical targets:

- **Initial response:** within **3 business days**
- **Triage / assessment:** within **7 business days**
- **Fix and release:** as soon as practical, depending on severity and complexity

If you have a strict deadline (e.g., conference talk, publication), please tell us up front so we can coordinate.

## Severity Guidelines

We generally prioritize issues that enable:

- Remote code execution
- Unauthorized access to secrets/credentials
- Memory safety issues with security impact
- Sandbox escapes / privilege escalation
- Network protocol flaws enabling spoofing, tampering, or data exfiltration
- Supply chain risks (malicious packages, compromised releases)

Lower priority (but still welcome) includes:

- Denial of service in non-default configurations
- Non-exploitable crashes without security impact

## Secure Development Notes (Project-Specific)

- This repository is a Rust workspace containing multiple crates.
- Please prefer reporting issues with clear crate/module scope where possible.
- If you suspect unsafe code is involved, note where you observed it (file/module/function names are helpful).

## Public Communication

For non-sensitive security hardening suggestions (e.g., “add fuzzing” / “enable a lint”), feel free to open a normal
issue or PR.

For anything that could plausibly be exploited, use the private reporting process above.

## Acknowledgements

Thank you for helping keep the project and its users safe.