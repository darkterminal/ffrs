# Security Policy

## Supported versions

Maintenance happens on `main`. Prebuilt release binaries are not currently published, so install from crates.io or from source and report issues against the current `main`.

| Version | Supported |
| --- | --- |
| 0.1.x (`main`) | yes |
| earlier than 0.1 | no |

## Reporting a vulnerability

Do not open a public issue for a security problem. Use GitHub's private vulnerability reporting for this repository (**Security → Report a vulnerability**), or email `darkterminal@duck.com` with:

- the affected version or commit,
- steps to reproduce,
- the potential impact,
- any suggested fix.

You should receive an acknowledgement within a week. We will coordinate the fix and disclosure with you before publishing details.

## Scope

This policy covers CLI argument handling, path validation, command generation, and the library API in this repository. Vulnerabilities in ffmpeg itself should be reported upstream at https://ffmpeg.org/security.html.
