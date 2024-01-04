+++
title = "This Month in Rust OSDev: December 2023"
date = 2024-01-05

[extra]
month = "December 2023"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (December 2023)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Announcements, News, and Blog Posts

Here we collect news, blog posts, etc. related to OS development in Rust.

- [Motūrus OS](https://github.com/moturus/motor-os) - a new Rust-based Operating System targeting virtual machines.
- [Maestro is a Unix-like kernel and operating system written from scratch in Rust](https://blog.lenot.re/a/introduction)
- [MOROS 0.10.2](https://github.com/vinc/moros/releases/tag/v0.10.2)
- [The first rust driver has been merged into netdev/net-next](https://www.reddit.com/r/rust/comments/18j23d3/the_first_rust_driver_has_been_merged_into/)
- [Linus on Rust in the Linux kernel (December 2023)](https://www.reddit.com/r/rust/comments/18e6qrl/linus_on_rust_in_the_linux_kernel_december_2023/)
- [Rust for Linux — in space](https://lwn.net/Articles/954974/)
- [A `no_std` client for PostgreSQL](https://www.reddit.com/r/rust/comments/18infyx/a_no_std_client_for_postgresql/)
- The Embedded Rustacean: [Issue #8](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-8) and [Issue #9](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-9)
- [`cargo-ft`](https://github.com/stormshield/cargo-ft) (cargo filter target) is a cargo extension for specifying supported targets for a crate

<!--
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->


## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

- [Advisory: Miscompilation with `opt-level="z"` on Cortex-M](https://github.com/rust-embedded/cortex-m/discussions/503)
- [Add `core::intrinsics::simd`](https://github.com/rust-lang/rust/pull/118853)
- [Stabilize `ip_in_core` feature](https://github.com/rust-lang/rust/pull/119276)
- [Stabilize `ptr::{from_ref, from_mut}`](https://github.com/rust-lang/rust/pull/117824)
- [Add all known `target_feature` configs to check-cfg](https://github.com/rust-lang/rust/pull/118908)
- [Add `-Zfunction-return={keep,thunk-extern}` option](https://github.com/rust-lang/rust/pull/116892)
- [Add lint against ambiguous wide pointer comparisons](https://github.com/rust-lang/rust/pull/117758)
- [Add new tier 3 `aarch64-apple-watchos` target](https://github.com/rust-lang/rust/pull/119074)
- [Add more SIMD platform-intrinsics](https://github.com/rust-lang/rust/pull/117953)

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

### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following PRs:

- [Fix: Enable test runner again](https://github.com/rust-osdev/bootloader/pull/407)
- [Fix: Mark `ramdisk` as used in memory map](https://github.com/rust-osdev/bootloader/pull/408)
- [Release `v0.11.5`](https://github.com/rust-osdev/bootloader/pull/410)
- [Embed bios and uefi binaries](https://github.com/rust-osdev/bootloader/pull/395)
- [Add a `take` method to `Optional`](https://github.com/rust-osdev/bootloader/pull/411)

Thanks to [@mysteriouslyseeing](https://github.com/mysteriouslyseeing) for their contribution!


### [`xhci`](https://github.com/rust-osdev/xhci)
<span class="maintainers">Maintained by [@toku-sa-n](https://github.com/toku-sa-n)</span>

The `xhci` crate provides types of xHCI structures, such as Registers and TRBs. We merged the following PRs this month:

- [Clarify the exact behavior of RW1C setters](https://github.com/rust-osdev/xhci/pull/160)
- [PR for Misc Issue #164](https://github.com/rust-osdev/xhci/pull/167)
- [Forgot to add a changelog for #167](https://github.com/rust-osdev/xhci/pull/168)
- [A little more `Doorbell` renaming](https://github.com/rust-osdev/xhci/pull/170)
- [Add an issue template](https://github.com/rust-osdev/xhci/pull/171)
- [Changelog for #170](https://github.com/rust-osdev/xhci/pull/172)

Thanks to [@paulsohn](https://github.com/paulsohn) for their contribution!


### [`ovmf-prebuilt`](https://github.com/rust-osdev/ovmf-prebuilt)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@nicholasbishop](https://github.com/nicholasbishop)</span>

The `ovmf-prebuilt` project provides pre-built [edk2](https://github.com/tianocore/edk2) releases to make it easier to set up OVMF. We merged the following improvement this month:

- [Configure Renovate](https://github.com/rust-osdev/ovmf-prebuilt/pull/5)
- [Enable the HTTP build flag](https://github.com/rust-osdev/ovmf-prebuilt/pull/15)
- [renovate: Change stategy to update-lockfile](https://github.com/rust-osdev/ovmf-prebuilt/pull/16)
- [Update Rust crate anyhow to v1.0.78](https://github.com/rust-osdev/ovmf-prebuilt/pull/17)

<!--
- [Update actions/checkout action to v4](https://github.com/rust-osdev/ovmf-prebuilt/pull/7)
- [Lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/8)
- [Lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/9)
- [Lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/10)
- [Lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/13)
- [Lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/14)
- [Lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/18)
-->

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

The `uefi-rs` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. We merged the following PRs this month:

- [Add some new char and string convenience impls/methods](https://github.com/rust-osdev/uefi-rs/pull/1013)
- [Simplify DevicePath `to_string` return type](https://github.com/rust-osdev/uefi-rs/pull/1014)
- [xtask: Allow unions in uefi-raw](https://github.com/rust-osdev/uefi-rs/pull/1018)
- [uefi-raw: Add Ipv4Address, Ipv6Address, and MacAddress types](https://github.com/rust-osdev/uefi-rs/pull/1019)
- [uefi-raw: Add ServiceBindingProtocol and Dhcp4Protocol](https://github.com/rust-osdev/uefi-rs/pull/1020)
- [fs: Remove a couple debug logs](https://github.com/rust-osdev/uefi-rs/pull/1015)
- [uefi-raw: Add HttpProtocol, Ip4Config2Protocol, and TlsConfigurationProtocol](https://github.com/rust-osdev/uefi-rs/pull/1021)
- [renovate: Change stategy to update-lockfile](https://github.com/rust-osdev/uefi-rs/pull/1025)
- [xtask: Upgrade OVMF prebuilt](https://github.com/rust-osdev/uefi-rs/pull/1027)
- [Fix logger connection after opening serial protocol](https://github.com/rust-osdev/uefi-rs/pull/1031)
- [uefi-raw: Add `IpAddress` type](https://github.com/rust-osdev/uefi-rs/pull/1032)

<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1017) -->
<!-- - [chore(deps): update cachix/install-nix-action action to v24](https://github.com/rust-osdev/uefi-rs/pull/1016) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.16.24](https://github.com/rust-osdev/uefi-rs/pull/1023) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1026) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.16.25](https://github.com/rust-osdev/uefi-rs/pull/1028) -->
<!-- - [fix(deps): update rust crate syn to v2.0.41](https://github.com/rust-osdev/uefi-rs/pull/1029) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1030) -->
<!-- - [fix(deps): update rust crate anyhow to v1.0.76](https://github.com/rust-osdev/uefi-rs/pull/1034) -->
<!-- - [chore(deps): update rust crate trybuild to v1.0.86](https://github.com/rust-osdev/uefi-rs/pull/1033) -->
<!-- - [fix(deps): update rust crate proc-macro2 to v1.0.71](https://github.com/rust-osdev/uefi-rs/pull/1035) -->
<!-- - [fix(deps): update rust crate syn to v2.0.43](https://github.com/rust-osdev/uefi-rs/pull/1036) -->


### [`pci_types`](https://github.com/rust-osdev/pci_types)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `pci_types` library provides types for accessing and configuring PCI devices from Rust operating systems. We merged the following change this month:

- [Add functionality needed to initialize PCIe on RISC-V](https://github.com/rust-osdev/pci_types/pull/10)


### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS. We merged the following changes this month:

- [fix: issue #200 increment `local_nmi_line_count` and `processor_count` on X2APIC](https://github.com/rust-osdev/acpi/pull/204)

Thanks to [@ytakano](https://github.com/ytakano) for their contributions!


### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

The `multiboot2` crate provides abstraction types for the multiboot information structure (MBI) of multiboot2 bootloaders. We merged the following changes this month:

<!-- - [build(deps): bump crate-ci/typos from 1.16.23 to 1.16.24](https://github.com/rust-osdev/multiboot2/pull/194) -->
<!-- - [build(deps): bump crate-ci/typos from 1.16.24 to 1.16.25](https://github.com/rust-osdev/multiboot2/pull/196) -->
- [updating header to not include multiboot with alloc](https://github.com/rust-osdev/multiboot2/pull/195)
- [prepare release multiboot2-header-v0.3.2](https://github.com/rust-osdev/multiboot2/pull/197)

Thanks to [@elbiazo](https://github.com/elbiazo)for their contribution!

### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following changes this month:

- [CI: Run `semver-checks` using stable Rust](https://github.com/rust-osdev/x86_64/pull/444)


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
