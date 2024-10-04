+++
title = "This Month in Rust OSDev: September 2024"
date = 2024-10-03

[extra]
month = "September 2024"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (September 2024)" post.
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

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [Fix unused warnings if log-debugcon is not enabled](https://github.com/rust-osdev/uefi-rs/pull/1389)
- [Add uefi::{boot,runtime,system} modules to the prelude](https://github.com/rust-osdev/uefi-rs/pull/1390)
- [uefi: Add panic doc to uefi::helpers::init](https://github.com/rust-osdev/uefi-rs/pull/1391)
- [uefi: Add uefi::runtime::variable_exists](https://github.com/rust-osdev/uefi-rs/pull/1392)
- [release: uefi-raw-0.8.0, uefi-macros-0.16.0, uefi-0.32.0](https://github.com/rust-osdev/uefi-rs/pull/1396)
- [Update the hello-world tutorial](https://github.com/rust-osdev/uefi-rs/pull/1397)
- [uefi: Delete the deprecated helpers::system_table function](https://github.com/rust-osdev/uefi-rs/pull/1398)
- [Drop FileSystem conversion from table::boot::ScopedProtocol](https://github.com/rust-osdev/uefi-rs/pull/1399)
- [test-runner: Remove BootServices from many protocol tests](https://github.com/rust-osdev/uefi-rs/pull/1405)
- [Delete deprecated RuntimeServices struct](https://github.com/rust-osdev/uefi-rs/pull/1404)
- [test-runner: Remove BootServices from remaining proto tests](https://github.com/rust-osdev/uefi-rs/pull/1406)
- [Fix handling of a null interface pointer in `boot::open_protocol`](https://github.com/rust-osdev/uefi-rs/pull/1410)
- [test-runner: Remove remaining tests for deprecated table types](https://github.com/rust-osdev/uefi-rs/pull/1415)
- [uefi: Delete deprecated allocator functions](https://github.com/rust-osdev/uefi-rs/pull/1416)

<!-- - [test-runner: Remove accidental debug log](https://github.com/rust-osdev/uefi-rs/pull/1412) -->
<!-- - [nix/niv: update formatter (nixpkgs-fmt is deprecated)](https://github.com/rust-osdev/uefi-rs/pull/1395) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.24.3](https://github.com/rust-osdev/uefi-rs/pull/1387) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1388) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.24.5](https://github.com/rust-osdev/uefi-rs/pull/1393) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1394) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1402) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.24.6](https://github.com/rust-osdev/uefi-rs/pull/1407) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1409) -->
<!-- - [chore(deps): update dependency ubuntu to v24](https://github.com/rust-osdev/uefi-rs/pull/1411) -->
<!-- - [chore(deps): update cachix/install-nix-action action to v29](https://github.com/rust-osdev/uefi-rs/pull/1413) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1414) -->
<!-- - [fix(deps): update rust crate regex to v1.11.0](https://github.com/rust-osdev/uefi-rs/pull/1420) -->
<!-- - [fix(deps): update rust crate syn to v2.0.79](https://github.com/rust-osdev/uefi-rs/pull/1419) -->


### [`xhci`](https://github.com/rust-osdev/xhci)
<span class="maintainers">Maintained by [@toku-sa-n](https://github.com/toku-sa-n)</span>

The `xhci` crate provides types of xHCI structures, such as Registers and TRBs. We merged the following PRs this month:

- [removed deprecated clippy check option](https://github.com/rust-osdev/xhci/pull/174)

Thanks to [@dbydd](https://github.com/dbydd) for their contributions!



### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS. We merged the following changes this month:

- [aml_tester:Make cli print more pretty](https://github.com/rust-osdev/acpi/pull/221)
- [Ignore `MultiprocessorWakeup` when parse interrupt model](https://github.com/rust-osdev/acpi/pull/220)

Thanks to [@jokemanfire](https://github.com/jokemanfire), and [@Hsy-Intel](https://github.com/Hsy-Intel) for their contributions!


### [`pci_types`](https://github.com/rust-osdev/pci_types)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `pci_types` library provides types for accessing and configuring PCI devices from Rust operating systems. We merged the following change this month:

- [Match the output of lspci in the PciAddress Display impl](https://github.com/rust-osdev/pci_types/pull/34)

Thanks to [@bjorn3](https://github.com/bjorn3) for their contributions!



### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

_Convenient and safe parsing of Multiboot2 Boot Information (MBI) structures and
the contained information tags. Usable in no_std environments, such as a kernel.
An optional builder feature also allows the construction of the corresponding
structures._

We merged the following PRs this month:

<!-- - [build(deps): bump crate-ci/typos from 1.23.6 to 1.24.3](https://github.com/rust-osdev/multiboot2/pull/230) -->
- [crate: fix latest clippy nightly complains](https://github.com/rust-osdev/multiboot2/pull/231)
- [multiboot2: add missing tags](https://github.com/rust-osdev/multiboot2/pull/229)
- [Replaces dead link home page multiboot2-header package definition](https://github.com/rust-osdev/multiboot2/pull/232)
- [multiboot2: streamline getters and public tags()](https://github.com/rust-osdev/multiboot2/pull/235)
- [multiboot2-common: improve documentation](https://github.com/rust-osdev/multiboot2/pull/236)
- [workspace: prepare updates](https://github.com/rust-osdev/multiboot2/pull/233)
- [talk about yanked versions in changelog files](https://github.com/rust-osdev/multiboot2/pull/239)
- [multiboot2-common: improve README and diagrams](https://github.com/rust-osdev/multiboot2/pull/240)

Thanks to [@filiplajszczak](https://github.com/filiplajszczak) for their contributions!


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
