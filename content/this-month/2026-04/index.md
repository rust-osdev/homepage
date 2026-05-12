+++
title = "This Month in Rust OSDev: April 2026"
date = 2026-05-05

[extra]
month = "April 2026"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

Please submit interesting posts and projects for the next issue [on Zulip](https://rust-osdev.zulipchat.com/#narrow/channel/435142-newsletter/topic/Content.20suggestions/with/580172810) or via a PR [on GitHub](https://github.com/rust-osdev/homepage/).

<span class="gray">
Disclaimer: Automated scripts and AI assistance were used for collecting and categorizing links.
Everything was proofread and checked manually and there were many manual tweaks.
</span>


<!--
    This is a draft for the upcoming "This Month in Rust OSDev (April 2026)" post.
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

- [octopos: xv6 based operating system for RISC-V in Rust](https://www.boranseckin.com/projects/octopos)
  - A Rust port of xv6 that targets RISC-V and runs without the standard library, including process management, Sv39 page tables, VirtIO disk support, and a POSIX-style syscall interface.
- [Bringing Rust to the Pixel Baseband](https://blog.google/security/bringing-rust-to-the-pixel-baseband/)
  - Google describes adding Rust to Pixel modem firmware, including `no_std` support for Hickory DNS dependencies, `core`/`alloc` integration, allocator and panic handler hooks, and firmware linking details.
- [tinyboot v0.4.0 Released -- The API is Stable](https://aaronqian.com/log/2026-04-22-tinyboot-v040-released/)
  - A minimal Rust bootloader for resource-constrained MCUs. This release adds CH32V00x support, collapses the CH32 crates, stabilizes the wire protocol, and fixes half-duplex UART bugs.
- [A minimal VMM in Rust with Apple Hypervisor](https://gigapotential.dev/blog/minimal-vmm-in-rust-with-apple-hypervisor/)
  - Walks through building a small virtual machine monitor on Apple Silicon macOS using Rust bindings to Apple's Hypervisor framework.
- [Rust: Memory safety in kernel space](https://oshub.org/users/OSHub/posts/rust-memory-safety-in-kernel-space-9178dd)
  - An OS-focused introduction to what Rust's ownership and `unsafe` boundaries change for kernel development.
- [Using Rust to Build a $1 Handheld Gaming Console](https://chrisdell.info/using-rust-to-build-a-1-dollar-handheld-gaming-console/)
  - A small embedded Rust project on the CH32V003 RISC-V microcontroller, with notes on RAM-constrained rendering and fixed-point math.

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [Rust 1.95.0](https://blog.rust-lang.org/2026/04/16/Rust-1.95.0/)
  - Stabilizes `cfg_select!`, atomic `update`/`try_update` methods, and allocator layout helpers such as `Layout::{dangling_ptr, repeat, repeat_packed, extend_packed}`.
  - Important for custom-target users: stable `rustc` no longer accepts JSON target specifications. Building `core` for custom targets still requires nightly-only features.
- [Add `-Zsanitize=kernel-hwaddress`](https://github.com/rust-lang/rust/pull/153049)
  - Adds nightly support for the kernel hardware address sanitizer mode used by Linux's `CONFIG_KASAN_SW_TAGS`.
- [Mitigation enforcement RFC](https://github.com/rust-lang/rfcs/pull/3855)
  - Approved RFC for tracking and enforcing target-wide exploit mitigation options such as stack protector settings.
- [build-std: always RFC](https://github.com/rust-lang/rfcs/pull/3874)
  - Proposed Cargo configuration for rebuilding standard library crates, part of the ongoing effort to make `build-std` usable on stable.
- Move `std::io` pieces toward `core::io`
  - [`ErrorKind`](https://github.com/rust-lang/rust/pull/154654) and [`RawOsError`](https://github.com/rust-lang/rust/pull/155574) were moved into `core::io`, continuing the groundwork for more IO abstractions in `no_std` contexts.

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

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS.

We merged the following changes this month:

- [Support multiple tables per test in `aml_tester`](https://github.com/rust-osdev/acpi/pull/284)
- [Implement Index and Bank fields](https://github.com/rust-osdev/acpi/pull/274)
- [Add support for running the tests from uACPI.](https://github.com/rust-osdev/acpi/pull/285)
- [Remove some AML-triggerable panics from library](https://github.com/rust-osdev/acpi/pull/276)
- [Derive Clone, Debug on useful objects](https://github.com/rust-osdev/acpi/pull/287)
- [(minor) Trace the table ID better](https://github.com/rust-osdev/acpi/pull/291)

Thanks to [@martin-hughes](https://github.com/martin-hughes) for their contributions!


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [uefi: make TimerTrigger more pleasant to use](https://github.com/rust-osdev/uefi-rs/pull/1934)
- [uefi-raw: Add SimpleTextInputEx protocol](https://github.com/rust-osdev/uefi-rs/pull/1936)
- [uefi: Add InputEx protocol](https://github.com/rust-osdev/uefi-rs/pull/1937)

<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1926) -->
<!-- - [chore(deps): update codecov/codecov-action action to v5.5.4](https://github.com/rust-osdev/uefi-rs/pull/1923) -->
<!-- - [chore(deps): update dorny/paths-filter action to v4](https://github.com/rust-osdev/uefi-rs/pull/1921) -->
<!-- - [misc: update to typos 1.45](https://github.com/rust-osdev/uefi-rs/pull/1927) -->
<!-- - [chore(deps): update codecov/codecov-action action to v6](https://github.com/rust-osdev/uefi-rs/pull/1924) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1929) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.45.1](https://github.com/rust-osdev/uefi-rs/pull/1930) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1933) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1935) -->

Thanks to [@JarlEvanson](https://github.com/JarlEvanson) for their contributions!


### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following change:

- [Update to v0.11.15 and add missing x86_64-unknown-uefi target ](https://github.com/rust-osdev/bootloader/pull/557)

<!-- - [Bump rand from 0.8.5 to 0.8.6](https://github.com/rust-osdev/bootloader/pull/559) -->
<!-- - [Bump rustls-webpki from 0.103.10 to 0.103.13 in /examples/basic](https://github.com/rust-osdev/bootloader/pull/560) -->

Thanks to [@peppergrayxyz](https://github.com/peppergrayxyz) for their contribution!


### [`linked-list-allocator`](https://github.com/rust-osdev/linked-list-allocator)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@jamesmunns](https://github.com/jamesmunns)</span>

The `linked-list-allocator` crate provides a basic `no_std` allocator that builds a linked list from freed memory blocks and thus needs no additional data structures. We merged the following PR this month:

- [fix(alloc_ref): use renamed `dangling`](https://github.com/rust-osdev/linked-list-allocator/pull/90)

Thanks to [@sermuns](https://github.com/sermuns) for their contribution!


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PR this month:

- [insert NOP after STI](https://github.com/rust-osdev/x86_64/pull/588)

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

### [`willamhou/hypervisor`](https://github.com/willamhou/hypervisor)
<span class="maintainers">(Section written by [@willamhou](https://github.com/willamhou))</span>

An ARM64 Type-1 bare-metal hypervisor written in Rust (`no_std`). It runs at EL2 and manages guest VMs at EL1, targeting the QEMU `virt` machine. The project boots Linux 6.12.12 to a BusyBox shell with 4 vCPUs, virtio-blk storage, and virtio-net inter-VM networking.

Key features:

- **S-EL2 SPMC**: Runs as BL32 in TF-A boot chain, replacing Hafnium. Manages multiple Secure Partitions (SPs) at S-EL1 with per-SP Secure Stage-2 page tables.
- **FF-A v1.1**: Full proxy implementation including DIRECT_REQ/RESP messaging, memory sharing (MEM_SHARE/LEND/DONATE/RETRIEVE/RELINQUISH/RECLAIM), descriptor fragmentation, PARTITION_INFO_GET, notifications, indirect messaging, and CONSOLE_LOG.
- **pKVM integration**: Coexists with Android pKVM at NS-EL2 — our SPMC at S-EL2, pKVM at NS-EL2. 35/35 `ffa_test.ko` tests pass including SP-to-SP DIRECT_REQ relay and SP-to-SP memory sharing through the real SPMD chain.
- **Multi-VM**: 2 Linux VMs time-sliced with VMID-tagged TLBs and per-VM Stage-2 page tables.
- **SMP**: Both 4-vCPU-on-1-pCPU (round-robin scheduler) and 4-vCPU-on-4-pCPU (1:1 affinity) modes.
- **SP-to-SP**: CallStack cycle detection, recursive dispatch, chain preemption, secure virtual interrupt injection via HCR_EL2.VI.
- **34 test suites** with ~457 assertions running on QEMU, plus 20/20 BL33 E2E integration tests.



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
