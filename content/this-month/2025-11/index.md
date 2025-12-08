+++
title = "This Month in Rust OSDev: November 2025"
date = 2025-12-08

[extra]
month = "November 2025"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (November 2025)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Announcements, News, and Blog Posts

Here we collect news, blog posts, etc. related to OS development in Rust.

<!--
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [moss: a Rust Linux-compatible kernel in about 26,000 lines of code](https://www.reddit.com/r/rust/comments/1p2dhex/moss_a_rust_linuxcompatible_kernel_in_about_26000/)
- [Sprout: a programmable UEFI bootloader written in Rust](https://github.com/edera-dev/sprout)
- [This Month in Redox - November 2025](https://www.redox-os.org/news/this-month-251130/)
- [Made a x86_32 bootloader in Rust](https://www.reddit.com/r/rust/comments/1ouqxmw/made_a_x86_32_bootloader_in_rust/)
- [I've improved the implementation behind all the string formatting macros in Rust](https://hachyderm.io/@Mara/115542621720999480)
- [Rust in Android: move fast and fix things](https://security.googleblog.com/2025/11/rust-in-android-move-fast-fix-things.html)
- Podcast: [Canonical - Jon Seager, VP Engineering for Ubuntu](https://corrode.dev/podcast/s05e05-canonical/)
- Video: [Keynote: Rust in the Linux Kernel, Why? - Greg Kroah-Hartman](https://www.youtube.com/watch?v=HX0GH-YJbGw)

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [Switching to Rust's own mangling scheme on nightly](https://blog.rust-lang.org/2025/11/20/switching-to-v0-mangling-on-nightly/)
- [Stabilize `asm_cfg`](https://github.com/rust-lang/rust/pull/147736)
- [Stabilize `-Zno-jump-tables` into `-Cjump-tables=bool`](https://github.com/rust-lang/rust/pull/145974)
- [Pass pointers to `const` in assembly](https://github.com/rust-lang/rfcs/pull/3848)
- [Add Allocator proxy impls for Box, Rc, and Arc](https://github.com/rust-lang/rust/pull/148539)
- [Stabilise `as_array` in `[_]` and `*const [_]`; stabilise `as_mut_array` in `[_]` and `*mut [_]`](https://github.com/rust-lang/rust/pull/147540)
- FCP: [Destabilise target-spec-json](https://github.com/rust-lang/compiler-team/issues/944)


## `rust-osdev` Projects

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`](https://github.com/rust-osdev/about) organization.

<!--
    Please use the following template:

    ### [`repo_name`](https://github.com/rust-osdev/repo_name)
    <span class="maintainers">Maintained by [@maintainer_1](https://github.com/maintainer_1)</span>

    The `repo_name` crate ...<<short introduction>>...

    We merged the following changes this month:
    <<changelog, either in list or text form>>
-->

### New Crate: [`mem_barrier`](https://github.com/rust-osdev/mem-barrier)
<span class="maintainers">Maintained by [@mkroening](https://github.com/mkroening)</span>

This crate provides cross-architecture, no-std memory barriers.

When compiling with optimizations, the compiler may try to improve performance by reordering independent memory accesses and instructions. Modern CPUs use similar techniques for improving performance, such as out-of-order execution. Memory barriers affect both the compiler and the CPU by restricting reordering of certain memory operations across these barriers respective to other CPUs or devices, allowing proper communication with them.

See the [docs](https://docs.rs/mem-barrier/latest/mem_barrier/index.html) for details!

### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following changes:

- [Add basic example](https://github.com/rust-osdev/bootloader/pull/519)
- [Use legacy symbol mangling for BIOS stage 2](https://github.com/rust-osdev/bootloader/pull/522)
- [release `v0.11.13`](https://github.com/rust-osdev/bootloader/pull/523)

Thanks to [@peppergrayxyz](https://github.com/peppergrayxyz) for their contribution!



### [`pci_types`](https://github.com/rust-osdev/pci_types)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `pci_types` library provides types for accessing and configuring PCI devices from Rust operating systems. We merged the following change this month:

- [fix: write compliant bit pattern for BAR sizing](https://github.com/rust-osdev/pci_types/pull/37)

Thanks to [@cagatay-y](https://github.com/cagatay-y) for their contribution!


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [clippy: latest nightly fixes](https://github.com/rust-osdev/uefi-rs/pull/1811)
- [book: Rephrase target installation](https://github.com/rust-osdev/uefi-rs/pull/1809)
- [Fix documentation for allocate_pages function](https://github.com/rust-osdev/uefi-rs/pull/1815)
- [AtaPassThru: Add read_pio method for AtaRequestBuilder](https://github.com/rust-osdev/uefi-rs/pull/1814)
- [release: uefi-raw-0.13.0, uefi-0.36.1](https://github.com/rust-osdev/uefi-rs/pull/1810)
- [efi shell interface protocol: add var(), vars(), and set_var()](https://github.com/rust-osdev/uefi-rs/pull/1679)
- [uefi: Implement PciRootBridgeIo bus device enumeration logic](https://github.com/rust-osdev/uefi-rs/pull/1819)
- [uefi-raw: add Tcp4 protocol type definitions](https://github.com/rust-osdev/uefi-rs/pull/1797)
- [uefi-raw: add Storage Security Command protocol type definitions](https://github.com/rust-osdev/uefi-rs/pull/1827)
- [uefi: Fix broken bridge recursion in PCI enumeration](https://github.com/rust-osdev/uefi-rs/pull/1829)
- [uefi: Make AtaDevice::execute_command() return AtaResponse on error](https://github.com/rust-osdev/uefi-rs/pull/1828)
- [uefi-raw: Add bindings for FMP](https://github.com/rust-osdev/uefi-rs/pull/1834)
- [uefi-raw: Add bindings for most HII protocols](https://github.com/rust-osdev/uefi-rs/pull/1822)
- [uefi: Add special broadcast nvme namespace](https://github.com/rust-osdev/uefi-rs/pull/1835)

<!-- - [chore(deps): update crate-ci/typos action to v1.39.2](https://github.com/rust-osdev/uefi-rs/pull/1824) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1826) -->
<!-- - [chore(deps): update actions/checkout action to v6](https://github.com/rust-osdev/uefi-rs/pull/1833) -->
<!-- - [chore(deps): update rust crate clap to v4.5.53](https://github.com/rust-osdev/uefi-rs/pull/1832) -->
<!-- - [chore(deps): update rust crate syn to v2.0.111](https://github.com/rust-osdev/uefi-rs/pull/1821) -->

Thanks to [@seijikun](https://github.com/seijikun), [@jasonbking](https://github.com/jasonbking), [@JayKickliter](https://github.com/JayKickliter), [@crawfxrd](https://github.com/crawfxrd), [@RenTrieu](https://github.com/RenTrieu), [@rymdbar](https://github.com/rymdbar), and [@splaled](https://github.com/splaled) for their contributions!


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PRs this month:

- [release 0.15.3](https://github.com/rust-osdev/x86_64/pull/565)
- [add SMAP helpers](https://github.com/rust-osdev/x86_64/pull/566)
- [mention #542 in Changelog for 0.15.3](https://github.com/rust-osdev/x86_64/pull/568)
- [fix docs.rs build](https://github.com/rust-osdev/x86_64/pull/569)
- [Add ability to add iomap to TSS (take 2)](https://github.com/rust-osdev/x86_64/pull/194)
- [Bump actions/checkout from 5 to 6](https://github.com/rust-osdev/x86_64/pull/571)
- [release 0.15.4](https://github.com/rust-osdev/x86_64/pull/570)

Thanks to [@Restioson](https://github.com/Restioson) for their contribution!


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


<!-- <span class="gray">No projects updates were submitted this month.</span> -->


## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
