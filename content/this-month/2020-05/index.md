+++
title = "This Month in Rust OSDev (May 2020)"
date = 2020-06-07

[extra]
month = "May 2020"
authors = [
    "phil-opp",
    "IsaacWoods",
    "GabrielMajeri",
    "stlankes",
    "andre-richter",
]
+++

Welcome to the second issue of _"This Month in Rust OSDev"_. In these posts, we will give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new).

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (May 2020)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.

    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

<!--

## News and Blog Posts

We try to collect posts that are relevant to Rust-based OS development each month. Please create pull requests for any posts that you want linked in the next issue.

-->

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

### [`x86_64`](https://github.com/rust-osdev/x86_64)

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

In May, the crate received a bugfix for inclusive page/frame ranges. We also landed a long-awaited change to the `Mapper::map_to` function to set certain page table flags also in higher level page tables:

- [Fix: Inclusive ranges is_empty() comparison](https://github.com/rust-osdev/x86_64/pull/156) <span class="gray">(published as `v0.10.3`)</span>
- [Handle parent table flags in Mapper methods](https://github.com/rust-osdev/x86_64/pull/114) <span class="gray">(published as `v0.11.0`)</span>

Thanks to [@haraldh](https://github.com/haraldh) and [@mrll](https://github.com/mrll) for their contributions!

### [`cargo-xbuild`](https://github.com/rust-osdev/cargo-xbuild)

The `cargo-xbuild` project provides `cargo` command wrappers to cross-compile the sysroot crates `core` and `alloc`.

This month, `rustc`/`cargo` [changed their codegen behavior for builds with link-time optimization (LTO)](https://github.com/rust-lang/cargo/pull/8192), which lead to [breakage for LTO builds with `cargo-xbuild`](https://github.com/rust-osdev/cargo-xbuild/issues/69). Fixing this issue was not easy and required [multiple](https://github.com/rust-osdev/cargo-xbuild/pull/70) [tries](https://github.com/rust-osdev/cargo-xbuild/pull/71) until we found a proper [solution](https://github.com/rust-osdev/cargo-xbuild/pull/73).

Unfortunately, the solution still led to [link issues](https://github.com/rust-osdev/cargo-xbuild/issues/72) for some people, which was caused by a [bug in `cargo`/`rustc`](https://github.com/rust-lang/cargo/issues/8239). Thanks to [@alexcrichton](https://github.com/alexcrichton), this was quickly [fixed in `rustc`](https://github.com/rust-lang/rust/pull/72325), so that now everything should work again.

Apart from these bugfixes, there were also two other changes this month:

- [Respect Cargo.lock file for sysroot build](https://github.com/rust-osdev/cargo-xbuild/pull/75) <span class="gray">(published as `v0.32`)</span>
- [Don't print warning about missing root package in quiet mode](https://github.com/rust-osdev/cargo-xbuild/pull/79) <span class="gray">(published as `v0.5.33`)</span>

Thanks to [@Nils-TUD](https://github.com/Nils-TUD) for their contribution and to all the people that helped investigating the build errors!

### [`acpi`](https://github.com/rust-osdev/acpi)

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern
computers use to relay information about the hardware to the OS.

Not a lot happened this month, but preparations
were made to change how the AML namespace is stored on the heap, in order to avoid a lot of small heap allocations
for AML paths, and reduce the number of heap allocations overall. However, some more profiling infrastructure will
need to be built before starting this.

In preparation, a [change to how `DefName`s are stored in the namespace](https://github.com/rust-osdev/acpi/commit/3b08721981d85e7bd82124db8c72e0c31d243771) was made, which avoids an extra heap
allocation per `DefName`, and also allows us to simplify some code around the `aml` crate.

### [`bootloader`](https://github.com/rust-osdev/bootloader)

The [`bootloader` crate](https://github.com/rust-osdev/bootloader) implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. It received the following updates this month:

- [Update x86_64 dependency to version 0.11.0](https://github.com/rust-osdev/bootloader/pull/117) <span class="gray">(published as `v0.9.3`)</span>
- [Remove unused feature gates](https://github.com/rust-osdev/bootloader/pull/118)
- [Add recursive_idx for boot info](https://github.com/rust-osdev/bootloader/pull/116) <span class="gray">(published as `v0.9.4`)</span>

Thanks to [@mark-i-m](https://github.com/mark-i-m) and [@Aaron1011](https://github.com/Aaron1011) for their contributions!

### [`bootimage`](https://github.com/rust-osdev/bootimage)

The `bootimage` tool allows the creation of bootable disk images for `bootloader`-based kernels. It also provides a runner executable for `cargo` to make `cargo xrun` and `cargo xtest` work using QEMU. In May, the crate was almost completely rewritten with a smaller API to make it more maintainable:

- [Rewrite: Remove support for `bootimage {run, test}`](https://github.com/rust-osdev/bootimage/pull/55) <span class="gray">(published as `v0.8.0`)</span>

### [`uart_16550`](https://github.com/rust-osdev/uart_16550)

The `uart_16550` crate provides basic support for serial port I/O for 16550-compatible UARTs. This month, the crate received the following updates:

- [Use `spin_loop_hint` while waiting for data](https://github.com/rust-osdev/uart_16550/pull/9) <span class="gray">(published as `v0.2.6`)</span>
- [Update x86_64 dependency to v0.11.0](https://github.com/rust-osdev/uart_16550/commit/7faedcab2d266e758913d394c499db8dc2d40aed) <span class="gray">(published as `v0.2.7`)</span>

Thanks to [@dbeckwith](https://github.com/dbeckwith) for their contribution!

### [`uefi`](https://github.com/rust-osdev/uefi-rs)

The `uefi` crate provides abstractions for the [`UEFI`](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface) standard that replaces the traditional BIOS on modern systems. This month, the crate's dependencies were updated, and a bug was fixed in the graphics protocol:

- [Bump x86_64 dependency version to fix build on latest nightlies](https://github.com/rust-osdev/uefi-rs/pull/134)
- [Fix `BltPixel::from` conversion](https://github.com/rust-osdev/uefi-rs/pull/135)

Thanks to [@imtsuki](https://github.com/imtsuki) and [@BinaryTENSHi](https://github.com/BinaryTENSHi) for their contributions!

<!--
## New Projects

There are a number of new projects in the `rust-osdev` organization:
-->

## Personal Projects

In this section, we describe updates to personal projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`IsaacWoods/pebble`](https://github.com/IsaacWoods/pebble)

<span class="gray">(Section written by [@IsaacWoods](https://github.com/IsaacWoods))</span>

This month, I've been tracking down a bug in the kernel that causes usermode stacks to become corrupted on the
return of some system calls. While I've not found the root cause, I've taken this opportunity to add some more
debugging capabilities, which have found a number of other kernel bugs.

I also started working on the Rust implementation of [Ptah](https://github.com/IsaacWoods/pebble/tree/master/lib/ptah/src),
the wire format for Pebble's message passing interface. It allows libraries to easily use [Serde](https://serde.rs/) to
serialize and deserialize Rust types to and from the wire format. In the future, a compiler will be able to
generate bindings for `Ptah` for a number of different supported languages, which will allow tasks written in
different languages to communicate over Pebble `Channel`s using idiomatic types.

### [`RustyHermit`](https://github.com/hermitcore/rusty-hermit)

<span class="gray">(Section written by [@stlankes](https://github.com/stlankes))</span>

RustyHermit is a [unikernel](http://unikernel.org) targeting a scalable and predictable runtime. Unikernel means, you bundle your application directly with the kernel library, so that it can run without any installed operating system.
This reduces image size and overhead, therefore, interesting applications include virtual machines and high-performance computing.

This month the integration of [smoltcp](https://github.com/smoltcp-rs/smoltcp) has been improved and first support for virtio has been integrated. The integration in Rust's standard runtime is already in progress and clients can be developed with `TcpStream`. Server side applications will follow soon.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

This month, support for the legacy `asm!` macro was removed from `rustc`. This lead to build errors for the _Writing an OS in Rust_ project because some dependencies were still using the macro. To fix this, I landed a number of dependency updates:

- [Update bootloader to v0.9.3](https://github.com/phil-opp/blog_os/pull/808)
- [Update x86_64 dependency to version 0.11.0](https://github.com/phil-opp/blog_os/pull/809)
- [Update Hardware Interrupts post to use pic8259_simple v0.2.0](https://github.com/phil-opp/blog_os/pull/810)

I also decided to change the design of the executor in the Async/Await post to simplify it:

- [Simplify executor by merging task_queue and wake_queue](https://github.com/phil-opp/blog_os/pull/804)
- [Update Async/Await post for simplified executor design](https://github.com/phil-opp/blog_os/pull/805)

While I already started a draft on the next blog post about processes, I currently plan to look into adding UEFI support to the `bootloader` crate first. The reason is that UEFI support will require some fundamental changes to the blog because the VGA text buffer and the legacy PIC are not supported on UEFI. This is also relevant to the upcoming post about processes, since different designs would be possible if we used the APIC instead of the legacy PIC. It therefore makes most sense to me to sort this out first.

### [`rust-embedded/rust-raspberrypi-OS-tutorials`](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)

<span class="gray">(Section written by [@andre-richter](https://github.com/andre-richter))</span>

May was a quiet month for the project, since I am currently taking a little hiatus. I am planning to restart working on new content two or three months down the road.

Therefore, only some maintenance updates went in, e.g. bumping all dependency crates to versions that support the new `llvm_asm!` macros. Also, the :cn: chinese translations received updates/additions ([main `Readme`](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/blob/master/README.CN.md); [`00_before_we_start`](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/blob/master/00_before_we_start/README.CN.md)).

<!--
## Call for Participation
-->

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
