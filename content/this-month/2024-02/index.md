+++
title = "This Month in Rust OSDev: February 2024"
date = 2024-03-05

[extra]
month = "February 2024"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (February 2024)" post.
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

- [Redox OS - Porting Strategy](https://www.redox-os.org/news/porting-strategy/)
- [This Month in Redox](https://redox-os.org/news/this-month-240229/)
- [Tock Compiles on Stable Rust!](https://tockos.org/blog/2024/talking-tock-55/)
- [Making an RISC-V OS (Part 2): Kernel in virtual addresses](https://traxys.me/riscv_os_2.html)
- The Embedded Rustacean [Issue 13](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-13) and [Issue 14](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-14)
- [Linux Kernel: Rewrite the VP9 codec library in Rust](https://lore.kernel.org/lkml/20240227215146.46487-1-daniel.almeida@collabora.com/)
- [Anouncing stabby 3.0.0](https://www.reddit.com/r/rust/comments/1amjknw/anouncing_stabby_300_and_rustconf_video_available/)

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [only set noalias on Box with the global allocator](https://github.com/rust-lang/rust/pull/122018)
- [Add stubs in IR and ABI for `f16` and `f128`](https://github.com/rust-lang/rust/pull/121728)
- [`f16` and `f128` step 2: intrinsics](https://github.com/rust-lang/rust/pull/121841)
- [Add armv8r-none-eabihf target for the Cortex-R52](https://github.com/rust-lang/rust/pull/110482)
- [Add a new `wasm32-wasip1` target to rustc](https://github.com/rust-lang/rust/pull/120468)
- [Add a new `wasm32-wasi-preview2` target](https://github.com/rust-lang/rust/pull/119616)
- [rename `ptr::invalid` -> `ptr::without_provenance`](https://github.com/rust-lang/rust/pull/117658)

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

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
