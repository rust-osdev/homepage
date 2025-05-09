+++
title = "This Month in Rust OSDev: April 2025"
date = 2025-05-04

[extra]
month = "April 2025"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (April 2025)" post.
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

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [uefi-raw: Add binding for EFI_DEVICE_PATH_UTILITIES_PROTOCOL](https://github.com/rust-osdev/uefi-rs/pull/1598)
- [clippy: fix](https://github.com/rust-osdev/uefi-rs/pull/1602)
- [helpers: Add AlignedBuffer](https://github.com/rust-osdev/uefi-rs/pull/1600)
- [xtask: fix nixfmt + update Nix flake + fix CI](https://github.com/rust-osdev/uefi-rs/pull/1607)
- [uefi/doc: improve documentation of exit_boot_services + change signature](https://github.com/rust-osdev/uefi-rs/pull/1605)
- [uefi: Some convenient DevicePathUtilities helper methods](https://github.com/rust-osdev/uefi-rs/pull/1599)
- [uefi: Add safe protocol wrapper for EFI_EXT_SCSI_PASS_THRU_PROTOCOL](https://github.com/rust-osdev/uefi-rs/pull/1589)
- [uefi-raw: Fix ATA_PASS_THRU_PROTOCOL bindings](https://github.com/rust-osdev/uefi-rs/pull/1619)
- [snp network test fixes](https://github.com/rust-osdev/uefi-rs/pull/1618)
- [uefi-raw: Add EFI_USB_IO_PROTOCOL bindings](https://github.com/rust-osdev/uefi-rs/pull/1623)
- [uefi: Add safe protocol wrapper for EFI_NVM_EXPRESS_PASS_THRU_PROTOCOL](https://github.com/rust-osdev/uefi-rs/pull/1594)
- [uefi: Move PoolString to enable additional use](https://github.com/rust-osdev/uefi-rs/pull/1624)
- [uefi-raw: Fix EFI_USB_IO_PROTOCOL implementation](https://github.com/rust-osdev/uefi-rs/pull/1626)
- [uefi-raw: Add EFI_USB2_HC_PROTOCOL bindings](https://github.com/rust-osdev/uefi-rs/pull/1629)
- [uefi: Add safe protocol wrapper for EFI_ATA_PASS_THRU_PROTOCOL](https://github.com/rust-osdev/uefi-rs/pull/1595)
- [uefi-raw: Move EFI_SIMPLE_NETWORK_PROTOCOL](https://github.com/rust-osdev/uefi-rs/pull/1634)
- [Enable access to hidden RAII guard components](https://github.com/rust-osdev/uefi-rs/pull/1635)
- [UEFI Allocator: add PAGE_SIZE shortcut ](https://github.com/rust-osdev/uefi-rs/pull/1611)
- [xtask: Update OVMF release to EDK2_STABLE202502_R2](https://github.com/rust-osdev/uefi-rs/pull/1637)
- [add ip4config2 + http protocols support](https://github.com/rust-osdev/uefi-rs/pull/1614)
- [uefi-raw: Redefine UsbPortStatus and UsbTransferStatus](https://github.com/rust-osdev/uefi-rs/pull/1638)
- [uefi: remove duplication in DevicePathHeader; use uefi-raw](https://github.com/rust-osdev/uefi-rs/pull/1613)
- [uefi: Implement CalculateCrc32() boot services function](https://github.com/rust-osdev/uefi-rs/pull/1649)

<!-- - [fix(deps): update rust crate clap to v4.5.35](https://github.com/rust-osdev/uefi-rs/pull/1610) -->
<!-- - [ci: fix](https://github.com/rust-osdev/uefi-rs/pull/1631) -->
<!-- - [chore(deps): update codecov/codecov-action action to v5.4.2](https://github.com/rust-osdev/uefi-rs/pull/1633) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.31.1](https://github.com/rust-osdev/uefi-rs/pull/1647) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1648) -->
<!-- - [ci: fix failing MSRV uefi-raw job](https://github.com/rust-osdev/uefi-rs/pull/1650) -->

Thanks to [@seijikun](https://github.com/seijikun), [@andersson](https://github.com/andersson), [@kraxel](https://github.com/kraxel), and [@JarlEvanson](https://github.com/JarlEvanson) for their contributions!


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PRs this month:

- [fix kani build job](https://github.com/rust-osdev/x86_64/pull/544)
- [improve `VirtAddr` `Add` & `Sub` impls](https://github.com/rust-osdev/x86_64/pull/543)
- [Revert #529 on master](https://github.com/rust-osdev/x86_64/pull/545)
- [merge master into next](https://github.com/rust-osdev/x86_64/pull/546)


### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following improvements:

- [implement Send+Sync for MemoryRegions](https://github.com/rust-osdev/bootloader/pull/502)



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

- [Added [[bin]] section to Cargo.toml](https://github.com/phil-opp/blog_os/pull/1412)
- [loading UEFI using ovmf_prebuilt=0.2.3 with ovmf_code and ovmf_vars](https://github.com/phil-opp/blog_os/pull/1410)

Thanks to [@tigeryant](https://github.com/tigeryant) and [@v4zha](https://github.com/v4zha) for their contributions!


## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
