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

- [Add llvm-tools to components](https://github.com/rust-osdev/bootloader/pull/529)
- [Fix overflow error during address calculation](https://github.com/rust-osdev/bootloader/pull/530)
- [fix "cargo build --release"](https://github.com/rust-osdev/bootloader/pull/532)
- [add kernel stack address to BootInfo](https://github.com/rust-osdev/bootloader/pull/531)
- [Release 0.11.14](https://github.com/rust-osdev/bootloader/pull/533)

Thanks to [@Wasabi375](https://github.com/Wasabi375), and [@the-ssd](https://github.com/the-ssd) for their contributions!


### [`multiboot2`](https://github.com/rust-osdev/multiboot2)

- [build(deps): bump crate-ci/typos from 1.40.0 to 1.41.0](https://github.com/rust-osdev/multiboot2/pull/285)
- [build(deps): bump actions/cache from 4 to 5](https://github.com/rust-osdev/multiboot2/pull/286)

Thanks to [@dependabot[bot]](https://github.com/apps/dependabot) for their contributions!


### [`ovmf-prebuilt`](https://github.com/rust-osdev/ovmf-prebuilt)

- [ovmf-prebuilt: Add download retry](https://github.com/rust-osdev/ovmf-prebuilt/pull/254)
- [release: 0.2.7](https://github.com/rust-osdev/ovmf-prebuilt/pull/256)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/258)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/259)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/260)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/261)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/263)

Thanks to [@nicholasbishop](https://github.com/nicholasbishop), and [@renovate[bot]](https://github.com/apps/renovate) for their contributions!


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

- [workspace: switch to resolver v3](https://github.com/rust-osdev/uefi-rs/pull/1859)
- [flake: update (Nix shell: rust 1.86 -> 1.92)](https://github.com/rust-osdev/uefi-rs/pull/1860)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1863)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1866)
- [uefi: improve ergonomics of Events](https://github.com/rust-osdev/uefi-rs/pull/1840)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1868)
- [chore(deps): update crate-ci/typos action to v1.42.0](https://github.com/rust-osdev/uefi-rs/pull/1869)
- [uefi: improve doc (add short glossary/important terms overview)](https://github.com/rust-osdev/uefi-rs/pull/1857)
- [Bump MSRV to 1.88](https://github.com/rust-osdev/uefi-rs/pull/1871)
- [Replace remaining uses of `addr_of`/`addr_of_mut` with raw pointer syntax](https://github.com/rust-osdev/uefi-rs/pull/1870)
- [export all text::{input, output} types](https://github.com/rust-osdev/uefi-rs/pull/1880)
- [ci/fix: upgrade QEMU on windows](https://github.com/rust-osdev/uefi-rs/pull/1876)
- [uefi-raw: serial: fix mutability](https://github.com/rust-osdev/uefi-rs/pull/1872)
- [uefi: serial: improve overall documentation](https://github.com/rust-osdev/uefi-rs/pull/1874)
- [uefi: boot: improve documentation for handles](https://github.com/rust-osdev/uefi-rs/pull/1877)
- [Revert "ci/fix: upgrade QEMU on windows"](https://github.com/rust-osdev/uefi-rs/pull/1884)
- [fix(deps): update rust crate nix to 0.31.0](https://github.com/rust-osdev/uefi-rs/pull/1883)

Thanks to [@nicholasbishop](https://github.com/nicholasbishop), [@phip1611](https://github.com/phip1611), [@renovate[bot]](https://github.com/apps/renovate), and [@yaroslav957](https://github.com/yaroslav957) for their contributions!


### [`virtio-spec-rs`](https://github.com/rust-osdev/virtio-spec-rs)

- [fix(virtq): clippy::manual_is_multiple_of](https://github.com/rust-osdev/virtio-spec-rs/pull/12)
- [feat(virtio-net): add `VIRTIO_NET_F_CTRL_RX_EXTRA`](https://github.com/rust-osdev/virtio-spec-rs/pull/13)
- [feat(virtio-mmio): add `MAGIC_VALUE` constant](https://github.com/rust-osdev/virtio-spec-rs/pull/11)
- [feat(virtio-net): add `VIRTIO_NET_F_GUEST_USO{4,6}`](https://github.com/rust-osdev/virtio-spec-rs/pull/14)
- [ci: update actions/checkout to v6](https://github.com/rust-osdev/virtio-spec-rs/pull/16)
- [feat(features): add `recommendations` and `recommendations_satisfied`](https://github.com/rust-osdev/virtio-spec-rs/pull/15)
- [ci: run cargo-semver-checks](https://github.com/rust-osdev/virtio-spec-rs/pull/19)

Thanks to [@mkroening](https://github.com/mkroening) for their contributions!


### [`x86_64`](https://github.com/rust-osdev/x86_64)

- [fix(instructions): allow unused_unsafe for cpuid](https://github.com/rust-osdev/x86_64/pull/575)
- [chore: merge master into next](https://github.com/rust-osdev/x86_64/pull/577)
- [feat(mapper): make `OffsetPageTable` a type alias](https://github.com/rust-osdev/x86_64/pull/576)
- [fix(mapper): inline internal `map_to_*` functions](https://github.com/rust-osdev/x86_64/pull/578)
- [docs: fix typos](https://github.com/rust-osdev/x86_64/pull/579)
- [ci: add typos job](https://github.com/rust-osdev/x86_64/pull/580)
- [docs(page): fix typos](https://github.com/rust-osdev/x86_64/pull/582)
- [feat(paging): make range types `!Copy`](https://github.com/rust-osdev/x86_64/pull/581)
- [feat: make page types `repr(transparent)` and range types `repr(Rust)`](https://github.com/rust-osdev/x86_64/pull/584)
- [feat(mapper): add `MappedPageTable::display`](https://github.com/rust-osdev/x86_64/pull/574)

Thanks to [@mkroening](https://github.com/mkroening) for their contributions!



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
