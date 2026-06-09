+++
title = "This Month in Rust OSDev: May 2026"
date = 2026-06-02

[extra]
month = "May 2026"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

Please submit interesting posts and projects for the next issue [on Zulip](https://rust-osdev.zulipchat.com/#narrow/channel/435142-newsletter/topic/Content.20suggestions/with/580172810) or via a PR [on GitHub](https://github.com/rust-osdev/homepage/).

<span class="gray">
Disclaimer: Automated scripts and AI assistance were used for collecting and categorizing links.
Everything was proofread and checked manually, with many manual tweaks.
</span>


<!--
    This is a draft for the upcoming "This Month in Rust OSDev (May 2026)" post.
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

- [Rust for Linux Live with Alice Ryhl and Greg Kroah-Hartman](https://corrode.dev/podcast/s06e04-rust4linux/)
  - A live Rust Week recording about Rust in the Linux kernel, C/Rust interoperability, kernel driver work, and the day-to-day realities of writing kernel code in Rust.
- [Finding the Time Part 2 - Rust Async and the Arm Generic Timer](https://thejpster.org.uk/blog/blog-2026-05-17/)
  - Explores implementing async timers for Armv8-R systems using the Arm Generic Timer, interrupts, and Embassy's `embassy-time` driver model.
- [Building an AsyncIO executor for the 3DS](https://blog.cat-girl.gay/3ds-async-part-one/)
  - Starts a series on cooperative multitasking for Nintendo 3DS homebrew by building a small Rust executor and connecting async wakeups to platform scheduling constraints.
- [Rust x GBA: Setup and Pixels](https://jonahnestrick.com/blog/rust-gba-tutorial-1/)
  - Walks through setting up a nightly Rust project for Game Boy Advance ROM development, including `arm-none-eabi` tooling, Cargo configuration, `core`-only code, and pixel output.
- [uFerris: A Versatile Learner Board for Rust Embedded Beginners](https://www.theembeddedrustacean.com/uferris)
  - A new open source learner board and board support crate for embedded Rust exercises across Seeed XIAO-compatible MCUs.

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [Rust 1.96.0](https://blog.rust-lang.org/2026/05/28/Rust-1.96.0/)
  - Stabilizes new `core::range` types and `core::assert_matches!`/`core::debug_assert_matches!`.
  - WebAssembly targets now fail on undefined linker symbols by default instead of implicitly treating them as `"env"` imports.
  - Includes Cargo fixes for [CVE-2026-5223](https://blog.rust-lang.org/2026/05/25/cve-2026-5223/) and CVE-2026-5222 for users of third-party registries.
- More `std::io` pieces move toward `core::io`
  - [`Cursor`](https://github.com/rust-lang/rust/pull/156428) and [`std::io::util`](https://github.com/rust-lang/rust/pull/156431) were moved into `core::io`, continuing the effort to make I/O building blocks available in `no_std` contexts.
- [Proposal to stabilize the `Allocator` trait](https://github.com/rust-lang/rust/pull/156882)
  - A draft PR that proposes stabilizing the `Allocator` trait together with `Global`, `System`, and the `*_in` constructors (e.g. `Box::new_in`, `Vec::new_in`). This would let kernel and `no_std` code use custom allocators with the standard collections on stable Rust. The design still needs to go through FCP and bake further before it can land.
- [Cargo `clean -p` now respects `build.target`](https://github.com/rust-lang/cargo/pull/16988)
  - Makes `cargo clean -p` behave more predictably for projects that set a default cross-compilation target in Cargo configuration.
- [Promote five Thumb-mode bare-metal Arm targets to Tier 2](https://github.com/rust-lang/compiler-team/issues/985)
  - A compiler-team proposal entered final comment period to promote additional `thumb*-none-eabi*` bare-metal targets.
- [Add `-Zdead-fn-elimination`](https://github.com/rust-lang/compiler-team/issues/976)
  - A compiler-team proposal entered final comment period for a nightly codegen option that skips functions unreachable from exported symbols, which could improve compile time. Improvements on binary size are [not expected](https://rust-osdev.zulipchat.com/#narrow/channel/435142-newsletter/topic/Incorrect.20characterization.20of.20-Zdead-fn-elimination) though since the linker should already eliminate dead functions today.

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

### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers uses to relay information about the hardware to the OS.

We merged the following changes this month:

- [Use `contribute_arg` to handle `NoCurrentOp` at the top-level](https://github.com/rust-osdev/acpi/pull/293)
- [Logical not only requires one arg](https://github.com/rust-osdev/acpi/pull/295)
- [Move method context to `do_execute_method`](https://github.com/rust-osdev/acpi/pull/296)
- [Get referenced-package size - unwrap reference in `do_size_of`](https://github.com/rust-osdev/acpi/pull/288)
- [Allow BufferField -> Integer conversion](https://github.com/rust-osdev/acpi/pull/286)
- [Compare strings and buffers like uACPI](https://github.com/rust-osdev/acpi/pull/303)
- [Update uACPI example tests since some now work](https://github.com/rust-osdev/acpi/pull/299)
- [Directly test a stream of opcodes](https://github.com/rust-osdev/acpi/pull/298)
- [Fix regression in implicit casting from integer -> string](https://github.com/rust-osdev/acpi/pull/304)

Thanks to [@martin-hughes](https://github.com/martin-hughes) for their contributions!


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [Fix connect_controller to take a list for `driver_image`.](https://github.com/rust-osdev/uefi-rs/pull/1939)
- [AtaPassThru: Make AtaStatusBlock and AtaCommandBlock derive Clone](https://github.com/rust-osdev/uefi-rs/pull/1942)
- [various updates (nothing major or critical) + bump MSRV from 1.88 to 1.91](https://github.com/rust-osdev/uefi-rs/pull/1953)
- [uefi: move runtime module to directory](https://github.com/rust-osdev/uefi-rs/pull/1954)

<!-- - [chore(deps): update crate-ci/typos action to v1.46.0](https://github.com/rust-osdev/uefi-rs/pull/1940) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1941) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.46.1](https://github.com/rust-osdev/uefi-rs/pull/1944) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1946) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.46.2](https://github.com/rust-osdev/uefi-rs/pull/1948) -->
<!-- - [ci: fix broken uefi-macros test](https://github.com/rust-osdev/uefi-rs/pull/1952) -->

Thanks to [@seijikun](https://github.com/seijikun) for their contributions!


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PR this month:

- [implement DoubleEndedIterator for ranges + implement more iterator methods](https://github.com/rust-osdev/x86_64/pull/589)

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

- [fix: lazy_static is not needed, keyboard methods are const, ](https://github.com/phil-opp/blog_os/pull/1477)
- [[Translation] Translated post 8 and 9 into Russian](https://github.com/phil-opp/blog_os/pull/1480)
- [Adding Turkish translations of blog posts](https://github.com/phil-opp/blog_os/pull/1481)
- [Adjust blog for PR #1477](https://github.com/phil-opp/blog_os/pull/1482)

Thanks to [@harsh-98](https://github.com/harsh-98), [@TakiMoysha](https://github.com/TakiMoysha), and [@rhotav](https://github.com/rhotav) for their contributions!


<!-- <span class="gray">No projects updates were submitted this month.</span> -->



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way to get in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
