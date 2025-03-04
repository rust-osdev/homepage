+++
title = "This Month in Rust OSDev: February 2025"
date = 2025-03-04

[extra]
month = "February 2025"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (February 2025)" post.
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

- [This Month in Redox - January 2025](https://www.redox-os.org/news/this-month-250131/)
- [RSoC 2024: Dynamic Linking - Part 2](https://www.redox-os.org/news/02_rsoc2024_dynamic_linker/)
- [Asterinas in 2024](https://asterinas.github.io/2025/01/20/asterinas-in-2024.html)
  - [Asterinas](https://github.com/asterinas/asterinas) is a Linux ABI-compatible OS written from scratch in Rust
- [Towards Practical Formal Verification for a General-Purpose OS in Rust](https://asterinas.github.io/2025/02/13/towards-practical-formal-verification-for-a-general-purpose-os-in-rust.html)
- [Writing a Simple Windows Driver in Rust](https://scorpiosoftware.net/2025/02/08/writing-a-simple-driver-in-rust/)
- [FOSDEM talk: Rust for Linux](https://fosdem.org/2025/schedule/event/fosdem-2025-6507-rust-for-linux/)
- [No-Panic Rust: A Nice Technique for Systems Programming](https://blog.reverberate.org/2025/02/03/no-panic-rust.html)
- [PSA: uuid's getrandom update may break no-std V4/V7 users](https://www.reddit.com/r/rust/comments/1ihd0hf/psa_uuids_getrandom_update_may_break_nostd_v4v7/)
- The Embedded Rustacean: [Issue #39](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-39) and [Issue #40](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-40)

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

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [relicensing: Rewrite PR "DevicePathToText and DevicePathFromText Protocol"](https://github.com/rust-osdev/uefi-rs/pull/1528)
- [Relicense to "MIT OR Apache-2.0"](https://github.com/rust-osdev/uefi-rs/pull/1531)
- [Add SPDX headers](https://github.com/rust-osdev/uefi-rs/pull/1532)
- [release: uefi-raw-0.10.0, uefi-macros-0.18.0, uefi-0.34.0](https://github.com/rust-osdev/uefi-rs/pull/1533)
- [uefi: Update license info in docstring](https://github.com/rust-osdev/uefi-rs/pull/1534)
- [release: uefi-0.34.1](https://github.com/rust-osdev/uefi-rs/pull/1535)
- [uefi-raw: add Boolean type](https://github.com/rust-osdev/uefi-rs/pull/1536)
- [uefi-raw: use new Boolean type ](https://github.com/rust-osdev/uefi-rs/pull/1538)
- [xtask/uefi-raw: improve check-raw error messages](https://github.com/rust-osdev/uefi-rs/pull/1537)
- [Fix upcoming lints in clippy 1.85](https://github.com/rust-osdev/uefi-rs/pull/1541)
- [uefi: clarify situation with `boot::memory_map()` and `Status::BUFFER_TOO_SMALL`](https://github.com/rust-osdev/uefi-rs/pull/1540)
- [Fix uefi-raw MSRV build, and improve CI for MSRV](https://github.com/rust-osdev/uefi-rs/pull/1542)
- [Add PXE types to uefi-raw](https://github.com/rust-osdev/uefi-rs/pull/1543)
- [uefi: Replace definition of MacAddress with a re-export from uefi-raw](https://github.com/rust-osdev/uefi-rs/pull/1547)
- [Use bitflags definitions from uefi-raw in uefi's pxe module](https://github.com/rust-osdev/uefi-rs/pull/1548)
- [uefi: Fix the BufferSize argument in SimpleNetwork::transmit](https://github.com/rust-osdev/uefi-rs/pull/1550)
- [Use newtype enum definitions from uefi-raw in uefi's pxe module](https://github.com/rust-osdev/uefi-rs/pull/1551)

<!-- - [chore(deps): update crate-ci/typos action to v1.29.7](https://github.com/rust-osdev/uefi-rs/pull/1544) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1546) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.29.9](https://github.com/rust-osdev/uefi-rs/pull/1554) -->


### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following improvements:

- [Update `x86_64` to `0.15.2`](https://github.com/rust-osdev/bootloader/pull/490)
- [change rustc-abi in custom targets to x86-softfloat](https://github.com/rust-osdev/bootloader/pull/491)
- [[v0.9] change rustc-abi in custom targets to x86-softfloat](https://github.com/rust-osdev/bootloader/pull/492)
- [Release `v0.11.10`](https://github.com/rust-osdev/bootloader/pull/493)
- [Update uart_16550 to 0.3.2](https://github.com/rust-osdev/bootloader/pull/495)
- [Specify kernel-code virt addr in BootloaderConfig](https://github.com/rust-osdev/bootloader/pull/494)
- [Use `Result::ok`](https://github.com/rust-osdev/bootloader/pull/496)

Thanks to [@Stazer](https://github.com/Stazer), [@Wasabi375](https://github.com/Wasabi375), and [@ChocolateLoverRaj](https://github.com/ChocolateLoverRaj) for their contributions!


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PRs this month:

- [Ensure that addition and subtraction of addresses panics on overflow/underflow](https://github.com/rust-osdev/x86_64/pull/535)
- [don't pass -Crelocation-model=static to host targets](https://github.com/rust-osdev/x86_64/pull/536)

Thanks to [@farnz](https://github.com/farnz) for their contribution!


### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS. We merged the following changes this month:

- [acpi: fix build errors](https://github.com/rust-osdev/acpi/pull/240)

Thanks to [@jellllly420](https://github.com/jellllly420) for their contribution!


### [`ovmf-prebuilt`](https://github.com/rust-osdev/ovmf-prebuilt)
<span class="maintainers">Maintained by [@nicholasbishop](https://github.com/nicholasbishop) and [@phil-opp](https://github.com/phil-opp)</span>

The `ovmf-prebuilt` project provides pre-built [edk2](https://github.com/tianocore/edk2) releases to make it easier to set up OVMF. We merged the following improvement this month:

- [Add 202408.01, 202411, and 202502 releases](https://github.com/rust-osdev/ovmf-prebuilt/pull/139)
- [fix(deps): update rust crate ureq to v3](https://github.com/rust-osdev/ovmf-prebuilt/pull/146)
- [Set `Source::LATEST` to `EDK2_STABLE202502_R1`](https://github.com/rust-osdev/ovmf-prebuilt/pull/145)
- [release: 0.2.2](https://github.com/rust-osdev/ovmf-prebuilt/pull/140)
y
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/135) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/136) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/137) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/138) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/144) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/148) -->

Thanks to [@mkroening](https://github.com/mkroening) for their contributions!




## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
