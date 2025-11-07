+++
title = "This Month in Rust OSDev: October 2025"
date = 2025-11-05

[extra]
month = "October 2025"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (October 2025)" post.
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

### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS. We merged the following changes this month:

- [aarch64 fixes](https://github.com/rust-osdev/acpi/pull/261)

Thanks to [@jackpot51](https://github.com/jackpot51) for their contributions!


### [`ovmf-prebuilt`](https://github.com/rust-osdev/ovmf-prebuilt)
<span class="maintainers">Maintained by [@nicholasbishop](https://github.com/nicholasbishop) and [@phil-opp](https://github.com/phil-opp)</span>

The `ovmf-prebuilt` project provides pre-built [edk2](https://github.com/tianocore/edk2) releases to make it easier to set up OVMF. We merged the following improvement this month:

- [Add loongarch support](https://github.com/rust-osdev/ovmf-prebuilt/pull/211)
- [Update the riscv64 toolchain](https://github.com/rust-osdev/ovmf-prebuilt/pull/216)
- [feat: add LoongArch64 architecture support and update to latest prebuilt version](https://github.com/rust-osdev/ovmf-prebuilt/pull/217)
- [release: 0.2.4 with edk2-stable202508-r1](https://github.com/rust-osdev/ovmf-prebuilt/pull/218)

<!-- - [chore(deps): update actions/checkout action to v5](https://github.com/rust-osdev/ovmf-prebuilt/pull/197) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/220) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/221) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/222) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/223) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/224) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/225) -->

Thanks to [@ZR233](https://github.com/ZR233) for their contribution!


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [ci: fix latest nightly issues](https://github.com/rust-osdev/uefi-rs/pull/1773)
- [style: apply/fix more clippy lints (from cargo::nursery)](https://github.com/rust-osdev/uefi-rs/pull/1761)
- [uefi-raw: net: add convenient into_* helpers](https://github.com/rust-osdev/uefi-rs/pull/1778)
- [uefi-raw: net: code improvements](https://github.com/rust-osdev/uefi-rs/pull/1776)
- [uefi-raw: net: add convenient From impls + integration with core::net](https://github.com/rust-osdev/uefi-rs/pull/1777)
- [uefi: Improve AlignedBuffer API with more accessors and copy_from_iter()](https://github.com/rust-osdev/uefi-rs/pull/1787)
- [uefi: Add bindings for the HII_CONFIG_ROUTING protocol](https://github.com/rust-osdev/uefi-rs/pull/1753)
- [uefi: document udp_read of PXE protocol + improve test](https://github.com/rust-osdev/uefi-rs/pull/1785)
- [uefi-raw: add unit test for typical high-level net API usage](https://github.com/rust-osdev/uefi-rs/pull/1781)
- [uefi: Wrap AtaPassThruProtocol in UnsafeCell](https://github.com/rust-osdev/uefi-rs/pull/1782)
- [uefi: Wrap ScsiPassThruProtocol in UnsafeCell](https://github.com/rust-osdev/uefi-rs/pull/1792)
- [uefi: Wrap NmvePassThruProtocol in UnsafeCell](https://github.com/rust-osdev/uefi-rs/pull/1793)
- [uefi: use core::net-types in public API + remove duplications](https://github.com/rust-osdev/uefi-rs/pull/1780)
- [uefi: remove inconsistent println!](https://github.com/rust-osdev/uefi-rs/pull/1794)
- [style: apply/fix more clippy lints (from cargo::nursery) (2/2)](https://github.com/rust-osdev/uefi-rs/pull/1774)
- [release: uefi-raw-0.12.0, uefi-macros-0.19.0, uefi-0.36.0](https://github.com/rust-osdev/uefi-rs/pull/1795)
- [uefi-raw/uefi: Replace doc_auto_cfg with doc_cfg](https://github.com/rust-osdev/uefi-rs/pull/1796)
- [uefi-raw: Replace remaining uses of bool with Boolean](https://github.com/rust-osdev/uefi-rs/pull/1801)
- [xtask: Check uefi-raw for uses of the primitive bool type](https://github.com/rust-osdev/uefi-rs/pull/1802)
- [uefi: improve doc for device path construction](https://github.com/rust-osdev/uefi-rs/pull/1800)

<!-- - [chore(deps): update crate-ci/typos action to v1.37.1](https://github.com/rust-osdev/uefi-rs/pull/1772) -->
<!-- - [chore(deps): update codecov/codecov-action action to v5.5.1](https://github.com/rust-osdev/uefi-rs/pull/1771) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.37.2](https://github.com/rust-osdev/uefi-rs/pull/1775) -->
<!-- - [chore(deps): update rust crate ovmf-prebuilt to v0.2.4](https://github.com/rust-osdev/uefi-rs/pull/1784) -->
<!-- - [chore(deps): update rust crate fs-err to v3.1.3](https://github.com/rust-osdev/uefi-rs/pull/1783) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1788) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.38.1](https://github.com/rust-osdev/uefi-rs/pull/1790) -->

Thanks to [@seijikun](https://github.com/seijikun) for their contribution!

### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

_Convenient and safe parsing of Multiboot2 Boot Information (MBI) structures and
the contained information tags. Usable in no_std environments, such as a kernel.
An optional builder feature also allows the construction of the corresponding
structures._

We merged the following PRs this month:

- [ci: dependabot auto-merge](https://github.com/rust-osdev/multiboot2/pull/274)
- [misc updates & improvements](https://github.com/rust-osdev/multiboot2/pull/275)
- [workspace: unify](https://github.com/rust-osdev/multiboot2/pull/276)
- [clippy: lifetime warnings](https://github.com/rust-osdev/multiboot2/pull/278)

<!-- - [build(deps): bump crate-ci/typos from 1.35.7 to 1.37.0](https://github.com/rust-osdev/multiboot2/pull/273) -->
<!-- - [build(deps): bump crate-ci/typos from 1.37.0 to 1.37.2](https://github.com/rust-osdev/multiboot2/pull/277) -->


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


<!-- <span class="gray">No projects updates were submitted this month.</span> -->


## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
