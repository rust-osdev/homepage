+++
title = "This Month in Rust OSDev (November 2020)"
date = 0000-01-01

[extra]
month = "November 2020"
authors = [
    "phil-opp",
    "IsaacWoods",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we will give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new).

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (November 2020)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

### [`acpi`](https://github.com/rust-osdev/acpi)

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern
computers use to relay information about the hardware to the OS. In November, we started fuzzing the AML parser to
help find inputs that crash it and we [found](https://github.com/rust-osdev/acpi/commit/56472490c9564b6740eb5e416624d73be8841faa)
[a](https://github.com/rust-osdev/acpi/commit/5ab486d1a8a8a8160025b88e369e22dc8d993273) [few](https://github.com/rust-osdev/acpi/commit/747bcfd28d44bbdfd39ad4805bba574ac320daf8).
We even found [a case](https://github.com/rust-osdev/acpi/commit/52b05fd91ebb40e9c5511d568b19cb5f10b33d83) where
we'd misinterpreted the spec. This is an important task for the project, as the AML parser will often run in
kernelspace, and so should not panic from any input, however invalid (some more work is needed to make this the
case, however).

[Lexicographic comparison was also implemented for `Buffer` and `String` AML objects](https://github.com/rust-osdev/acpi/commit/6d2045de3acb9b74347ac6ce9ad01051be7bea82),
which means we should now be able to perform all comparisons tables are allowed to make (bar some object
conversions, which still need some work).

The changes this month, as well as some made in December that should improve compile speed a little, have been
published as [`aml v0.10.0`](https://crates.io/crates/aml).

### [`x86_64`](https://github.com/rust-osdev/x86_64)

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

In November, we merged the following updates:

- [Don't deny warnings on CI](https://github.com/rust-osdev/x86_64/pull/201)
- [Rename `enable_interrupts_and_hlt` to `enable_and_hlt`](https://github.com/rust-osdev/x86_64/pull/206)
- [Release version 0.12.3](https://github.com/rust-osdev/x86_64/pull/200)

Thanks to [@toku-sa-n](https://github.com/toku-sa-n) for their contribution!

### [`multiboot2`](https://github.com/rust-osdev/multiboot2-elf64)

The `multiboot2` crate provides abstraction types for the boot information of multiboot2 bootloaders. We merged the following updates this month:

- [Access to non-available memory areas](https://github.com/rust-osdev/multiboot2-elf64/pull/71) <span class="gray">(published as `v0.10.0`)</span>
- [Fix a few warnings](https://github.com/rust-osdev/multiboot2-elf64/commit/a1237bcf357e5d4a5a6c40038fd1e690ef7305d9) <span class="gray">(published as `v0.10.1`)</span>

Thanks to [@CalebLBaker](https://github.com/CalebLBaker) for their contribution!

## Personal Projects

In this section, we describe updates to personal projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

This month, ...

### [`IsaacWoods/pebble`](https://github.com/IsaacWoods/pebble)

<span class="gray">(Section written by [@IsaacWoods](https://github.com/IsaacWoods))</span>

Between university and work on `acpi`, I haven't had a huge amount of time to work on Pebble for the last couple of
months, but in November I:
- Implemented a basic form of [TLS](https://en.wikipedia.org/wiki/Thread-local_storage) for userspace tasks. Pebble
  doesn't have threads, but an Address Space can have multiple Tasks running from the same image, each of which
  need their copy of the master TLS record. TLS support is also needed very early in Rust's `std`, so this was the
  next step in creating a `std` implementation for Pebble.
- Tried to fix a bug in Pebble's UEFI bootloader, where we crash if memory allocated to Boot Services is unmapped
  after `ExitBootServices`. This may be a bug in OVMF - please get in touch if you've come across something
  similar and know what's going on!
- Continued work on the USB XHCI driver
- Improved detection of Intel microarchitectures - we can now differentiate Kaby Lake and Coffee Lake processors
  based on their `cpuid` steppings

### [`lucis-fluxum/ps2-rs`](https://github.com/lucis-fluxum/ps2-rs)

<span class="gray">(Section written by [@lucis-fluxum](https://github.com/lucis-fluxum))</span>

This is a new library I created to provide OS kernels with low-level access to the PS/2 controller and devices. It
uses a poll-based approach with a timeout to read and write data to the IO ports.

While some of the library's functionality won't work on modern devices due to differing implementations of PS/2
emulation between manufacturers, it should be enough to get initialized and receiving scancodes and mouse events.
Theoretically, it should work with PS/2-compatible keyboards all the way back to the IBM Model M!

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).


<!--
TODO: Update publication date
-->
