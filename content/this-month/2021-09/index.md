+++
title = "This Month in Rust OSDev (September 2021)"
date = 0000-01-01

[extra]
month = "September 2021"
authors = [
    "phil-opp",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

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

In September, …

### [`multboot2`](https://github.com/rust-osdev/multiboot2)

The `multiboot2` crate provides abstraction types for the boot information of multiboot2 bootloaders.

It is now part of a workspace and lives next to the new crate `multiboot2-header`.

The following changes were merged this month:

- [Code style improvements + optional CI job (clippy, rustfmt, rustdoc)](https://github.com/rust-osdev/multiboot2/pull/92)
- [editorconfig file](https://github.com/rust-osdev/multiboot2/pull/93)
- [std in tests; hash for TagType](https://github.com/rust-osdev/multiboot2/pull/94) 
- [editorconfig file](https://github.com/rust-osdev/multiboot2/pull/93)
- [prepared cargo workspace, as discussed in PR #79](https://github.com/rust-osdev/multiboot2/pull/86)

The changes were published as <span class="gray">`v0.12.2`</span>.


### [`multboot2-header`](https://github.com/rust-osdev/multiboot2) (**new**)
The `multiboot2-header` crate provides abstraction types for the Multiboot2 header 
and a builder struct to construct these headers. The corresponding repository was 
prepared (<https://github.com/rust-osdev/multiboot2/pull/86>) and the initial release 
is expected in early October. See <https://github.com/rust-osdev/multiboot2/pull/95> for more details.

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

If you maintain a Rust OSDev project and are looking for contributors, especially for tasks suited to people
getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the
`next` branch with the tasks you want to include in the next issue.


## Personal Projects

In this section, we describe updates to personal projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

This month, ...

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).


<!--
TODO: Update publication date
-->
