+++
title = "This Month in Rust OSDev (September 2021)"
date = 2021-10-07

[extra]
month = "September 2021"
authors = [
    "phil-opp",
    "phip1611",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (September 2021)" post.
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

In September, we merged the following changes:

- [Add `clean_up` and `clean_up_with_filter`](https://github.com/rust-osdev/x86_64/pull/264) for deallocating unused page tables
- [Add exception vector type](https://github.com/rust-osdev/x86_64/pull/303)
- [Bump `bit_field` to 0.10.1](https://github.com/rust-osdev/x86_64/pull/306)
- [Release version 0.14.5](https://github.com/rust-osdev/x86_64/pull/304)
- [Move segment types into a new registers::segmentation module](https://github.com/rust-osdev/x86_64/pull/309)
- [Release version 0.14.6](https://github.com/rust-osdev/x86_64/pull/310)

Thanks to [@Freax13](https://github.com/Freax13), [@npmccallum](https://github.com/npmccallum), and [@mkroening](https://github.com/mkroening) for their contributions!


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS.

This month, we merged the following changes:

- [Improve macro errors](https://github.com/rust-osdev/uefi-rs/pull/277)
- [Implement missing methods of `DebugSupport`](https://github.com/rust-osdev/uefi-rs/pull/290)
- [macros: add compilation tests](https://github.com/rust-osdev/uefi-rs/pull/286)
- [Remove attribute to enable `const_panic`](https://github.com/rust-osdev/uefi-rs/pull/296)
- [Add a test command to build.py and also use it in the CI](https://github.com/rust-osdev/uefi-rs/pull/283)

Thanks to [@necauqua](https://github.com/necauqua) and [@timrobertsdev](https://github.com/timrobertsdev) for their contributions!

### [`bootloader`](https://github.com/rust-osdev/bootloader)

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables.

We finally merged a long-desired feature this month:

- [Framebuffer configuration](https://github.com/rust-osdev/bootloader/pull/179) <span class="gray">(published as `v0.10.9`)</span>

Thanks to [@anellie](https://github.com/anellie) for this contribution!

### [`multboot2`](https://github.com/rust-osdev/multiboot2)

The `multiboot2` crate provides abstraction types for the boot information of multiboot2 bootloaders.

It is now part of a workspace and lives next to the new crate `multiboot2-header`.

The following changes were merged this month:

- [Code style improvements + optional CI job (clippy, rustfmt, rustdoc)](https://github.com/rust-osdev/multiboot2/pull/92)
- [std in tests; hash for TagType](https://github.com/rust-osdev/multiboot2/pull/94) 
- [editorconfig file](https://github.com/rust-osdev/multiboot2/pull/93)
- [prepared cargo workspace, as discussed in PR #79](https://github.com/rust-osdev/multiboot2/pull/86)

The changes were published as <span class="gray">`v0.12.2`</span>.


### [`multboot2-header`](https://github.com/rust-osdev/multiboot2) (**new**)

The `multiboot2-header` crate provides abstraction types for the Multiboot2 header 
and a builder struct to construct these headers. The corresponding repository was 
prepared ([#86](https://github.com/rust-osdev/multiboot2/pull/86)) and the initial release 
is expected in early October. See [#95](https://github.com/rust-osdev/multiboot2/pull/95) for more details.


## Call for Participation

Want to contribute to a Rust OSDev project, but don't know where to start? Pick up one of these outstanding
issues in one of our projects and get started!

<!--
Please use the following template for adding items:
- [(`repo_name`) Issue Description](https://example.com/link-to-issue)
-->

- [(`phil-opp/blog_os`) Looking for a reviewer for a French translation](https://github.com/phil-opp/blog_os/pull/1053)

<!--
<span class="gray">

_No tasks were proposed for this section._

</span>
-->

If you maintain a Rust OSDev project and are looking for contributors, especially for tasks suited to people
getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the
`next` branch with the tasks you want to include in the next issue.


## Personal Projects

In this section, we describe updates to personal projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

We merged two small fixes to the [_Writing an OS in Rust_](https://os.phil-opp.com/) blog this month:

- [Fix typos in code examples in Async/Await post](https://github.com/phil-opp/blog_os/pull/1051)
- [Fix link syntax in Russian translation](https://github.com/phil-opp/blog_os/pull/1046)

Thanks to [@jongillham](https://github.com/jongillham) and [@non-descriptive](https://github.com/non-descriptive) for these contributions!

I don't have any notable news about the upcoming third edition of the blog yet, but I'm doing my best to get back up to speed soon.

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
