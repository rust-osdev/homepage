+++
title = "This Month in Rust OSDev: April 2024"
date = 2024-04-10

[extra]
month = "April 2024"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (April 2024)" post.
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

- [Testing Virtualization Stacks by Utilizing Mini Operating System Kernels](https://cyberus-technology.de/articles/testing-virtualization-stacks-utilizing-mini-kernels) \
  Multiple mini OS kernels help Cyberus Technology to investigate issues 
  related to complicated problems in virtualization stacks, such as never
  delivered interrupts. Although the Cyberus Guest Tests are written in C++,
  they help to find issues and problems in Cloud-Hypervisor, 
  **a VMM written in Rust**. For better debugging capabilities of the Guest
  Tests, Cyberus Technology [upstreamed support for the Debug Console](https://github.com/cloud-hypervisor/cloud-hypervisor/pull/6012)
  to Cloud Hypervisor, which is present since `v38`. The [source code of the Guest Tests is on GitHub](https://github.com/cyberus-technology/guest-tests).
- [Redox OS - April 2024 Report](https://redox-os.org/news/this-month-240430/)
- [Giving Rust a chance for in-kernel codecs](https://lwn.net/SubscriberLink/970565/ac5ffc2e9ad20f1e/)
- Video: [From C to Rust: Bringing Rust Abstractions to Embedded Linux](https://www.youtube.com/watch?v=hmQr4fq6sH0)
- The Embedded Rustacean [Issue #17](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-17) and [Issue #18](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-18)
- [Rust-Written LAVD Kernel Scheduler Shows Promising Results For Linux Gaming](https://www.phoronix.com/news/LAVD-Scheduler-Linux-Gaming)
- Video: [LinuxFest Northwest 2024: Meet COSMIC DE](https://www.youtube.com/watch?v=JHLfsWhDvz0)
- New [`offset-allocator`](https://github.com/pcwalton/offset-allocator) crate, providing a fast, simple, hard real time allocator
  - not `no_std` yet, but should be easy to port (only requires a `Vec`-like type)
- [Making an RISC-V OS (Part 3): Managing free memory](https://traxys.me/riscv_os_buddy.html)
- [Asterinas](https://asterinas.github.io/): a secure, fast, and general-purpose OS kernel written in Rust and compatible with Linux

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

### [`volatile`](https://github.com/rust-osdev/volatile)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `volatile` crate provides a safe wrapper type for implementing volatile read and write operations. This is useful for accessing memory regions that have side-effects, such as memory-mapped hardware registers.

We merged the following PRs this month:

- [Add `VolatileRef::restrict` and `VolatilePtr::restrict`](https://github.com/rust-osdev/volatile/pull/47)
- [Add `VolatileRef::borrow` and `VolatileRef::borrow_mut`](https://github.com/rust-osdev/volatile/pull/46)
- [docs: remove unused `NonNull` imports](https://github.com/rust-osdev/volatile/pull/48)
- [Add support for nested `map_field` operations](https://github.com/rust-osdev/volatile/pull/50)
- [Add `#[derive(VolatileFieldAccess)]` for easy, access-limited field-based access to structs](https://github.com/rust-osdev/volatile/pull/49)
- [fix(Cargo.toml): add categories](https://github.com/rust-osdev/volatile/pull/52)
- [ci: migrate away from unmaintained actions](https://github.com/rust-osdev/volatile/pull/51)
- [Enable all features and `doc_auto_cfg` on docs.rs](https://github.com/rust-osdev/volatile/pull/55)
- [Release v0.5.3](https://github.com/rust-osdev/volatile/pull/53)
- [Fix warnings and deny warnings in CI](https://github.com/rust-osdev/volatile/pull/56)
- [fix(macro): support `#[repr(align(N))]`](https://github.com/rust-osdev/volatile/pull/57)
- [fix(access): properly seal access traits](https://github.com/rust-osdev/volatile/pull/59)
- [Release v0.5.4](https://github.com/rust-osdev/volatile/pull/62)
- [Add a semver checks CI job](https://github.com/rust-osdev/volatile/pull/63)
- [feat: introduce `RestrictAccess<To>` and generalize `restrict` to all access types](https://github.com/rust-osdev/volatile/pull/60)
- [feat: implement derive macro for all access types](https://github.com/rust-osdev/volatile/pull/61)
- [fix: add `#[must_use]` to volatile types, `read`, and `as_raw_ptr`](https://github.com/rust-osdev/volatile/pull/58)

Thanks to [@mkroening](https://github.com/mkroening) for their contributions!

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

The `uefi-rs` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. We merged the following PRs this month:

- [Add timestamp protocol](https://github.com/rust-osdev/uefi-rs/pull/1109)
- [Replace some `as` casts](https://github.com/rust-osdev/uefi-rs/pull/1108)
- [uefi: Add UnalignedSlice::as_ptr](https://github.com/rust-osdev/uefi-rs/pull/1117)
- [Add device path text protocols to uefi-raw and re-use in uefi](https://github.com/rust-osdev/uefi-rs/pull/1118)
- [uefi-services: Fix warning with `--no-default-features`](https://github.com/rust-osdev/uefi-rs/pull/1119)
- [uefi: Add more derives for Handle and Event](https://github.com/rust-osdev/uefi-rs/pull/1120)
- [misc: nix/niv updates and typo updates](https://github.com/rust-osdev/uefi-rs/pull/1125)
- [doc: update fs documentation](https://github.com/rust-osdev/uefi-rs/pull/1126)
- [doc: update README](https://github.com/rust-osdev/uefi-rs/pull/1127)
- [Deprecate `uefi-services` and add `uefi::helpers` as replacement](https://github.com/rust-osdev/uefi-rs/pull/1128)
- [Fix some warnings, and ensure CI catches them in the future](https://github.com/rust-osdev/uefi-rs/pull/1134)
- [release: uefi-raw-0.5.2, uefi-0.28.0, uefi-services-0.25.0](https://github.com/rust-osdev/uefi-rs/pull/1140)
- [build(deps): bump rustls from 0.22.2 to 0.22.4](https://github.com/rust-osdev/uefi-rs/pull/1142)
- [Nuke uefi services from repository](https://github.com/rust-osdev/uefi-rs/pull/1141)
- [[Misc] Add ResetNotification protocol. Add Misc to uefi-test-runner.](https://github.com/rust-osdev/uefi-rs/pull/1116)
- [Minor import/export cleanups](https://github.com/rust-osdev/uefi-rs/pull/1144)
- [uefi: Drop the panic-on-logger-errors feature](https://github.com/rust-osdev/uefi-rs/pull/1143)
- [Replace `cstr16!` with a declarative macro](https://github.com/rust-osdev/uefi-rs/pull/1145)
- [Replace `cstr8!` with a declarative macro](https://github.com/rust-osdev/uefi-rs/pull/1151)
- [Remove xtask from MSRV build and update clap](https://github.com/rust-osdev/uefi-rs/pull/1152)

<!-- - [fix(deps): update rust crate serde_json to v1.0.115](https://github.com/rust-osdev/uefi-rs/pull/1111) -->
<!-- - [chore(deps): update rust crate trybuild to v1.0.91](https://github.com/rust-osdev/uefi-rs/pull/1110) -->
<!-- - [fix(deps): update rust crate syn to v2.0.57](https://github.com/rust-osdev/uefi-rs/pull/1112) -->
<!-- - [chore(deps): update rust crate bitflags to v2.5.0](https://github.com/rust-osdev/uefi-rs/pull/1113) -->
<!-- - [fix(deps): update rust crate ucs2 to v0.3.3](https://github.com/rust-osdev/uefi-rs/pull/1122) -->
<!-- - [fix(deps): update rust crate syn to v2.0.58](https://github.com/rust-osdev/uefi-rs/pull/1121) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.20.8](https://github.com/rust-osdev/uefi-rs/pull/1132) -->
<!-- - [fix(deps): update rust crate anyhow to v1.0.82](https://github.com/rust-osdev/uefi-rs/pull/1133) -->
<!-- - [fix(deps): update rust crate quote to v1.0.36](https://github.com/rust-osdev/uefi-rs/pull/1136) -->
<!-- - [fix(deps): update rust crate proc-macro2 to v1.0.80](https://github.com/rust-osdev/uefi-rs/pull/1135) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.20.9](https://github.com/rust-osdev/uefi-rs/pull/1146) -->
<!-- - [fix(deps): update rust crate proc-macro2 to v1.0.81](https://github.com/rust-osdev/uefi-rs/pull/1147) -->
<!-- - [fix(deps): update rust crate syn to v2.0.60](https://github.com/rust-osdev/uefi-rs/pull/1149) -->
<!-- - [fix(deps): update rust crate serde_json to v1.0.116](https://github.com/rust-osdev/uefi-rs/pull/1148) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1150) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.20.10](https://github.com/rust-osdev/uefi-rs/pull/1153) -->
<!-- - [fix(deps): update rust crate ureq to v2.9.7](https://github.com/rust-osdev/uefi-rs/pull/1154) -->

Thanks to [@sky5454](https://github.com/sky5454) for their contributions!

### [`ucs2-rs`](https://github.com/rust-osdev/ucs2-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

- [Add `ucs2_cstr!` macro](https://github.com/rust-osdev/ucs2-rs/pull/21)
- [release: 0.3.3](https://github.com/rust-osdev/ucs2-rs/pull/22)


### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS. We merged the following changes this month:

- [AML: Allow Field in ToInteger (rebased)](https://github.com/rust-osdev/acpi/pull/213)

Thanks to [@rw-vanc](https://github.com/rw-vanc) for their contribution!


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PRs this month:

- [docs: fix and detect warnings](https://github.com/rust-osdev/x86_64/pull/475)
- [docs: add aliases for `in{,b,w,l}` and `out{,b,w,l}`](https://github.com/rust-osdev/x86_64/pull/474)
- [ci: migrate away from unmaintained actions](https://github.com/rust-osdev/x86_64/pull/478)
- [chore: migrate from legacy `rust-toolchain` to `rust-toolchain.toml`](https://github.com/rust-osdev/x86_64/pull/479)
- [test: replace `x86_64-bare-metal.json` with `x86_64-unknown-none`](https://github.com/rust-osdev/x86_64/pull/477)
- [fix and detect warnings](https://github.com/rust-osdev/x86_64/pull/476)
- [CI: Set `-Crelocation-model=static` in `RUSTFLAGS` for bootloader test job](https://github.com/rust-osdev/x86_64/pull/480)
- [silence warning about cast](https://github.com/rust-osdev/x86_64/pull/482)
- [Only enable instructions on `x86_64`](https://github.com/rust-osdev/x86_64/pull/483)

Thanks to [@mkroening](https://github.com/mkroening) for their contributions!


### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following improvements:

- [CI: Fix macOS errors](https://github.com/rust-osdev/bootloader/pull/435)
- [[v0.9] Silence dead code warning](https://github.com/rust-osdev/bootloader/pull/436)
- [[v0.9] Rename `.cargo/config` to `.cargo/config.toml`](https://github.com/rust-osdev/bootloader/pull/437)


### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

The `multiboot2` crate provides abstraction types for the multiboot information structure (MBI) of multiboot2 bootloaders. We merged the following changes this month:

- [misc: various improvements](https://github.com/rust-osdev/multiboot2/pull/208)
<!-- - [build(deps): bump crate-ci/typos from 1.16.26 to 1.19.0](https://github.com/rust-osdev/multiboot2/pull/206)
- [build(deps): bump cachix/install-nix-action from 24 to 26](https://github.com/rust-osdev/multiboot2/pull/207)
- [build(deps): bump actions/cache from 3 to 4](https://github.com/rust-osdev/multiboot2/pull/200) -->


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

### [RatCornu/efs](https://codeberg.org/RatCornu/efs)

[`efs`](https://crates.io/efs) is a recently published `no-std` library which provides an OS and architecture independent implementation of some UNIX filesystems in Rust.

Currently only the [ext2 filesystem](https://fr.wikipedia.org/wiki/Ext2) is directly implemented, but I will soonly work on other filesystems!

It's still young so it may contain bugs, but it's hugely tested so that it does not happen.

Some of the features provided :

* `no_std` support (enabled by default)

* General interface for UNIX files and filesystems

* `read/write` regular files

* retrieve, add and remove directory entries directly from a path and a current working directory.

I hope you will find this useful! If you have any remark, idea or issue, do not hesitate to submit an issue!

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
