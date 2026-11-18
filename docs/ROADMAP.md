# Roadmap

The goal is reliable setup and recovery that opens **vanilla Minecraft**.
These are acceptance gates, not promised dates or platform guarantees.

| Next step | Done when |
| --- | --- |
| Qualify source setup | Follow the [fresh-install recipe](CLEAN_INSTALL.md) on an independently recorded Apple Silicon Mac; record prerequisites, exact source, setup/download/retry and save/reopen. A same-Mac smoke test or unbound M1 Air report alone does not close this gate. |
| Verify the compiled party fix | The exact rebuilt WinHTTP artifact passes Minecraft party creation/leave/recreate, invite reception, party joining and friend-world joining. The temporary in-game correction and isolated regression tests are supporting evidence. |
| Qualify a distributable app | A specific app passes first-run, update/recovery, save/restore, multiplayer and extended-session checks; corresponding source/notices, signing, notarization and downloaded-app checks are complete. |
| Establish an OSS Windows 11 ARM path | Verify an official ARM64 Bedrock client and an OSS-only macOS VM/graphics stack; preserve macOS 27 Wine unless the entire vanilla game path passes acceptance. |

The [source preview](SOURCE_PREVIEW.md) gives contributors a versioned baseline.
[Release readiness](RELEASE.md) tracks binary gates; [compatibility](COMPATIBILITY.md)
separates source behavior, historical gameplay and current-candidate evidence.
Mullion ray tracing and native ARM game execution remain separate research tracks.

## macOS 28 Windows ARM migration

This migration is limited to macOS and the native Windows ARM Bedrock client.
All launcher, VM, translation, graphics, and integration components must be
open source; the proprietary Minecraft client is the sole exception. No
supporting VM code is implemented because a complete, validated OSS graphics
path has not been established.

Next gates:

1. Verify with Microsoft whether the user's owned game is available as a
   native ARM64 Windows package. The current repository downloader explicitly
   accepts an AMD64 executable; x64 emulation is not a substitute for this
   scope.
2. Identify an OSS-only macOS ARM64 VM, Windows guest graphics, and DirectX 11
   path. Do not use proprietary virtualization, translation, graphics, or
   integration components.
3. If those prerequisites pass, build only a minimal VM/game launch handoff
   for Windows 11 ARM and the unmodified vanilla client. Preserve macOS 27
   Wine behavior and user data; do not add visual modifications or Linux
   support.
4. Require a real Apple Silicon macOS 28 test of ownership/install, native ARM
   client execution, DirectX 11 graphics, save/reopen, VM lifecycle and clean
   setup before integrating or claiming support.

Until these gates pass, no VM adapter is claimed and macOS 28 support remains
unverified. See the
[compatibility evidence and blockers](COMPATIBILITY.md#macos-28-and-windows-11-arm).
