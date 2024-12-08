+++
title = "This Month in Rust OSDev: November 2024"
date = 2024-12-08

[extra]
month = "November 2024"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (November 2024)" post.
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

### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PRs this month:

- [fix field order for INVPCID descriptor](https://github.com/rust-osdev/x86_64/pull/508)
- [fix CI job for building on MSRV](https://github.com/rust-osdev/x86_64/pull/510)
- [gate HandlerFunc behind target_arch = "x86{_64}"](https://github.com/rust-osdev/x86_64/pull/507)
- [fix typo in "InvPicdCommand"](https://github.com/rust-osdev/x86_64/pull/509)
- [TryFrom implementation for ExceptionVector](https://github.com/rust-osdev/x86_64/pull/506)
- [Typo fix in TaskStateSegment comment](https://github.com/rust-osdev/x86_64/pull/504)
- [Minor clarification DescriptorTablePointer::limit comment](https://github.com/rust-osdev/x86_64/pull/503)
- [fix signature of Step::steps_between implementations](https://github.com/rust-osdev/x86_64/pull/513)
- [release 0.15.2](https://github.com/rust-osdev/x86_64/pull/519)
- [backport #513](https://github.com/rust-osdev/x86_64/pull/520)

Thanks to [@mrjbom](https://github.com/mrjbom) for their contribution!


### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS. We merged the following changes this month:

- [Implement the multiprocessor wakeup mechanism.](https://github.com/rust-osdev/acpi/pull/225)
- [acpi-dumper: fix invocation of compile_error!](https://github.com/rust-osdev/acpi/pull/229)
- [aml: make resource descriptor fields public](https://github.com/rust-osdev/acpi/pull/228)
- [acpi: fix clippy warnings and run clippy in CI](https://github.com/rust-osdev/acpi/pull/230)
- [acpi: fix doc warnings and add missing links](https://github.com/rust-osdev/acpi/pull/231)
- [acpi: spcr: fix typo](https://github.com/rust-osdev/acpi/pull/232)

Thanks to [@pjhades](https://github.com/pjhades), [@00xc](https://github.com/00xc), and [@Hsy-Intel](https://github.com/Hsy-Intel) for their contributions!


### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following improvements:

- [Remove 3dnow features from stage4 target](https://github.com/rust-osdev/bootloader/pull/471)
- [mention E820 in docs for UnknownBios](https://github.com/rust-osdev/bootloader/pull/461)
- [release v0.11.8](https://github.com/rust-osdev/bootloader/pull/469)
- [copy more PML4 entries](https://github.com/rust-osdev/bootloader/pull/466)
- [Convert LF to CRLF when writing to serial port](https://github.com/rust-osdev/bootloader/pull/474)
- [Update x86_64](https://github.com/rust-osdev/bootloader/pull/478)
- [release v0.11.9](https://github.com/rust-osdev/bootloader/pull/479)

Thanks to [@Ollrogge](https://github.com/Ollrogge), [@ChocolateLoverRaj](https://github.com/ChocolateLoverRaj), and [@Makonede](https://github.com/Makonede) for their contributions!


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [chore(config): migrate renovate config](https://github.com/rust-osdev/uefi-rs/pull/1456)
- [uefi: Deny clippy::ref_as_ptr](https://github.com/rust-osdev/uefi-rs/pull/1455)
- [Move the UnicodeCollation protocol definition to `uefi-raw` and use it from `uefi`](https://github.com/rust-osdev/uefi-rs/pull/1459)
- [Use ovmf-prebuilt](https://github.com/rust-osdev/uefi-rs/pull/1454)
- [xtask: Update OVMF prebuilts](https://github.com/rust-osdev/uefi-rs/pull/1463)
- [uefi-raw: Drop unused dependency on ptr_meta](https://github.com/rust-osdev/uefi-rs/pull/1465)
- [Fix `cargo xtask fmt --check`](https://github.com/rust-osdev/uefi-rs/pull/1464)
- [Fix minor typo in protocols.md](https://github.com/rust-osdev/uefi-rs/pull/1467)
- [Add TCG protocols to `uefi-raw` and use them from `uefi`](https://github.com/rust-osdev/uefi-rs/pull/1469)
- [Mention to turn off secure boot.](https://github.com/rust-osdev/uefi-rs/pull/1468)
- [Clean up some use of unsafe in MemoryMapRefMut](https://github.com/rust-osdev/uefi-rs/pull/1483)

<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1457) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.27.0](https://github.com/rust-osdev/uefi-rs/pull/1460) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1461) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.27.3](https://github.com/rust-osdev/uefi-rs/pull/1471) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1475) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1479) -->
<!-- - [chore(deps): update codecov/codecov-action action to v5](https://github.com/rust-osdev/uefi-rs/pull/1478) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1482) -->

Thanks to [@diekmann](https://github.com/diekmann) for their contribution!


### [`ovmf-prebuilt`](https://github.com/rust-osdev/ovmf-prebuilt)
<span class="maintainers">Maintained by [@nicholasbishop](https://github.com/nicholasbishop) and [@phil-opp](https://github.com/phil-opp)</span>

The `ovmf-prebuilt` project provides pre-built [edk2](https://github.com/tianocore/edk2) releases to make it easier to set up OVMF. We merged the following improvement this month:

- [ovmf-prebuilt: Update readme](https://github.com/rust-osdev/ovmf-prebuilt/pull/101)
- [release: 0.2.1](https://github.com/rust-osdev/ovmf-prebuilt/pull/102)

<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/105) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/106) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/107) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/109) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/110) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/111) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/112) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/113) -->


### [`uart_16550`](https://github.com/rust-osdev/uart_16550)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `uart_16550` crate provides basic support for serial port I/O for 16550-compatible UARTs. We merged the following change this month:

- [feat(mmio): add `new_with_stride`](https://github.com/rust-osdev/uart_16550/pull/36)

Thanks to [@kouchekiniad](https://github.com/kouchekiniad) for their contribution!

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
