+++
title = "This Month in Rust OSDev: April 2024"
date = 2024-04-10

[extra]
month = "April 2024"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (April 2024)" post.
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

- [Testing Virtualization Stacks by Utilizing Mini Operating System Kernels](https://cyberus-technology.de/articles/testing-virtualization-stacks-utilizing-mini-kernels) \
  Testing and debugging erroneous behaviour by a guest under a virtualization stack is hard and difficult. By leveraging
  multiple mini operating system kernels, Cyberus Technology can investigate issues related to complicated topics, such
  as never delivered interrupts, with a precise focus on where to look at. Although the Cyberus Guest Tests are still a
  project written in C++, they help to find issues and problems in Cloud-Hypervisor\, **a VMM written in Rust**. For better
  debugging capabilities of the Guest Tests, Cyberus Technology [upstreamed support for the Debug Console](https://github.com/cloud-hypervisor/cloud-hypervisor/pull/6012)
  to Cloud Hypervisor, which is present since `v38`.
- [Redox OS - April 2024 Report](https://redox-os.org/news/this-month-240430/)

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

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

### [RatCornu/efs](https://codeberg.org/RatCornu/efs)

[`efs`](https://crates.io/efs) is a recently published `no-std` library which provides an OS and architecture independent implementation of some UNIX filesystems in Rust.

Currently only the [ext2 filesystem](https://fr.wikipedia.org/wiki/Ext2) is directly implemented, but I will soonly work on other filesystems!

It's still young so it may contain bugs, but it's hugely tested so that it does not happen.

Some of the features provided :

* `no_std` support (enabled by default)

* General interface for UNIX files and filesystems

* `read/write` regular files

* retrieve, add and remove directory entries directly from a path and a current working directory.

I hope you will find this useful! If you have any remark, idea or issue, do not hesitate to submit an issue!

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
