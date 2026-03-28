# Windows LTSC Final Setup Report

## Current State

- the repository now uses one setup script only: `Scripts\00_QuickSetup.ps1`
- all previously separate bootstrap, repair, audit, and install helpers were merged into that script
- the repository is now a rebuild backup, not a multi-script toolkit

## Main Capabilities

- repairs HTTPS, TLS, DNS, Winsock, and TCP settings
- installs Microsoft Store, Winget dependencies, Winget, Scoop, and Chocolatey
- repairs Microsoft Store Start menu visibility
- restores common LTSC UWP apps and installs practical alternatives
- installs core applications and optional developer stacks
- installs PowerShell 7, Rust, Cargo tools, npm globals, pip packages, and `uv`
- applies common LTSC registry tweaks
- emits a final component audit summary to the log

## Operational Notes

- run in an elevated PowerShell window
- reboot after completion
- review the generated `Logs\setup_YYYYMMDD_HHMMSS.log`
- use `-SkipDevTools` if you want a leaner baseline
- use `-NetworkMode Basic|Optimized|Extreme` to control network tuning intensity

## Repository Intent

This repository is now optimized for one-shot restoration after reinstalling Windows LTSC on the same machine class.

Last Updated: 2026-03-29
