# ewt-rs

Emacs Tokenizer tokenizing CJK words with WinRT API or ICU.

EWT stands for Emacs Windows Tokenizer. But it works on all platforms, if built with [ICU](https://github.com/unicode-org/icu4x).

## Installation

This crate provides dynamic module which [emt.el](https://github.com/roife/emt) consumes. Install emt.el first, put the module dynamic lib into `emt-lib-path` (by default located at `~/.emacs.d/modules/libEMT.{dll,so,etc}`).

### Pre-built

Download from Releases, or [CI Artifact](https://github.com/Master-Hash/ewt-rs/actions/workflows/build.yml) for unversioned binaries.

<!-- Please ignore the binary from Releases page. -->

The Windows `.dll` files are only compatible with Emacs built with UCRT. MSVCRT is not supported.

### Manually build

1. Install Rust toolchain (On Windows, please target x86_64-pc-windows-gnu)
2. (On Windows) Install MSYS2
3. `cargo build --release` to use ICU
4. `cargo build --release --no-default-features -F windows` to use WinRT API

## Adjustment

The segmenter language with WinRT API is hardcoded. Users can adjust `zh-CN` to the favoured language.

## C vs C++ vs Rust

Microsoft doesn't and will never provide WinRT API for C.

C++ 20 is required for cppwinrt. I encounter auto type deduction error in the cppwinrt header file, which I cannot fix. The size could be much smaller (~100k?) though, if it works, it's favourable.

I have to use unsafe extern "C" all the way to write Rust binding. The safety no better than C++, but it has better WinRT API support and type inference. When built with lto, the size ~260K is acceptable.

## WinRT API vs ICU

Personally I recommand WinRT API for Simplified Chinese and ICU for Traditional Chinese.

| WinRT API | ICU |
|-------|-------|
| '有\|异曲同工\|之\|妙' | '有异\|曲\|同工\|之\|妙' |
| '有\|異\|曲\|同工\|之\|妙' | '有\|異曲同工\|之\|妙' |
| '丧心病狂\|的\|异想天开' | '丧心病狂\|的\|异\|想\|天\|开' |

## Note on UTF-8 Grapheme Cluster

This crate handles String on char level instead of grapheme cluster level. However, this causes no problem, probally because emt.el only use the helper function when moving in CJK characters.

## Future Work

- [x] Try ICU Backend
- [x] Find out why M-S-{F,B} doesn't select anything

## Credit

* emt.el
* [ubolonton/emacs-module-rs](https://github.com/ubolonton/emacs-module-rs) I don't use it because of [issue](https://github.com/ubolonton/emacs-module-rs/issues/60), but it helps me learn how Emacs Dynamic Module works, and provides useful functions.
* Article: [Writing an Emacs module in Rust](https://ryanfaulhaber.com/posts/first-emacs-module-rust/)
