+++
title = "This Month in Rust OSDev (January 2022)"
date = 0000-01-01

[extra]
month = "January 2022"
authors = [
    "phil-opp",
    "ColinFinck",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (January 2022)" post.
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

In January, we merged the following pull requests:

- [Add `Cr2::read_raw`](https://github.com/rust-osdev/x86_64/pull/334)
- [Add support for `MXCSR` register](https://github.com/rust-osdev/x86_64/pull/336)
- [Bump version to `0.14.8`](https://github.com/rust-osdev/x86_64/pull/339)

Thanks to [@jarkkojs](https://github.com/jarkkojs) for their contribution!

### [`bootloader`](https://github.com/rust-osdev/bootloader)

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables.

This month, we merged the following changes:

- [Use `set_reg` method of `CS`, `DS`, `ES` and `SS` segment structs](https://github.com/rust-osdev/bootloader/pull/211)
- [Remove feature flag for `lang_items`, `asm` and `global_asm`](https://github.com/rust-osdev/bootloader/pull/210) <span class="gray">(published as `v0.10.11`)</span>
- [Add support for position independent executables](https://github.com/rust-osdev/bootloader/pull/206)
- [Logger: nicer font rendering into framebuffer](https://github.com/rust-osdev/bootloader/pull/213)
- [Rework `UsedLevel4Entries`](https://github.com/rust-osdev/bootloader/pull/219)
- [Add small doc-comment to `entry_point!` macro](https://github.com/rust-osdev/bootloader/pull/220)

Thanks to [@abachmann](https://github.com/abachmann), [@Freax13](https://github.com/Freax13), [@phip1611](https://github.com/phip1611), and [@georglauterbach](https://github.com/georglauterbach) for their contributions!

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS.

We merged the following changes in January:

- [Release version 0.14.0](https://github.com/rust-osdev/uefi-rs/pull/341)
- [Release `uefi-services` version 0.11.0](https://github.com/rust-osdev/uefi-rs/pull/342)
- [System Table and Handle: From-implementation to create objects from raw pointers](https://github.com/rust-osdev/uefi-rs/pull/338)
- [Replace build.py with the xtask pattern](https://github.com/rust-osdev/uefi-rs/pull/335)
- [Fix logs cut off after screenshot test](https://github.com/rust-osdev/uefi-rs/pull/336)
- [Remove `vec_spare_capacity` nightly feature](https://github.com/rust-osdev/uefi-rs/pull/347)
- [uefi-macros: fix new clippy lint](https://github.com/rust-osdev/uefi-rs/pull/350)

Thanks to [@phip1611](https://github.com/phip1611) for their contribution!

### [`multiboot2`](https://github.com/rust-osdev/multiboot2)

The `multiboot2` crate provides abstraction types for the boot information of multiboot2 bootloaders.
The latest release of the `multiboot2`-crate is now `v.0.13.1` (was `v0.12.2`). It contains minor improvements,
such as new getters that were originally missing.

The combined diff of all changes can be found [here](https://github.com/rust-osdev/multiboot2/compare/multiboot2-header-v0.1.0...multiboot2-v0.13.1).

## Call for Participation

Want to contribute to a Rust OSDev project, but don't know where to start? Pick up one of these outstanding
issues in one of our projects and get started!

<!--
Please use the following template for adding items:
- [(`repo_name`) Issue Description](https://example.com/link-to-issue)
-->

<span class="gray">

_No tasks were proposed for this section._

</span>

If you maintain a Rust project related to operating system development and are looking for contributors, especially for tasks suited to people getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the `next` branch with the tasks you want to include in the next issue.

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`ntfs`](https://github.com/ColinFinck/ntfs)

<span class="gray">(Section written by [@ColinFinck](https://github.com/ColinFinck))</span>

Colin Finck released an initial version of his [ntfs](https://github.com/ColinFinck/ntfs) crate this month, a Rust library to access Microsoft's proprietary NTFS filesystem.

For those of you who are not running Windows:
NTFS is the primary filesystem in Windows, from Windows NT's release in 1993 up to the current Windows 11.
Unlike FAT32, NTFS has no practical limits for file and partition sizes, comes with B-Tree Indexes for faster lookups, and adds a few resilience and efficiency features (such as journaling, compression, and sparse files). 

The ntfs crate supports Rust's no_std environment and is therefore not tied to a specific platform API.
It aims to be embeddable in firmware-level code and kernels just as well as in user-mode applications.

Colin Finck will [talk about NTFS](https://fosdem.org/2022/schedule/event/misc_ntfs_rust/) and his adventures in writing a filesystem crate in Rust on the upcoming FOSDEM conference.
The talk is on Saturday, 5 February at 17:00 (CET, UTC+1).
The conference is virtual and admission is free.


### [`phip1611/noto-sans-mono-bitmap-rs`](https://github.com/phip1611/noto-sans-mono-bitmap-rs)

<span class="gray">(Section written by [@phip1611](https://github.com/phip1611))</span>

Philipp Schuster released an initial version of his [noto-sans-mono-bitmap](https://github.com/phip1611/noto-sans-mono-bitmap-rs)
crate this month. It provides a pre-rasterized bitmap font from *Noto Sans Mono*, an open font from Google.
The crate is a replacement for legacy bitmap fonts, such as the [font8x8 crate](https://crates.io/crates/font8x8).
It is suitable for printing high quality/nice looking text to a framebuffer in bootloaders, kernels and similar 
environments where you don't want or can't use the FPU.

To avoid CPU intensive soft float workloads, the crate contains pre-rendered symbols from the [Noto Sans Mono font](https://fonts.google.com/noto/specimen/Noto+Sans+Mono)
in different sizes and font weights (light, regular, bold) as Rust constants paired with a convenient getter function.

![Symbols from the crate 'noto-sans-mono-bitmap' in an UEFI framebuffer.](framebuffer-font-noto-sans-mono.png "Symbols from the crate 'noto-sans-mono-bitmap' in an UEFI framebuffer.")

The screenshot above shows text that is rendered into an UEFI framebuffer using the bitmap font 
from `noto-sans-mono-bitmap`.

An example of usage can be found in [PR#213](https://github.com/rust-osdev/bootloader/pull/213) of the 
`rust-osdev/bootloader` crate, where this crate was merged and replaced `font8x8`.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

This month, we merged three small improvements for the _Writing an OS in Rust_ blog:

- [Add Chinese translation for index page](https://github.com/phil-opp/blog_os/pull/1067)
- [Upgrade to zola 0.15.3](https://github.com/phil-opp/blog_os/pull/1061)
- [Remove dark mode warning again](https://github.com/phil-opp/blog_os/commit/b24122a6044879d2305e65d30960dc03cd50ff17)

Thanks to [@TisnKu](https://github.com/TisnKu) for their contribution!

I have also brought the [`edition-3`](https://github.com/phil-opp/blog_os/commits/edition-3) branch up to date again. I marked all the stub posts of the upcoming third edition as drafts, which should allow us to merge the unfinished branch now and then prepare the new edition directly in the `main` branch (without publishing them yet). This way, we can avoid that the branches diverge too much.

Regarding the state of the new edition: I'm planning to release an extra post about UEFI booting first because that article is almost ready. For the main posts of the edition, I'm still waiting for a few cargo features, namely artifact dependencies, a package/target-specific way to enable the unstable `build-std` feature, and fixes for the experimental `package.forced-target` manifest key. I also intend to have the new version of the `bootloader` crate ready soon, which should make the build and test process simpler and more robust.

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).


<!--
TODO: Update publication date
-->
