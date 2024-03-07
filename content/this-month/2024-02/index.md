+++
title = "This Month in Rust OSDev: February 2024"
date = 2024-03-07

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


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PRs this month:

- [Fix data layout in custom target used for testing](https://github.com/rust-osdev/x86_64/pull/454)
- [optimize `from_page_table_indices`](https://github.com/rust-osdev/x86_64/pull/456)
- [mark as 0.15 as beta release](https://github.com/rust-osdev/x86_64/pull/455)
- [Release v0.14.12](https://github.com/rust-osdev/x86_64/pull/457)
- [Fix release script](https://github.com/rust-osdev/x86_64/pull/459)
- [Merge next into master: releasing `v0.15.0-beta`](https://github.com/rust-osdev/x86_64/pull/458)
- [Update data layout of test target for LLVM 18](https://github.com/rust-osdev/x86_64/pull/460)
- [optimize `Step` impl for `VirtAddr`](https://github.com/rust-osdev/x86_64/pull/462)
- [Miscellaneous improvements](https://github.com/rust-osdev/x86_64/pull/464)
- [Release v0.15.0](https://github.com/rust-osdev/x86_64/pull/463)


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

The `uefi-rs` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. We merged the following PRs this month:


<!--
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1067)
- [chore(deps): update crate-ci/typos action to v1.18.0](https://github.com/rust-osdev/uefi-rs/pull/1066)
- [fix(deps): update rust crate itertools to v0.12.1](https://github.com/rust-osdev/uefi-rs/pull/1063)
- [fix(deps): update rust crate ureq to v2.9.4](https://github.com/rust-osdev/uefi-rs/pull/1065)
- [chore(deps): update crate-ci/typos action to v1.18.2](https://github.com/rust-osdev/uefi-rs/pull/1069)
- [fix(deps): update rust crate tempfile to v3.10.0](https://github.com/rust-osdev/uefi-rs/pull/1072)
- [fix(deps): update rust crate ureq to v2.9.5](https://github.com/rust-osdev/uefi-rs/pull/1070)
- [fix(deps): update rust crate syn to v2.0.49](https://github.com/rust-osdev/uefi-rs/pull/1075)
- [chore(deps): update dorny/paths-filter action to v3](https://github.com/rust-osdev/uefi-rs/pull/1078)
- [fix(deps): update rust crate ureq to v2.9.6](https://github.com/rust-osdev/uefi-rs/pull/1076)
- [fix(deps): update rust crate anyhow to v1.0.80](https://github.com/rust-osdev/uefi-rs/pull/1079)
- [fix(deps): update rust crate serde_json to v1.0.114](https://github.com/rust-osdev/uefi-rs/pull/1080)
- [fix(deps): update rust crate crates-index to v2.6.0](https://github.com/rust-osdev/uefi-rs/pull/1083)
- [fix(deps): update rust crate syn to v2.0.50](https://github.com/rust-osdev/uefi-rs/pull/1082)
-->

- [Add a method to create a MemoryMap from a raw buffer](https://github.com/rust-osdev/uefi-rs/pull/1074)

Thanks to [@bjorn3](https://github.com/bjorn3) for their contribution!


### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following improvements:

- [Set `NO_EXECUTE` flag for all writable memory regions](https://github.com/rust-osdev/bootloader/pull/409)
- [[v0.9] Fix data layout for custom targets for LLVM 18](https://github.com/rust-osdev/bootloader/pull/421)
- [[v0.9] Fix map errors during kernel loading](https://github.com/rust-osdev/bootloader/pull/422)
- [[v0.9] Fix: unify flags if multiple segments are mapped to same frame with different flags](https://github.com/rust-osdev/bootloader/pull/423)
- [Fix invalid mapping to zero page caused by off-by-one bug](https://github.com/rust-osdev/bootloader/pull/424)
- [adapt data layout to match LLVM's](https://github.com/rust-osdev/bootloader/pull/420)
- [Release `v0.11.7`](https://github.com/rust-osdev/bootloader/pull/426)
- [Remove unused paging imports](https://github.com/rust-osdev/bootloader/pull/430)

Thanks to [@vinc](https://github.com/vinc) and [@tsatke](https://github.com/tsatke) for their contributions!

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

<span class="gray">No projects updates were submitted this month.</span>

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
