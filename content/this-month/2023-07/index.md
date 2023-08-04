+++
title = "This Month in Rust OSDev: July 2023"
date = 2023-08-04

[extra]
month = "July 2023"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (July 2023)" post.
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

- [Mutex without lock, Queue without push: cancel safety in lilos](https://cliffle.com/blog/lilos-cancel-safety/)
- [Kani 0.32.0 has been released!](https://www.reddit.com/r/KaniRustVerifier/comments/14xytrg/kani_0320_has_been_released/) (verification tool for unsafe code blocks)
- [bwrap: A fast, lightweight, embedded environment-friendly Rust library for wrapping text](https://www.reddit.com/r/rust/comments/151usd5/bwrap_a_fast_lightweight_embedded/)
- ESP32 Standard Library Embedded Rust
  - [GPIO Control](https://apollolabsblog.hashnode.dev/esp32-standard-library-embedded-rust-gpio-control)
  - [UART Communication](https://apollolabsblog.hashnode.dev/esp32-standard-library-embedded-rust-uart-communication)
  - [I2C Communication](https://apollolabsblog.hashnode.dev/esp32-standard-library-embedded-rust-i2c-communication)
- [`allocator_api2`](https://docs.rs/allocator-api2/latest/allocator_api2/) crate to use Rust's nightly allocator API on stable Rust

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [Enable `chkstk`/`alloca` intrinsics on `x86_64-unknown-uefi`](https://github.com/rust-lang/compiler-builtins/pull/541)
- [Add `riscv64gc-unknown-hermit` target](https://github.com/rust-lang/rust/pull/114004)
- [Add `x86_64-unikraft-linux-musl` target](https://github.com/rust-lang/rust/pull/113411)
- [Optimize `AtomicBool` for target that don't support byte-sized atomics](https://github.com/rust-lang/rust/pull/114034)


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

### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

The [multiboot2](https://crates.io/crates/multiboot2) was bumped from `0.16.0` 
to `0.17.0`. The new release includes the builder pattern for the MBI builder 
and the ability to use custom memory types in the memory map in addition to 
pre-defined ones. For more info, look [here](https://docs.rs/multiboot2/0.17.0/multiboot2/struct.MemoryAreaTypeId.html).

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

The `uefi-rs` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. We merged the following PRs this month:

- [Cargo.lock: Update proc-macro2 to fix nightly CI](https://github.com/rust-osdev/uefi-rs/pull/885)
- [Return a SimpleFileSystem from BootServices::get_image_file_system](https://github.com/rust-osdev/uefi-rs/pull/886)
- [uefi-raw: Derive Ord, PartialOrd, and Hash for all newtype enums](https://github.com/rust-osdev/uefi-rs/pull/887)
- [Add `GraphicsOutputProtocol` to `uefi_raw` and use it from `uefi`](https://github.com/rust-osdev/uefi-rs/pull/888)
- [uefi: Use uefi_raw's SimplePointerProtocol to implement Pointer](https://github.com/rust-osdev/uefi-rs/pull/889)
- [build(deps): bump regex from 1.8.1 to 1.9.0](https://github.com/rust-osdev/uefi-rs/pull/890)
- [Fix target_arch name: i386 -> x86](https://github.com/rust-osdev/uefi-rs/pull/891)
- [Add some uefi-raw links/badges](https://github.com/rust-osdev/uefi-rs/pull/892)
- [Fix a new lint in 1.71](https://github.com/rust-osdev/uefi-rs/pull/894)
- [Add raw serial protocol and use it in `uefi`](https://github.com/rust-osdev/uefi-rs/pull/897)
- [Add raw disk protocols and use them in `uefi`](https://github.com/rust-osdev/uefi-rs/pull/895)
- [uefi: Change try_exists to return FileSystemResult<bool>](https://github.com/rust-osdev/uefi-rs/pull/898)
- [dependabot: Enable updates for Github Actions](https://github.com/rust-osdev/uefi-rs/pull/900)
- [Rework `FileSystem::copy` to operate on 1MiB chunks](https://github.com/rust-osdev/uefi-rs/pull/899)
- [build(deps): bump crate-ci/typos from 1.13.20 to 1.16.1](https://github.com/rust-osdev/uefi-rs/pull/902)
- [ci: Add merge_group trigger](https://github.com/rust-osdev/uefi-rs/pull/903)
- [build(deps): bump cachix/install-nix-action from 20 to 22](https://github.com/rust-osdev/uefi-rs/pull/901)
- [add event to smp](https://github.com/rust-osdev/uefi-rs/pull/907)

Thanks to [@devsnek](https://github.com/devsnek) for their contribution!


### [`xhci`](https://github.com/rust-osdev/xhci)
<span class="maintainers">Maintained by [@toku-sa-n](https://github.com/toku-sa-n)</span>

The `xhci` crate provides types of xHCI structures, such as Registers and TRBs. We merged the following PRs this month:

- [Switch Clippy runner on CI](https://github.com/rust-osdev/xhci/pull/156)
- [Fix wrong calculation of data_buffer_pointer of transfer::Normal](https://github.com/rust-osdev/xhci/pull/154)
- [Release 0.9.2](https://github.com/rust-osdev/xhci/pull/157)

Thanks to [@lemolatoon](https://github.com/lemolatoon) for their contribution!


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PR this month:

- [Add `Descriptor::tss_segment_unchecked`](https://github.com/rust-osdev/x86_64/pull/428)

Thanks to [@SamZhang3](https://github.com/SamZhang3) for their contribution!


### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we released a new patch version to fix the build on the latest nightlies:

- [Run `cargo update` to fix build on nightly](https://github.com/rust-osdev/bootloader/pull/385)
- [Release `v0.11.4`](https://github.com/rust-osdev/bootloader/pull/386)


### [`ovmf-prebuilt`](https://github.com/rust-osdev/ovmf-prebuilt)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `ovmf-prebuilt` project provides pre-built [edk2](https://github.com/tianocore/edk2) releases to make it easier to set up OVMF. We merged the following improvement this month:

- [Dockerfile: use QemuBuild.py for aarch64 build](https://github.com/rust-osdev/ovmf-prebuilt/pull/2)

Thanks to [@nicholasbishop](https://github.com/nicholasbishop) for their contribution!

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.


<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


### [`SFBdragon/talc`](https://github.com/SFBdragon/talc)
<span class="maintainers">(Section written by [@SFBdragon](https://github.com/SFBdragon))</span>

`Talc` is a recently published, fast, and flexible `no-std` memory allocator. 
* It's the fastest allocator I've tested as of yet (galloc falls short, buddy_alloc is close but lacks heap efficiency).
* It features a OOM-handling component with dynamic arena resizing.

By the time you're seeing this, hopefully v2 should be out or coming soon:
* The OOM handler system has been made more powerful.
* `lock_api` is used to allow for custom allocator synchronization.
* The internals and API has been improved to pass miri's stacked borrows validation.
* You can now move the allocator struct around freely.
* And more :3

I hope you find it useful!

### [`vinc/moros`](https://github.com/vinc/moros)
<span class="maintainers">(Section written by [@vinc](https://github.com/vinc))</span>

[MOROS](http://moros.cc) is a text-based hobby operating system targeting computers with a x86-64 architecture and a BIOS.

Since last month's [release](https://github.com/vinc/moros/releases/tag/v0.10.0), I focused on adding new syscalls to interact with [network sockets](https://github.com/vinc/moros/pull/512) from userspace. The DNS and HTTP clients are now using the new UDP and TCP sockets.

I also added another syscall to poll multiple handles at the same time, to read from the console and a socket, improving the main network tool that can now be used as a simple chat program.

The VGA driver, the filesystem, and the editor got a few significant [improvements](https://github.com/vinc/moros/blob/trunk/CHANGELOG.md) as well, to support downloading and reading larger files.

### [`valibali/cluu`](https://github.com/valibali/cluu)
<span class="maintainers">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

There is a new project featured in our [Showcase](@/showcase/_index.md) series:

- [**CLUU (Compact Lightweight Unix Utopia)**](@/showcase/cluu/index.md) by [@valibali](https://github.com/valibali)



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
