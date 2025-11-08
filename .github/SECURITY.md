# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

The Prime Physics Engine team takes security seriously. We appreciate your efforts to responsibly disclose your findings.

### Where to Report

Please report security vulnerabilities by emailing: security@primephysics.dev

Alternatively, you can report via GitHub Security Advisories: https://github.com/mikepurvis/prime-physics-engine/security/advisories/new

### What to Include

- Type of vulnerability (e.g., memory safety, data exposure, DoS)
- Affected components or files
- Steps to reproduce the vulnerability
- Potential impact
- Any suggested fixes or mitigations

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Resolution Target**: Within 30 days for critical issues

### Disclosure Policy

- We follow a 90-day disclosure timeline
- We will credit researchers who report valid vulnerabilities (unless anonymity is requested)
- Please allow us time to fix the issue before public disclosure

## Security Best Practices

When using Prime Physics Engine:

1. **Input Validation**: Always validate membrane configuration parameters
2. **Resource Limits**: Set appropriate limits for computation time and memory
3. **WASM Deployment**: Use subresource integrity (SRI) when loading WASM modules
4. **GPU Resources**: Monitor GPU memory usage to prevent exhaustion

## Known Security Considerations

- Large prime computations can be resource-intensive
- GPU implementations require appropriate sandboxing
- WASM bindings should be used in secure contexts only

Thank you for helping keep Prime Physics Engine secure!