+++
title = "This Month in Rust OSDev (February 2022)"
date = 0000-01-01

[extra]
month = "February 2022"
authors = [
    "phil-opp",
    "GabrielMajeri",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (February 2022)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

### [`x86_64`](https://github.com/rust-osdev/x86_64)

<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13), and [@rybot666](https://github.com/orgs/rust-osdev/people/rybot666)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

In February, …

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS.

One of the pain points of developers building software using `uefi-rs` has been the `Completion` type, which is like an expanded `Result` type which also handles warnings (besides successes and errors). There's an [open proposal](https://github.com/rust-osdev/uefi-rs/issues/360#issuecomment-1056004728) to drop this type and revert to using more standard `Result`s everywhere, by treating all warnings as errors. Initial feedback suggests that such a change would be beneficial to the project, but comments and suggestions are welcome on the linked issue.

We merged the following changes in February:

- [Run tests on AArch64 VM in CI](https://github.com/rust-osdev/uefi-rs/pull/353)
- [Add IA32 target to `xtask` and test it in CI](https://github.com/rust-osdev/uefi-rs/pull/354)
- [Fix links in project template `README` file](https://github.com/rust-osdev/uefi-rs/pull/356)
- [Remove `CStr16::as_string`](https://github.com/rust-osdev/uefi-rs/pull/357)
- [Fix status code check at end of VM test](https://github.com/rust-osdev/uefi-rs/pull/355)
- [Automatically test latest crate release on latest nightly in CI](https://github.com/rust-osdev/uefi-rs/pull/348)
- [Fix various phrasing inconsistencies and spelling errors in protocol docs](https://github.com/rust-osdev/uefi-rs/pull/193)
- [Remove implicit string conversion from `File::open`](https://github.com/rust-osdev/uefi-rs/pull/363)
- [Expand `Align` trait docstring](https://github.com/rust-osdev/uefi-rs/pull/367)
- [Add string equality operator impls](https://github.com/rust-osdev/uefi-rs/pull/366)
- [Fix file info structures' sizes and add tests](https://github.com/rust-osdev/uefi-rs/pull/365)
- [Implicitly run tests with `+nightly`](https://github.com/rust-osdev/uefi-rs/pull/364)
- [Remove more implicit string conversions](https://github.com/rust-osdev/uefi-rs/pull/368)
- [Add a `CHANGELOG.md`](https://github.com/rust-osdev/uefi-rs/pull/369)
- [Add minimal test for `LoadedImage` protocol](https://github.com/rust-osdev/uefi-rs/pull/370)
- [Fix `ProtocolsPerHandle` internal slice property](https://github.com/rust-osdev/uefi-rs/pull/374)
- [Update changelog for file info changes](https://github.com/rust-osdev/uefi-rs/pull/373)
- [Make the load options API on `LoadedImage` protocol safer](https://github.com/rust-osdev/uefi-rs/pull/375)
- [Switch all packages to the 2021 edition](https://github.com/rust-osdev/uefi-rs/pull/376)

Thanks to [@nicholasbishop](https://github.com/nicholasbishop), [@Stzx](https://github.com/Stzx), [@avirule](https://github.com/avirule) and [@AtsukiTak](https://github.com/AtsukiTak) for their contributions!

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

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

This month, ...

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).


<!--
TODO: Update publication date
-->
