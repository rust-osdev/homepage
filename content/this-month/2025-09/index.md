+++
title = "This Month in Rust OSDev: September 2025"
date = 2025-10-09

[extra]
month = "September 2025"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (September 2025)" post.
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

- [Redox OS: Development Priorities for 2025/26](https://www.redox-os.org/news/development-priorities-2025-09/)
- [This Month in Redox - September 2025](https://www.redox-os.org/news/this-month-250930/)
- [Upcoming Rust language features for kernel development](https://lwn.net/SubscriberLink/1039073/abf96f38b178f988/)
- [Kernel hackers at Cauldron, 2025 edition](https://lwn.net/SubscriberLink/1039784/d2548814efb78046/)
- [Linting Rust code in the kernel](https://lwn.net/Articles/1038750/)
- [Tracking trust with Rust in the kernel](https://lwn.net/Articles/1034603/)
- [MOROS 0.12.0](https://github.com/vinc/moros/releases/tag/v0.12.0)
- [blazesym 0.2 stable release: batteries included address symbolization](https://github.com/libbpf/blazesym/discussions/1318)
- [Building a tiling window manager for macOS in Rust](https://www.reddit.com/r/rust/comments/1nimfdf/building_a_tiling_window_manager_for_macos_in_rust/)
- Video: [(Kernel) Task Switching in Rust — by Jayden Qi — Seattle Rust User Group](https://www.youtube.com/watch?v=JP4-JJefY_A)

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [This Development-cycle in Cargo: 1.90](https://blog.rust-lang.org/inside-rust/2025/10/01/this-development-cycle-in-cargo-1.90/)
- [Allow `&raw [mut | const]` for union field in safe code](https://github.com/rust-lang/rust/pull/141469)

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
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following fix:

- [Fix: `target-pointer-width` field now expects an integer](https://github.com/rust-osdev/bootloader/pull/516)
- [Release `v0.11.12`](https://github.com/rust-osdev/bootloader/pull/517)
- [Fix(v0.9): target-pointer-width field now expects an integer](https://github.com/rust-osdev/bootloader/pull/518)


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [test-runner: Add example of using the Input protocol](https://github.com/rust-osdev/uefi-rs/pull/1755)
- [style: misc clippy fixes](https://github.com/rust-osdev/uefi-rs/pull/1760)
- [style: cargo doc fixes and improvements](https://github.com/rust-osdev/uefi-rs/pull/1759)
- [uefi: memory safety fixes (UB!)](https://github.com/rust-osdev/uefi-rs/pull/1758)
- [uefi: doc: Parameters -> Arguments](https://github.com/rust-osdev/uefi-rs/pull/1764)
- [Remove unnecessary unsafe block in example](https://github.com/rust-osdev/uefi-rs/pull/1765)
- [uefi: remove support for unstable allocator_api feature](https://github.com/rust-osdev/uefi-rs/pull/1763)

<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1762) -->
<!-- - [fix(deps): update rust crate serde_json to v1.0.145](https://github.com/rust-osdev/uefi-rs/pull/1767) -->
<!-- - [chore(deps): update rust crate trybuild to v1.0.111](https://github.com/rust-osdev/uefi-rs/pull/1766) -->

Thanks to [@BrokenC1oud](https://github.com/BrokenC1oud) for their contribution!


### [`virtio-spec-rs`](https://github.com/rust-osdev/virtio-spec-rs)
<span class="maintainers">Maintained by [@mkroening](https://github.com/mkroening)</span>

The `virtio-spec` crate provides definitions from the Virtual I/O Device (VIRTIO) specification.
This project aims to be unopinionated regarding actual VIRTIO drivers that are implemented on top of this crate.

We merged the following PRs this month:

- [feat: add traditional mem balloon device](https://github.com/rust-osdev/virtio-spec-rs/pull/6)
- [chore: release version 0.3.2](https://github.com/rust-osdev/virtio-spec-rs/pull/10)

Thanks to [@hcsch](https://github.com/hcsch) for their contribution!



## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


<!-- <span class="gray">No projects updates were submitted this month.</span> -->

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)
<span class="maintainers">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

We merged the following changes to the [_Writing an OS in Rust_](https://os.phil-opp.com/) blog this month:

- [Update post texts to Rust 2024](https://github.com/phil-opp/blog_os/pull/1432) (thanks to [@thaliaarchi](https://github.com/thaliaarchi)!)
- [Fix: `target-pointer-width` field now expects an integer](https://github.com/phil-opp/blog_os/pull/1436)
  - [Update blog for `target-pointer-width` change](https://github.com/phil-opp/blog_os/pull/1437)
- [Translate post 12 'Async/Await' into Russian](https://github.com/phil-opp/blog_os/pull/1439) (thanks to [@TakiMoysha](https://github.com/TakiMoysha)!)
- [Fix(post-02) Korean translation and typo](https://github.com/phil-opp/blog_os/pull/1440) (thanks to [reddevilmidzy](https://github.com/reddevilmidzy)!)

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
