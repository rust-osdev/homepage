+++
title = "This Month in Rust OSDev (February 2021)"
date = 0000-01-01

[extra]
month = "February 2021"
authors = [
    "phil-opp",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we will give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (February 2021)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. In February, we merged the following changes:

- [Add helper function for retrieving the boot filesystem](https://github.com/rust-osdev/uefi-rs/pull/201)
- [Add support for the block I/O protocol](https://github.com/rust-osdev/uefi-rs/pull/200)
- [Update `x86_64` dependency to version 0.13.2](https://github.com/rust-osdev/uefi-rs/pull/198) (to fix nightly breakage)
- [Fix some issues with the documentation of the `DevicePath` and `LoadedImage` protocols](https://github.com/rust-osdev/uefi-rs/pull/193)

Thanks to [@gil0mendes](https://github.com/gil0mendes), [@sreehax](https://github.com/sreehax), and [@avirule](https://github.com/avirule) for their contributions!

### [`x86_64`](https://github.com/rust-osdev/x86_64)

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

In February, the unstable [`const_in_array_repeat_expressions` feature](https://github.com/rust-lang/rust/issues/49147) was [removed](https://github.com/rust-lang/rust/pull/80404) from Rust because it [implicitly changed drop behavior](https://github.com/rust-lang/rust/issues/49147#issuecomment-766372999). This lead to a compile error of the `x86_64` crate because it still had that feature enabled. Interestingly, we no longer needed this feature after [#175](https://github.com/rust-osdev/x86_64/pull/175) (and an [accidental stabilization](https://github.com/rust-lang/rust/pull/79270) in Rust), so the fix was quite simple:

- [Fix build on latest nightly](https://github.com/rust-osdev/x86_64/pull/230) <span class="gray">(published as `v0.13.2`)</span>

Thanks to [@KernelFreeze](https://github.com/KernelFreeze) for this contribution!

### [`bootloader`](https://github.com/rust-osdev/bootloader)

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged two small updates to fix build errors and warnings on newer Rust nightlies:

- [Fix build on latest nightly by updating x86_64 to v0.13.2](https://github.com/rust-osdev/bootloader/pull/135) <span class="gray">(published as `v0.9.12`)</span>
- [Fix "panic message is not a string literal"](https://github.com/rust-osdev/bootloader/pull/138) <span class="gray">(published as `v0.9.14`)</span>

Thanks to [@dspencer12](https://github.com/dspencer12) for their contribution!

There was also some more progress on the `uefi` branch, which contains the upcoming new bootloader version with UEFI support:

- [Improve reporting of config parse errors](https://github.com/rust-osdev/bootloader/commit/d55f1c87c34e8bba61adc6abffa78ba431aac69f)
- [Add a test for the `map-physical-memory` config key](https://github.com/rust-osdev/bootloader/commit/6a0fd74ecb052ef3f1fa7ce3e556c895c66dfc4e)
- Add more checks for the given `--kernel-manifest` path: it should [point to a file named `Cargo.toml`](https://github.com/rust-osdev/bootloader/commit/38fd48622c3a6f22d64a65528a56d2471168cb78), [exist](https://github.com/rust-osdev/bootloader/commit/9a8ace78650d75189d567618a90a4f039525f369), and the referenced `Cargo.toml` should [depend on the bootloader crate](https://github.com/rust-osdev/bootloader/commit/873351c575bdefd1c6c78b27de2bc0494698c0d5).

The UEFI rewrite is almost done, but we still need to update the docs, improve the configurability of the framebuffer, and add more testing.

### [`uart_16550`](https://github.com/rust-osdev/uart_16550)

The `uart_16550` crate provides basic support for serial port I/O for 16550-compatible UARTs. Since the crate also depends on `x86_64`, it needed a dependency update to fix the mentioned build error on the latest nightly:

- [Fix build on nightly by updating to x86_64 v0.13.2](https://github.com/rust-osdev/uart_16550/pull/12) <span class="gray">(published as `v0.2.12`)</span>

### [`vga`](https://github.com/rust-osdev/vga)

The work-in-progress `vga` crate allows the configuration of the VGA hardware, e.g. switching from text-based mode to a pixel-based graphics mode. The nightly build error of `x86_64` also affected this crate, so it needed a fix too:

- [fix: should now compile](https://github.com/rust-osdev/vga/pull/20) <span class="gray">(published as `v0.2.6`)</span>

Thanks to [@Pollux3737](https://github.com/Pollux3737) for this contribution!

## Personal Projects

In this section, we describe updates to personal projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

This month, ...

### [`cdrzewiecki/celos`](https://gitlab.com/cdrzewiecki/celos)

<span class="gray">(Section written by [@drzewiec](https://github.com/drzewiec))</span>

I have been working on an OS following along with [@phil-opp](https://github.com/phil-opp)'s tutorial series for a while, but recently decided I would rework my OS based on the first edition of the blog (since I preferred to use GRUB as my bootloader). This is the first progress I have to share on CelOS, and indeed the first time I've published one of these updates in general.

In February, I made a lot of great progress on CelOS. I have the complete physical memory (plus the framebuffer provided by GRUB) mapped to virtual memory, and a pixel-based framebuffer working with text output. Things are not very optimized right now (for one thing I'm stretching the `font8x8` font into 8x12), but this is a great first step that I can build on. Next planned steps are:

* Move the kernel in virtual memory so that it occupies the higher half of the 48-bit address space
* Create some page fault interrupt handling so that the kernel can at least attempt to handle page faults (rather than triple faulting as it does now)
* Set up memory allocation for the kernel, to get heap allocation
* Once heap allocation is in place, utilize some existing crate to handle TrueType fonts so that text will look a bit nicer on screen

I probably won't get all of that done in March, but those are my planned next steps. Thanks to this great community and to [@phil-opp](https://github.com/phil-opp) for being so helpful in the osdev journey!

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).


<!--
TODO: Update publication date
-->
