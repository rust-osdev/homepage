+++
title = "This Month in Rust OSDev (April 2021)"
date = 0000-01-01

[extra]
month = "April 2021"
authors = [
    "phil-opp",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (April 2021)" post.
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

In April, …

### [`xhci`](https://github.com/rust-osdev/xhci)

The `xhci` crate provides types of xHCI structures such as Contexts, Extended Capabilities, Registers, and TRBs.

Previously the repository was hosted under [`@toku-sa-n`](https://github.com/toku-sa-n). Since April, the Rust OSDev team hosts the repository.

This crate is still under depelopment. Some types or field accessors may be missing. If you find missing features, feel free to send a PR!

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. In April, we merged the following changes:

- [Expose NamedFileProtocolInfo's Header types](https://github.com/rust-osdev/uefi-rs/pull/205)
- [Upgrade to GitHub-native Dependabot](https://github.com/rust-osdev/uefi-rs/pull/207)

Thanks to [@ocadaruma](https://github.com/ocadaruma) for their contribution!

## Call for Participation
Want to contribute to a Rust OSDev project, but don't know where to start? Pick up one of these outstanding
issues in one of our projects and get started!

If you maintain a Rust OSDev project and are looking for contributors, especially for tasks suited to people
getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the
`next` branch with the tasks you want to include in the next issue.

<!--
Please use the following template for adding items:

- [(`repo_name`) Issue Description](https://example.com/link-to-issue)
-->

## Personal Projects

In this section, we describe updates to personal projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

This month, ...

### [`cdrzewiecki/celos`](https://gitlab.com/cdrzewiecki/celos)

<span class="gray">(Section written by [@drzewiec](https://github.com/drzewiec))</span>

Over the past couple of months I have made some great strides on my OS.

* Migrated the kernel to the higher half of virtual memory
* Added double fault handling
* Added page fault handling which will attempt to (safely) expand the kernel stack if it overflows
* Related to the above, added more robust frame allocation
* Added basic heap allocation

Still working hard on squashing bugs, adding all of the things above gave me some pretty serious memory allocation bugs and it's taken me a while to get those worked out. I still want to do more testing to make sure I have all the bugs here worked out before moving on.

Next steps will be to get nicer font drawing, and then attempt to implement process support.

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).


<!--
TODO: Update publication date
-->
