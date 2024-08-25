+++
title = "This Month in Rust OSDev: August 2024"
date = 2024-09-03

[extra]
month = "August 2024"
editors = ["phil-opp", "phip1611"]
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

### [`uefi`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality. Recently, we released 
version `v0.31` with some notable changes:

1. We introduced a completely new API to access boot and runtime services
2. We reorganized the MemoryMap-related types.
3. We reorganized and improved the documentation, and also added documentation
   about how the `std` implementation for UEFI targets compares to `uefi` and
   how both can be used together.

### 1. New API: Freestanding Functions API Change

We are planning a massive API change and introduced a new replacement API in
release `v0.31`. The old API co-exists and will be marked as 
`#[deprecated]` in `0.32`. The new API will make it easier to call boot or 
runtime services without having to pass `&BootServices` around all the time.

The typical use-case of our library users is to write a UEFI bootloader
application that heavily leverages boot services. Then control is handed over to
the next stage, which is typically the kernel of an OS. Boot services are
usually exited either just prior to launching the kernel or by the kernel
itself. In any case, the UEFI application spends almost all of its time with
boot services active.

Although the old API design ensures via the type system that no boot
services can be called after they have been exited, the test of time has proven
that this adds unjustified complexity without bringing much real value add.

Instead, with the new API, which we provide **additionally** at this point,
one can use freestanding functions which are behind the new modules:

- `uefi::system`: is a new module that provides freestanding functions for
  accessing fields of the global system table.
- `uefi::boot`:
  is a new module that provides freestanding functions for boot services using
  the global system table.
- `uefi::runtime`: is a new module that provides freestanding functions for
  runtime services using the global system table.

The freestanding functions itself are close to the originals ones but without
`&BootServices` or `&RuntimeServices` parameters, as you can see for example
[here](https://github.com/rust-osdev/uefi-rs/pull/1344/files#diff-46f1e3c04d719fff471faf35c4d218430e1d664ac0a0fab9d2c15870c2d0f066).

The new API design solves API inconsistencies and restrictions already existing
so far, and makes the overall handling a lot easier. This comes with the costs
that functions may panic, if the boot services were exited but one tries to use
them. However, the massive API simplification justifies this.

Find more and follow the progress and discussions on:
- [the GitHub Issue](https://github.com/rust-osdev/uefi-rs/issues/893#issuecomment-2241610633).
- [The function Migration Guide](https://github.com/rust-osdev/uefi-rs/blob/main/docs/funcs_migration.md)

#### 2. Memory Map Refactoring

TL;DR: What used to return `MemoryMap<'buf>` in the API, now returns
`MemoryMapOwned`. Additionally, you can parse a chunk of memory using
`MemoryMapRef` or `MemoryMapRefMut`.

We put significant effort into refactoring our abstractions for the UEFI memory 
map. These started in release v0.29 and were finalized in release v0.31. 
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

#### 3. & 4.: Documentation Reorganization

We put notable work into our README and `lib.rs` files to improve the
structure of our documentation. The main value-add and improvement is that we 
clearly, directly, and precisely talk about:

- What is `uefi`?
- Which problems does it solve?
- How can it be used?
- How does it compare to the ecosystem, such as `std` for Rust targets?

To see an example how `uefi` and `std` can be used together, please head to
[our repository](https://github.com/rust-osdev/uefi-rs/tree/main/uefi-std-example).

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

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
