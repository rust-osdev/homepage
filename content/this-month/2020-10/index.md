+++
title = "This Month in Rust OSDev (October 2020)"
date = 2020-11-07

[extra]
month = "October 2020"
authors = [
    "phil-opp",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we will give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new).

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (October 2020)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

### [`x86_64`](https://github.com/rust-osdev/x86_64)

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

In October, we merged following changes:

- [Provide functions for accessing the underlying L4 table for mapper types](https://github.com/rust-osdev/x86_64/pull/184)
- [Make `GlobalDescriptorTable::add_entry` a const fn](https://github.com/rust-osdev/x86_64/pull/191)
- [Update docs to suggest `TryFrom` trait](https://github.com/rust-osdev/x86_64/pull/198)

Thanks to [@toku-sa-n](https://github.com/toku-sa-n) for their contribution! We plan to publish the above changes as [version `0.12.3`](https://github.com/rust-osdev/x86_64/pull/200) in the next few days.

### [`volatile`](https://github.com/rust-osdev/volatile)

The `volatile` crate provides a safe wrapper type for implementing volatile read and write operations. This is useful for accessing memory regions that have side-effects, such as memory-mapped hardware registers or framebuffers.

In October, we published a new version to fix the crate's `unstable` feature on newer Rust nightlies:

- [Change `slice::check_range` to `RangeBounds::assert_len`](https://github.com/rust-osdev/volatile/pull/16) <span class="gray">(published as `v0.4.2`)</span>

Thanks to [@vetio](https://github.com/vetio) for this contribution!


### [`bootloader`](https://github.com/rust-osdev/bootloader)

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we published versions `0.9.9` to `0.9.11` to fix build errors on the latest nightlies, caused by the new feature gate names for some `const fn` features.

We we didn't merge any changes to the `master` branch this month, we made more progress on the rewrite that adds UEFI support: There is now a [draft pull request](https://github.com/rust-osdev/bootloader/pull/130) that tracks the remaining issues.

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. In October, we merged the following changes:

- [Made `panic_handler` optional](https://github.com/rust-osdev/uefi-rs/pull/179)
- [Fix Clippy lints](https://github.com/rust-osdev/uefi-rs/pull/180)

Thanks to [@Swampman08](https://github.com/Swampman08) for their contribution!


### [`pci_types`](https://github.com/rust-osdev/pci_types)

The `pci_types` library provides types for accessing and configuring PCI devices from Rust operating systems. Lots of this code (e.g. identifying devices by class codes) can be shared
between projects, and would benefit from community contributions.

This month, we published version `0.2.0` with the following changes:

- [Untie accessor from the actual PciHeader type](https://github.com/rust-osdev/pci_types/commit/e1201d7d8986ff1133e2880b0ba62a3b2d7d891b)
- [Split out endpoint header to separate struct](https://github.com/rust-osdev/pci_types/commit/d9cd5809148084d31fe5cc6ddbb5c8129bf23dae)
- [Provide method for accessing BARs on endpoint headers](https://github.com/rust-osdev/pci_types/commit/aeb1b249cf6e4563b815011f7ed759198b283405)


### [`cargo-xbuild`](https://github.com/rust-osdev/cargo-xbuild)

The `cargo-xbuild` project provides `cargo` command wrappers to cross-compile the sysroot crates `core` and `alloc`. This month, we merged the following changes:

- [Document `build-std-features` flag for Cargo's `build-std` feature](https://github.com/rust-osdev/cargo-xbuild/pull/95)
- [Upgrade the crate to edition 2018](https://github.com/rust-osdev/cargo-xbuild/pull/97)  <span class="gray">(published as `v0.6.3`)</span>

Thanks to [@luqmana](https://github.com/luqmana) and [@koushiro](https://github.com/koushiro) for these contributions!

Even though we still maintain the `cargo-xbuild` crate, we recommend switching to cargo's own `build-std` feature that is always up-to-date with the latest Rust/Cargo changes. We wrote a short guide on how to switch to it, which is available [in our Readme](https://github.com/rust-osdev/cargo-xbuild#alternative-the-build-std-feature-of-cargo).

## Personal Projects

In this section, we describe updates to personal projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`rust-embedded/rust-raspberrypi-OS-tutorials`](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)

<span class="gray">(Section written by [@andre-richter](https://github.com/andre-richter))</span>

The [_Operating System development tutorials in Rust on the Raspberry Pi_](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials) project now provides [Tutorial 15 - `Virtual Memory Part 2: MMIO Remap`](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/tree/master/15_virtual_mem_part2_mmio_remap).

It introduces a first set of changes which are eventually needed for separating `kernel` and `user` address spaces:
- The memory mapping strategy gets more sophisticated and no longer `identity maps` the _whole_ of the board's address space.
- Instead, only ranges that are actually needed are mapped:
    - The `kernel binary` stays `identity mapped` for now.
    - Device `MMIO regions` are remapped lazily to a special virtual address region at the top of the virtual address space during the device driver's `init()`.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

This month, the _Writing an OS in Rust_ series received the following updates:

- [Remove rlibc and use compiler-builtins-mem feature](https://github.com/phil-opp/blog_os/pull/865)
  - [Update `post-04` to use compiler_builtins `mem` feature instead of `rlibc`](https://github.com/phil-opp/blog_os/pull/868)
  - [Update blog to use `mem` feature of `compiler_builtins`](https://github.com/phil-opp/blog_os/pull/866)
- [Translate post 02 to Japanese](https://github.com/phil-opp/blog_os/pull/871)
  - The translated post is now [published](https://os.phil-opp.com/ja/minimal-rust-kernel/).
- [Add Right-to-Left support for template](https://github.com/phil-opp/blog_os/pull/875) in preparation for a Persian translation

See [the merged pull request list](https://github.com/phil-opp/blog_os/pulls?q=is%3Apr+is%3Aclosed+merged%3A2020-10-01..2020-11-01) for the complete set of changes this month. Thanks a lot to all contributors!

In case you speak Persian: There is currently an open pull request to [add a Persian translation of `Bare Bones` chapter](https://github.com/phil-opp/blog_os/pull/878) that needs reviews. Thanks to everyone involved!

In [in our previous status update](@/this-month/2020-09/index.md#phil-opp-blog-os) I described my plans to rewrite the blog on top of the upcoming UEFI bootloader. In the past month I started rewriting the _Minimal Rust Kernel_ post for this. Unfortunately, I'm still facing build-related issues because of [limitations of Cargo's config files](https://github.com/rust-lang/cargo/pull/8757#issuecomment-713897532). To resolve these (and other) `.cargo/config` issues, I created a proposal on the Rust internals forum to [make some `.cargo/config` options available in `Cargo.toml` too](https://internals.rust-lang.org/t/proposal-move-some-cargo-config-settings-to-cargo-toml/13336). Feel free to join the discussion if it's relevant to you!

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
