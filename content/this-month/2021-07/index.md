+++
title = "This Month in Rust OSDev (July 2021)"
date = 0000-01-01

[extra]
month = "July 2021"
authors = [
    "phil-opp",
    "IsaacWoods",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (July 2021)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. In July, we merged the following changes:

- [Add support for getting/setting variables](https://github.com/rust-osdev/uefi-rs/pull/245)
- [Better logger that includes filename and line](https://github.com/rust-osdev/uefi-rs/pull/246)
- [device path: add constants for all subtypes](https://github.com/rust-osdev/uefi-rs/pull/250)
- [device path: change the length type to u16](https://github.com/rust-osdev/uefi-rs/pull/251)
- [Implement `BootServices::protocols_per_handle`](https://github.com/rust-osdev/uefi-rs/pull/253)
- [Add method to get variable names](https://github.com/rust-osdev/uefi-rs/pull/254)
- [Better `fmt::Debug` for `Time` struct + `fmt::Display`](https://github.com/rust-osdev/uefi-rs/pull/256)
- [Fix `BltRegion::Full` bounds check](https://github.com/rust-osdev/uefi-rs/pull/257)
- [Fix procedural macros](https://github.com/rust-osdev/uefi-rs/pull/260)
- [device path: add header struct](https://github.com/rust-osdev/uefi-rs/pull/263)
- [device path: add iter method and test](https://github.com/rust-osdev/uefi-rs/pull/264)

Thanks to [@nicholasbishop](https://github.com/nicholasbishop), [@MaulingMoneky](https://github.com/MaulingMonkey), [@phip1611](https://github.com/phip1611) and [@necauqua](https://github.com/necauqua) for their contributions!

### [`acpi`](https://github.com/rust-osdev/acpi)

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS.
This month:

- [Definitions were added for the FADT's fixed and architecture flags](https://github.com/rust-osdev/acpi/pull/103)
- [Table signatures were added for more static tables](https://github.com/rust-osdev/acpi/pull/103)
- [AML: support was added for the `DefPowerResource` opcode](https://github.com/rust-osdev/acpi/commit/7f6bb2ee53c9abb6c552434dbdb4e13cf38b6b26)
- [AML: support was added for the `DefThermalZone` opcode](https://github.com/rust-osdev/acpi/commit/a55d82bad3e5b7ffd42d19487a57ca65359e3bad)
- [AML: support was added for the `DefExternal` opcode](https://github.com/rust-osdev/acpi/commit/188d62fdab853c16e9c3e66bb183acc3e1c9f134)
- [AML: support was added for the `DefConcat` opcode](https://github.com/rust-osdev/acpi/commit/6f92f675a4b0e21a5bc63edd99de1010efdb61fa)
- [AML: support was added for the `DefConcatRes` opcode](https://github.com/rust-osdev/acpi/commit/a883868dd57473a61a095c56d3e7490dfe012700)
- [AML: support was added for the `DefMid` opcode](https://github.com/rust-osdev/acpi/commit/a37008df127c6f2160c1a2ac3ba5f536f8616732)

These changes were published as `acpi v3.1.0` and `aml v0.14.0`. Thanks to [@ethindp](https://github.com/ethindp)
and [@toku-sa-n](https://github.com/toku-sa-n) for their contributions.

### [`multboot2`](https://github.com/rust-osdev/multiboot2)

The `multiboot2` crate provides abstraction types for the boot information of multiboot2 bootloaders.

In July, our `multiboot2` maintanence team gained [@phip1611](https://github.com/phip1611) as a new member. Welcome!

The following changes were merged this month:

- [much improved debug output of BootInformation + enum TagType](https://github.com/rust-osdev/multiboot2/pull/76) <span class="gray">(published as `v0.11.0`)</span>
- [Set up CI on Github Actions](https://github.com/rust-osdev/multiboot2/commit/1d7c0e21fe532550f5ee9757252881e18c88a063)
- [Add multiboot2 magic number](https://github.com/rust-osdev/multiboot2/pull/77)
- [Fixing future compiler error "unaligned_references" (82523)](https://github.com/rust-osdev/multiboot2/pull/82)
- [Rust edition 2018 + formatting + clippy](https://github.com/rust-osdev/multiboot2/pull/84)
- [**Breaking:** `load` returns a result now (no more assertions that could panic)](https://github.com/rust-osdev/multiboot2/pull/80)
- [Renamed multiboot2 bootloader magic constant](https://github.com/rust-osdev/multiboot2/pull/85)
- [Cargo toml prepare relase v0.12 + changelog](https://github.com/rust-osdev/multiboot2/pull/87) <span class="gray">(published as `v0.12.0`)</span>
- [Rename old Github urls in README](https://github.com/rust-osdev/multiboot2/pull/88)

### [`x86_64`](https://github.com/rust-osdev/x86_64)

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

In July, …

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

If you maintain a Rust OSDev project and are looking for contributors, especially for tasks suited to people
getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the
`next` branch with the tasks you want to include in the next issue.


## Personal Projects

In this section, we describe updates to personal projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

This month, ...

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).


<!--
TODO: Update publication date
-->
