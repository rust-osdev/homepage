+++
title = "This Month in Rust OSDev (March 2022)"
date = 0000-01-01

[extra]
month = "March 2022"
authors = [
    "phil-opp",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

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

<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13), and [@rybot666](https://github.com/orgs/rust-osdev/people/rybot666)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

In March, …

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri) and [@nicholasbishop](https://github.com/orgs/rust-osdev/people/nicholasbishop)</span>

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

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

This month, ...

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

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).


<!--
TODO: Update publication date
-->
