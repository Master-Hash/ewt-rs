# ewt-rs

Emacs Tokenizer tokenizing CJK words with WinRT API or ICU on all platforms, including Windows, MacOS and Linux.

## Installation

This crate provides dynamic module which [emt.el](https://github.com/roife/emt) consumes. Install emt.el first, put the module dynamic lib into `emt-lib-path` (by default located at `~/.emacs.d/modules/libEMT.{dll,so,etc}`).

### Pre-built

| Architecture \ OS | Windows                                                                                                                                                                                                                                  | GNU / Linux                                                                                                                                                                                                                                                                                                                                                                                  | MacOS                                                                                                                        |
|-------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------|
| x86_64            | [WinRT](https://github.com/Master-Hash/ewt-rs/releases/latest/download/libewt-windows-x86_64-pc-windows-msvc.zip), [ICU](https://github.com/Master-Hash/ewt-rs/releases/latest/download/libewt-windows-icu-x86_64-pc-windows-msvc.zip)   | ICU ([70](https://github.com/Master-Hash/ewt-rs/releases/latest/download/libewt-rust_icu_ubrk-x86_64-unknown-linux-gnu-70.zip), [74](https://github.com/Master-Hash/ewt-rs/releases/latest/download/libewt-rust_icu_ubrk-x86_64-unknown-linux-gnu.zip), [static](https://github.com/Master-Hash/ewt-rs/releases/latest/download/libewt-icu_segmenter-x86_64-unknown-linux-gnu.zip))          | ICU ([static](https://github.com/Master-Hash/ewt-rs/releases/latest/download/libewt-icu_segmenter-x86_64-apple-darwin.zip )) |
| AArch64           | [WinRT](https://github.com/Master-Hash/ewt-rs/releases/latest/download/libewt-windows-aarch64-pc-windows-msvc.zip), [ICU](https://github.com/Master-Hash/ewt-rs/releases/latest/download/libewt-windows-icu-aarch64-pc-windows-msvc.zip) | ICU ([70](https://github.com/Master-Hash/ewt-rs/releases/latest/download/libewt-rust_icu_ubrk-aarch64-unknown-linux-gnu-70.zip), [74](https://github.com/Master-Hash/ewt-rs/releases/latest/download/libewt-rust_icu_ubrk-aarch64-unknown-linux-gnu.zip), [static](https://github.com/Master-Hash/ewt-rs/releases/latest/download/libewt-icu_segmenter-aarch64-unknown-linux-gnu.zip))       | ICU ([static](https://github.com/Master-Hash/ewt-rs/releases/latest/download/libewt-icu_segmenter-aarch64-apple-darwin.zip)) |
| RISC-V 64         |                                                                                                                                                                                                                                          | ICU ([70](https://github.com/Master-Hash/ewt-rs/releases/latest/download/libewt-rust_icu_ubrk-riscv64gc-unknown-linux-gnu-70.zip), [74](https://github.com/Master-Hash/ewt-rs/releases/latest/download/libewt-rust_icu_ubrk-riscv64gc-unknown-linux-gnu.zip), [static](https://github.com/Master-Hash/ewt-rs/releases/latest/download/libewt-icu_segmenter-riscv64gc-unknown-linux-gnu.zip)) |                                                                                                                              |

<a href="https://repology.org/project/icu/versions">
    <img src="https://repology.org/badge/vertical-allrepos/icu.svg?header=ICU Packaging status" alt="Packaging status" align="right">
</a>

Note:

* For Linux user, check the ICU version on your system first. A quick reference is on the table to the right. If I didn't pre-build for your system, please use static version, or build it yourself.
* Not all feature combination is listed above, but most of the users would be content about them. Find all from [CI Artifact](https://github.com/Master-Hash/ewt-rs/actions/workflows/build.yml).
* Find MacOS module with Foundation backend from [emt](https://github.com/roife/emt/releases)

<!-- Please ignore the binary from Releases page. -->

### Manually build

* `cargo build --release --no-default-features -F icu_segmenter`: ICU4X (static)
* `cargo build --release --no-default-features -F rust_icu_ubrk`: ICU4C (system / MSYS2 on Windows)
* `cargo build --release --no-default-features -F windows`: WinRT
* `cargo build --release --no-default-features -F windows-icu` ICU4C (system)

For build dependencies and environment, you may refer to the [CI script](https://github.com/Master-Hash/ewt-rs/blob/main/.github/workflows/build.yml).

## Hardcoded

The segmenter language with WinRT API is hardcoded. Users can adjust `zh-CN` to the favoured language.

<!-- ## C vs C++ vs Rust

Microsoft doesn't and will never provide WinRT API for C.

C++ 20 is required for cppwinrt. I encounter auto type deduction error in the cppwinrt header file, which I cannot fix. The size could be much smaller (~100k?) though, if it works, it's favourable.

I have to use unsafe extern "C" all the way to write Rust binding. The safety no better than C++, but it has better WinRT API support and type inference. When built with lto, the size ~260K is acceptable. -->

## WinRT API vs ICU

WinRT is best for Simplified Chinese users, and ICU is best for Traditional Chinese users.

Testing command:

* `cargo test --no-default-features -F windows --lib -- --nocapture`
* `cargo test --no-default-features -F windows-icu --lib -- --nocapture`

| WinRT API           | ICU                   |
|---------------------|-----------------------|
| '有\|异曲同工\|之\|妙'     | '有异\|曲\|同工\|之\|妙'     |
| '有\|異\|曲\|同工\|之\|妙' | '有\|異曲同工\|之\|妙'       |
| '丧心病狂\|的\|异想天开'     | '丧心病狂\|的\|异\|想\|天\|开' |

<!-- ## Note on UTF-8 Grapheme Cluster

This crate handles String on char level instead of grapheme cluster level. However, this causes no problem, probably because emt.el only use the helper function when moving in CJK characters. -->

## Future Work

- [x] Try ICU Backend
- [x] Find out why M-S-{F,B} doesn't select anything
- [x] Link against system icu
- [ ] Stop linking against libunwind.dll

## Credit

* emt.el
* [ubolonton/emacs-module-rs](https://github.com/ubolonton/emacs-module-rs) I don't use it because of [issue](https://github.com/ubolonton/emacs-module-rs/issues/60), but it helps me learn how Emacs Dynamic Module works, and provides useful functions.
* Article: [Writing an Emacs module in Rust](https://ryanfaulhaber.com/posts/first-emacs-module-rust/)
