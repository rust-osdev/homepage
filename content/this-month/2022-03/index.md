+++
title = "This Month in Rust OSDev: March 2022"
date = 2022-04-07

[extra]
month = "March 2022"
authors = [
    "phil-opp",
    "GabrielMajeri",
    "josephlr",
    "phip1611",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (March 2022)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

### [`x86_64`](https://github.com/rust-osdev/x86_64)

<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/josephlr), [@Freax13](https://github.com/Freax13), and [@rybot666](https://github.com/rybot666)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

This month, we released version **`0.14.9`** of the `x86_64` crate with lots of improvements:

#### New Features

- [Add `UCet` and `SCet` registers](https://github.com/rust-osdev/x86_64/pull/349)
- [Specific MSRV now noted in `README`](https://github.com/rust-osdev/x86_64/pull/355)
- Use [`rustversion`](https://crates.io/crates/rustversion) to [mark certain functions `const fn` on Rust 1.61](https://github.com/rust-osdev/x86_64/pull/353)
- [`Entry::handler_addr()` is now public](https://github.com/rust-osdev/x86_64/pull/354)
- [Increase packed structure alignment](https://github.com/rust-osdev/x86_64/pull/362)
- [Make more address methods `const fn`](https://github.com/rust-osdev/x86_64/pull/369)
  - `VirtAddr::as_ptr()`
  - `VirtAddr::as_mut_ptr()`
  - `PhysAddr::new()`
  - `PhysAddr::try_new()`

_Already merged last month:_

- [Remove all uses of external assembly](https://github.com/rust-osdev/x86_64/pull/343)
  - `external_asm` and `inline_asm` features are deprecated and now have no effect.
  - `instructions` feature (on by default) now requires Rust 1.59
- [Implement `core::iter::Step` for `VirtAddr` and `Page`](https://github.com/rust-osdev/x86_64/pull/342)
  - This trait is only available on nightly.
  - Gated behind `step_trait` feature flag
- [Address in `VirtAddrNotValid` and `PhysAddrNotValid` is now public](https://github.com/rust-osdev/x86_64/pull/340)
  - [This field now contains the whole invalid address](https://github.com/rust-osdev/x86_64/pull/347)

#### Bug fixes and Documentation

- [Fixed overflow bug in `PageRangeInclusive`](https://github.com/rust-osdev/x86_64/pull/351)
- [Remove stabilized `const_fn_fn_ptr_basics` and `const_fn_trait_bound` features](https://github.com/rust-osdev/x86_64/pull/352)
- [Don't set `nomem` in `load_tss`](https://github.com/rust-osdev/x86_64/pull/358)
- [Correctly initialize TSS's IOPB to be empty](https://github.com/rust-osdev/x86_64/pull/357)
- [Improve `GlobalDescriptorTable::add_entry` error handling](https://github.com/rust-osdev/x86_64/pull/361))
- [Update `tss_segment` documentation](https://github.com/rust-osdev/x86_64/pull/366))

Thanks to [@jarkkojs](https://github.com/jarkkojs), [@drzewiec](https://github.com/drzewiec), and [@kevinaboos](https://github.com/kevinaboos) for contributing to this release!

#### v0.15

We also merged some breaking changes which will be published in the upcoming `v0.15` release:

- [Allow the GDT to be of any length](https://github.com/rust-osdev/x86_64/pull/360)
  - [gdt: Check that MAX is in range](https://github.com/rust-osdev/x86_64/pull/365)
- [`VirtAddr` improvements](https://github.com/rust-osdev/x86_64/pull/370)
- [Remove `software_interrupt!` macro](https://github.com/rust-osdev/x86_64/pull/363)
- [Remove usize trait impls](https://github.com/rust-osdev/x86_64/pull/364)
- [Remove deprecated functions/flags](https://github.com/rust-osdev/x86_64/pull/368)
- [Update "next" MSRV to 1.59](https://github.com/rust-osdev/x86_64/pull/359)

Special thanks to our co-maintainer [@josephlr](https://github.com/josephlr), who did a lot of great work this month!

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri) and [@nicholasbishop](https://github.com/nicholasbishop)</span>

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS.

One of the pain points of developers building software using `uefi-rs` has been the `Completion` type, which is like an expanded `Result` type which also handles warnings (besides successes and errors). The [RFC for the removal of the `Completion` type](https://github.com/rust-osdev/uefi-rs/issues/360) has been accepted and the corresponding changes [have been merged](https://github.com/rust-osdev/uefi-rs/pull/361) in March: the `Completion` type has been removed and the crate has reverted to using more standard `Result`s everywhere, by treating all warnings as errors.

We merged the following changes in March:

#### New features/protocols

- [Implement the `connect_controller`/`disconnect_controller` methods](https://github.com/rust-osdev/uefi-rs/pull/311)
- [Implement `BootServices::locate_handle_buffer` function](https://github.com/rust-osdev/uefi-rs/pull/380)
- [Add rng protocol](https://github.com/rust-osdev/uefi-rs/pull/386)
- [Add `BootServices::load_image`](https://github.com/rust-osdev/uefi-rs/pull/383)
- [Add `GptPartitionAttributes` bitflags](https://github.com/rust-osdev/uefi-rs/pull/388)
- [Add `FileHandle` convenience methods and new file system tests](https://github.com/rust-osdev/uefi-rs/pull/392)
- [Add `RuntimeServices::query_variable_info`](https://github.com/rust-osdev/uefi-rs/pull/396)

#### Refactorings

- [Make `Error` public](https://github.com/rust-osdev/uefi-rs/pull/382)
- [Simplify `uefi::Result` type and remove `Completion`](https://github.com/rust-osdev/uefi-rs/pull/361)
- [Improve `Time` struct](https://github.com/rust-osdev/uefi-rs/pull/395)

#### Bug fixes

- [Fix alignment issues in file info types](https://github.com/rust-osdev/uefi-rs/pull/377)
- [Update changelog for file info changes](https://github.com/rust-osdev/uefi-rs/pull/373)
- [Make `LoadedImage`'s load options API safer](https://github.com/rust-osdev/uefi-rs/pull/375)
- [Fix status to `Result` conversions](https://github.com/rust-osdev/uefi-rs/pull/389)

#### CI & testing

- [Add miri action to `xtask` and CI](https://github.com/rust-osdev/uefi-rs/pull/381)
- [Don't run doctests with invalid pointers](https://github.com/rust-osdev/uefi-rs/pull/378)

#### Misc & chores

- [Add package sections to changelog](https://github.com/rust-osdev/uefi-rs/pull/385)
- [Remove some no-longer-needed unstable features](https://github.com/rust-osdev/uefi-rs/pull/387)
- [Drop maintenance badges from the README](https://github.com/rust-osdev/uefi-rs/pull/393)
- [Remove no-longer-needed allows for clippy lints](https://github.com/rust-osdev/uefi-rs/pull/394)
- [Publish new versions of the crates](https://github.com/rust-osdev/uefi-rs/pull/390)

Thanks to [@nicholasbishop](https://github.com/nicholasbishop), [@sven-eliasen](https://github.com/sven-eliasen), [@necauqua](https://github.com/necauqua) and [@AtsukiTak](https://github.com/AtsukiTak) for their contributions!

### [`uart_16550`](https://github.com/rust-osdev/uart_16550)

<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `uart_16550` crate provides basic support for serial port I/O for 16550-compatible UARTs. We merged the following changes this month:

- [Remove stabilized nightly feature `const_ptr_offset`](https://github.com/rust-osdev/uart_16550/pull/22) <span class="gray">(published as `v0.2.17`)</span>

Thanks to [@tsatke](https://github.com/tsatke) for this contribution!

### [`xhci`](https://github.com/rust-osdev/xhci)

<span class="maintainers">Maintained by [@toku-sa-n](https://github.com/toku-sa-n)</span>

The `xhci` crate provides types of xHCI structures such as Contexts, Extended Capabilities, Registers, and TRBs. This month, we merged some cleanups:

- [fix: clippy warnings](https://github.com/rust-osdev/xhci/pull/130)
- [ci(deps): enable dependabot](https://github.com/rust-osdev/xhci/pull/129)

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

### [`phip1611/simple-chunk-allocator`](https://github.com/phip1611/simple-chunk-allocator)

<span class="gray">(Section written by [@phip1611](https://github.com/phip1611))</span>

Philipp Schuster recently released an initial version of his [simple-chunk-allocator](https://github.com/phip1611/simple-chunk-allocator)
crate. It focuses on being a very simple-to-use general purpose allocator that "just works" for various workloads 
in `no_std` context. A bitmap is used for bookkeeping of used blocks/chunks. This enables a simple algorithm that is easy
to understand. The allocator uses a combination of the strategies "next fit" and "best fit". It is usable as `#[global_allocator]` 
and operates on static memory, i.e., no paging mechanism involved. The crate is suited to manage the heap inside a kernel 
or in a similar `no_std` application. It is part of the roottask in [Philipp's Diplom (Master) Thesis](https://github.com/phip1611/diplomarbeit-impl) 
where he wrote a runtime system for a Microkernel in Rust.


### [`phip1611/linux-libc-auxv`](https://github.com/phip1611/linux-libc-auxv)

<span class="gray">(Section written by [@phip1611](https://github.com/phip1611))</span>

Philipp Schuster recently released an initial version of his [linux-libc-auxv](https://github.com/phip1611/linux-libc-auxv)
crate. The crate enables the creation and the parsing of the initial Linux stack layout. This layout is a 
special data structure that Linux prepares for applications before they start execution. The C runtime behind the 
`_start` symbol of a libc implementation uses this to find program arguments, environment variables, and the 
auxiliary vector. The layout is tricky to create because the creator must ensure that the layout is valid in the 
address space of the target. However, `linux-libc-auxv` found a way to cope with this.  

You can write a "freestanding" binary, i.e., without libc, with this crate, run it under Linux and parse the stack 
layout yourself. This is similar to what the libc does, before Rust's runtime starts, that eventually calls the 
main function of a Rust program.

The crate is part of [Philipp's Diplom (Master) Thesis](https://github.com/phip1611/diplomarbeit-impl)
where he wrote a runtime system for a Microkernel in Rust that can emulate Linux behaviour and run unmodified 
Linux applications.

### [`phip1611/diplomarbeit-impl`](https://github.com/phip1611/diplomarbeit-impl)

<span class="gray">(Section written by [@phip1611](https://github.com/phip1611))</span>

Philipp Schuster submitted his Diplom (Master) Thesis at TU Dresden where he build a policy-free system-call layer for 
the Hedron microhypervisor. The project comes with a runtime system written in Rust for the microkernel and involves 
a roottask that enables the execution of unmodified Linux binaries through an OS personality/Linux emulation. The 
runtime system covers several interesting aspects of OS development, such as interaction with a kernel, system call
emulation, and starting programs from ELF files.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

We merged a [new Korean translation](https://github.com/JOE1994) of first post of [_Writing an OS in Rust_](https://os.phil-opp.com/) blog this month. Thanks a lot to [@JOE1994](https://github.com/JOE1994) for creating this translation and [@QuqqU](https://github.com/QuqqU) for reviewing it!

We also received lots of smaller fixes, by [@MaxDesiatov](https://github.com/phil-opp/blog_os/pull/1080), [@alaincao](https://github.com/phil-opp/blog_os/pull/1089), [@Programatic](https://github.com/phil-opp/blog_os/pull/1096), [@ruhuang2001](https://github.com/phil-opp/blog_os/pull/1091), [@Hofer-Julian](https://github.com/phil-opp/blog_os/pull/1093), [@SilensAngelusNex](https://github.com/phil-opp/blog_os/pull/1094), and [@julien-me](https://github.com/phil-opp/blog_os/pull/1095). Thank you all for your contributions!

Unfortunately I didn't have time to work on the new version of the `bootloader` crate for the upcoming third edition of the blog this month. However, there was some surprising development on the Rust side that should help us with the new build system: [@bstrie](https://github.com/bstrie) created a _Major Change Proposal_ to [promote the `x86_64-unknown-none` target to Tier 2](https://github.com/rust-lang/compiler-team/issues/499). This is a bare-metal target that should be compatible with our kernel, so we might not need `-Zbuild-std` anymore in the future. Instead, we could download a precompiled version of the `core`/`alloc` crates via `rustup target add`. The great news is that the proposal was already accepted and the [corresponding implementation PR](https://github.com/rust-lang/rust/pull/95705) is ready for review too!

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
