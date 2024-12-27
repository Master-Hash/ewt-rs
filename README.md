# ewt-rs

Emacs Tokenizer tokenizing CJK words with WinRT API or ICU (to be done).

EWT stands for Emacs Windows Tokenizer. But if built with [ICU](https://github.com/unicode-org/icu4x), it should be working on all platforms.

## Installation

This crate provides dynamic module which [emt.el](https://github.com/roife/emt) consumes. Install emt.el first, put the module dynamic lib into `emt-lib-path` (by default located at `~/.emacs.d/modules/libEMT.{dll,so,etc}`).

### Pre-built

Be aware, I use radical compile-time arguments, so I don't guarantee any compatibility.

My env:

* MSYS2 UCRT Emacs 29.4
* CPU >= with avx2 (or x86-64-v3)
* Windows 11 build 26100

### Manually build

1. Install Rust toolchain (On Windows, please target x86_64-pc-windows-gnu)
2. (On Windows) Install MSYS2
3. `cargo build --target=x86_64-pc-windows-gnu --release`

## Adjustment

The segmenter language is hardcoded. Users can adjust `zh-CN` to the favoured language.

## WinRT API vs ICU

Personally I prefer the result of WinRT API. ICU does much poorer when segmenting idioms, showing a lack of vocabulary:

| WinRT API | ICU |
|-------|-------|
| '有\|异曲同工\|之\|妙' | '有异\|曲\|同工\|之\|妙' |
| '丧心病狂\|的\|异想天开' | '丧心病狂\|的\|异\|想\|天\|开' |

## Future Work

* Try ICU Backend
* Find out why M-S-{F,B} doesn't select anything

## Credit

* emt.el
* [ubolonton/emacs-module-rs](https://github.com/ubolonton/emacs-module-rs) I don't use it because of [issue](https://github.com/ubolonton/emacs-module-rs/issues/60), but it helps me learn how Emacs Dynamic Module works, and provides useful functions.
* Article: [Writing an Emacs module in Rust](https://ryanfaulhaber.com/posts/first-emacs-module-rust/)
