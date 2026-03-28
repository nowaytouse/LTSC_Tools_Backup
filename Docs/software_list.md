# LTSC Software Coverage

## Included By Default

The main script already installs these baseline desktop applications:

- `7zip.7zip`
- `VideoLAN.VLC`
- `Google.Chrome`
- `Notepad++.Notepad++`
- `ShareX.ShareX`
- `IrfanSkiljan.IrfanView`

## Included With Developer Mode Enabled

Without `-SkipDevTools`, the script also installs:

- desktop tools: `Bitwarden.CLI`, `LocalSend.LocalSend`, `GnuPG.Gpg4win`, `Microsoft.OpenJDK.21`, `EFF.Certbot`
- Scoop packages for git, runtimes, media tooling, compression tools, OCR, backup, and CLI utilities
- Rust and Cargo packages
- npm global packages
- pip packages
- `uv` plus `kimi-cli`

## Not Automatically Included

These are still reasonable manual add-ons depending on preference:

- Firefox
- VS Code
- PowerToys
- Obsidian
- SumatraPDF
- Wireshark
- RustDesk

Last Updated: 2026-03-29
