+++
title = "This Month in Rust OSDev: June 2024"
date = 2024-07-03

[extra]
month = "June 2024"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (June 2024)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Announcements, News, and Blog Posts

Here we collect news, blog posts, etc. related to OS development in Rust.

- [Redox OS - Software Showcase #1](https://youtu.be/s-gxAsBTPxA?si=IAPJ25EYP-XxS_FB)
  - This video show several programs running on Redox OS
- [This Month in Redox - June 2024](https://www.redox-os.org/news/this-month-240630/)
- [k23 - Experimental WASM Microkernel](https://github.com/JonasKruckenberg/k23)
  - First announced at RustNL 2024, k23 is an experimental microkernel written in Rust that runs WebAssembly programs. Now Open Source!

<!--
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->


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

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

The `uefi-rs` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. We merged the following PRs this month:

- [uefi: Stop enabling `error_in_core` feature](https://github.com/rust-osdev/uefi-rs/pull/1194)
- [reorganize deps](https://github.com/rust-osdev/uefi-rs/pull/1196)
- [uefi: Add TryFrom u8 slice to DevicePathHeader ](https://github.com/rust-osdev/uefi-rs/pull/1199)
- [test-runner: Consistently use global image handle](https://github.com/rust-osdev/uefi-rs/pull/1189)
- [uefi: Add TryFrom u8 slice to DevicePathNode](https://github.com/rust-osdev/uefi-rs/pull/1197)
- [Clean up duplicate global system table pointer in `uefi::helpers`](https://github.com/rust-osdev/uefi-rs/pull/1188)
- [uefi: make DevicePathHeader::try_from idiomatic](https://github.com/rust-osdev/uefi-rs/pull/1202)
- [uefi: Add TryFrom u8 slice to DevicePath](https://github.com/rust-osdev/uefi-rs/pull/1201)
- [uefi: re-export CapsuleFlags](https://github.com/rust-osdev/uefi-rs/pull/1203)
- [ci: Temporarily turn off warnings-as-errors for nightly CI jobs](https://github.com/rust-osdev/uefi-rs/pull/1206)
- [uefi: use UNSPECIFIED_TIMEZONE instead of magic number](https://github.com/rust-osdev/uefi-rs/pull/1210)
- [uefi: Make TimeError more descriptive](https://github.com/rust-osdev/uefi-rs/pull/1211)
- [Fix versions](https://github.com/rust-osdev/uefi-rs/pull/1187)
- [uefi-test-runner: speed up ploting of sierpinski triangle by updating changed pixel only](https://github.com/rust-osdev/uefi-rs/pull/1209)
- [memory map: improvements to fight pitfalls (MemoryMap now owns the memory)](https://github.com/rust-osdev/uefi-rs/pull/1175)
- [uefi: simplify usage of Mode](https://github.com/rust-osdev/uefi-rs/pull/1214)
- [template: fix build failure](https://github.com/rust-osdev/uefi-rs/pull/1217)
- [uefi: Add TryFrom<&[u8]> for Time](https://github.com/rust-osdev/uefi-rs/pull/1212)

Thanks to [@LightAndLight](https://github.com/LightAndLight), [@andre-braga](https://github.com/andre-braga), and [@JeffLi01](https://github.com/JeffLi01) for their contributions!

<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1184) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.22.3](https://github.com/rust-osdev/uefi-rs/pull/1193) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1195) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.22.7](https://github.com/rust-osdev/uefi-rs/pull/1207) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1208) -->
<!-- - [nix/niv: update nixpkgs to nixos-24.05](https://github.com/rust-osdev/uefi-rs/pull/1213) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.22.9](https://github.com/rust-osdev/uefi-rs/pull/1215) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1218) -->

### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following improvements:

- [Guard the lower 1MB of memory](https://github.com/rust-osdev/bootloader/pull/446)

Thanks to [@Wasabi375](https://github.com/Wasabi375) for their contributions!


### [`pci_types`](https://github.com/rust-osdev/pci_types)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `pci_types` library provides types for accessing and configuring PCI devices from Rust operating systems. We merged the following change this month:

- [feat: accept `FnOnce` for updating commands](https://github.com/rust-osdev/pci_types/pull/15)
- [ci: add workflow](https://github.com/rust-osdev/pci_types/pull/17)
- [feat: don't truncate reserved bits in `CommandRegister`](https://github.com/rust-osdev/pci_types/pull/16)
- [fix(breaking): clippy lints](https://github.com/rust-osdev/pci_types/pull/18)
- [style: update `rustfmt.toml`](https://github.com/rust-osdev/pci_types/pull/19)
- [feat: derive additional traits for `CommandRegister`](https://github.com/rust-osdev/pci_types/pull/20)
- [feat: add `EndpointHeader::update_interrupt`](https://github.com/rust-osdev/pci_types/pull/21)
- [feat: don't take explicit references to `ConfigRegionAccess`](https://github.com/rust-osdev/pci_types/pull/22)
- [feat: implement `Copy` for all types that already implement `Clone`](https://github.com/rust-osdev/pci_types/pull/23)
- [fix: don't take explicit reference in `update_interrupt`](https://github.com/rust-osdev/pci_types/pull/24)
- [Add method to get the address of any capability](https://github.com/rust-osdev/pci_types/pull/27)
- [Fix docs rendering for MsixCapability::set_enabled](https://github.com/rust-osdev/pci_types/pull/28)
- [feat: make `update_command` take `&mut self`](https://github.com/rust-osdev/pci_types/pull/32)
- [Add method to enable/disable function masking for MSI-X](https://github.com/rust-osdev/pci_types/pull/29)
- [Add unwrap_io and unwrap_mem convenience methods on Bar](https://github.com/rust-osdev/pci_types/pull/25)
- [Remove the function_exists method of ConfigRegionAccess](https://github.com/rust-osdev/pci_types/pull/26)
- [Support 64-bit addressing in MSI capabilities and improve uniformity with rest of library](https://github.com/rust-osdev/pci_types/pull/33)

Thanks to [@IsaacWoods](https://github.com/IsaacWoods), [@bjorn3](https://github.com/bjorn3), and [@mkroening](https://github.com/mkroening) for their contributions!



### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PR this month:

- [constify PhysFrame functions](https://github.com/rust-osdev/x86_64/pull/489)

### [`volatile`](https://github.com/rust-osdev/volatile)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `volatile` crate provides a safe wrapper type for implementing volatile read and write operations. This is useful for accessing memory regions that have side-effects, such as memory-mapped hardware registers.

We merged the following PR this month:

- [Release v0.6.0](https://github.com/rust-osdev/volatile/pull/64)


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

### [`nicholasbishop/ext4-view-rs`](https://github.com/nicholasbishop/ext4-view-rs)
<span class="maintainers">(Section written by [@nicholasbishop](https://github.com/nicholasbishop))</span>

I've released a new Rust crate for reading ext4 filesystems. It's easy to use, with an API very similar to [`std::fs`](https://doc.rust-lang.org/std/fs/index.html). The crate is no-std compatible, but does require an allocator.

Note that by design this crate will remain read-only; writing to an ext4 filesystem is not a goal.

Thanks to [@tedbrandston](https://github.com/tedbrandston) for doing a ton of code review on this project!

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
