+++
title = "This Month in Rust OSDev: November 2023"
date = 2023-12-07

[extra]
month = "November 2023"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (November 2023)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Announcements, News, and Blog Posts

Here we collect news, blog posts, etc. related to OS development in Rust.

- [Officially Qualified - Ferrocene](https://ferrous-systems.com/blog/officially-qualified-ferrocene/)
- [Edge IoT with Rust on ESP: NTP](https://apollolabsblog.hashnode.dev/edge-iot-with-rust-on-esp-ntp)

<!--
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->


## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

There weren't really any OS-related infrastructure updates this month, but there was some great progress on several upcoming language and tooling features that may also be of interest to OS development:

- [Stabilize C string literals](https://github.com/rust-lang/rust/pull/117472)
- [Stabilize `ptr::addr_eq`](https://github.com/rust-lang/rust/pull/117968)
- [Feature gate enums in `offset_of`](https://github.com/rust-lang/rust/pull/117537)

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

The `uefi-rs` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. We merged the following PRs this month:

- [Configure Renovate](https://github.com/rust-osdev/uefi-rs/pull/986)
- [uefi-raw: Add AbsolutePointerProtocol](https://github.com/rust-osdev/uefi-rs/pull/990)
- [Add SimpleFileSystemProtocol & file types to `uefi-raw`, use those types from `uefi`](https://github.com/rust-osdev/uefi-rs/pull/991)
- [uefi-raw: Add API guidelines](https://github.com/rust-osdev/uefi-rs/pull/992)
- [uefi-macros: Change uefi dev-dependency from version to path](https://github.com/rust-osdev/uefi-rs/pull/998)
- [Add per-package changelogs](https://github.com/rust-osdev/uefi-rs/pull/997)
- [test-runner: Improve uninstall_protocol_interface example](https://github.com/rust-osdev/uefi-rs/pull/931)
- [Release via Github Actions workflow](https://github.com/rust-osdev/uefi-rs/pull/999)
- [release: uefi-raw-0.5.0, uefi-macros-0.13.0, uefi-0.26.0, uefi-services-0.23.0](https://github.com/rust-osdev/uefi-rs/pull/1001)
- [uefi-raw: Fill in [un]install_multiple_protocol_interfaces pointers](https://github.com/rust-osdev/uefi-rs/pull/1000)
- [book: Use `cargo add` command](https://github.com/rust-osdev/uefi-rs/pull/1002)
- [doc: update PUBLISHING.md](https://github.com/rust-osdev/uefi-rs/pull/959)
- [uefi(data-types): allow `is_ascii` function on `Char16` and `CStr16`](https://github.com/rust-osdev/uefi-rs/pull/1008)

<!---
- [chore(deps): update crate-ci/typos action to v1.16.22](https://github.com/rust-osdev/uefi-rs/pull/988)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/989)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/993)
- [chore(deps): update crate-ci/typos action to v1.16.23](https://github.com/rust-osdev/uefi-rs/pull/1005)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1006)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1010)
- [nix: update rust-toolchain in shell](https://github.com/rust-osdev/uefi-rs/pull/1007)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1012)
- [fix(deps): update rust crate itertools to 0.12.0](https://github.com/rust-osdev/uefi-rs/pull/1009)
--->

Thanks to [@RaitoBezarius](https://github.com/RaitoBezarius) for their contributions!


### [`linked-list-allocator`](https://github.com/rust-osdev/linked-list-allocator)

<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@jamesmunns](https://github.com/jamesmunns)</span>

The `linked-list-allocator` crate provides a basic `no_std` allocator that builds a linked list from freed memory blocks and thus needs no additional data structures. We merged the following PR this month:

- [fuzz: remove potential undefined behavior in chaos harness](https://github.com/rust-osdev/linked-list-allocator/pull/80)

Thanks to [@00xc](https://github.com/00xc) for their contribution!


### [`pic8259`](https://github.com/rust-osdev/pic8259)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `pic_8259` crate provides abstractions for 8259 and 8259A Programmable Interrupt Controllers (PICs).

We merged the following PR this month:

- [docs: Remove the redundant word and space](https://github.com/rust-osdev/pic8259/pull/5)

Thanks to [@zoo868e](https://github.com/zoo868e) for their contribution!

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<span class="gray">No projects updates were submitted this month.</span>

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
