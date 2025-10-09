+++
title = "This Month in Rust OSDev: September 2025"
date = 2025-10-08

[extra]
month = "September 2025"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (September 2025)" post.
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

### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following fix:

- [Fix: `target-pointer-width` field now expects an integer](https://github.com/rust-osdev/bootloader/pull/516)
- [Release `v0.11.12`](https://github.com/rust-osdev/bootloader/pull/517)
- [Fix(v0.9): target-pointer-width field now expects an integer](https://github.com/rust-osdev/bootloader/pull/518)


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [test-runner: Add example of using the Input protocol](https://github.com/rust-osdev/uefi-rs/pull/1755)
- [style: misc clippy fixes](https://github.com/rust-osdev/uefi-rs/pull/1760)
- [style: cargo doc fixes and improvements](https://github.com/rust-osdev/uefi-rs/pull/1759)
- [uefi: memory safety fixes (UB!)](https://github.com/rust-osdev/uefi-rs/pull/1758)
- [uefi: doc: Parameters -> Arguments](https://github.com/rust-osdev/uefi-rs/pull/1764)
- [Remove unnecessary unsafe block in example](https://github.com/rust-osdev/uefi-rs/pull/1765)
- [uefi: remove support for unstable allocator_api feature](https://github.com/rust-osdev/uefi-rs/pull/1763)

<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1762) -->
<!-- - [fix(deps): update rust crate serde_json to v1.0.145](https://github.com/rust-osdev/uefi-rs/pull/1767) -->
<!-- - [chore(deps): update rust crate trybuild to v1.0.111](https://github.com/rust-osdev/uefi-rs/pull/1766) -->

Thanks to [@BrokenC1oud](https://github.com/BrokenC1oud) for their contribution!


### [`virtio-spec-rs`](https://github.com/rust-osdev/virtio-spec-rs)
<span class="maintainers">Maintained by [@mkroening](https://github.com/mkroening)</span>

The `virtio-spec` crate provides definitions from the Virtual I/O Device (VIRTIO) specification.
This project aims to be unopinionated regarding actual VIRTIO drivers that are implemented on top of this crate.

We merged the following PRs this month:

- [feat: add traditional mem balloon device](https://github.com/rust-osdev/virtio-spec-rs/pull/6)
- [chore: release version 0.3.2](https://github.com/rust-osdev/virtio-spec-rs/pull/10)

Thanks to [@hcsch](https://github.com/hcsch) for their contribution!



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
