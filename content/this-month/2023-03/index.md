+++
title = "This Month in Rust OSDev: March 2023"
date = 2023-04-06

[extra]
month = "March 2023"
editors = ["phil-opp", "phip1611"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (March 2023)" post.
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

- [Writing a Linux Driver for QEMU’s Debugcon Device](https://phip1611.de/blog/writing-a-linux-driver-for-qemus-debugcon-device/) \
  In this blogpost, [@phip1611](https://github.com/phip1611) shows you can 
  write a Linux driver for the QEMU debugcon device. Although, the driver 
  still uses C, it is a wonderful example to demonstrate a minimal yet useful
  driver. Additionally, it is a good starting point for a rewrite in Rust, once
  the Rust tooling and API bindings in the kernel are more mature. Perhaps, 
  the rewrite in Rust is your next learning project?


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

### [`multiboot2`](https://github.com/rust-osdev/multiboot2)

<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods), [@phip1611](https://github.com/phip1611), [@robert-w-gries](https://github.com/robert-w-gries), [@ahmedcharles](https://github.com/ahmedcharles), and [@Caduser2020](https://github.com/Caduser2020)</span>

The `multiboot2` crate provides abstraction types for the multiboot information 
structure (MBI) of multiboot2 bootloaders. The latest release of the 
`multiboot2`-crate is now `v0.15.0` (was `v0.14.0`), which fixed a 
[bug](https://github.com/rust-osdev/multiboot2/pull/119). Furthermore, the 
documentation was improved. However, the biggest change is that the library now 
allows the parsing of custom multiboot tags, which are not prohibited by the 
spec. For a full changelog, please refer to the
[GitHub repo](https://github.com/rust-osdev/multiboot2/blob/main/multiboot2/Changelog.md).

#### CI Refactoring
In the CI, we want to run many tests that cover a big portion of the cartesian
product of the following properties:
- rust version: stable, nightly, msrv
- type: build, test, style check
- target: default, no_std
 
As I (@phip1611) was annoyed by all the boilerplate configuration and 
repetition, I've investigated new ways to improve that situation and created
a reusable workflow can be used like that:
```yaml
jobs:
  build_msrv:
    name: build (msrv)
    uses: ./.github/workflows/_build-rust.yml
    with:
      rust-version: 1.56.1
      do-style-check: false

  style_nightly:
    name: style (nightly)
    needs: build_nightly
    uses: ./.github/workflows/_build-rust.yml
    with:
      rust-version: nightly
      do-style-check: true
      do-test: false
```

The `./.github/workflows/_build-rust.yml` workflow abstracts setting up the 
toolchain, setting up a cargo cache for a faster CI, and, depending on the 
configuration, running `cargo test|clippy|doc|build|fmt`. I think that the 
outcome is quite nice and might also help others. Feel free to check out the 
corresponding [PR](https://github.com/rust-osdev/multiboot2/pull/126).

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
