+++
title = "This Month in Rust OSDev: July 2024"
date = 2024-08-03

[extra]
month = "July 2024"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (July 2024)" post.
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

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

The `uefi-rs` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS.
Recently, we deprecated the [`uefi-services`] crate and removed all usages from
the [`uefi`](https://github.com/rust-osdev/uefi-rs) repository. A new drop-in
replacement exists in [`uefi::helpers`](https://docs.rs/uefi/latest/uefi/helpers/index.html).

Recently, we released version `v0.30` with some notable changes.

#### Memory Map Refactoring

TL;DR: What used to return `MemoryMap<'buf>` in the API, now returns
`MemoryMapOwned`. Additionally, you can parse a chunk of memory using
`MemoryMapRef` or `MemoryMapRefMut`.

We put significant effort into refactoring our abstractions for the UEFI memory 
map. These started in release v0.29 and were finalized in release v0.30. 
Instead of one `MemoryMap<'buf>` type, we now have the traits `MemoryMap` and 
`MemoryMapMut` along with the implementations `MemoryMapRef`, `MemoryMapRefMut`, 
and `MemoryMapOwned`. It is recommended to work with the specific 
implementations, as the main purpose for the traits is only to enforce a 
consistent API for these three implementations. This gives users the 
flexibility to cover all possible use cases one can have with an UEFI memory 
map. Read more:
- [#1175](https://github.com/rust-osdev/uefi-rs/pull/1175)
- [#1251](https://github.com/rust-osdev/uefi-rs/pull/1251)
- [#1240](https://github.com/rust-osdev/uefi-rs/pull/1240)
- [#1263](https://github.com/rust-osdev/uefi-rs/pull/1263)

In any case, obtaining the memory map from UEFI is hidden behind the
public API of the crate, but we made it very easy to read/parse it in all
possible scenarios!

### Outlook: Freestanding Functions API Change

On the long term, we are planning a massive API change. The typical use-case
of our library users is to write EFI image with code leveraging boot services 
and exiting them before handing over to the next step (typically load an ELF 
kernel). These EFI images typically are "99% code" using boot services, until
they are exited and control is handed over.

Although the exiting API design ensures via the type system that no boot 
service scan be called after they have been exited, the test of time has proven
that this adds unjustified complexity without bringing much value add.

Instead, with the new API, which we provide **additionally** at this point,
one can use freestanding functions which are behind the new modules:

- `uefi::system`: is a new module that provides freestanding functions for
  accessing fields of the global system table.
- `uefi::boot`:
  is a new module that provides freestanding functions for boot services using
  the global system table.
- `uefi::runtime`: is a new module that provides freestanding functions for
  runtime services using the global system table.

This solves API inconsistencies and restrictions already existing so far, and
makes the overall handling a lot easier. This comes with the costs that 
functions may panic, if the boot services were exited but one tries to use 
them. However, the massive API simplification justifies this.

The work has just been started. Follow the progress and discussions on 
[GitHub](https://github.com/rust-osdev/uefi-rs/issues/893#issuecomment-2241610633).

### Other

There were also a ton of other interesting fixes, changes, and additions! 
Check out our [Changelog](https://github.com/rust-osdev/uefi-rs/blob/main/uefi/CHANGELOG.md).

We merged the following PRs this month:
<!-- todo -->


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

### [`phip1611/bit_ops`](https://github.com/phip1611/bit_ops)
<span class="maintainers">(Section written by [@phip1611](https://github.com/phip1611))</span>

I've recently created and published [`bit_ops`](https://github.com/phip1611/bit_ops).
It offers common bit-oriented operations on primitive integer types with a focus on
`no_std` and `const` compatibility. Unlike other crates that provide tooling to
create sophisticated high-level types with bitfields, the focus of `bit_ops` is
on raw primitive integer types.


## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
