+++
title = "This Month in Rust OSDev (May 2021)"
date = 2021-06-07

[extra]
month = "May 2021"
authors = [
    "phil-opp",
    "GabrielMajeri",
    "IsaacWoods",
    "toku-sa-n",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (May 2021)" post.
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

In May, we merged the following changes:

- [Use new `const_fn_trait_bound` feature to fix build on latest nightly](https://github.com/rust-osdev/x86_64/pull/250) <span class="gray">(published as `v0.14.1`)</span>
- [Multiple improvements to the inline assembly code](https://github.com/rust-osdev/x86_64/pull/251) <span class="gray">(published as `v0.14.2`)</span>
- [Minor lint fixes](https://github.com/rust-osdev/x86_64/pull/253)
- [Cleanup `const_fn!`](https://github.com/rust-osdev/x86_64/pull/255)
- [Use marker types for Port read/write access](https://github.com/rust-osdev/x86_64/pull/248)
- [Bump version to 0.14.3](https://github.com/rust-osdev/x86_64/pull/256) <span class="gray">(published as `v0.14.3`)</span>

We also started to [prepare a `v0.15` release](https://github.com/rust-osdev/x86_64/issues/262), for which we already implemented the following breaking changes:

- [Replace `software_interrupt!` macro with generic function](https://github.com/rust-osdev/x86_64/pull/259)
- [`software_interrupt`: Add additional testing](https://github.com/rust-osdev/x86_64/pull/260)
- [Fix typo in docs](https://github.com/rust-osdev/x86_64/pull/265)
- [idt: Fixup Options structure and cleanup `set_handler_fn`](https://github.com/rust-osdev/x86_64/pull/226) (resubmitted in [#261](https://github.com/rust-osdev/x86_64/pull/261))
- [Use SegmentSelector in InterruptStackFrame](https://github.com/rust-osdev/x86_64/pull/263)

Thanks to [@dbeckwith](https://github.com/dbeckwith) and [@Freax13](https://github.com/Freax13) for their contributions!

### [`bootloader`](https://github.com/rust-osdev/bootloader)

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following changes:

- [Change register used in setting `SS` in `stage_4`](https://github.com/rust-osdev/bootloader/pull/156) <span class="gray">(published as `v0.10.3`)</span>
- [Fix build on latest Rust nightly by updating to `uefi` v0.9.0](https://github.com/rust-osdev/bootloader/pull/162)
- [Fix higher half kernels by identity mapping context switch fn earlier](https://github.com/rust-osdev/bootloader/pull/161) <span class="gray">(published as `v0.10.4`)</span>
- [Make bootloader v0.10 compatible with latest Rust nightlies by updating uefi-rs dependency](https://github.com/rust-osdev/bootloader/pull/170) <span class="gray">(published as `v0.10.5`)</span>
- [Add some usage examples](https://github.com/rust-osdev/bootloader/pull/166)
- [Uefi: Look for an ACPI2 RSDP first](https://github.com/rust-osdev/bootloader/pull/174)
- [Identity-map GDT into kernel address space](https://github.com/rust-osdev/bootloader/pull/175)
- [Don't check target architecture for builder crate to support cross-compiling](https://github.com/rust-osdev/bootloader/pull/176) <span class="gray">(published as `v0.10.6`)</span>

Thanks to [@Elekrisk](https://github.com/Elekrisk) for their contribution!

We also published the following backport to `v0.9`:

- [Fix nightly regression by manually passing `--gc-sections`](https://github.com/rust-osdev/bootloader/pull/168) <span class="gray">(published as `v0.9.18`)</span>

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. In May, we merged the following changes:

- [Switch to the newer `Try` trait API](https://github.com/rust-osdev/uefi-rs/pull/221)
- [Add `FromResidual<Result<!, Error>>` impl for `Status`](https://github.com/rust-osdev/uefi-rs/pull/223)
- [Add `PartitionInfo` protocol](https://github.com/rust-osdev/uefi-rs/pull/225)
- [Add shim lock protocol](https://github.com/rust-osdev/uefi-rs/pull/226)

Thanks to [@phil-opp](https://github.com/phil-opp) and [@nicholasbishop](https://github.com/nicholasbishop) for their contributions!

### [`uart_16550`](https://github.com/rust-osdev/uart_16550)

The `uart_16550` crate provides basic support for serial port I/O for 16550-compatible UARTs. We merged the following changes this month:

- [SerialPort::new() no longer requires nightly](https://github.com/rust-osdev/uart_16550/pull/16) <span class="gray">(published as `v0.2.14`)</span>
- [Add support for memory mapped UARTs](https://github.com/rust-osdev/uart_16550/pull/15)
- [Improvements to new MMIO support](https://github.com/rust-osdev/uart_16550/pull/18) <span class="gray">(published as `v0.2.15`)</span>

Thanks to [@josephlr](https://github.com/josephlr) and [@remimimimi](https://github.com/remimimimi) for their contributions!

### [`pic_8259`](https://github.com/rust-osdev/pic8259)

The `pic_8259` crate provides abstractions for 8259 and 8259A Programmable Interrupt Controllers (PICs). It is a new fork of the [`pic8259_simple`](https://github.com/emk/toyos-rs/tree/master/crates/pic8259_simple) crate, which appears to be no longer maintained.

We merged the following changes on top of the original `pic8259_simple` crate:

- [PIC: Masks](https://github.com/emk/toyos-rs/pull/7)
- [cpuio: Use new feature flag for const functions](https://github.com/emk/toyos-rs/pull/9)
- [Replace `cpuio` dependency with `x86_64` crate](https://github.com/rust-osdev/pic8259/commit/92f7a123224e7fa1ce3813fc62b84e290d2fc799)
- [Rename to pic8259 and bump version to 0.10.0](https://github.com/rust-osdev/pic8259/commit/3e5602aaff3d30f6371c4976149eb693d5838d7c)

Thanks to [@mkroening](https://github.com/mkroening) and [@hanmertens](https://github.com/hanmertens) for their contributions!

### [`acpi`](https://github.com/rust-osdev/acpi)

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS.

This month was fairly quiet, but an important regression was fixed. When native methods were introduced in March,
`AmlContext` lost its `Send + Sync`, as these native methods weren't required to be thread-safe. This meant that
`AmlContext` could no longer be stored in types such as `spin::Once`, or shared between threads/tasks. This is
undesirable for `AmlContext`, as it is very expensive to construct, and AML does (in theory) provide mechanisms to
make itself thread-safe, so any probject should only need one.

This was fixed by requiring native methods to be `Send + Sync`. If you're hitting this issue, please upgrade to the
latest version. <span class="gray">(published as `v0.13.0`)</span>

Thanks to [@michaelmelanson](https://github.com/michaelmelanson) for his contribution!

### [`xhci`](https://github.com/rust-osdev/xhci)

The `xhci` crate provides types of xHCI structures such as Contexts, Extended Capabilities, Registers, and TRBs.

In May we finished implementing all of these structures and field getters/setters. Still there may exist missings. If you find one, feel free to send a PR!

### [`spinning_top`](https://github.com/rust-osdev/spinning_top)

The `spinning_top` crate provides a simple spinlock implementation based on the abstractions of the [`lock_api`](https://docs.rs/lock_api/0.4.1/lock_api/) crate. This month, we released version `v0.2.4` with the following small improvements:

- [Define `MappedSpinlockGuard` alias](https://github.com/rust-osdev/spinning_top/pull/12)
- [Fix unclosed code block in doc comment](https://github.com/rust-osdev/spinning_top/pull/11/files)

Thanks to [@toku-sa-n](https://github.com/toku-sa-n) for these contributions!

### [`vga`](https://github.com/rust-osdev/vga)

The work-in-progress `vga` crate allows the configuration of the VGA hardware, e.g. switching from text-based mode to a pixel-based graphics mode. This month, we fixed a nightly build error:

- [Update dependencies to latest versions to fix E0557](https://github.com/rust-osdev/vga/pull/23)
- [Update testing](https://github.com/rust-osdev/vga/pull/24) <span class="gray">(published as `v0.2.7`)</span>

Thanks to [@ethindp](https://github.com/ethindp) for their contribution!

### [`ps2-mouse`](https://github.com/rust-osdev/ps2-mouse)

The `ps2-mouse` library provides a basic interface for interacting with a PS/2 mouse. It was also affected by the nightly breakage in `x86_64`, so it required a dependency update too:

- [Update x86_64 to 0.14.2](https://github.com/rust-osdev/ps2-mouse/pull/2) <span class="gray">(published as `v0.1.4`)</span>

Thanks to [@littledivy](https://github.com/littledivy) for this contribution!

## Call for Participation

Want to contribute to a Rust OSDev project, but don't know where to start? Pick up one of these outstanding
issues in one of our projects and get started!

<!--
Please use the following template for adding items:
- [(`repo_name`) Issue Description](https://example.com/link-to-issue)
-->

- [(`acpi`) Add initial support for `DefExternal` opcodes](https://github.com/rust-osdev/acpi/issues/96)
- [(`acpi`) Add test to make sure `AmlContext` remains `Send + Sync`](https://github.com/rust-osdev/acpi/issues/98)

If you maintain a Rust OSDev project and are looking for contributors, especially for tasks suited to people
getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the
`next` branch with the tasks you want to include in the next issue.


## Personal Projects

In this section, we describe updates to personal projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

We merged the following improvements to the [_Writing an OS in Rust_](https://os.phil-opp.com/) blog this month:

- [Update posts to fix build on latest nightly](https://github.com/phil-opp/blog_os/pull/990)
    - [Update x86_64 dependency to v0.14.2 to fix nightly breakage](https://github.com/phil-opp/blog_os/pull/984)
    - Adjust `post-XX` tags for x86_64 v0.14: [`post-05`](https://github.com/phil-opp/blog_os/pull/985), [`post-06`](https://github.com/phil-opp/blog_os/pull/986), [`post-07`](https://github.com/phil-opp/blog_os/pull/987), [`post-08`](https://github.com/phil-opp/blog_os/pull/988)
    - [Switch to `pic8259` fork](https://github.com/phil-opp/blog_os/pull/987)
    - [Update `linked_list_allocator` dependency to v0.9.0](https://github.com/phil-opp/blog_os/pull/989)
- [Translate post 08 to Japanese](https://github.com/phil-opp/blog_os/pull/954)
- [Remove wrong suggestion part](https://github.com/phil-opp/blog_os/pull/983)
- [Fix typo](https://github.com/phil-opp/blog_os/pull/978)
- [Fix typos in edition 3 uefi booting post](https://github.com/phil-opp/blog_os/pull/981)

Thanks to [@woodyZootopia](https://github.com/woodyZootopia), [@kahirokunn](https://github.com/kahirokunn), [@HKalbasi](https://github.com/HKalbasi), and [@bjorn3](https://github.com/bjorn3) for their contributions!

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
