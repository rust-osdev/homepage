+++
title = "This Month in Rust OSDev (April 2021)"
date = 0000-01-01

[extra]
month = "April 2021"
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
    This is a draft for the upcoming "This Month in Rust OSDev (April 2021)" post.
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

In April, we merged the following changes:

- [Added XCR0 register](https://github.com/rust-osdev/x86_64/pull/239) <span class="gray">(published as `v0.13.5`)</span>
- _Breaking:_ [Fixes for x86-interrupt calling convention](https://github.com/rust-osdev/x86_64/pull/242) <span class="gray">(published as `v0.14.0`)</span>
- [Fix some warnings](https://github.com/rust-osdev/x86_64/pull/243)
- [Add `sidt` support](https://github.com/rust-osdev/x86_64/pull/246)
- [Fix `Debug` and `PartialEq` implementations for `IDT` entry type](https://github.com/rust-osdev/x86_64/pull/249)
- [Looser trait bounds for `Port` types](https://github.com/rust-osdev/x86_64/pull/247)

Thanks to [@Luis-Hebendanz](https://github.com/Luis-Hebendanz), [@CraftSpider](https://github.com/CraftSpider), and [@dbeckwith](https://github.com/dbeckwith) for their contributions!

We also prepared a pull request to fix the build on the latest Rust nightlies:

- Use new `const_fn_trait_bound` feature to fix build on latest nightly ([#250](https://github.com/rust-osdev/x86_64/pull/250))

Since `rustfmt` is [currently broken](https://github.com/rust-lang/rust/issues/84538) on the affected newer nightlies, many users are still on older nightlies where the `const_fn_trait_bound` feature does not exist yet (`rustup update` skips nightlies where an installed component is missing). For this reason, we decided to wait with merging the fix until the `rustfmt` component is fixed. For people that want to use the latest nightly already, we pre-published the above fix as version `v0.14.1-beta`.

### [`bootloader`](https://github.com/rust-osdev/bootloader)

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we finally merged the UEFI rewrite branch:

- [Rewrite for UEFI support](https://github.com/rust-osdev/bootloader/pull/130) <span class="gray">(published as `v0.10.0`)</span>

This pull request changes the build process completely so that `v0.10.0` is no longer compatible with `v0.9.x` and below. Instead of using the [`bootimage`](https://github.com/rust-osdev/bootimage) crate, it is now recommended to create a custom builder crate. See the [API docs](https://docs.rs/bootloader/0.10.2/bootloader/) for more details. In addition to the build system changes, there are also some API changes such as a new `BootInfo` struct and a different system init state (e.g. a pixel-based framebuffer instead of the VGA text mode). Right now the documentation for the new version is still a bit sparse. We plan to improve this soon, including an update to the [_Writing an OS in Rust_](https://os.phil-opp.com/) blog (see below).

In addition to the UEFI pull request, we merged the following changes this month:

- [Fix documented type for entry point function](https://github.com/rust-osdev/bootloader/pull/147)
- [`llvm_asm!` -> `asm!`](https://github.com/rust-osdev/bootloader/pull/154)
- [Reduce the number of used unstable features of `x86_64` crate](https://github.com/rust-osdev/bootloader/pull/155) <span class="gray">(published as `v0.10.2`)</span>
  - Backported as `v0.9.17` to fix compilation on latest nighlies.

Thanks to [@mkroening](https://github.com/mkroening) and [@CraftSpider](https://github.com/CraftSpider) for their contributions!

### [`xhci`](https://github.com/rust-osdev/xhci)

The `xhci` crate provides types of xHCI structures such as Contexts, Extended Capabilities, Registers, and TRBs.

Previously the repository was hosted under [`@toku-sa-n`](https://github.com/toku-sa-n). Since April, the Rust OSDev team hosts the repository.

This crate is still under depelopment. Some types or field accessors may be missing. If you find missing features, feel free to send a PR!

### [`acpi`](https://github.com/rust-osdev/acpi)

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS.
This month, we improved diagnostics by recording the original and target AML types in the `AmlError::IncompatibleValueConversion` error. This error is emitted when AML tries to convert a value
to a data type that it can't be interpreted as - AML's rules on possible type conversions and where they can occur are very elaborate, so it helps to know the conversion that was actually attempted.
<span class="gray">(published as `v0.12.0`)</span>

Thanks to [@Knapsac](https://github.com/KnapSac) and [@toothbrush7777777](https://github.com/toothbrush7777777) for their contributions!

### [`uart_16550`](https://github.com/rust-osdev/uart_16550)

The `uart_16550` crate provides basic support for serial port I/O for 16550-compatible UARTs. In April, we updated the `x86_64` dependency to fix the build on the latest nightlies:

- [Update x86_64 dependency and make it more robust](https://github.com/rust-osdev/uart_16550/pull/14) <span class="gray">(published as `v0.2.13`)</span>

This pull request also minimizes the number of unstable features that are enabled for the `x86_64` dependency to prevent breakage on future `const_fn` changes.

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. In April, we merged the following changes:

- [Expose NamedFileProtocolInfo's Header types](https://github.com/rust-osdev/uefi-rs/pull/205)
- [Upgrade to GitHub-native Dependabot](https://github.com/rust-osdev/uefi-rs/pull/207)

Thanks to [@ocadaruma](https://github.com/ocadaruma) for their contribution!

### [`spinning_top`](https://github.com/rust-osdev/spinning_top)

The `spinning_top` crate provides a simple spinlock implementation based on the abstractions of the [`lock_api`](https://docs.rs/lock_api/0.4.1/lock_api/) crate. This month, we fixed a compiler warning:

- [Fix `spin_loop_hint` warning on Rust 1.51](https://github.com/rust-osdev/spinning_top/pull/10) <span class="gray">(published as `v0.2.3`)</span>


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

### [`cdrzewiecki/celos`](https://gitlab.com/cdrzewiecki/celos)

<span class="gray">(Section written by [@drzewiec](https://github.com/drzewiec))</span>

Over the past couple of months I have made some great strides on my OS.

* Migrated the kernel to the higher half of virtual memory
* Added double fault handling
* Added page fault handling which will attempt to (safely) expand the kernel stack if it overflows
* Related to the above, added more robust frame allocation
* Added basic heap allocation

Still working hard on squashing bugs, adding all of the things above gave me some pretty serious memory allocation bugs and it's taken me a while to get those worked out. I still want to do more testing to make sure I have all the bugs here worked out before moving on.

Next steps will be to get nicer font drawing, and then attempt to implement process support.

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).


<!--
TODO: Update publication date
-->
