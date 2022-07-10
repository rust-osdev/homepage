+++
title = "This Month in Rust OSDev: June 2022"
date = 0000-01-01

[extra]
month = "June 2022"
authors = [
    "phil-opp",
    "phip1611",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (June 2022)" post.
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

In June, …

### [`linked-list-allocator`](https://github.com/rust-osdev/linked-list-allocator)

<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@jamesmunns](https://github.com/jamesmunns)</span>

The `linked-list-allocator` crate provides a basic `no_std` allocator that builds a linked list from freed memory blocks and thus needs no additional data structures.

This month, [@jamesmunns](https://github.com/jamesmunns) redesigned the internal API for traversing the linked list in order to pass the strict tests of [`miri`](https://github.com/rust-lang/miri), which is an experimental interpreter for Rust's intermediate representation (MIR). Among other things, `miri` checks that pointers are correctly aligned and that no invalid aliasing occurs (based on different formal models). By checking our implementation against `miri`, we make it more robust and reduce the chance that undefined behavior occurs with future compiler versions.

The change was implemented across the following pull requests:

- [Refactor with Cursor-based API, and passing Miri tests](https://github.com/rust-osdev/linked-list-allocator/pull/62) <span class="gray">(published as `v0.10.0`, but yanked later)</span>
- ([fix(deallocate_middle): advance to next in `try_insert_after`](https://github.com/rust-osdev/linked-list-allocator/pull/63))
- [Fix free logic](https://github.com/rust-osdev/linked-list-allocator/pull/64) <span class="gray">([published](https://github.com/rust-osdev/linked-list-allocator/pull/65) as `v0.10.1`)</span>

Thanks to [@jamesmunns](https://github.com/jamesmunns) for the redesign and [@haraldh](https://github.com/haraldh) for reporting a critical issue in the `v0.10.0` release and providing a fix! Also, we would like to welcome [@jamesmunns](https://github.com/jamesmunns) as a maintainer of this crate!

### [`multiboot2`](https://github.com/rust-osdev/multiboot2)

<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods), [@phip1611](https://github.com/phip1611), [@robert-w-gries](https://github.com/robert-w-gries), [@ahmedcharles](https://github.com/ahmedcharles), and [@Caduser2020](https://github.com/Caduser2020)</span>

The `multiboot2` crate provides abstraction types for the multiboot information structure (MBI)
of multiboot2 bootloaders. The latest release of the `multiboot2`-crate is now `v.0.14.0` (was
`v0.13.2`). It contains some small breaking changes because so far, we used some unsafe code and
relied on having not to verify UTF-8 strings. For a full changelog, please refer to the 
[GitHub repo](https://github.com/rust-osdev/multiboot2/blob/main/multiboot2/Changelog.md).

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri) and [@nicholasbishop](https://github.com/nicholasbishop)</span>

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS.

We merged the following changes in June:

#### Fixes

- [Remove `exts::allocate_buffer`](https://github.com/rust-osdev/uefi-rs/pull/443)
- [Fix incorrect pointer cast in get_rng](https://github.com/rust-osdev/uefi-rs/pull/447)

#### Improvements

- [Remove unused `From<ucs2::Error>` impls](https://github.com/rust-osdev/uefi-rs/pull/450)
- [Remove `eh_personality` lang item](https://github.com/rust-osdev/uefi-rs/pull/451)
- [Simplify `uefi-services` panic handler](https://github.com/rust-osdev/uefi-rs/pull/453)

#### Internal changes

- [Fix `query_variable_info` test](https://github.com/rust-osdev/uefi-rs/pull/442)
- [Fix errors due to deprecations in the `clap` API](https://github.com/rust-osdev/uefi-rs/pull/445)

### [`bootloader`](https://github.com/rust-osdev/bootloader)

<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@rybot666](https://github.com/rybot666), and [@64](https://github.com/64)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we made more progress on the upcoming `v0.11` version:

- [WIP: try to start loading FAT partition in second stage](https://github.com/rust-osdev/bootloader/commit/3b6aa00dc33005f4353568c5dccad694d0f54f90)
- [Start custom FAT implementation based on `mini_fat` crate](https://github.com/rust-osdev/bootloader/commit/abfdba522e990fcaf865302b7b7d91f0abef04a7) (to keep the executable size down)
- [Read root dir entries from FAT partition](https://github.com/rust-osdev/bootloader/commit/96d2bdbae6fdc91c8df958fd09d4207a3ab20020)
- [Use segment-based addressing for DAP to support loading larger second stages](https://github.com/rust-osdev/bootloader/commit/eef2109861d718e0da82b3c5e4c2d9308d34da56)
- [Implement looking up `kernel-x86_64` file on FAT partition](https://github.com/rust-osdev/bootloader/commit/1d7ff1f0627fa33dab3f811af150a547e1f48172)
- [Read FAT clusters of kernel file](https://github.com/rust-osdev/bootloader/commit/06d3b9521cc8bac981068bafe0ab84b5c4a7e8c0)
- [Ensure proper alignment of DAP target buffer](https://github.com/rust-osdev/bootloader/commit/6e8f9fa964bfcd23b95fe902fe3a5f2c55065e27)
- [Enter unreal mode](https://github.com/rust-osdev/bootloader/commit/7a24837692d96c8e9b04377fa2e7646648637dcb)

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

### [`google/gpt-disk-rs`](https://github.com/google/gpt-disk-rs)

<span class="gray">(Section written by [@nicholasbishop](https://github.com/nicholasbishop))</span>

`gpt-disk-rs` is a collection of three libraries related to [GPT](https://en.wikipedia.org/wiki/GUID_Partition_Table) (GUID Partition Table) disk data. The crates are no-std by default, have minimal dependencies, and include a lot of documentation. I'm hoping they'll be a suitable base for any project that wants to read or write GPT data.

* [`uguid`](https://crates.io/crates/uguid): GUID data type.
* [`gpt_disk_types`](https://crates.io/crates/gpt_disk_types): all the GPT types defined by the UEFI specification
* [`gpt_disk_io`](https://crates.io/crates/gpt_disk_io): types for reading and writing GPT data to an abstract block IO device.

### [`vinc/moros`](https://github.com/vinc/moros)

<span class="gray">(Section written by [@vinc](https://github.com/vinc))</span>

MOROS is a text-based hobby operating system targeting computers with a x86-64 architecture and a BIOS. It is inspired by Unix and ITS but is closer to a modern DOS at the moment in term of features.

In the last month I finally managed to run rust programs (without alloc) inside the OS after being limited to nasm programs for a long time. This triggered a lot of refactoring and improvements in the shell and the lisp language used for scripting.

Next step will be publishing a 0.8.0 release and working on userspace allocation.

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).


<!--
TODO: Update publication date
-->
Footer
