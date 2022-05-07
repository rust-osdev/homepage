+++
title = "This Month in Rust OSDev (November 2021)"
date = 2021-12-06

[extra]
month = "November 2021"
authors = [
    "phil-opp",
    "berkus"
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (November 2021)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS.

We merged the following PRs this month:

- [loaded_image: document size argument unit](https://github.com/rust-osdev/uefi-rs/pull/308)
- [Revert "Temporarily disable a false-positive clippy lint"](https://github.com/rust-osdev/uefi-rs/pull/312)
- [Fix `locate_device_path` impl argument pointer](https://github.com/rust-osdev/uefi-rs/pull/310)
- [Change `Handle` representation to be non-nullable so that `Option<Handle>` is FFI-safe](https://github.com/rust-osdev/uefi-rs/pull/309)
- [Improve `Handle` buffer handling code](https://github.com/rust-osdev/uefi-rs/pull/314)
- [Add `CStr16::from_str_with_buf`](https://github.com/rust-osdev/uefi-rs/pull/291)
- [Update and reorganize documentation](https://github.com/rust-osdev/uefi-rs/pull/315)
- [Add flag to `build.py` for disabling KVM](https://github.com/rust-osdev/uefi-rs/pull/316)

Thanks to [@necauqua](https://github.com/necauqua) and [@baloo](https://github.com/baloo) for their contributions!

### [`x86_64`](https://github.com/rust-osdev/x86_64)

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

In November, [@Freax13](https://github.com/Freax13) joined our `x86_64` maintenance team after doing [a lot of great work](https://github.com/rust-osdev/x86_64/pulls?q=is%3Apr+is%3Aclosed+author%3AFreax13) on the crate. Welcome!

We merged the following non-breaking changes this month:

- [Add `set_general_handler` macro](https://github.com/rust-osdev/x86_64/pull/285)
- [Add the VMM Communication Exception (`#VC`) to the `InterruptDescriptorTable`](https://github.com/rust-osdev/x86_64/pull/313)
- [Derive common traits for number, range and enum types](https://github.com/rust-osdev/x86_64/pull/315)
- [Remove redundant alignment check](https://github.com/rust-osdev/x86_64/pull/314)
- [fix(idt): fix panic messages for index and `#VC`](https://github.com/rust-osdev/x86_64/pull/321)

We also merged a number of breaking changes that will go into the [upcoming 0.15 release](https://github.com/rust-osdev/x86_64/issues/262):

- [Implement `Index<u8>` for IDT instead of `Index<usize>`](https://github.com/rust-osdev/x86_64/pull/319)
- [Fix memory safety of `load_tss` and `GlobalDescriptorTable`](https://github.com/rust-osdev/x86_64/pull/323)
- [Change type of `InterruptStackFrameValue::cpu_flags` to `RFlags`](https://github.com/rust-osdev/x86_64/pull/324)
- [Add `InvalidStarSegmentSelectors` error type](https://github.com/rust-osdev/x86_64/pull/317)
- [Add `PcidTooBig` error](https://github.com/rust-osdev/x86_64/pull/316)
- [Activate `feature(asm_const)`](https://github.com/rust-osdev/x86_64/pull/320)

Thanks to [@Freax13](https://github.com/Freax13), [@haraldh](https://github.com/haraldh), and [@mpajkowski](https://github.com/mpajkowski) for their contributions!

## Call for Participation

Want to contribute to a Rust OSDev project, but don't know where to start? Pick up one of these outstanding
issues in one of our projects and get started!

<!--
Please use the following template for adding items:
- [(`repo_name`) Issue Description](https://example.com/link-to-issue)
-->

<span class="gray">

_No tasks were proposed for this section._

</span>

If you maintain a Rust project related to operating system development and are looking for contributors, especially for tasks suited to people getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the `next` branch with the tasks you want to include in the next issue.

## Personal Projects

In this section, we describe updates to personal projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`metta-systems/vesper`](https://github.com/metta-systems/vesper)

<span class="gray">(Section written by [@berkus](https://github.com/berkus))</span>

Vesper is a capability-based single-address-space nanokernel. This means it is aiming to be small, to provide only isolation primitives; at the same time SAS makes it a lot easier to perform cross-process operations (because all addresses are the same across all processes). It uses capabilities to provide security for such operations, so that unauthorized processes will not be able to intervene in legitimate traffic.

It's in very early stages of development and is a basis for a larger envisioned system. The progress is fairly slow, only allowed as my available time permits. This month to motivate me to move it faster I've decided to start posting monthly development updates. The first post is about the tools I use.

Since [rebooting to Rust](https://metta.systems/blog/reboot-to-rust/) almost 4 years ago I've been constantly amazed by the language ecosystem and what wonders are possible. This time I want to tell about incredible tooling that makes my OSdev experience a sunny warm place in contrast to the barren lands of my previous OSdev environments. [Read the full article here](https://metta.systems/blog/osdev-tooling/).

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

There were no visible changes to the [_Writing an OS in Rust_](https://os.phil-opp.com/) series this month, but I continued working on the new build system for the upcoming third edition. One particular change that I want to highlight is that I redesigned the configuration of the `bootloader` crate. Instead of passing it via [a `package.metadata.bootloader` section](https://docs.rs/bootloader/0.10.9/bootloader/struct.Config.html) in the kernel's `Cargo.toml`, users will pass a configuration struct to the [`entry_point`](https://docs.rs/bootloader/0.10.9/bootloader/macro.entry_point.html) macro. This struct is then serialized at compile time into a separate section in the ELF executable, which the bootloader can then read on loading. This change should make the build process easier and more flexible.

I plan to simplify the build system further, but I'm currently waiting on some upcoming `cargo` features for that. In particular, I think that [_artifact dependencies_](https://github.com/rust-lang/cargo/pull/9992) and [_per-packet configuration_](https://internals.rust-lang.org/t/proposal-move-some-cargo-config-settings-to-cargo-toml/13336) options will be very useful for the project, especially the config options I mentioned in my [status update comment](https://github.com/phil-opp/blog_os/issues/1063#issuecomment-968341112) on GitHub.

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
