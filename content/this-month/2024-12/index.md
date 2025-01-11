+++
title = "This Month in Rust OSDev: December 2024"
date = 2025-01-03

[extra]
month = "December 2024"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (December 2024)" post.
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

- [This Month in Redox OS - December 2024](https://www.redox-os.org/news/this-month-241231/)
- Funding Opportunities - with [Redox OS](https://redox-os.org/) or on your own
  - The NGI Zero Commons Fund and NGI Zero Fediversity Fund each have a call for proposals with a Feb. 1 deadline.
If the proposal is successful, it would be to start roughly in June or July (based on our experience) and run for up to 12 months,
with an amount up to 50,000 EUR.
There must be a "European component", so a EU-based developer would be an ideal fit,
or perhaps a project where the maintainer is EU-based. Here are the links:
  - [NGI Zero Commons Fund](https://nlnet.nl/commonsfund/)
  - [NGI Zero Fediversity Fund](https://nlnet.nl/fediversity/)
  - Redox is looking for a part-time or short-term developer to help with implementing device drivers, ACPI support, and similar,
who would like to join our proposal.
You must be knowledgeable in Rust and drivers, and have good reputation in the open source community.
We have an existing relationship with NLnet, so we can craft the proposal, based on your skillset  and our priorities.
Please join us on Matrix and let us know you are interested.
  - https://matrix.to/#/#redox-join:matrix.org


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

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [Fix clippy::needless_lifetimes in Rust 1.83 and ShimLock ABI on ia32](https://github.com/rust-osdev/uefi-rs/pull/1485)
- [Fix clippy::use_self](https://github.com/rust-osdev/uefi-rs/pull/1486)
- [uefi-raw: Add DriverBindingProtocol](https://github.com/rust-osdev/uefi-rs/pull/1487)
- [Increase MSRV to 1.81](https://github.com/rust-osdev/uefi-rs/pull/1484)
- [Update ptr_meta to 0.3.0](https://github.com/rust-osdev/uefi-rs/pull/1496)
- [Remove unstable gate for `core::error::Error` impls](https://github.com/rust-osdev/uefi-rs/pull/1497)
- [Use size_of/size_of_val/align_of/align_of_val from the prelude](https://github.com/rust-osdev/uefi-rs/pull/1498)
- [book: Set driver link-arg in `build.rs`](https://github.com/rust-osdev/uefi-rs/pull/1502)

<!-- - [chore(deps): update crate-ci/typos action to v1.28.2](https://github.com/rust-osdev/uefi-rs/pull/1488) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1493) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.28.3](https://github.com/rust-osdev/uefi-rs/pull/1499) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1500) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.28.4](https://github.com/rust-osdev/uefi-rs/pull/1504) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1506) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1509) -->

Thanks to [@crawfxrd](https://github.com/crawfxrd) and [@JarlEvanson](https://github.com/JarlEvanson) for their contributions!


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PRs this month:

- [fix(idt): panic in `impl fmt::Debug for EntryOptions`](https://github.com/rust-osdev/x86_64/pull/522)
- [Add `MapperFlush` method to get page](https://github.com/rust-osdev/x86_64/pull/525)
- [feat: add `update()` to `Cr3`, `Dr7`, `SFMask`, `UCet`, `SCet`, `mxcsr`, `rflags`, and `XCr0`](https://github.com/rust-osdev/x86_64/pull/527)
- [fix(model_specific): make `{Fs,Gs,KernelGs}Base::write()` unsafe](https://github.com/rust-osdev/x86_64/pull/528)
- [Merge master into next](https://github.com/rust-osdev/x86_64/pull/521)

Thanks to [@mkroening](https://github.com/mkroening) and [@adavis628](https://github.com/adavis628) for their contributions!


### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following improvements:

- [Remove "UEFI boot" log message](https://github.com/rust-osdev/bootloader/pull/476)

Thanks to [@ChocolateLoverRaj](https://github.com/ChocolateLoverRaj) for their contributions!



## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

### [`roeeshoshani/genesis`](https://github.com/roeeshoshani/genesis)
<span class="maintainers">(Section written by [@roeeshoshani](https://github.com/roeeshoshani))</span>

`genesis` is a bare metal firmware implementation for mips. it implements everything from the bottom up, from
initializing the cpu caches, to configuring pci devices and the interrupt controller.

i noticed that every kernel implementation is always for x86, so i decided to implement it for something a
little more esoteric - mips.

the project is currently in very early stages but the basics are there.

it is my hobby project for me to learn about embedded programming.

feel free to follow along the development of it :).

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
