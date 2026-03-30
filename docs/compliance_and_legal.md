# Compliance and Legal

## Regulatory Boundaries

This repository is an engineering reference platform, not a market-approved receiver product. Regulatory obligations for commercial deployment (radio, EMC, product safety, and local approvals) must be handled by the integrating organization.

## EMC/EMI and Productization Considerations

- RF front-end and high-speed digital interfaces require controlled layout, shielding strategy, and pre-compliance testing.
- Reference design guidance in this repository is concept-level and intended for prototype/lab progression.
- Formal EMC pre-scan and certified lab testing are required before product release.

## Electrical Safety Considerations

- Use bench supplies with current limiting during bring-up.
- Include input protection and fuse strategy in hardware implementation.
- Validate thermal rise and regulator margins in prototype testing.
- Follow local electrical safety standards for any mains-connected accessories.

## Conditional Access Limitations

Conditional access and protected broadcast reception are not implemented here. Any protected service access requires licensed, compliant modules and agreements in the appropriate ecosystem.

## B-CAS/ACAS Non-Circumvention Statement

This repository does not provide and must not be used to develop circumvention of B-CAS, ACAS, encryption, or protected broadcast access controls. Contributions that attempt this are explicitly out of policy.

## What This Repository Intentionally Does Not Implement

- Unauthorized decryption or key handling for protected streams.
- Reverse-engineering workflows targeting protected-access bypass.
- Instructions to alter ecosystem trust boundaries or licensed module behavior.

## What Requires Licensing, Approvals, or Legal Review

- Integration with commercial protected broadcast networks.
- Use of certified conditional-access subsystems.
- Any redistribution path that carries protected broadcast content.
- Commercial sale or field deployment of receiver hardware.
- Claims of standards compliance or regulated-market conformance.
