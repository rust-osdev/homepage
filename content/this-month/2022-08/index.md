+++
title = "This Month in Rust OSDev: August 2022"
date = 0000-01-01

[extra]
month = "August 2022"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (August 2022)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## `rust-osdev` Projects

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

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

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS.

We merged the following changes in August:

- [Add `DiskIo` and `DiskIo2` protocols](https://github.com/rust-osdev/uefi-rs/pull/467)
- [Added macros `print!` and `println!`](https://github.com/rust-osdev/uefi-rs/pull/430)
- [Use `doc_auto_cfg` to show feature requirements on docs.rs](https://github.com/rust-osdev/uefi-rs/pull/487)
- [add `Ord` and `PartialOrd` for `Guid` struct](https://github.com/rust-osdev/uefi-rs/pull/493)
- [QEMU/OVMF improvements](https://github.com/rust-osdev/uefi-rs/pull/474)
- [Update `nix` requirement from 0.24.1 to 0.25.0](https://github.com/rust-osdev/uefi-rs/pull/480)
- [Update deprecation version in `ScopedProtocol::interface`](https://github.com/rust-osdev/uefi-rs/pull/485)
- [Relax version requirements for various deps](https://github.com/rust-osdev/uefi-rs/pull/482)

We also published a subset of the above changes as a new `v0.16.1` release:

- [Cherry-pick fixes to the `version-0.16` branch](https://github.com/rust-osdev/uefi-rs/pull/490)
- [Update changelog for 0.16.1](https://github.com/rust-osdev/uefi-rs/pull/492)

Thanks to [@kendase3](https://github.com/kendase3), [@JonahPlusPlus](https://github.com/JonahPlusPlus), and [@e820](https://github.com/e820) for their contributions!

### [`xhci`](https://github.com/rust-osdev/xhci)

<span class="maintainers">Maintained by [@toku-sa-n](https://github.com/toku-sa-n)</span>

The `xhci` crate provides types of xHCI structires, such as Registers and TRBs.

We merged the following changes this month:

- [Rename `InterruptRegisterSet` to `InterrupterRegisterSet`](https://github.com/rust-osdev/xhci/pull/143) <span class="gray">([published as `0.8.7`](https://github.com/rust-osdev/xhci/pull/144))</span>
- [Allow updating single fields of InterrupterRegisterSet](https://github.com/rust-osdev/xhci/pull/142) <span class="gray">([published as `0.9.0`](https://github.com/rust-osdev/xhci/pull/145))</span>

Thanks to [@Demindiro](https://github.com/Demindiro) for their contribution!


### [`pci_types`](https://github.com/rust-osdev/pci_types)

<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>


The `pci_types` library provides types for accessing and configuring PCI devices from Rust operating systems.

We merged the following change in August:

- [64-bit `BAR` clarification](https://github.com/rust-osdev/pci_types/pull/6)

Thanks to [@0Killian](https://github.com/0Killian) for this contribution!


### [`acpi`](https://github.com/rust-osdev/acpi)

<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods), [@Restioson](https://github.com/Restioson), and [@Gegy](https://github.com/Gegy)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS.

We merged the following changes this month:


- [fix `HpetInfo::num_comparators`](https://github.com/rust-osdev/acpi/pull/121)
- [Various comments, fix `PM_TMR_BLK` checking of `PM_TMR_LEN` to match spec](https://github.com/rust-osdev/acpi/pull/123)

Thanks to [@semiviral](https://github.com/semiviral), and [@Freax13](https://github.com/Freax13) for their contributions!


## Call for Participation

Want to contribute to a Rust OSDev project, but don't know where to start? Help with one of these outstanding issues!

<!--
    Please use the following template for adding items:
    - [(`repo_name`) Issue Description](https://example.com/link-to-issue)
-->

If you maintain a Rust project related to operating system development and are looking for contributors, especially for tasks suited to people getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the `next` branch with the tasks you want to include in the next issue.

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`cdrzewiecki/celos`](https://gitlab.com/cdrzewiecki/celos)
<span class="maintainers">(Section written by [@drzewiec](https://github.com/drzewiec))</span>

It's been a while since my last project update! That's partly because life has been busy, but also because this update concerns a huge feature. I'm very pleased to report that I have been able to add preemptive multitasking to CelOS.

This was feature that took a good bit of foundation to be able to implement (hence why it took me so long). I had to spend a good bit of time getting memory allocation (both physical and virtual) into a happier place, as well as add support for ACPI and the APIC. And, of course, there were many snags along the way as I learned (at least some of) the traps that are easy to step into when doing something as delicate as context switching.

Now that I have finished this key feature, I plan to work on setting up the other infrastructure needed to begin writing services in userspace (such as message passing and synchronization primitives). And, hopefully soon, finally make the jump into ring 3!

As always, many thanks to [@phil-opp](https://github.com/phil-opp) for his hard work on supporting the Rust osdev community, and for writing the [apic](https://github.com/rust-osdev/apic) crate which helped serve as a sanity check while I wrote my own driver for the IOAPIC and LAPIC. Thanks as well to the maintainers of the excellent [acpi](https://github.com/rust-osdev/acpi) crate, you guys are doing incredible work out there!

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).



<!--
TODO: Update publication date
-->
