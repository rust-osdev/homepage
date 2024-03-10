+++
title = "This Month in Rust OSDev (August 2020)"
date = 2020-09-09

[extra]
month = "August 2020"
authors = [
    "phil-opp",
    "JohnTitor",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we will give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new).

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (August 2020)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

The `uefi-rs` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. In August, the crate received a large number of improvements:

- [Add method to return (image_base, image_size) to LoadedImage](https://github.com/rust-osdev/uefi-rs/pull/149)
- [Rename `image_info` to `info`](https://github.com/rust-osdev/uefi-rs/pull/153)
- [Hide the `MemoryMapIter` type from the API](https://github.com/rust-osdev/uefi-rs/pull/154)
- [Upstream two changes from Pebble's fork](https://github.com/rust-osdev/uefi-rs/pull/156)
- [Update to new `alloc` API](https://github.com/rust-osdev/uefi-rs/pull/157)
- [Publish new versions of the crates](https://github.com/rust-osdev/uefi-rs/pull/158)
- [Make memory map iterator clonable](https://github.com/rust-osdev/uefi-rs/pull/161)
- [Fix the `text::Output::current_mode` method](https://github.com/rust-osdev/uefi-rs/pull/163)
- [Use exact size iterators where possible](https://github.com/rust-osdev/uefi-rs/pull/164)
- [Use `ExactSizeIterator` in `exit_boot_services`](https://github.com/rust-osdev/uefi-rs/pull/165)

Thanks to [@tomoyat1](https://github.com/tomoyat1) for their contribution!

### [`multiboot2`](https://github.com/rust-osdev/multiboot2-elf64)

The `multiboot2` crate provides abstraction types for the boot information of multiboot2 bootloaders. The most important change this month was the update to the latest release of the multiboot2 specification:

- [Add support for multiboot2](https://github.com/rust-osdev/multiboot2-elf64/pull/66)
- [Improve usage of repr(packed)](https://github.com/rust-osdev/multiboot2-elf64/pull/68)

Thanks to [@Caduser2020](https://github.com/Caduser2020) for these contributions and welcome to the `multiboot2` team!

### [`bootloader`](https://github.com/rust-osdev/bootloader)

The [`bootloader` crate](https://github.com/rust-osdev/bootloader) implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. We did not publish any changes to the bootloader crate this month, but we made good progress on the UEFI implementation. See [these issue comments](https://github.com/phil-opp/blog_os/issues/349#issuecomment-677645694) for a detailed status report.

The rewrite of the real mode and protected mode stages of the BIOS bootloader is also [making progress](https://gitter.im/rust-osdev/bootloader?at=5f4594ed9566774dfe3167bc). The goal is to replace as much of the existing assembly code with Rust as possible, in order to make the code more robust and easier to understand.

### [`ansi_rgb`](https://github.com/rust-osdev/ansi_rgb)

The `ansi_rgb` crate implements `no_std`-compatible support for colored terminal text using ANSI escape sequences. The crate is still in an early state, but it received lots of new features this month:

- [More colors and some minor changes](https://github.com/rust-osdev/ansi_rgb/pull/11)
- [Demo using RGB and 3- and 4-bit colors](https://github.com/rust-osdev/ansi_rgb/commit/19891574e3a4df81716973cf4c88ad965596c043)
- [Merge `Foreground` and `Background` traits](https://github.com/rust-osdev/ansi_rgb/commit/ebcb75b4f273fdeaef6fde051ca5dccb83560c13)
- [Add 8-bit colors](https://github.com/rust-osdev/ansi_rgb/commit/d6ed54ae50ab3653f9c94275efc369472c5278a9)
- [Combine `WithForeground` and `WithBackground` into `Colored`](https://github.com/rust-osdev/ansi_rgb/commit/0d77122be5a449a9c03f04421d43df90a7fd708e)
- [Document extension to other color types](https://github.com/rust-osdev/ansi_rgb/commit/9e10f6bac075b980cef66f4ddc9efab25c0a5504)
- [Add Color8 constructors for RGB and gray](https://github.com/rust-osdev/ansi_rgb/pull/12)
- [Fix bounds check for Color8::new_rgb](https://github.com/rust-osdev/ansi_rgb/pull/13)
- [Go back to `WithForeground` and `WithBackground` structs](https://github.com/rust-osdev/ansi_rgb/commit/3eb8c16681878ae97f8249524609f0611d9eddf6)
- [Fix (and document/test) nesting](https://github.com/rust-osdev/ansi_rgb/commit/9dd8d1828b1d0b3b707cb1f22c28074a6ce82ca5)

Thanks to [@hanmertens](https://github.com/hanmertens) for their contributions!

### [`bootimage`](https://github.com/rust-osdev/bootimage)

The `bootimage` tool allows the creation of bootable disk images for `bootloader`-based kernels. It also provides a runner executable for `cargo` to make `cargo run` and `cargo test` work using QEMU. In August, we changed the test behavior to fix a bug where a triple fault is interpreted as a test success. We also fixed a small bug related to the `--version` argument. The relevant pull requests are:

- [Consider all other exit codes besides 'test-success-exit-code' as failures](https://github.com/rust-osdev/bootimage/pull/65) <span class="gray">(published as `v0.10.0`)</span>
    - Also runs tests with -no-reboot by default, configurable through a new test-no-reboot config key
    - This is technically **breaking change**, but it should not affect most people.
- [Parse `--version` argument without subcommand (`bootimage --version`)](https://github.com/rust-osdev/bootimage/pull/67) <span class="gray">(published as `v0.10.1`)</span>

Thanks to [@Freax13](https://github.com/Freax13) for their contribution!

### [`x86_64`](https://github.com/rust-osdev/x86_64)

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

This month, we added some functions for reading and writing the `FS` and `GS` segment base registers:

- [Add rdfsbase, rdgsbase, wrfsbase, wrgsbase](https://github.com/rust-osdev/x86_64/pull/172) <span class="gray">(published as `v0.11.2`)</span>

Thanks to [@haraldh](https://github.com/haraldh) for this contribution!

### [`acpi`](https://github.com/rust-osdev/acpi)

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS. This month, support for more opcodes was added to the AML parser:

- [Factor out value comparison code from DefLEqual](https://github.com/rust-osdev/acpi/commit/438bd9e4cc98bdac29a9eeccf0877592dd70c540)
- [Make all type-2 opcode parsers concrete to avoid type limit](https://github.com/rust-osdev/acpi/commit/28e46b3cd2f68b033ae9559d84e0d8784a527422)
- [Implement some `DefL*` opcodes](https://github.com/rust-osdev/acpi/compare/438bd9e4cc98bdac29a9eeccf0877592dd70c540...8dd5b21cf225e267d9786036ed134a45fc34b5f1)
- [Only read from field if update rule is Preserve](https://github.com/rust-osdev/acpi/commit/e54158ee0128c47acb5e34509e390010ee1feb74)

### [`spinning_top`](https://github.com/rust-osdev/spinning_top)

The `spinning_top` crate provides a simple spinlock implementation based on the abstractions of the [`lock_api`](https://docs.rs/lock_api/0.4.1/lock_api/) crate. This month, we added an optional feature to make the crate compatible with the `owning_ref` crate:

- [Add owning_ref support](https://github.com/rust-osdev/spinning_top/pull/7) <span class="gray">(published as `v0.2.2`)</span>

Thanks to [@not-a-seagull](https://github.com/not-a-seagull) for this contribution!

### [`cargo-xbuild`](https://github.com/rust-osdev/cargo-xbuild)

The `cargo-xbuild` project provides `cargo` command wrappers to cross-compile the sysroot crates `core` and `alloc`. At the beginning of this month, we had to update the crate for the new rustc directory layout. The crate also received a small cleanup:

- [Update cargo-xbuild to new rust directory layout](https://github.com/rust-osdev/cargo-xbuild/pull/87) <span class="gray">(published as `v0.6.0`)</span>
- [Cleanup: Use eprintln! instead of writeln! with stderr](https://github.com/rust-osdev/cargo-xbuild/pull/86)

Thanks to [@toku-sa-n](https://github.com/toku-sa-n) for their contribution!

Even though we still maintain the `cargo-xbuild` crate, we recommend switching to cargo's own `build-std` feature that is always up-to-date with the latest Rust/Cargo changes. We wrote a short guide on how to switch to it, which is available [in our Readme](https://github.com/rust-osdev/cargo-xbuild#alternative-the-build-std-feature-of-cargo).

## Personal Projects

In this section, we describe updates to personal projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

I'm still mostly working on the new bootloader with UEFI support, so there were no major changes to the _Writing an OS in Rust_ series this month. However, I'm making good progress on the bootloader [as noted above](#bootloader) and I hope to finish the rewrite soon.

If all goes well, the new version will no longer require the `bootimage` tool and instead let the users create a simple build script for the bootloader themselves. It will also set up a pixel based framebuffer, which means that we will be able to do display proper graphics instead of just VGA-based text. Unfortunately, I'm currently a bit blocked by limitations of cargo configuration files (our default target should not apply to our build script), but I hope that we can [find a solution for this](https://github.com/rust-lang/cargo/issues/8687) soon.

While I focused most of my time on the bootloader and the cargo config files, I also merged a few few minor improvements for my blog:

- [Change `rustup override add` to `rustup override set`](https://github.com/phil-opp/blog_os/pull/843)
- [Add (initial) Japanese translation](https://github.com/phil-opp/blog_os/pull/845) (see below)
- [Increase double fault stack size in Double Faults post](https://github.com/phil-opp/blog_os/commit/0425bd3c819bd26910c4e82a7a24c2a5126d4116)
    - The previous stack size was too small in debug mode since our double fault handler uses the rather stack-intensive `core::fmt` functions.
    - Since the dobule fault stack has no guard page, this stack overflow [resulted in undefined behavior](https://github.com/phil-opp/blog_os/issues/449#issuecomment-667638975). For example, it caused the system to hang or lead to a triple fault.
    - The stack size increase only works around this problem, but is not a clear fix. I plan to introduce a proper stack allocation function in the upcoming post about threading, which will create stacks with a proper guard page so that stack overflows deterministically lead to a page fault.
- [Use workflow dispatch event to trigger scheduled builds of code branches](https://github.com/phil-opp/blog_os/pull/846)
    - Enables nightly builds of all `post-XX` branches, so that we can be sure that our code always builds with the latest Rust nightly versions.
- [Update Zola to 0.11.0](https://github.com/phil-opp/blog_os/pull/850)

Thanks to [@RWOverdijk](https://github.com/RWOverdijk) and [@JohnTitor](https://github.com/JohnTitor) for their contributions!

#### Japanese translations

<span class="gray">(Section written by [@JohnTitor](https://github.com/JohnTitor))</span>

This month, we also [added a Japanese translation](https://github.com/phil-opp/blog_os/pull/845) newly. You can find the languages list in the sidebar and Japanese if you enable that language in your browser.
Only one post is currently translated, but we'd like to translate more posts in the future.
Want to participate in the translation or find a typo? Feel free to open a PR/issue on [the repository](https://github.com/phil-opp/blog_os)
(please use English in the description if possible)!

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
