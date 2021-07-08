+++
title = "This Month in Rust OSDev (June 2021)"
date = 0000-01-01

[extra]
month = "June 2021"
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
    This is a draft for the upcoming "This Month in Rust OSDev (June 2021)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

### [`acpi`](https://github.com/rust-osdev/acpi)

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS.

This month, both the `rsdp` and `acpi` crates saw breaking changes. These changes should require minimal work to migrate to;
please file an issue if you encounter any difficulties. <span class="gray">(published as `rsdp v2.0.0` and `acpi v3.0.0`)</span>

- [Basic support for the MADT's new Multiprocessor Wakeup Structure was added](https://github.com/rust-osdev/acpi/pull/99)
- [`PhysicalMapping` now implements `Send`](https://github.com/rust-osdev/acpi/pull/101)
- [`PhysicalMapping`'s fields were made private, preventing construction of unsound mappings in safe code](https://github.com/rust-osdev/acpi/pull/102).
  The `unmap_physical_region` method of `AcpiHandler` also lost its `self` type - handlers that used `self` should
  instead access themselves through the `PhysicalMapping::handler` method. This prevents a mapping from being
  unmapped using a different handler to the one that mapped it.
- [Accesses to potentially unaligned packed field were fixed](https://github.com/rust-osdev/acpi/commit/d58e64b39e9f22367bc76b64a68826a519615226).
  `repr(packed)` structures are very common in OS Dev, and make sure the layout of Rust's structures matches the
  hardware's. Unfortunately, they can be slightly tricky to work with - creating an unaligned reference is
  undefined behaviour, and references can transiently be created by, for example, calling a method on an unaligned
  field of a packed structure (e.g. `entry.flags.get_bit(4)`). You can read more about this issue [here](https://github.com/rust-lang/rust/issues/82523).
- [`acpi::platform` no longer re-exports the contents of its `interrupt` submodule](https://github.com/rust-osdev/acpi/commit/fdd88add32497411d439c2d18fe28258a3fe6525)

Thanks to [@wusyong](https://github.com/wusyong) and [@Freax13](https://github.com/wusyong) for their contributions!


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. In June, we merged the following changes:

- [Make the `qemu-exit` dependency optional](https://github.com/rust-osdev/uefi-rs/pull/229)
- [Fix type of the media field in the `BlockIO` protocol](https://github.com/rust-osdev/uefi-rs/pull/234)
- [Use `newtype_enum` for `DevicePath` enums](https://github.com/rust-osdev/uefi-rs/pull/230)
- [Make `DevicePath` and `AcpiDevicePath` unaligned](https://github.com/rust-osdev/uefi-rs/pull/231)
- [Derive `Debug` for `CharConversionError`](https://github.com/rust-osdev/uefi-rs/pull/233)
- [Rename boot services' `memset` to `set_mem`](https://github.com/rust-osdev/uefi-rs/pull/235)
- [Implement image loading/starting](https://github.com/rust-osdev/uefi-rs/pull/237)
- [Add `num_blocks` method to `GptPartitionEntry`](https://github.com/rust-osdev/uefi-rs/pull/238)
- [`ShimLock` protocol uses `sysv64` function ABI](https://github.com/rust-osdev/uefi-rs/pull/227)
- [Make using the `stdio` handles require a mutable ref](https://github.com/rust-osdev/uefi-rs/pull/240)
- [Fix AArch64 build](https://github.com/rust-osdev/uefi-rs/pull/243)

Thanks to [@nicholasbishop](https://github.com/nicholasbishop), [@iankronquist](https://github.com/iankronquist) and [@josephlr](https://github.com/josephlr) for their contributions!

### [`x86_64`](https://github.com/rust-osdev/x86_64)

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

In June, we merged the following changes:

- [Add common abstractions for x86 Segments](https://github.com/rust-osdev/x86_64/pull/258)
- [Specify sysv64 for the calling convention of the external assembly functions](https://github.com/rust-osdev/x86_64/pull/267)
- [Make IDT module available on stable Rust](https://github.com/rust-osdev/x86_64/pull/271)
- [Fix off-by-one error in GDT `from_raw_slice()`](https://github.com/rust-osdev/x86_64/pull/269)

We did not issue a new crates.io release with these changes yet, but we plan to do so soon.

Thanks to [@toku-sa-n](https://github.com/toku-sa-n) for their contribution!

## Call for Participation

Want to contribute to a Rust OSDev project, but don't know where to start? Pick up one of these outstanding
issues in one of our projects and get started!

<!--
Please use the following template for adding items:

> **[`repo_name`](link-to-repo):**
>
> - [Issue Description](https://example.com/link-to-issue)
-->

**[`phil-opp/blog_os`](https://github.com/phil-opp/blog_os):**

-  [New Russian translation needs a reviewer](https://github.com/phil-opp/blog_os/pull/1029): We're looking for someone that is speaking Russian to review the new Russian translation of [@MrZloHex](https://github.com/MrZloHex).

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
