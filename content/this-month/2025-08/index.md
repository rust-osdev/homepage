+++
title = "This Month in Rust OSDev: August 2025"
date = 2025-09-05

[extra]
month = "August 2025"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (August 2025)" post.
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

- [This Month in Redox - August 2025](https://www.redox-os.org/news/this-month-250831/)
- [Announcing Asterinas 0.16.0](https://asterinas.github.io/2025/08/04/announcing-asterinas-0.16.0.html)
- [minimal FAT32 file system driver written in #[no_std] rust](https://www.reddit.com/r/rust/comments/1mrz2lu/i_just_published_a_minimal_fat32_file_system/)
- [Writing a Hypervisor in 1,000 Lines](https://seiya.me/blog/hypervisor-in-1000-lines)
- [Proka Kernel - A kernel for ProkaOS](https://github.com/RainSTR-Studio/proka-kernel)
- [Introducing Rusted Firmware-A (RF-A) - A Rust-Based reimagination of Trusted Firmware-A](https://www.trustedfirmware.org/blog/rf-a-blog)
- [nanomp3: A pure Rust `no_std` MP3 decoding library](https://github.com/robbie01/nanomp3)
- [Video: Intrusive Linked Lists for Fun and Profit (on embedded)](https://www.youtube.com/watch?v=ct10kgmcFmE)

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [Make target pointer width in target json an integer](https://github.com/rust-lang/rust/pull/144443)
- [Implement support for become and explicit tail call codegen for the LLVM backend](https://github.com/rust-lang/rust/pull/144232)


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


### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS. We merged the following changes this month:

- [Rewrite `acpi` crate and entire AML interpreter](https://github.com/rust-osdev/acpi/pull/246)


### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following fix:

- [fix target-c-int-width for 0.9.x](https://github.com/rust-osdev/bootloader/pull/512)
- [release v0.11.11](https://github.com/rust-osdev/bootloader/pull/510)


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:


- [Use size_of/align_of from prelude](https://github.com/rust-osdev/uefi-rs/pull/1734)
- [Add (partial) safe protocol implementation for EFI_HII_DATABASE_PROTOCOL](https://github.com/rust-osdev/uefi-rs/pull/1719)
- [xtask: improved error output for "wrong" repr](https://github.com/rust-osdev/uefi-rs/pull/1742)
- [EFI Shell Interface: CurDir Functions](https://github.com/rust-osdev/uefi-rs/pull/1740)
- [uefi-raw: move types to net module](https://github.com/rust-osdev/uefi-rs/pull/1747)
- [uefi-raw: various small net improvements](https://github.com/rust-osdev/uefi-rs/pull/1748)
- [uefi-raw: changelog update](https://github.com/rust-osdev/uefi-rs/pull/1751)

<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1737) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.35.3](https://github.com/rust-osdev/uefi-rs/pull/1739) -->
<!-- - [fix(deps): update rust crate proc-macro2 to v1.0.96](https://github.com/rust-osdev/uefi-rs/pull/1738) -->
<!-- - [fix(deps): update rust crate clap to v4.5.44](https://github.com/rust-osdev/uefi-rs/pull/1736) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.35.4](https://github.com/rust-osdev/uefi-rs/pull/1743) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1746) -->
<!-- - [chore(deps): update actions/checkout action to v5](https://github.com/rust-osdev/uefi-rs/pull/1745) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.35.5](https://github.com/rust-osdev/uefi-rs/pull/1749) -->
<!-- - [fix(deps): update rust crate cfg-if to v1.0.3](https://github.com/rust-osdev/uefi-rs/pull/1750) -->
<!-- - [chore(deps): update rust crate bitflags to v2.9.3](https://github.com/rust-osdev/uefi-rs/pull/1744) -->

Thanks to [@seijikun](https://github.com/seijikun) and [@RenTrieu](https://github.com/RenTrieu) for their contributions!


### [`virtio-spec-rs`](https://github.com/rust-osdev/virtio-spec-rs)
<span class="maintainers">Maintained by [@mkroening](https://github.com/mkroening)</span>

The `virtio-spec` crate provides definitions from the Virtual I/O Device (VIRTIO) specification.
This project aims to be unopinionated regarding actual VIRTIO drivers that are implemented on top of this crate.

We merged the following PRs this month:

- [fix(pci): capabilities are always little-endian](https://github.com/rust-osdev/virtio-spec-rs/pull/7)
- [fix(pci): actually convert MMIO access to little endian](https://github.com/rust-osdev/virtio-spec-rs/pull/8)
- [chore: release version 0.3.1](https://github.com/rust-osdev/virtio-spec-rs/pull/9)

Thanks to [@Gelbpunkt](https://github.com/Gelbpunkt) for their contributions!


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PRs this month:

- [Bump actions/checkout from 4 to 5](https://github.com/rust-osdev/x86_64/pull/563)
- [add PageFaultErrorCode::HLAT](https://github.com/rust-osdev/x86_64/pull/564)


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


<!-- <span class="gray">No projects updates were submitted this month.</span> -->

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)
<span class="maintainers">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

We merged the following changes to the [_Writing an OS in Rust_](https://os.phil-opp.com/) blog this month:

- [Update post texts to Rust 2024](https://github.com/phil-opp/blog_os/pull/1432) (thanks to [thaliaarchi](https://github.com/phil-opp/blog_os/pull/1432))
- [fix edition2@post-11 Chinese translation error](https://github.com/phil-opp/blog_os/pull/1428) (thanks to [ttttyy](https://github.com/phil-opp/blog_os/pull/1428))
- [Add post-12 simplified Chinese translation](https://github.com/phil-opp/blog_os/pull/1429) (thanks to [ic3-w1ne](https://github.com/phil-opp/blog_os/pull/1429))
- [fix(post-01): typo](https://github.com/phil-opp/blog_os/pull/1430) (thanks to [L3Sota](https://github.com/phil-opp/blog_os/pull/1430))
- [Set `test=true` to enable `main.rs` testing again](https://github.com/phil-opp/blog_os/pull/1434)
- [Update testing post to set `test = true` in Cargo.to](https://github.com/phil-opp/blog_os/pull/1435)
- [Fix: `target-pointer-width` field now expects an integer](https://github.com/phil-opp/blog_os/pull/1436)
- [Update blog for `target-pointer-width` change](https://github.com/phil-opp/blog_os/pull/1437)


## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
