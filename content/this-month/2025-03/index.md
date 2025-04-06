+++
title = "This Month in Rust OSDev: March 2025"
date = 2025-04-04

[extra]
month = "March 2025"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (March 2025)" post.
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

- [Rust, compiled to Holy C, running on TempleOS](https://www.reddit.com/r/rust/comments/1jp9227/media_rust_compiled_to_holly_c_running_on_templeos/)
- [My non-unix like rust OS SafaOS, now has a rust libstd port](https://www.reddit.com/r/rust/comments/1jkkufh/media_my_nonunix_like_rust_os_safaos_now_has_a/)
- [Introducing Ariel OS - an embedded library OS for small MCUs](https://www.reddit.com/r/rust/comments/1jo070l/introducing_ariel_os_an_embedded_library_os_for/)
- [Stalloc: fast memory allocation on the stack](https://www.reddit.com/r/rust/comments/1jqjs6n/stalloc_fast_memory_allocation_on_the_stack/)

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [Allow `*const W<dyn A> -> *const dyn A` ptr cast](https://github.com/rust-lang/rust/pull/136127)
- [Stabilize `asm_goto` feature gate](https://github.com/rust-lang/rust/pull/133870)

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

- [uefi: Implement SignalEvent() boot services function](https://github.com/rust-osdev/uefi-rs/pull/1556)
- [uefi: Improve handling of null-address allocations in allocate_pages](https://github.com/rust-osdev/uefi-rs/pull/1558)
- [uefi: Mark all function pointers in pxe::BaseCode unsafe](https://github.com/rust-osdev/uefi-rs/pull/1552)
- [uefi: Exclude null byte from CStr8 Display impl](https://github.com/rust-osdev/uefi-rs/pull/1553)
- [nix: switch to Nix Flake](https://github.com/rust-osdev/uefi-rs/pull/1560)
- [Fix unwanted rebuilds in xtask commands](https://github.com/rust-osdev/uefi-rs/pull/1559)
- [Create FUNDING.yml](https://github.com/rust-osdev/uefi-rs/pull/1563)
- [Implement conversions for IpAddress and MacAddress](https://github.com/rust-osdev/uefi-rs/pull/1564)
- [uefi: Clean up error docs in media protocols](https://github.com/rust-osdev/uefi-rs/pull/1568)
- [uefi: Reformat `use` items in pxe module](https://github.com/rust-osdev/uefi-rs/pull/1567)
- [uefi: Consistently use `&mut self` for pxe::BaseCode methods](https://github.com/rust-osdev/uefi-rs/pull/1566)
- [ci: fix typos](https://github.com/rust-osdev/uefi-rs/pull/1571)
- [Update Protocol/unsafe_protocol docs](https://github.com/rust-osdev/uefi-rs/pull/1574)
- [uefi: Use uefi_raw's `PxeBaseCodeProtocol` to implement the internals of `pxe::BaseCode`](https://github.com/rust-osdev/uefi-rs/pull/1576)
- [uefi-raw: Add DiskInfo protocol binding](https://github.com/rust-osdev/uefi-rs/pull/1580)
- [uefi-raw: Add EXT_SCSI_PASS_THRU protocol binding](https://github.com/rust-osdev/uefi-rs/pull/1581)
- [uefi runtime: Increase default size of name buffer](https://github.com/rust-osdev/uefi-rs/pull/1579)
- [uefi-raw: Add conversions to/from core::net IP address types](https://github.com/rust-osdev/uefi-rs/pull/1582)
- [uefi: Enable unsafe_op_in_unsafe_fn lint](https://github.com/rust-osdev/uefi-rs/pull/1585)
- [uefi: Make pxe::Mode an opaque struct](https://github.com/rust-osdev/uefi-rs/pull/1583)
- [uefi: Implement safe wrapper for EFI_DISK_INFO_PROTOCOL](https://github.com/rust-osdev/uefi-rs/pull/1590)
- [uefi-raw: Add EFI_NVM_EXPRESS_PASS_THRU_PROTOCOL bindings](https://github.com/rust-osdev/uefi-rs/pull/1591)
- [uefi-raw: Add documentation to ScsiIoScsiRequestPacket](https://github.com/rust-osdev/uefi-rs/pull/1593)
- [uefi-raw: Add EFI_ATA_PASS_THRU_PROTOCOL bindings](https://github.com/rust-osdev/uefi-rs/pull/1592)

<!-- - [chore(deps): update rust crate log to v0.4.26](https://github.com/rust-osdev/uefi-rs/pull/1555) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1565) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.30.2](https://github.com/rust-osdev/uefi-rs/pull/1577) -->
<!-- - [chore(deps): update rust crate log to v0.4.27](https://github.com/rust-osdev/uefi-rs/pull/1596) -->
<!-- - [fix(deps): update rust crate anyhow to v1.0.97](https://github.com/rust-osdev/uefi-rs/pull/1561)
- [fix(deps): update rust crate clap to v4.5.31](https://github.com/rust-osdev/uefi-rs/pull/1562)
- [fix(deps): update rust crate syn to v2.0.100](https://github.com/rust-osdev/uefi-rs/pull/1569)
- [fix(deps): update rust crate tempfile to v3.19.1](https://github.com/rust-osdev/uefi-rs/pull/1588)
- [fix(deps): update rust crate quote to v1.0.40](https://github.com/rust-osdev/uefi-rs/pull/1587)
- [fix(deps): update rust crate clap to v4.5.34](https://github.com/rust-osdev/uefi-rs/pull/1597) -->

Thanks to [@seijikun](https://github.com/seijikun), [@ifd3f](https://github.com/ifd3f), [@ptf2](https://github.com/ptf2), and [@quic-bjorande](https://github.com/quic-bjorande) for their contributions!


### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following improvements:

- [remove #[no_mangle] from panic handler](https://github.com/rust-osdev/bootloader/pull/500)


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

We merged the following changes to the [_Writing an OS in Rust_](https://os.phil-opp.com/) blog this month:

- [Update blog to Rust 2024 edition](https://github.com/phil-opp/blog_os/pull/1405)
- [Latam Spanish translation](https://github.com/phil-opp/blog_os/pull/1368)
- [Fix translation in `zh-CN` testing post](https://github.com/phil-opp/blog_os/pull/1407)

Thanks to [@dobleuber](https://github.com/dobleuber) and [@JINHUILYU](https://github.com/JINHUILYU) for their contributions!

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
