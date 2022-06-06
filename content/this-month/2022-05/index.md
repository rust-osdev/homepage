+++
title = "This Month in Rust OSDev: May 2022"
date = 0000-01-01

[extra]
month = "May 2022"
authors = [
    "phil-opp",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (May 2022)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri) and [@nicholasbishop](https://github.com/nicholasbishop)</span>

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS.

We merged the following changes in May:

#### Improvements

- [Change logger max level to be set by feature](https://github.com/rust-osdev/uefi-rs/pull/429)
- [Release new versions of the crates](https://github.com/rust-osdev/uefi-rs/pull/432) <span class="gray">(published `uefi v0.16.0`, `uefi-macros v0.7.0`, and `uefi-services v0.13.0`)</span>
- [Implement `Deref` and `DerefMut` for `ScopedProtocol`](https://github.com/rust-osdev/uefi-rs/pull/434)
- [Implement `core::fmt::Write` for `Serial`](https://github.com/rust-osdev/uefi-rs/pull/437)
- [Add documentation links](https://github.com/rust-osdev/uefi-rs/pull/426)

#### Fixes

- [Fix an accidental `*const` conversion](https://github.com/rust-osdev/uefi-rs/pull/423)
- [Fix compilation of the xtask package under Windows and add to CI](https://github.com/rust-osdev/uefi-rs/pull/438)
- [Switch back to automatic `Debug` derive for `Header` struct](https://github.com/rust-osdev/uefi-rs/pull/435)
- [Check table version before calling UEFI 2.0+ functions](https://github.com/rust-osdev/uefi-rs/pull/436)

Thanks to [@JonahPlusPlus](https://github.com/JonahPlusPlus), [@raccog](https://github.com/raccog), and [@verticalegg](https://github.com/verticalegg) for their contributions!

### [`bootloader`](https://github.com/rust-osdev/bootloader)

<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@rybot666](https://github.com/rybot666), and [@64](https://github.com/64)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following new feature:

- [Add UEFI PXE support](https://github.com/rust-osdev/bootloader/pull/237) to load a kernel from a TFTP server

This feature will be part of the upcoming `v0.11` release. Thanks to [@Freax13](https://github.com/Freax13) for this contribution!

### [`pci_types`](https://github.com/rust-osdev/pci_types)

The `pci_types` library provides types for accessing and configuring PCI devices from Rust operating systems. Lots of this code (e.g. identifying devices by class codes) can be shared
between projects, and would benefit from community contributions.

We merged the following change in May:

- [PCI capabilities and status register support](https://github.com/rust-osdev/pci_types/pull/3)

Thanks to [@alesharik](https://github.com/alesharik) for this contribution!

### [`xhci`](https://github.com/rust-osdev/xhci)

<span class="maintainers">Maintained by [@toku-sa-n](https://github.com/toku-sa-n)</span>

The `xhci` crate provides types of xHCI structires, such as Registers and TRBs.

We merged the following fix this month:

- [fix(StructuralParameters2): Bit range in `max_scratchpad_buffers_hi`](https://github.com/rust-osdev/xhci/pull/134) <span class="gray">([published](https://github.com/rust-osdev/xhci/pull/135) as `v0.8.4`)</span>

Thanks to [@Yuna-Tomi](https://github.com/Yuna-Tomi) for this contribution!

## Call for Participation

Want to contribute to a Rust OSDev project, but don't know where to start? Pick up one of these outstanding
issues in one of our projects and get started!

<!--
Please use the following template for adding items:
- [(`repo_name`) Issue Description](https://example.com/link-to-issue)
-->

<span class="gray">

_No tasks were proposed for this section this month._

</span>

If you maintain a Rust project related to operating system development and are looking for contributors, especially for tasks suited to people getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the `next` branch with the tasks you want to include in the next issue.

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<span class="gray">

_No projects were proposed for this section this month._

</span>

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).


<!--
TODO: Update publication date
-->
