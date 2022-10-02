+++
title = "This Month in Rust OSDev: September 2022"
date = 0000-01-01

[extra]
month = "September 2022"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (September 2022)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
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

### [`bootloader`](https://github.com/rust-osdev/bootloader)

<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@rybot666](https://github.com/rybot666), and [@64](https://github.com/64)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables.

We encountered various boot issues with the new `v7.1.0` release of QEMU in September. This also affected downstream projects such as [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os/issues/1138) and [`hawkw/mycelium`](https://github.com/hawkw/mycelium/issues/321). After some debugging, we found out that QEMU now reports a new memory region at offset ~1 TiB in the physical memory map, which was much higher than expected by the bootloader crate. As a result, BIOS booting became unusably slow (because of the initial identity mapping) and the automated offset selection for the physical memory map ran into a panic. For full details, see the [summary issue](https://github.com/rust-osdev/bootloader/issues/259).

We fixed the issues in the following way:

- [Limit BIOS bootloader's `max_phys_addr` to 4 GiB](https://github.com/rust-osdev/bootloader/pull/260)
- [Only perform a single TLB flush after identity mapping](https://github.com/rust-osdev/bootloader/pull/265)
- [fix `get_free_address` for large sizes (0.10)](https://github.com/rust-osdev/bootloader/pull/263) <span class="gray">(published as `0.10.13`)
- [allow allocating more than one level 4 entry (0.9)](https://github.com/rust-osdev/bootloader/pull/264)  <span class="gray">(published as `0.9.23`)

Thanks to [@hawkw](https://github.com/hawkw) and [@Freax13](https://github.com/Freax13) for these contributions!

We also brought the [upcoming `v0.11` release](https://github.com/rust-osdev/bootloader/pull/232) even closed to the finish line. Thanks to an extensive review of [@Freax13](https://github.com/Freax13), we found and fixed multiple remaining issues. Also thanks to [@asensio-project](https://github.com/asensio-project) and [@TheBotlyNoob](https://github.com/TheBotlyNoob) for testing the new version and reporting bugs! Some notable changes are:

- [Allocate kernel as normal UEFI loader data](https://github.com/rust-osdev/bootloader/commit/08e4b5829bf5882d9d396e641e32b65de72704b2)
- [Preserve `RUNTIME_SERVICES_*` memory regions](https://github.com/rust-osdev/bootloader/commit/667e57f552e214f9c19848306e03b00d91a8114f)
- [Update usage instructions and architecture description in README](https://github.com/rust-osdev/bootloader/commit/454f70740df13107d4748d63b1d646f176f6fa62)


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

- [Add an mdBook](https://github.com/rust-osdev/uefi-rs/pull/515)
  - Read it at <https://rust-osdev.github.io/uefi-rs/HEAD/>
- [More protocol safety improvements](https://github.com/rust-osdev/uefi-rs/pull/478)
- [Implement `HardDriveMediaDevicePath` (along with MBR and GPT tests). ](https://github.com/rust-osdev/uefi-rs/pull/497)
- [Drop use of unstable try_trait_v2 feature](https://github.com/rust-osdev/uefi-rs/pull/479)
- ["is_regular_file" for file trait + integration test](https://github.com/rust-osdev/uefi-rs/pull/475)
- [`CStr8` cleanup and enhancements](https://github.com/rust-osdev/uefi-rs/pull/506)
- [Transform feature "ignore-logger-errors" to additive feature.](https://github.com/rust-osdev/uefi-rs/pull/476)
- [Release `uefi-macros-0.8.0`, `uefi-0.17.0`, `uefi-services-0.14.0`](https://github.com/rust-osdev/uefi-rs/pull/513)
- [Add `PhysicalAddress` and `VirtualAddress` type aliases](https://github.com/rust-osdev/uefi-rs/pull/518)
- [Change DevicePath[From|To]Text methods to return a Result](https://github.com/rust-osdev/uefi-rs/pull/514)

<!--
- [`is_media_preset` -> `is_media_present` in `BlockIOMedia`](https://github.com/rust-osdev/uefi-rs/pull/495)
- [Update changelog](https://github.com/rust-osdev/uefi-rs/pull/499)
- [Add rust-toolchain.toml and pin to a working nightly](https://github.com/rust-osdev/uefi-rs/pull/502)
- [Create `pull_request_template.md`](https://github.com/rust-osdev/uefi-rs/pull/503)
- [Update changelog for uefi-macros](https://github.com/rust-osdev/uefi-rs/pull/505)
- [Temporarily revert use of `core::ffi::CStr`](https://github.com/rust-osdev/uefi-rs/pull/509)
- [Update uefi changelog](https://github.com/rust-osdev/uefi-rs/pull/512)
- [ci: Test the build on an old nightly](https://github.com/rust-osdev/uefi-rs/pull/517)
- [Switch the toolchain back to latest nightly](https://github.com/rust-osdev/uefi-rs/pull/516)
- [Update clap requirement from 3.2.1 to 4.0.4](https://github.com/rust-osdev/uefi-rs/pull/521)

-->

Thanks to [@ColinFinck](https://github.com/ColinFinck) for their contribution!


### [`xhci`](https://github.com/rust-osdev/xhci)

<span class="maintainers">Maintained by [@toku-sa-n](https://github.com/toku-sa-n)</span>

The `xhci` crate provides types of xHCI structires, such as Registers and TRBs.

We merged the following changes in September:

- [Implement Default for registers without RsvdP bits](https://github.com/rust-osdev/xhci/pull/147)
- [Add set_0_* to rw1c! macro](https://github.com/rust-osdev/xhci/pull/148)
- [Release 0.9.1](https://github.com/rust-osdev/xhci/pull/149)

Thanks to [@Demindiro](https://github.com/Demindiro) for their contribution!

## Call for Participation

Want to contribute to a Rust OSDev project, but don't know where to start? Help with one of these outstanding issues!

<!--
    Please use the following template for adding items:
    - [(`repo_name`) Issue Description](https://example.com/link-to-issue)
-->

- [(`phil-opp/blog_os`) Looking for a reviewer for a Chinese translation](https://github.com/phil-opp/blog_os/pull/1131#issuecomment-1251963782)
- [(`phil-opp/blog_os`) Looking for a reviewer for a Korean translation](https://github.com/phil-opp/blog_os/pull/1135#issuecomment-1264665246)
- [(`phil-opp/blog_os`) Looking for a reviewer for a French translation](https://github.com/phil-opp/blog_os/pull/1144)

If you maintain a Rust project related to operating system development and are looking for contributors, especially for tasks suited to people getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the `next` branch with the tasks you want to include in the next issue.

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="maintainers">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

We merged the following changes this month:

- [Translate post-07 to Japanese](https://github.com/phil-opp/blog_os/pull/1141)
- [Copyedit edition 2 `paging-introduction/index.md`](https://github.com/phil-opp/blog_os/pull/1129)
- [Fix typo in french translation `01-freestanding-rust-binary`](https://github.com/phil-opp/blog_os/pull/1142)

Thanks to [@shimomura1004](https://github.com/shimomura1004), [@woodyZootopia](https://github.com/woodyZootopia), [@bolded](https://github.com/bolded), and [@Firenezz](https://github.com/Firenezz) for their contributions!

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).



<!--
TODO: Update publication date
-->
