+++
title = "This Month in Rust OSDev: June 2025"
date = 2025-07-08

[extra]
month = "June 2025"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (June 2025)" post.
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

- [Updating Uefi Raw for EFI Shell Protocol](https://github.com/rust-osdev/uefi-rs/pull/1680)
- [SNP Integration Test: Improve Clarity](https://github.com/rust-osdev/uefi-rs/pull/1621)
- [ci: Temporarily pin to older nightly](https://github.com/rust-osdev/uefi-rs/pull/1689)
- [fix typo `ctr16!` to `cstr16!`](https://github.com/rust-osdev/uefi-rs/pull/1686)
- [xtask: Allow clippy::collapsible_if lint](https://github.com/rust-osdev/uefi-rs/pull/1694)
- [uefi: Fix clippy::uninlined_format_args lint](https://github.com/rust-osdev/uefi-rs/pull/1693)
- [test-runner: Fix clippy::uninlined_format_args lint](https://github.com/rust-osdev/uefi-rs/pull/1692)
- [uefi: Fix clippy::unnecessary_unwrap lint](https://github.com/rust-osdev/uefi-rs/pull/1691)
- [Fix mismatched_lifetime_syntaxes lint](https://github.com/rust-osdev/uefi-rs/pull/1690)
- [uefi: Make PciIoAddress orderable and hashable](https://github.com/rust-osdev/uefi-rs/pull/1682)
- [uefi: Fix io-align == 0 edgecase handling for ata & nvme](https://github.com/rust-osdev/uefi-rs/pull/1698)
- [uefi-raw: Add HII_CONFIG_ACCESS and CONFIG_KEYWORD_HANDLER protocol bindings](https://github.com/rust-osdev/uefi-rs/pull/1683)
- [doc/uefi: improve Protocol documentation](https://github.com/rust-osdev/uefi-rs/pull/1612)
- [Fix lints/tests on nightly and unpin the CI nightly version](https://github.com/rust-osdev/uefi-rs/pull/1703)
- [uefi-services: prepare v0.26.0 release to accelerate migration/deprecation](https://github.com/rust-osdev/uefi-rs/pull/1709)
- [uefi-services: remove again](https://github.com/rust-osdev/uefi-rs/pull/1712)
- [uefi/test-runner: Remove `crate::` and `uefi::` prefix from Status](https://github.com/rust-osdev/uefi-rs/pull/1714)

<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1681) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.33.1](https://github.com/rust-osdev/uefi-rs/pull/1696) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1697) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1702) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1707) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1715) -->

Thanks to [@seijikun](https://github.com/seijikun), [@nicholasbishop](https://github.com/nicholasbishop), [@phip1611](https://github.com/phip1611), [@renovate[bot]](https://github.com/apps/renovate), [@RenTrieu](https://github.com/RenTrieu), and [@diamant3](https://github.com/diamant3) for their contributions!


### [`ovmf-prebuilt`](https://github.com/rust-osdev/ovmf-prebuilt)
<span class="maintainers">Maintained by [@nicholasbishop](https://github.com/nicholasbishop) and [@phil-opp](https://github.com/phil-opp)</span>

The `ovmf-prebuilt` project provides pre-built [edk2](https://github.com/tianocore/edk2) releases to make it easier to set up OVMF. We merged the following improvement this month:

<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/181)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/182)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/183)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/185) -->
- [Split Source constants into a new file](https://github.com/rust-osdev/ovmf-prebuilt/pull/186)
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/187) -->


### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

_Convenient and safe parsing of Multiboot2 Boot Information (MBI) structures and
the contained information tags. Usable in no_std environments, such as a kernel.
An optional builder feature also allows the construction of the corresponding
structures._

We merged the following PRs this month:

- [update workspace, Rust edition 2024, fixes](https://github.com/rust-osdev/multiboot2/pull/267)
<!-- - [build(deps): bump crate-ci/typos from 1.31.2 to 1.32.0](https://github.com/rust-osdev/multiboot2/pull/269)
- [build(deps): bump uefi-raw from 0.8.0 to 0.11.0](https://github.com/rust-osdev/multiboot2/pull/268) -->


### [`virtio-spec-rs`](https://github.com/rust-osdev/virtio-spec-rs)
<span class="maintainers">Maintained by [@mkroening](https://github.com/mkroening)</span>

The `virtio-spec` crate provides definitions from the Virtual I/O Device (VIRTIO) specification. 
This project aims to be unopinionated regarding actual VIRTIO drivers that are implemented on top of this crate.

We merged the following PRs this month:

- [ci: run Miri](https://github.com/rust-osdev/virtio-spec-rs/pull/4)
<!-- - [build(deps): upgrade dependencies](https://github.com/rust-osdev/virtio-spec-rs/pull/3) -->
- [fix(net): make HdrGso an enum instead of a flag](https://github.com/rust-osdev/virtio-spec-rs/pull/2)
- [add support of console](https://github.com/rust-osdev/virtio-spec-rs/pull/1)
- [chore: release version 0.3.0](https://github.com/rust-osdev/virtio-spec-rs/pull/5)

Thanks to [@stlankes](https://github.com/stlankes) for their contribution!


### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following improvements:

- [Document physical memory mapping size](https://github.com/rust-osdev/bootloader/pull/506)

Thanks to [@aaronzper](https://github.com/aaronzper) for their contribution!


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PRs this month:

- [Add "FMask" alias for SFMask.](https://github.com/rust-osdev/x86_64/pull/552)

Thanks to [@ChocolateLoverRaj](https://github.com/ChocolateLoverRaj) for their contribution!


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


<span class="gray">No projects updates were submitted this month.</span>

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
