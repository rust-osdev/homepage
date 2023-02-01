+++
title = "This Month in Rust OSDev: January 2023"
date = 2023-02-06

[extra]
month = "January 2023"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (January 2023)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

    ### Summary
    <span class="maintainers">(Section written by [@author](https://github.com/author))</span>

    <text>
-->

## Announcements, News, and Blog Posts

<!--
Here we collect news, blog posts, etc. related to OS development in Rust.
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->


## `rust-osdev` Projects

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

<!--
    Please use the following template:

    ### [`repo_name`](https://github.com/rust-osdev/repo_name)
    <span class="maintainers">Maintained by [@maintainer_1](https://github.com/maintainer_1)</span>

    The `repo_name` crate ...<<short introduction>>...

    We merged the following changes this month:
    <<changelog, either in list or text form>>
-->

### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@rybot666](https://github.com/rybot666)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables.

We merged lots of improvements this month:

#### Features

- [Load ramdisk feature](https://github.com/rust-osdev/bootloader/pull/302)
- [Add support for a boot configuration file](https://github.com/rust-osdev/bootloader/pull/326)
- [Make log level configurable](https://github.com/rust-osdev/bootloader/pull/303)
- [Add support for logging to serial port (configurable)](https://github.com/rust-osdev/bootloader/pull/314)
- [Add `bios` and `uefi` cargo features](https://github.com/rust-osdev/bootloader/pull/304)
- [Add a `FrameBuffer::into_buffer` method for taking ownership](https://github.com/rust-osdev/bootloader/pull/319)
- [Implement faster bios builds](https://github.com/rust-osdev/bootloader/pull/324)
- [Support higher half position independent kernels](https://github.com/rust-osdev/bootloader/pull/289)

#### Fixes

- [Correctly allocate last frame in memory descriptor](https://github.com/rust-osdev/bootloader/pull/316)
- [Fix: treat `kernel_slice_end` as an exclusive bound when checking for overlaps](https://github.com/rust-osdev/bootloader/pull/334)
- [Map BIOS stage-4 at lower address to avoid conflicts with the kernel](https://github.com/rust-osdev/bootloader/pull/337)
- [Correctness fixes for stage2](https://github.com/rust-osdev/bootloader/pull/328)
- [Fix loading of boot configuration](https://github.com/rust-osdev/bootloader/pull/342)

#### Docs

- [Fix Create Disk Image Example](https://github.com/rust-osdev/bootloader/pull/300)
- [Make a link in the documentation clickable](https://github.com/rust-osdev/bootloader/pull/341)

#### Other

- [Fix spelling and add a check](https://github.com/rust-osdev/bootloader/pull/340)
- [Check for breaking changes on CI](https://github.com/rust-osdev/bootloader/pull/325)
- [Cancel in progress PR builds when a new commit is pushed for that PR](https://github.com/rust-osdev/bootloader/pull/322)
- [Remove dependency on `time` crate](https://github.com/rust-osdev/bootloader/pull/332)
- [[test runner] Print QEMU output directly instead of waiting until it finishes](https://github.com/rust-osdev/bootloader/pull/333)
- [Fix warnings from Clippy](https://github.com/rust-osdev/bootloader/pull/336)
- [Test framework: Don't inherit `stdin` when spawning QEMU](https://github.com/rust-osdev/bootloader/pull/339)

Thanks to [@jasoncouture](https://github.com/jasoncouture), [@Stary2001](https://github.com/Stary2001), [@AlexJMohr](https://github.com/AlexJMohr), [@Freax13](https://github.com/Freax13), [@tsoutsman](https://github.com/tsoutsman), and [@asensio-project](https://github.com/asensio-project) for their contributions!


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS.

We merged the following changes last month:

#### Features

- [Implement the rest of the TPM v1 interface](https://github.com/rust-osdev/uefi-rs/pull/631)
- [Implement the rest of the TPM v2 interface](https://github.com/rust-osdev/uefi-rs/pull/634)
- [Release uefi-macros-0.10.0, uefi-0.19.0, and uefi-services-0.16.0](https://github.com/rust-osdev/uefi-rs/pull/642)
- [Add PAGE_SIZE constant and update MemoryProtection protocol docs](https://github.com/rust-osdev/uefi-rs/pull/645)

#### Fixes

- [PXE: Fix BaseCode::discover optional argument](https://ygithub.com/rust-osdev/uefi-rs/pull/630)
- [Fix warnings from `abi_efiapi` stabilization](https://github.com/rust-osdev/uefi-rs/pull/635)
- [uefi: Fix protocol functions to work with unsized protocols](https://github.com/rust-osdev/uefi-rs/pull/643)
- [Fix new lints related to derives on a packed struct](https://github.com/rust-osdev/uefi-rs/pull/646)

#### Docs

- [uefi: Update MSRV in the readme](https://github.com/rust-osdev/uefi-rs/pull/626)
- [book: Fix link to handles page](https://github.com/rust-osdev/uefi-rs/pull/627)
- [changelog: Move some macro-related changes to correct section](https://github.com/rust-osdev/uefi-rs/pull/639)

#### Other

- [Minor alloc-related cleanups](https://github.com/rust-osdev/uefi-rs/pull/623)
- [uefi: Remove useless padding field](https://github.com/rust-osdev/uefi-rs/pull/629)
- [Move some util code from TCG to a new top-level module](https://github.com/rust-osdev/uefi-rs/pull/640)
- [media/test: add integration test for creating a directory](https://github.com/rust-osdev/uefi-rs/pull/625)
- [ci: Update checkout action to latest version](https://github.com/rust-osdev/uefi-rs/pull/633)
- [test-runner: Simplify and slightly refactor the disk test](https://github.com/rust-osdev/uefi-rs/pull/641)
- [ci: fix book/deploy in forks](https://github.com/rust-osdev/uefi-rs/pull/644)

Thanks to [@nsemmel](https://github.com/nsemmel) and [@liferooter](https://github.com/liferooter) for their contributions!


### [`volatile`](https://github.com/rust-osdev/volatile)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `volatile` crate provides a safe wrapper type for implementing volatile read and write operations. This is useful for accessing memory regions that have side-effects, such as memory-mapped hardware registers or framebuffers.

We merged the following PRs this month:

- [Fix UB in slice methods when `Deref` returns different references](https://github.com/rust-osdev/volatile/pull/27) <span class="gray">(published as `v0.4.6`)</span>
- [various improvements for the new design](https://github.com/rust-osdev/volatile/pull/28)

Thanks to [@Freax13](https://github.com/Freax13) for their contributions!

We are still working on a new, safer design. This month, we opened PR [#29](https://github.com/rust-osdev/volatile/pull/29) to compare and discuss two alternative designs. The main question is whether the provided `VolatilePtr` type should implement `Copy` or `Send`. Only one of these trait implementations is possible, otherwise there could be data races that lead to undefined behavior. Since both variants have valid use cases, the latest proposal is to implement two different types with conversion methods between them. We haven't reached a decision yet, so if anyone has more input on this, please join the discussion.

### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13), and [@rybot666](https://github.com/orgs/rust-osdev/people/rybot666)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following changes in January:

- [Set `repr` to transparent for various types](https://github.com/rust-osdev/x86_64/pull/402)
- [Check for breaking changes on CI](https://github.com/rust-osdev/x86_64/pull/401)


## Call for Participation

Want to contribute to a Rust OSDev project, but don't know where to start? Help with one of these outstanding issues!

<!--
    Please use the following template for adding items:
    - [(`repo_name`) Issue Description](https://example.com/link-to-issue)
-->

<span class="gray">

_No tasks were proposed for this section this month._

</span>

If you maintain a Rust project related to operating system development and are looking for contributors, especially for tasks suited to people getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the `next` branch with the tasks you want to include in the next issue.

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
