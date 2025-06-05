+++
title = "This Month in Rust OSDev: May 2025"
date = 2025-06-04

[extra]
month = "May 2025"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (May 2025)" post.
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
- [This Month in Redox](https://redox-os.org/news/this-month-250531/)
  - Redox is looking for a developer in the EU - see the job description in our monthly report!

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [Miralis, a RISC-V virtual firmware monitor](https://github.com/CharlyCst/miralis)

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

- [release: uefi-raw-0.11.0, uefi-macros-0.18.1, uefi-0.35.0](https://github.com/rust-osdev/uefi-rs/pull/1652)
- [xtask: Add --ovmf-shell arg](https://github.com/rust-osdev/uefi-rs/pull/1656)
- [xtask: simplify env variables](https://github.com/rust-osdev/uefi-rs/pull/1657)
- [uefi: use Duration for boot::stall](https://github.com/rust-osdev/uefi-rs/pull/1659)
- [xtask: Improve error message for enums in uefi-raw](https://github.com/rust-osdev/uefi-rs/pull/1660)
- [ConfigTableEntry move constants and add example](https://github.com/rust-osdev/uefi-rs/pull/1661)
- [uefi: `system::with_*` now take mutably closure](https://github.com/rust-osdev/uefi-rs/pull/1663)
- [uefi-test-runner: streamline memory related tests](https://github.com/rust-osdev/uefi-rs/pull/1666)
- [rust: edition 2024](https://github.com/rust-osdev/uefi-rs/pull/1586)
- [Unpin uguid](https://github.com/rust-osdev/uefi-rs/pull/1673)
- [uefi: SNP transmit: document parameters](https://github.com/rust-osdev/uefi-rs/pull/1664)
- [doc: improved documentation for boot allocation functions](https://github.com/rust-osdev/uefi-rs/pull/1665)
- [uefi-raw: Add binding for EFI_PCI_ROOT_BRIDGE_IO_PROTOCOL](https://github.com/rust-osdev/uefi-rs/pull/1658)
- [doc: streamline device path documentation between uefi-raw and uefi](https://github.com/rust-osdev/uefi-rs/pull/1641)
- [clippy: fix upcoming nightly lints](https://github.com/rust-osdev/uefi-rs/pull/1675)
- [cleanup IPConfig2/Http Protocol](https://github.com/rust-osdev/uefi-rs/pull/1640)
- [uefi: deprecated since 0.36.0](https://github.com/rust-osdev/uefi-rs/pull/1677)
- [doc: help with feature selection](https://github.com/rust-osdev/uefi-rs/pull/1676)
- [uefi: Add safe EFI_USB_IO_PROTOCOL bindings](https://github.com/rust-osdev/uefi-rs/pull/1625)
- [uefi: Add (partial) safe protocol implementation for PCI_ROOT_BRIDGE_IO_PROTOCOL](https://github.com/rust-osdev/uefi-rs/pull/1674)

<!-- - [chore(deps): update crate-ci/typos action to v1.32.0](https://github.com/rust-osdev/uefi-rs/pull/1654) -->
<!-- - [fix(deps): update rust crate clap to v4.5.38](https://github.com/rust-osdev/uefi-rs/pull/1668) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1669) -->
<!-- - [chore(deps): update rust crate uguid to v2.2.1](https://github.com/rust-osdev/uefi-rs/pull/1653) -->
<!-- - [fix(deps): update rust crate nix to 0.30.0](https://github.com/rust-osdev/uefi-rs/pull/1670) -->
<!-- - [fix(deps): update rust crate mbrman to 0.6.0](https://github.com/rust-osdev/uefi-rs/pull/1671) -->
<!-- - [chore(deps): update codecov/codecov-action action to v5.4.3](https://github.com/rust-osdev/uefi-rs/pull/1672) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1678) -->

Thanks to [@seijikun](https://github.com/seijikun), [@fox0](https://github.com/fox0), and [@JarlEvanson](https://github.com/JarlEvanson) for their contributions!


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PRs this month:

- [feat(sev): add AMD SEV support](https://github.com/rust-osdev/x86_64/pull/542)
- [implement functions for accessing CR8](https://github.com/rust-osdev/x86_64/pull/547)
- [Add page attribute table support](https://github.com/rust-osdev/x86_64/pull/548)

Thanks to [@zyuiop](https://github.com/zyuiop) for their contribution!



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
