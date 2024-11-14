+++
title = "This Month in Rust OSDev: October 2024"
date = 2024-11-14

[extra]
month = "October 2024"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (October 2024)" post.
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

- [This Month in Redox - October 2024](https://www.redox-os.org/news/this-month-241031/)
- [Smart pointers for the kernel](https://lwn.net/Articles/992055/)
- [On Rust in enterprise kernels](https://lwn.net/Articles/993337/)
- [Unsafe Rust Is Harder Than C](https://chadaustin.me/2024/10/intrusive-linked-list-in-rust/)
- [Version `v2.0.0` of `thiserror` released with `no_std` support](https://github.com/dtolnay/thiserror/releases/tag/2.0.0)

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [Taking a raw ref (`&raw (const|mut)`) of a deref of pointer (`*ptr`) is always safe](https://github.com/rust-lang/rust/pull/129248)
- [mark some target features as 'forbidden' so they cannot be (un)set with `-Ctarget-feature`](https://github.com/rust-lang/rust/pull/129884)
- [make `unsupported_calling_conventions` a hard error](https://github.com/rust-lang/rust/pull/129935)


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

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [uefi-macros: Require that the entry function takes zero args](https://github.com/rust-osdev/uefi-rs/pull/1418)
- [uefi: Delete the deprecated `uefi::table::{boot,system}` modules](https://github.com/rust-osdev/uefi-rs/pull/1417)
- [uefi: Derive Eq/PartialEq for GptPartitionEntry](https://github.com/rust-osdev/uefi-rs/pull/1421)
- [uefi: Add CStr16 conversions from slices with interior nuls](https://github.com/rust-osdev/uefi-rs/pull/1422)
- [uefi: Improve the VariableKey type and iterator](https://github.com/rust-osdev/uefi-rs/pull/1424)
- [Update timeline section of funcs_migration.md](https://github.com/rust-osdev/uefi-rs/pull/1429)
- [xtask: Add action to generate a code coverage report](https://github.com/rust-osdev/uefi-rs/pull/1423)
- [nix: add cargo-llvm-cov to nix shell + niv update](https://github.com/rust-osdev/uefi-rs/pull/1439)
- [Codecov configuration updates](https://github.com/rust-osdev/uefi-rs/pull/1440)
- [Move DeviceType and DeviceSubType enums to uefi-raw](https://github.com/rust-osdev/uefi-rs/pull/1442)
- [Add device path node types to uefi-raw](https://github.com/rust-osdev/uefi-rs/pull/1445)
- [release: uefi-raw-0.9.0, uefi-macros-0.17.0, uefi-0.33.0](https://github.com/rust-osdev/uefi-rs/pull/1446)
- [Increase MSRV to 1.79](https://github.com/rust-osdev/uefi-rs/pull/1448)

<!-- - [fix(deps): update rust crate clap to v4.5.19](https://github.com/rust-osdev/uefi-rs/pull/1408) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1427) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.25.0](https://github.com/rust-osdev/uefi-rs/pull/1425) -->
<!-- - [chore(deps): update cachix/install-nix-action action to v30](https://github.com/rust-osdev/uefi-rs/pull/1428) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.26.0](https://github.com/rust-osdev/uefi-rs/pull/1432) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1433) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1434) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1437) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1444) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.26.8](https://github.com/rust-osdev/uefi-rs/pull/1449) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1451) -->
<!-- - [fix(deps): update rust crate fs-err to v3](https://github.com/rust-osdev/uefi-rs/pull/1452) -->

### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PRs this month:

- [Remove stabilized `const_mut_refs` feature](https://github.com/rust-osdev/x86_64/pull/501)
- [Fix clippy warnings](https://github.com/rust-osdev/x86_64/pull/502)


### [`pci_types`](https://github.com/rust-osdev/pci_types)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `pci_types` library provides types for accessing and configuring PCI devices from Rust operating systems. We merged the following change this month:

- [Add set bus number funcs](https://github.com/rust-osdev/pci_types/pull/36)

Thanks to [@ZR233](https://github.com/ZR233) for their contributions!


### [`ovmf-prebuilt`](https://github.com/rust-osdev/ovmf-prebuilt)
<span class="maintainers">Maintained by [@nicholasbishop](https://github.com/nicholasbishop) and [@phil-opp](https://github.com/phil-opp)</span>

The `ovmf-prebuilt` project provides pre-built [edk2](https://github.com/tianocore/edk2) releases to make it easier to set up OVMF. We merged the following improvement this month:

- [Implement ovmf-prebuilt library](https://github.com/rust-osdev/ovmf-prebuilt/pull/92)
- [Add release workflow](https://github.com/rust-osdev/ovmf-prebuilt/pull/93)
- [release: 0.2.0](https://github.com/rust-osdev/ovmf-prebuilt/pull/96)
- [release: 0.2.0 (take two)](https://github.com/rust-osdev/ovmf-prebuilt/pull/98)


<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/89) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/90) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/91) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/95) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/97) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/99) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/100) -->

### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

_Convenient and safe parsing of Multiboot2 Boot Information (MBI) structures and
the contained information tags. Usable in no_std environments, such as a kernel.
An optional builder feature also allows the construction of the corresponding
structures._

We merged the following PRs this month:

<!-- - [build(deps): bump crate-ci/typos from 1.24.3 to 1.25.0](https://github.com/rust-osdev/multiboot2/pull/242) -->
<!-- - [build(deps): bump cachix/install-nix-action from V27 to 29](https://github.com/rust-osdev/multiboot2/pull/241) -->
- [multiboot2: various small fixes and doc improvements](https://github.com/rust-osdev/multiboot2/pull/245)
- [multiboot2: bug fixes](https://github.com/rust-osdev/multiboot2/pull/246)


### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS. We merged the following changes this month:

- [Add `from_rsdt` method for `AcpiTables`.](https://github.com/rust-osdev/acpi/pull/222)

Thanks to [@Hsy-Intel](https://github.com/Hsy-Intel) for their contributions!


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

_No updates were proposed for this section this month._

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
