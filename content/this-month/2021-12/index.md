+++
title = "This Month in Rust OSDev (December 2021)"
date = 0000-01-01

[extra]
month = "December 2021"
authors = [
    "phil-opp",
    "IsaacWoods",
    "GabrielMajeri",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (December 2021)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

### [`x86_64`](https://github.com/rust-osdev/x86_64)

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

In December, …

### [`acpi`](https://github.com/rust-osdev/acpi)

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS.

December was a fairly quiet month, but [an important bug-fix landed](https://github.com/rust-osdev/acpi/pull/114) that corrected the way we handled `_CRS` objects in a structure
called the `_PRT`, which are found on PCI root bridges and tell the OS how interrupt pins on PCI devices have been routed to the platform's interrupt controller. Each pin can be
hardwired to a specific interrupt, or more commonly, can be dynamically assigned using a 'Link Object' through a set of control methods: `_PRS`, `_CRS`, `_SRS`, and `_DIS`.
However, many platforms implement Link Objects that actually hardcode the interrupts (including QEMU) and this is where the bug slipped in: `_CRS` was being evaluated as a
hardcoded object. We now treat these objects correctly as control methods, supporting properly-configured tables. <span class="gray">(published as `aml v0.16.1`)</span>

Thanks to [@Dentosal](https://github.com/Dentosal) for this contribution!

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS.

We merged the following PRs this month:

- [Implement `open_protocol`, use it to fix flaky screenshot test](https://github.com/rust-osdev/uefi-rs/pull/318)
- [Change `memory_map_size` to return entry size as well](https://github.com/rust-osdev/uefi-rs/pull/326)
- [Document how to publish new versions of the crates](https://github.com/rust-osdev/uefi-rs/pull/322)
- [Improve clippy linting in `build.py` and CI](https://github.com/rust-osdev/uefi-rs/pull/319)
- [`build.py`: fix `clippy --verbose`](https://github.com/rust-osdev/uefi-rs/pull/323)
- [`build.py`: deny warnings when running clippy](https://github.com/rust-osdev/uefi-rs/pull/324)
- [Move `build.py` to the root of the repo](https://github.com/rust-osdev/uefi-rs/pull/334)
- [Fix unused use warning that shows up with some build configs](https://github.com/rust-osdev/uefi-rs/pull/330)
- [Fix build error on latest nightly](https://github.com/rust-osdev/uefi-rs/pull/328)
- [Update the version of `qemu-exit`](https://github.com/rust-osdev/uefi-rs/pull/331)
- [Add missing `#[must_use]` marker attributes](https://github.com/rust-osdev/uefi-rs/pull/332)

Thanks to [@StevenDoesStuffs](https://github.com/StevenDoesStuffs) and [@toku-sa-n](https://github.com/toku-sa-n) for their contributions!

## Call for Participation

Want to contribute to a Rust OSDev project, but don't know where to start? Pick up one of these outstanding
issues in one of our projects and get started!

<!--
Please use the following template for adding items:
- [(`repo_name`) Issue Description](https://example.com/link-to-issue)
-->

<span class="gray">

_No tasks were proposed for this section._

</span>

If you maintain a Rust project related to operating system development and are looking for contributors, especially for tasks suited to people getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the `next` branch with the tasks you want to include in the next issue.

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`metta-systems/vesper`](https://github.com/metta-systems/vesper)

<span class="gray">(Section written by [@berkus](https://github.com/berkus))</span>

Vesper is a capability-based single-address-space nanokernel. This means it is aiming to be small, to provide only isolation primitives; at the same time SAS makes it a lot easier to perform cross-process operations (because all addresses are the same across all processes). It uses capabilities to provide security for such operations, so that unauthorized processes will not be able to intervene in legitimate traffic.

The kernel is in very early stages of development, while I am building up tooling support to make future development fast and painless. This is my second post here and as usual, I will link directly to my blog for more details. [Read the full article here](https://metta.systems/blog/osdev-tooling-2/).

Just a note: since features described in the article are not fully finalized, they are not merged to the main development branch yet and live in [their own branch](https://github.com/metta-systems/vesper/tree/feature/chainboot), which is subject to frequent rebases. Caveat emptor!

### [`rusty-hermit`](https://crates.io/crates/rusty-hermit)

<span class="gray">(Section written by [@stlankes](https://github.com/stlankes))</span>

RustyHermit is a unikernel targeting a scalable and predictable runtime for high-performance and cloud computing.

This month, we integrated a [virtual i/o device driver](https://docs.oasis-open.org/virtio/virtio/v1.1/csprd01/virtio-v1.1-csprd01.html), which is based on memory mapped i/o and doesn't depend on PCI device specification.
For instance, micro VMs like [Firecracker](https://firecracker-microvm.github.io) and Qemu's [microvm machine type](https://qemu.readthedocs.io/en/latest/system/i386/microvm.html) don't support the PCI specification to accelerate the boot time and to improve the performance.
With this device driver, `rusty-hermit` is able to run on Qemu's microvm platform.
We are working to support Firecracker in the near future.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

This month, ...

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).


<!--
TODO: Update publication date
-->
