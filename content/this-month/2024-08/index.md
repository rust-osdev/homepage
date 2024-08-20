+++
title = "This Month in Rust OSDev: August 2024"
date = 2024-09-03

[extra]
month = "August 2024"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (August 2024)" post.
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

### [`multiboot2`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

_Convenient and safe parsing of Multiboot2 Boot Information (MBI) structures and
the contained information tags. Usable in no_std environments, such as a kernel. 
An optional builder feature also allows the construction of the corresponding 
structures._

🎉 We are happy to announce release `v0.22` of the `multiboot2` crate. This is a
**major step forward** as all Undefined Behaviour (UB) and a bunch of safety and
memory issues have been removed / fixed. 🎉 Now, all unit tests (and there are
many) **pass Miri**! This was possible due to massive internal refactorings done
in [#226](https://github.com/rust-osdev/multiboot2/pull/226) and [#227](https://github.com/rust-osdev/multiboot2/pull/227).

The quite complex logic to parse all these structures **safe and correctly**
has been moved to a new [`multiboot2-common`](https://docs.rs/multiboot2-common)
crate. This way, the [`multiboot2-header`](https://crates.io/crates/multiboot2-header)
crate can also leverage the new abstractions. A comprehensive writeup about why 
the parsing of Multiboot2 structures and typing them correctly in Rust is much
more complex than one initially might think is written down in 
[`lib.rs` of `multiboot2-common`](https://docs.rs/multiboot2-common).

`multiboot2-header` also got a new release `v0.5` and can now be considered 
safe and free of UB when parsing the corresponding structures. The internal
refactorings only slightly leak to the public interfaces of `multiboot2`
and `multiboot2-header`.

Read more:
- [Changelog of `multiboot2`](https://github.com/rust-osdev/multiboot2/blob/b95b73508925b5484a0cf531a45c9c9ce555d560/multiboot2/Changelog.md)
- [Changelog of `multiboot2-header`](https://github.com/rust-osdev/multiboot2/blob/b95b73508925b5484a0cf531a45c9c9ce555d560/multiboot2-header/Changelog.md)

<details>
<summary>Background - Read More</summary>

The crate grew historically without a centralized design or approach how to work
with memory and pointers. Therefore, many UB way unintentionally created. We 
are happy to get lack of the technical debt. Nevertheless, thanks to everyone
who contributed over the years - sorry if we had to rewrite your code in this 
one! :)
</details>

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

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
