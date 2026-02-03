+++
title = "This Month in Rust OSDev: January 2026"
date = 2027-02-04

[extra]
month = "January 2026"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (January 2026)" post.
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

- [Patina - Developing UEFI With Rust](https://opendevicepartnership.github.io/patina/)
  - Repo: https://github.com/OpenDevicePartnership/patina
  - [Project Overview on Youtube](https://www.youtube.com/watch?v=iKzHrhpCgUI)

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

<!-- <span class="gray">No content was submitted for this section this month.</span> -->


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

### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following changes:

- [Add llvm-tools to components](https://github.com/rust-osdev/bootloader/pull/529)
- [Fix overflow error during address calculation](https://github.com/rust-osdev/bootloader/pull/530)
- [add kernel stack address to BootInfo](https://github.com/rust-osdev/bootloader/pull/531)
- [fix "cargo build --release"](https://github.com/rust-osdev/bootloader/pull/532)
- [Release 0.11.14](https://github.com/rust-osdev/bootloader/pull/533)

Thanks to [@Wasabi375](https://github.com/Wasabi375) and [@the-ssd](https://github.com/the-ssd) for their contributions!


### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

_Convenient and safe parsing of Multiboot2 Boot Information (MBI) structures and
the contained information tags. Usable in no_std environments, such as a kernel.
An optional builder feature also allows the construction of the corresponding
structures._

We merged the following PRs this month:

<!-- - [build(deps): bump crate-ci/typos from 1.40.0 to 1.41.0](https://github.com/rust-osdev/multiboot2/pull/285) -->
<!-- - [build(deps): bump actions/cache from 4 to 5](https://github.com/rust-osdev/multiboot2/pull/286) -->

<span class="gray">No notable changes this month.</span>


### [`ovmf-prebuilt`](https://github.com/rust-osdev/ovmf-prebuilt)
<span class="maintainers">Maintained by [@nicholasbishop](https://github.com/nicholasbishop) and [@phil-opp](https://github.com/phil-opp)</span>

The `ovmf-prebuilt` project provides pre-built [edk2](https://github.com/tianocore/edk2) releases to make it easier to set up OVMF. We merged the following improvement this month:

- [ovmf-prebuilt: Add download retry](https://github.com/rust-osdev/ovmf-prebuilt/pull/254)
- [release: 0.2.7](https://github.com/rust-osdev/ovmf-prebuilt/pull/256)

<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/258) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/259) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/260) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/261) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/263) -->


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [uefi: improve ergonomics of Events](https://github.com/rust-osdev/uefi-rs/pull/1840)
- [uefi: improve doc (add short glossary/important terms overview)](https://github.com/rust-osdev/uefi-rs/pull/1857)
- [workspace: switch to resolver v3](https://github.com/rust-osdev/uefi-rs/pull/1859)
- [flake: update (Nix shell: rust 1.86 -> 1.92)](https://github.com/rust-osdev/uefi-rs/pull/1860)
- [Replace remaining uses of `addr_of`/`addr_of_mut` with raw pointer syntax](https://github.com/rust-osdev/uefi-rs/pull/1870)
- [Bump MSRV to 1.88](https://github.com/rust-osdev/uefi-rs/pull/1871)
- [uefi-raw: serial: fix mutability](https://github.com/rust-osdev/uefi-rs/pull/1872)
- [uefi: serial: improve overall documentation](https://github.com/rust-osdev/uefi-rs/pull/1874)
- [uefi: boot: improve documentation for handles](https://github.com/rust-osdev/uefi-rs/pull/1877)
- [export all text::{input, output} types](https://github.com/rust-osdev/uefi-rs/pull/1880)
- [fix(deps): update rust crate nix to 0.31.0](https://github.com/rust-osdev/uefi-rs/pull/1883)

<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1863) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1866) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1868) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.42.0](https://github.com/rust-osdev/uefi-rs/pull/1869) -->
<!-- - [ci/fix: upgrade QEMU on windows](https://github.com/rust-osdev/uefi-rs/pull/1876) -->
<!-- - [Revert "ci/fix: upgrade QEMU on windows"](https://github.com/rust-osdev/uefi-rs/pull/1884) -->

Thanks to [@yaroslav957](https://github.com/yaroslav957) for their contribution!


### [`virtio-spec-rs`](https://github.com/rust-osdev/virtio-spec-rs)
<span class="maintainers">Maintained by [@mkroening](https://github.com/mkroening)</span>

The `virtio-spec` crate provides definitions from the Virtual I/O Device (VIRTIO) specification.
This project aims to be unopinionated regarding actual VIRTIO drivers that are implemented on top of this crate.

We merged the following PRs this month:

- [feat(virtio-mmio): add `MAGIC_VALUE` constant](https://github.com/rust-osdev/virtio-spec-rs/pull/11)
- [fix(virtq): clippy::manual_is_multiple_of](https://github.com/rust-osdev/virtio-spec-rs/pull/12)
- [feat(virtio-net): add `VIRTIO_NET_F_CTRL_RX_EXTRA`](https://github.com/rust-osdev/virtio-spec-rs/pull/13)
- [feat(virtio-net): add `VIRTIO_NET_F_GUEST_USO{4,6}`](https://github.com/rust-osdev/virtio-spec-rs/pull/14)
- [feat(features): add `recommendations` and `recommendations_satisfied`](https://github.com/rust-osdev/virtio-spec-rs/pull/15)

<!-- - [ci: update actions/checkout to v6](https://github.com/rust-osdev/virtio-spec-rs/pull/16) -->
<!-- - [ci: run cargo-semver-checks](https://github.com/rust-osdev/virtio-spec-rs/pull/19) -->


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PRs this month:

- [feat(mapper): add `MappedPageTable::display`](https://github.com/rust-osdev/x86_64/pull/574)
- [fix(instructions): allow unused_unsafe for cpuid](https://github.com/rust-osdev/x86_64/pull/575)
- [feat(mapper): make `OffsetPageTable` a type alias](https://github.com/rust-osdev/x86_64/pull/576)
- [fix(mapper): inline internal `map_to_*` functions](https://github.com/rust-osdev/x86_64/pull/578)
- [docs: fix typos](https://github.com/rust-osdev/x86_64/pull/579)
- [feat(paging): make range types `!Copy`](https://github.com/rust-osdev/x86_64/pull/581)
- [docs(page): fix typos](https://github.com/rust-osdev/x86_64/pull/582)
- [feat: make page types `repr(transparent)` and range types `repr(Rust)`](https://github.com/rust-osdev/x86_64/pull/584)

<!-- - [chore: merge master into next](https://github.com/rust-osdev/x86_64/pull/577) -->
<!-- - [ci: add typos job](https://github.com/rust-osdev/x86_64/pull/580) -->



## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


<!-- <span class="gray">No projects updates were submitted this month.</span> -->



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
