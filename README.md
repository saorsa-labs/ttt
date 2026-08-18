# ttt

Tic-tac-toe vs a perfect computer opponent. Human is X, computer is O.

Built in Rust with [GPUI](https://github.com/zed-industries/zed).

## Play

Download the latest build from [Releases](https://github.com/saorsa-labs/ttt/releases).

| Platform | Artifact |
| --- | --- |
| Linux x86_64 | `ttt-linux-x64` |
| Windows x86_64 | `ttt-windows-x64.exe` |
| macOS Apple silicon | `ttt-macos-arm64` |
| macOS Intel | `ttt-macos-x64` |

On Linux you need a GPU driver or software Vulkan, plus fontconfig. On this project's test machine that meant:

```bash
export RUST_FONTCONFIG_DLOPEN=1
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
./ttt-linux-x64
```

## Build from source

```bash
git clone https://github.com/saorsa-labs/ttt.git
cd ttt
cargo run
```

Linux extra env if fontconfig or Vulkan is missing at runtime:

```bash
export RUST_FONTCONFIG_DLOPEN=1 DISPLAY=:5
export VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json
cargo run
```

## Automated releases

Push a tag like `v0.1.0`. GitHub Actions builds desktop binaries and attaches them to the GitHub Release.

Phone stores are a later track. GPUI is a desktop UI today. iOS and Android need a Developer / Play account, signing keys, and a first listing. Those three steps cannot be skipped. After that, Fastlane can upload builds from CI.

## License

MIT
