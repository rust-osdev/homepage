+++
title = "This Month in Rust OSDev: April 2022"
date = 2022-05-07

[extra]
month = "April 2022"
authors = [
    "phil-opp",
    "toku-sa-n",
    "phip1611",
    "andre-richter",
    "berkus",
    "GabrielMajeri",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (April 2022)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

### [`x86_64`](https://github.com/rust-osdev/x86_64)

<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13), and [@rybot666](https://github.com/orgs/rust-osdev/people/rybot666)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

In April, we merged the following improvements and fixes:

- [Provide null segment selector as associated constant on SegmentSelector](https://github.com/rust-osdev/x86_64/pull/373)
- [Correct wrong comment](https://github.com/rust-osdev/x86_64/pull/374)
- [Fix align functions](https://github.com/rust-osdev/x86_64/pull/375)
- [Cleanup Segment macros](https://github.com/rust-osdev/x86_64/pull/376)
- [Update comment and docs](https://github.com/rust-osdev/x86_64/pull/382)

Thanks to [@prinzdezibel](https://github.com/prinzdezibel) for their contribution!

We also merged two updates into the `next` branch for the upcoming `v0.15` release:

- [Add `structures::gdt::Entry` type](https://github.com/rust-osdev/x86_64/pull/380)
- [Allow GDT to be loaded with shared reference](https://github.com/rust-osdev/x86_64/pull/381)

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri) and [@nicholasbishop](https://github.com/nicholasbishop)</span>

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS.

We merged the following changes in April:

#### Features

- [Add support to get the file path of loaded image](https://github.com/rust-osdev/uefi-rs/pull/398)
- [Add `FilePathMediaDevicePath` (and a bunch of supporting code)](https://github.com/rust-osdev/uefi-rs/pull/404)
- [Improve device path API](https://github.com/rust-osdev/uefi-rs/pull/421)

#### Bug fixes

- [Fix undefined behavior in `File::get_boxed_info`](https://github.com/rust-osdev/uefi-rs/pull/407)
- [Fix potential undefined behavior in file info](https://github.com/rust-osdev/uefi-rs/pull/408)
- [Fix `test_get_boxed_info` when `-Zmiri-tag-raw-pointers` is enabled](https://github.com/rust-osdev/uefi-rs/pull/415)
- [Fix off-by-one test error](https://github.com/rust-osdev/uefi-rs/pull/422)

#### CI and linting

- [Enable the `clippy::ptr_as_ptr` lint](https://github.com/rust-osdev/uefi-rs/pull/410)

#### Documentation improvements

- [Add changelog entries for recent PRs](https://github.com/rust-osdev/uefi-rs/pull/405)
- [Add documentation on why `UnsafeCell` is used for protocols](https://github.com/rust-osdev/uefi-rs/pull/409)
- [Add documentation links for `BootServices` and `RuntimeServices`](https://github.com/rust-osdev/uefi-rs/pull/419)

Thanks to [@supdrewin](https://github.com/supdrewin), [@nicholasbishop](https://github.com/nicholasbishop) and [@raccog](https://github.com/raccog) for their contributions!

### [`bootloader`](https://github.com/rust-osdev/bootloader)

<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@rybot666](https://github.com/rybot666), and [@64](https://github.com/64)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following changes:

- [Add dynamic range configuration](https://github.com/rust-osdev/bootloader/pull/229)
- [Correct typos in `src/binary/level_4_entries.rs`](https://github.com/rust-osdev/bootloader/pull/228)

Thanks to [@Freax13](https://github.com/Freax13) and [@toothbrush7777777](https://github.com/toothbrush7777777) for these contributions!

We also made some good progress on the upcoming `v0.11` version of the crate:

- [Replace artifact dependency with build script](https://github.com/rust-osdev/bootloader/commit/ee9bf20314be3e304f4210c03ba73348d4868c7c)
- [Add function to create UEFI disk image to library](https://github.com/rust-osdev/bootloader/commit/aefa4aa68fa25c7c52a2d84dbf4f664dee3e6390)
- [Adjust and merge PR #219 into next](https://github.com/rust-osdev/bootloader/commit/fd7b2cfc1c5ddf0c6253a61fbba9012718c91754)
- [Integrate latest changes from `main`](https://github.com/rust-osdev/bootloader/commit/fdf9d4bfd10adc66c23c1b3a07ba22f3b716d35a)
- [Update test framework to rewrite](https://github.com/rust-osdev/bootloader/commit/4d4b5849831c12e81e2c6897fbf051fda73aa02d)
- [Merge PR #229 into next and adjust it to new config system](https://github.com/rust-osdev/bootloader/commit/3aef58c1b59c1ee1529d079145cf3c2de1c51302)
- [Remove uefi dependency of common crate](https://github.com/rust-osdev/bootloader/commit/5dbc5038d71af72095315c7de194370f1ddb8540)
- [Replace `rand_chacha` with `rand_hc`](https://github.com/rust-osdev/bootloader/commit/6134fedfa1ce0cf7adddc3f8b13cca21201f7cbd)
- [Start integrating BIOS bootloader into build system](https://github.com/rust-osdev/bootloader/commit/eb6d71f244c54031bddbf87042d710bcdd1aa0bb)
- [Simplify MBR code and choose bootable partition](https://github.com/rust-osdev/bootloader/commit/0d77948df2c331612c94e186672a34a1694f23de)
- [Refactor FAT creation function to take arbitrary file list](https://github.com/rust-osdev/bootloader/commit/d8b50dcb801dfc775e6f211a8f0e3d3bf2486ad7)
- [Integrate BIOS bootsector into build system](https://github.com/rust-osdev/bootloader/commit/5310e4b97743094d45dd0711939e1961463da9e3)
- [Build and load a second stage](https://github.com/rust-osdev/bootloader/commit/b3207dbf62b27202f3afa2fc19e0b8bf4893b1fb)
- [Use pie relocation model for second stage](https://github.com/rust-osdev/bootloader/commit/4d41efe84c14487d824f5270667ed93baa274772)
- [Clean up boot sector code](https://github.com/rust-osdev/bootloader/commit/5fd311542c73f3014136ce0b4ecbed133443c01b)

The next steps now are: setting up unreal mode on the CPU, loading the kernel from the FAT partition, loading the memory map, and setting up the page tables.

### [`multiboot2`](https://github.com/rust-osdev/multiboot2)

<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods), [@phip1611](https://github.com/phip1611), [@robert-w-gries](https://github.com/robert-w-gries), [@ahmedcharles](https://github.com/ahmedcharles), and [@Caduser2020](https://github.com/Caduser2020)</span>

The `multiboot2` crate provides abstraction types for the multiboot information structure (MBI) of multiboot2 
bootloaders. The latest release of the `multiboot2`-crate is now `v.0.13.2` (was `v0.13.1`). It contains minor 
improvements, such as that `TagType` implements `Ord`.

### [`multboot2-header`](https://github.com/rust-osdev/multiboot2)

<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods), [@phip1611](https://github.com/phip1611), [@robert-w-gries](https://github.com/robert-w-gries), [@ahmedcharles](https://github.com/ahmedcharles), and [@Caduser2020](https://github.com/Caduser2020)</span>

The `multiboot2-header` crate provides abstraction types for Multiboot2 headers and a builder struct to construct such 
headers. The latest release of the `multiboot2-header`-crate is now `v0.2.0` (was `v0.1.0`). The changes include a 
bugfix that prevented the usage in `no_std` contexts. Furthermore, overall code quality was improved. The internal CI 
was updated to verify `no_std` builds as well as regular builds. 
Full changelog: <https://github.com/rust-osdev/multiboot2/releases/tag/multiboot2-header-v0.2.0>

### [`uart_16550`](https://github.com/rust-osdev/uart_16550)

<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `uart_16550` crate provides basic support for serial port I/O for 16550-compatible UARTs. We merged the following change this month:

- [Remove use of `stable` and `nightly` features](https://github.com/rust-osdev/uart_16550/pull/24) <span class="gray">(published as `v0.2.18`)</span>

Thanks to [@josephlr](https://github.com/josephlr) for this contribution!

### [`volatile`](https://github.com/rust-osdev/volatile)

<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `volatile` crate provides a safe wrapper type for implementing volatile read and write operations. This is useful for accessing memory regions that have side-effects, such as memory-mapped hardware registers or framebuffers. In April, we merged the following pull request:

- [Remove the const_generics feature flag](https://github.com/rust-osdev/volatile/pull/25) <span class="gray">(published as `v0.4.5`)</span>

Thanks to [@hawkw](https://github.com/hawkw) for this contribution!

### [`xhci`](https://github.com/rust-osdev/xhci)

<span class="maintainers">Maintained by [@toku-sa-n](https://github.com/toku-sa-n)</span>

The `xhci` crate provides types of xHCI structures, such as Registers and TRBs.

In this month, we released a new version 0.8.3 which includes [a bug fix](https://github.com/rust-osdev/xhci/pull/132) reported and committed by [@Yuna-Tomi](https://github.com/Yuna-Tomi). The bug was that `EventRingDequeuePointerRegister::event_ring_dequeue_pointer()` did not return the correct address. Thanks for the contribution!.


## Call for Participation

Want to contribute to a Rust OSDev project, but don't know where to start? Pick up one of these outstanding
issues in one of our projects and get started!

- [(`multiboot2`) Missing Tags](https://github.com/rust-osdev/multiboot2/issues/100)

<!--
Please use the following template for adding items:
- [(`repo_name`) Issue Description](https://example.com/link-to-issue)
-->

If you maintain a Rust project related to operating system development and are looking for contributors, especially for tasks suited to people getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the `next` branch with the tasks you want to include in the next issue.

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`rust-embedded/rust-raspberrypi-OS-tutorials`](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)

<span class="gray">(Section written by [@andre-richter](https://github.com/andre-richter))</span>

The [Operating System development tutorials in Rust on the Raspberry Pi](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials) project
saw two more tutorial releases:

- [Tutorial 17 - `Kernel Symbols`](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/tree/master/17_kernel_symbols)
- [Tutorial 18 - `Backtracing`](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/tree/master/18_backtrace)

The two tutorials implement the generation of backtraces that show address and symbol information. Here is an example of the of a backtrace generated as part of  the kernel's panic handler:

```
[    0.002782] Writing to bottom of address space to address 1 GiB...
[    0.004623] Kernel panic!

Panic location:
      File 'kernel/src/_arch/aarch64/exception.rs', line 59, column 5

[...]

Backtrace:
      ----------------------------------------------------------
          Address            Function containing address
      ----------------------------------------------------------
       1. ffffffffc0001294 | core::fmt::write
       2. ffffffffc0005560 | libkernel::panic_wait::_panic_print
       3. ffffffffc00054a0 | rust_begin_unwind
       4. ffffffffc0002950 | core::panicking::panic_fmt
       5. ffffffffc0004898 | current_elx_synchronous
       6. ffffffffc0000a74 | __vector_current_elx_synchronous
       7. ffffffffc000111c | kernel_init
      -----------------------------------------------------------
```

### [`metta-systems/vesper`](https://github.com/metta-systems/vesper)

<span class="gray">(Section written by [@berkus](https://github.com/berkus))</span>

Vesper is a capability-based single-address-space nanokernel. This means it is aiming to be small, to provide only isolation primitives; at the same time SAS makes it a lot easier to perform cross-process operations (because all addresses are the same across all processes). It uses capabilities to provide security for such operations, so that unauthorized processes will not be able to intervene in legitimate traffic.

The kernel is in very early stages of development. This time I will update on the progress of tooling and my next steps. As usual, I will link directly to my blog for more details. [Read the full article here](https://metta.systems/blog/osdev-tooling-3/).

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

We merged the following improvements for the "Writing an OS in Rust" blog this in April:

- [Korean translation for chapter 2](https://github.com/phil-opp/blog_os/pull/1101)
- [Update the code in the `post-06` branch](https://github.com/phil-opp/blog_os/pull/1099)
- [fix: typo](https://github.com/phil-opp/blog_os/pull/1103)

Thanks to [@JOE1994](https://github.com/JOE1994), [@QuqqU](https://github.com/QuqqU), [@ruhuang2001](https://github.com/ruhuang2001), and [@PoorlyDefinedBehaviour](https://github.com/PoorlyDefinedBehaviour) for their contributions!

For the current status of the upcoming third edition, see [my comment](https://github.com/phil-opp/blog_os/issues/1025#issuecomment-1107843473) on the related GitHub issue.

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
