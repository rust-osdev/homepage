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

### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

The [multiboot2](https://crates.io/crates/multiboot2) and the [multiboot2-header](https://crates.io/crates/multiboot2)
crates got a big overhaul. The list of new features includes but is not limited to:
- use DSTs for tags where applicable ([#134](https://github.com/rust-osdev/multiboot2/pull/134))
- model the MBI as DST ([#155](https://github.com/rust-osdev/multiboot2/pull/155))
- add a runtime builder for an MBI ([#133](https://github.com/rust-osdev/multiboot2/pull/133)) \
  Huge thanks to [YtvwlD / Niklas](https://github.com/YtvwlD) for this greate external contribution
- added an integration test including a multiboot2 chainloader for better test coverage ([#129](https://github.com/rust-osdev/multiboot2/pull/129))
- added miri to the CI for more memory safety ([#128](https://github.com/rust-osdev/multiboot2/pull/128))
- several fixes and small improvements

`multiboot2` was updated from `0.15.1` to `0.16.0` and `multiboot2-header` was updated from `0.2.0` to `0.3.0`. Both 
releases come with a large amount of [breaking changes](https://github.com/rust-osdev/multiboot2/blob/main/Changelog.md). 
However, after a sensible consideration, they are all worth it for a more streamlined API and more memory safety.

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

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
