+++
title = "This Month in Rust OSDev: March 2024"
date = 2024-04-05

[extra]
month = "March 2024"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (March 2024)" post.
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

- [This Month in Redox](https://redox-os.org/news/this-month-240330/)
- [Redox Kernel Improvements](https://redox-os.org/news/kernel-10/)
- [MOROS 0.10.3](https://github.com/vinc/moros/releases/tag/v0.10.3)

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

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

- [AML: Correctly invoke `_SEG`,`_BBN`, and `_ADR` methods for PCI region accesses, plus assorted bits](https://github.com/rust-osdev/acpi/pull/208)
- [acpi: Add `SdtHeaderIterator` to get all headers.](https://github.com/rust-osdev/acpi/pull/202)
- [aml: add extra debug info on parsing error](https://github.com/rust-osdev/acpi/pull/207)
- [AML: implement boolean field](https://github.com/rust-osdev/acpi/pull/211)

Thanks to [@fslongjin](https://github.com/fslongjin) and [@rw-vanc](https://github.com/rw-vanc) for their contributions!


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PRs this month:

- [miscellaneous improvements](https://github.com/rust-osdev/x86_64/pull/464)
- [release v0.15.0](https://github.com/rust-osdev/x86_64/pull/463)
- [doc: added help on update_flags to get flags](https://github.com/rust-osdev/x86_64/pull/465)
- [Feat: add constructor for `InterruptStackFrameValue`](https://github.com/rust-osdev/x86_64/pull/467)
- [properly jump the address gap in CleanUp](https://github.com/rust-osdev/x86_64/pull/469)
- [expose DEBUG_STR more directly](https://github.com/rust-osdev/x86_64/pull/471)
- [add write_pcid_no_flush](https://github.com/rust-osdev/x86_64/pull/472)
- [release 0.15.1](https://github.com/rust-osdev/x86_64/pull/473)
- [Implement function for creating a gdt in a const environment](https://github.com/rust-osdev/x86_64/pull/413)

Thanks to [@uglyoldbob](https://github.com/uglyoldbob), [@GZTimeWalker](https://github.com/GZTimeWalker), and [@Sxmourai](https://github.com/Sxmourai) for their contributions!


### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following improvements:

- [Change gitter badge to zulip badge](https://github.com/rust-osdev/bootloader/pull/431)
- [avoid 32-bit relocation to `__BOOTLOADER_CONFIG`](https://github.com/rust-osdev/bootloader/pull/428)

Thanks to [@nicholasbishop](https://github.com/nicholasbishop), and [@Freax13](https://github.com/Freax13) for their contributions!


### [`ucs2-rs`](https://github.com/rust-osdev/ucs2-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

- [Update Cargo.toml metadata](https://github.com/rust-osdev/ucs2-rs/pull/16)
- [Add a short changelog](https://github.com/rust-osdev/ucs2-rs/pull/17)
- [Add auto-release workflow](https://github.com/rust-osdev/ucs2-rs/pull/18)
- [Set MSRV and add CI job to check it](https://github.com/rust-osdev/ucs2-rs/pull/19)
- [Improve test coverage](https://github.com/rust-osdev/ucs2-rs/pull/20)


### [`pic8259`](https://github.com/rust-osdev/pic8259)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `pic_8259` crate provides abstractions for 8259 and 8259A Programmable Interrupt Controllers (PICs).

We merged the following PR this month:

- [Update x86_64 dependency to version 0.15.0](https://github.com/rust-osdev/pic8259/pull/6)

Thanks to [@iTitus](https://github.com/iTitus) for their contribution!


### [`ovmf-prebuilt`](https://github.com/rust-osdev/ovmf-prebuilt)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@nicholasbishop](https://github.com/nicholasbishop)</span>

The `ovmf-prebuilt` project provides pre-built [edk2](https://github.com/tianocore/edk2) releases to make it easier to set up OVMF. We merged the following improvement this month:

- [Transition to a workspace](https://github.com/rust-osdev/ovmf-prebuilt/pull/39)
- [Add empty ovmf-prebuilt package](https://github.com/rust-osdev/ovmf-prebuilt/pull/45)
- [fix(deps): update rust crate clap to v4.5.4](https://github.com/rust-osdev/ovmf-prebuilt/pull/46)


### [`volatile`](https://github.com/rust-osdev/volatile)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `volatile` crate provides a safe wrapper type for implementing volatile read and write operations. This is useful for accessing memory regions that have side-effects, such as memory-mapped hardware registers.

We merged the following PRs this month:

- [Implement some useful traits](https://github.com/rust-osdev/volatile/pull/41)
- [Fix clippy warning about clone implementation](https://github.com/rust-osdev/volatile/pull/44)
- [Fix build with `very_unstable` feature](https://github.com/rust-osdev/volatile/pull/45)
- [Remove `Sized` requirement for `Send` and `Sync` on `VolatileRef`](https://github.com/rust-osdev/volatile/pull/42)

Thanks to [@nspin](https://github.com/nspin) and [@kadiwa4](https://github.com/kadiwa4) for their contributions!


### [`linked-list-allocator`](https://github.com/rust-osdev/linked-list-allocator)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@jamesmunns](https://github.com/jamesmunns)</span>

The `linked-list-allocator` crate provides a basic `no_std` allocator that builds a linked list from freed memory blocks and thus needs no additional data structures. We merged the following PR this month:

- [Remove stabilized feature](https://github.com/rust-osdev/linked-list-allocator/pull/81)


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

The `uefi-rs` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. We merged the following PRs this month:

<!--
- [chore(deps): update rust crate log to v0.4.21](https://github.com/rust-osdev/uefi-rs/pull/1087)
- [fix(deps): update rust crate syn to v2.0.52](https://github.com/rust-osdev/uefi-rs/pull/1088)
- [chore(deps): update crate-ci/typos action to v1.19.0](https://github.com/rust-osdev/uefi-rs/pull/1090)
- [fix(deps): update rust crate tempfile to v3.10.1](https://github.com/rust-osdev/uefi-rs/pull/1089)
- [fix(deps): update rust crate proc-macro2 to v1.0.79](https://github.com/rust-osdev/uefi-rs/pull/1095)
- [fix(deps): update rust crate anyhow to v1.0.81](https://github.com/rust-osdev/uefi-rs/pull/1094)
- [chore(deps): update rust crate trybuild to v1.0.90](https://github.com/rust-osdev/uefi-rs/pull/1093)
- [fix(deps): update rust crate syn to v2.0.53](https://github.com/rust-osdev/uefi-rs/pull/1096)
- [chore(deps): update cachix/install-nix-action action to v26](https://github.com/rust-osdev/uefi-rs/pull/1098)
- [fix(deps): update rust crate walkdir to v2.5.0](https://github.com/rust-osdev/uefi-rs/pull/1102)
- [fix(deps): update rust crate os_info to v3.8.1](https://github.com/rust-osdev/uefi-rs/pull/1101)
- [fix(deps): update rust crate nix to 0.28.0](https://github.com/rust-osdev/uefi-rs/pull/1100)
- [fix(deps): update rust crate regex to v1.10.4](https://github.com/rust-osdev/uefi-rs/pull/1106)
- [fix(deps): update rust crate os_info to v3.8.2](https://github.com/rust-osdev/uefi-rs/pull/1105)

-->

- [uefi: Derive Hash for all char and string types](https://github.com/rust-osdev/uefi-rs/pull/1086)
- [uefi_raw: Add firmware_storage module](https://github.com/rust-osdev/uefi-rs/pull/1085)
- [Use auto-release from crates.io to release](https://github.com/rust-osdev/uefi-rs/pull/1068)
- [uefi-services: Use "dep:" syntax](https://github.com/rust-osdev/uefi-rs/pull/1091)
- [release: uefi-raw-0.5.1, uefi-0.27.0, uefi-services-0.24.0](https://github.com/rust-osdev/uefi-rs/pull/1092)
- [Fix some new lints/warnings](https://github.com/rust-osdev/uefi-rs/pull/1103)


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
