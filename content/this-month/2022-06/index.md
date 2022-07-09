+++
title = "This Month in Rust OSDev: June 2022"
date = 0000-01-01

[extra]
month = "June 2022"
authors = [
    "phil-opp",
    "phip1611",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (June 2022)" post.
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

In June, …

### [`multiboot2`](https://github.com/rust-osdev/multiboot2)

<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods), [@phip1611](https://github.com/phip1611), [@robert-w-gries](https://github.com/robert-w-gries), [@ahmedcharles](https://github.com/ahmedcharles), and [@Caduser2020](https://github.com/Caduser2020)</span>

The `multiboot2` crate provides abstraction types for the multiboot information structure (MBI)
of multiboot2 bootloaders. The latest release of the `multiboot2`-crate is now `v.0.14.0` (was
`v0.13.2`). It contains some small breaking changes because so far, we used some unsafe code and
relied on having not to verify UTF-8 strings. For a full changelog, please refer to the 
[GitHub repo](https://github.com/rust-osdev/multiboot2/blob/main/multiboot2/Changelog.md).

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

### [`google/gpt-disk-rs`](https://github.com/google/gpt-disk-rs)

<span class="gray">(Section written by [@nicholasbishop](https://github.com/nicholasbishop))</span>

`gpt-disk-rs` is a collection of three libraries related to [GPT](https://en.wikipedia.org/wiki/GUID_Partition_Table) (GUID Partition Table) disk data. The crates are no-std by default, have minimal dependencies, and include a lot of documentation. I'm hoping they'll be a suitable base for any project that wants to read or write GPT data.

* [`uguid`](https://crates.io/crates/uguid): GUID data type.
* [`gpt_disk_types`](https://crates.io/crates/gpt_disk_types): all the GPT types defined by the UEFI specification
* [`gpt_disk_io`](https://crates.io/crates/gpt_disk_io): types for reading and writing GPT data to an abstract block IO device.

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).


<!--
TODO: Update publication date
-->
Footer
