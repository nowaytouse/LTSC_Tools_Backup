# Windows LTSC Setup Notes

## Summary

The setup flow has been fully collapsed into `Scripts\00_QuickSetup.ps1`.

## Why The Script Exists

LTSC installs commonly miss:

- Microsoft Store
- Winget
- common UWP utilities
- PowerShell 7
- optional developer features
- a consistent package manager baseline

## Embedded Strategy

The script now performs this sequence internally:

1. repair TLS, DNS, Winsock, proxy, and TCP settings
2. bootstrap Store, Winget dependencies, Winget, Scoop, and Chocolatey
3. repair Store registration and shell visibility
4. restore LTSC apps and install fallback desktop alternatives
5. install common desktop applications
6. install developer tooling when `-SkipDevTools` is not used
7. install PowerShell 7
8. apply LTSC registry tweaks
9. write a final component audit summary

## Practical Notes

- `Start-BitsTransfer` is used for Winget dependency downloads because it is more resilient on LTSC
- `winget` remains the primary application installer
- practical alternatives such as ShareX, IrfanView, and Paint.NET are included because some UWP apps remain unreliable on LTSC
- a reboot is recommended after the run

## Key Parameters

- `-SkipDevTools`
- `-SkipOptionalFeatures`
- `-SkipSystemTweaks`
- `-NetworkMode Basic|Optimized|Extreme`

Last Updated: 2026-03-29
