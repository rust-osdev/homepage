+++
title = "This Month in Rust OSDev: June 2023"
date = 2023-07-05

[extra]
month = "June 2023"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (June 2023)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

    ### Summary
    <span class="maintainers">(Section written by [@author](https://github.com/author))</span>

    <text>
-->


## Announcements, News, and Blog Posts

Here we collect news, blog posts, etc. related to OS development in Rust.

<!--
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

## `rust-osdev` Projects

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

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

### [`hermitcore/rusty-hermit`](https://github.com/hermitcore/rusty-hermit)
<span class="maintainers">(Section written by [@mkroening](https://github.com/mkroening))</span>

The Hermit library operating system allows you to bundle a whole OS directly with your application, creating a freestanding, bootable _Unikernel_ image.
This month, we achieved several milestones for reaching more users with Hermit:

- Stable Rust support.

  You can now compile your programs for Hermit using the stable Rust toolchain! 🥳
  
  While the Hermit targets (`x86_64-unknown-hermit` and `aarch64-unknown-hermit`) are still at tier 3, we now distribute pre-built artifacts of the Rust standard library for use with stable toolchains!
  This means, no more `-Zbuild-std`, resulting in faster builds, and the bliss of the stable Rust compiler.
  
  See [`hermitcore/rust-std-hermit`](https://github.com/hermitcore/rust-std-hermit) for details on our `rust-std` artifacts.

- Windows support.

  Thanks to Rust's awesome cross-compilation capabilities, you can now compile Hermit applications from anywhere! 😎

  We have resolved a longstanding issue when building Hermit applications on Windows ([hermitcore/rusty-hermit#431](https://github.com/hermitcore/rusty-hermit/pull/431)).
  The issue is all sorted out now and Windows, macOS, and Linux are tested and verified by our CI.
  

- AArch64 (ARM64) support.

  You can now run real applications on AArch64, with scheduling, network and everything! 🤯

  - PCI now works on AArch64, which allows us to use the network devices ([hermitcore/libhermit-rs#748](https://github.com/hermitcore/libhermit-rs/pull/748)).

  - We merged scheduling support for AArch64 ([hermitcore/libhermit-rs#765](https://github.com/hermitcore/libhermit-rs/pull/765)).

  - You can also now chainload Hermit apps for AArch64 using our loader ([hermitcore/rusty-loader#201](https://github.com/hermitcore/rusty-loader/pull/201)).
    
    This means, you don't have to statically compile the full application into the final loader image anymore.

We'd love if you gave Hermit a try. Just start with our "Hello, World!" application template: [`hermitcore/rusty-demo`](https://github.com/hermitcore/rusty-demo).

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
