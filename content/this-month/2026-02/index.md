+++
title = "This Month in Rust OSDev: February 2026"
date = 2026-03-04

[extra]
month = "February 2026"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (February 2026)" post.
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

- [Linux 7.0 Officially Concluding The Rust Experiment](https://www.phoronix.com/news/Linux-7.0-Rust)
  - Rust is now formally accepted as a permanent part of the Linux kernel, shipping in production across multiple distributions and millions of Android devices.
- [The future for Tyr](https://lwn.net/SubscriberLink/1055590/12d48275b6f81988/)
  - Progress on Tyr, a Rust GPU driver for Arm Mali hardware. The DRM subsystem is reportedly "about a year away" from requiring new drivers to be written in Rust.
- [Microsoft LiteBox](https://github.com/microsoft/litebox)
  - A Rust library OS supporting kernel- and user-mode execution, with sandboxing for running unmodified Linux programs on Windows and SEV SNP.
- [This Month in Redox - January 2026](https://www.redox-os.org/news/this-month-260131/)
  - Redox got Cargo and rustc compiling natively and made progress on capability-based security.
- [CHERIoT Rust: Status update #0](https://rust.cheriot.org/2026/02/15/status-update.html)
  - Six months of progress porting Rust to the CHERIoT capability-based hardware architecture. Core and alloc now compile for the new `riscv32cheriot-unknown-cheriotrtos` target.
- [Ariel OS v0.3.0: BLE, Sensors, UART, and More!](https://ariel-os.org/blog/ariel-os-0.3.0/)
  - New release adds BLE support, hardware-agnostic UART drivers, sensor abstraction, and expanded MCU support (ESP32, STM32, Nordic, RP).
- [Async/await on the GPU](https://www.vectorware.com/blog/async-await-on-gpu/)
  - Rust's Future trait and async/await running on GPU hardware, reusing the Embassy embedded executor.


## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [RFC: Unsafe fields](https://github.com/rust-lang/rfcs/pull/3458)
  - Approved for implementation. Allows marking struct fields as `unsafe` when they maintain safety invariants.
- [Add `try_shrink_to` and `try_shrink_to_fit` to Vec](https://github.com/rust-lang/rust/pull/152366)
  - Fallible shrinking methods that return `TryReserveError` instead of panicking on OOM.
- [Stabilize `ptr_as_ref_unchecked`](https://github.com/rust-lang/rust/pull/151995)
  - Raw-pointer-to-reference conversion without null/alignment checks.
- [Stabilize `atomic_try_update` and deprecate `fetch_update`](https://github.com/rust-lang/rust/pull/148590)
  - Cleaner API for lock-free atomic compare-and-swap loops. `fetch_update` will be deprecated starting in Rust 1.99.0.
- [Add `avr_target_feature`](https://github.com/rust-lang/rust/pull/146900)
  - Unstable target features for AVR microcontrollers including `tinyencoding`, `lowbytefirst`, and various instruction features.
- [Stabilize `cfg_select!`](https://github.com/rust-lang/rust/pull/149783)
  - Builtin macro for selecting code based on cfg predicates, simplifying conditional compilation.
- [Stabilize `core::hint::cold_path`](https://github.com/rust-lang/rust/pull/151576)
  - Mark unlikely code paths to help the compiler optimize branch layout.

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

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS.

We merged the following changes this month:

- [Add support for extracting NUMA information from SRAT and SLIT](https://github.com/rust-osdev/acpi/pull/264)
- [Suggestion: Use colored instead of termion](https://github.com/rust-osdev/acpi/pull/265)

Thanks to [@martin-hughes](https://github.com/martin-hughes) for their contributions!


### [`bootimage`](https://github.com/rust-osdev/bootimage)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `bootimage` tool allows the creation of bootable disk images for `bootloader`-based kernels. It also provides a runner executable for `cargo` to make `cargo run` and `cargo test` work using QEMU.

We merged the following changes this month:

- [Add an example kernel that uses the built-in `x86_64-unknown-none` target](https://github.com/rust-osdev/bootimage/pull/104)
- [Fix bootloader build by passing `-Zjson-target-spec`](https://github.com/rust-osdev/bootimage/pull/105)
- [Prepare release](https://github.com/rust-osdev/bootimage/pull/106)
- [Fix QEMU test flakiness on ARM64 macOS](https://github.com/rust-osdev/bootimage/pull/107)


### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following changes:

- [Expose data for custom boot image creation](https://github.com/rust-osdev/bootloader/pull/535)
- [build with -Zjson-target-spec](https://github.com/rust-osdev/bootloader/pull/536)
- [Enable json-target-spec](https://github.com/rust-osdev/bootloader/pull/537)
- [release 0.11.15](https://github.com/rust-osdev/bootloader/pull/538)
- [fix typo in year](https://github.com/rust-osdev/bootloader/pull/539)
- [update bootloader crates](https://github.com/rust-osdev/bootloader/pull/545)

<!-- - [Bump bytes from 1.10.1 to 1.11.1 in /examples/basic](https://github.com/rust-osdev/bootloader/pull/540) -->

Thanks to [@Freax13](https://github.com/Freax13) and [@Wasabi375](https://github.com/Wasabi375) for their contributions!


### [`endian-num`](https://github.com/rust-osdev/endian-num)
<span class="maintainers">Maintained by [@mkroening](https://github.com/mkroening)</span>

The `endian-num` crate provides the `Be` (big-endian) and `Le` (little-endian) byte-order-aware numeric types.

We merged the following changes this month:

- [feat: add MSRV of 1.71](https://github.com/rust-osdev/endian-num/pull/7)
- [fix(docsrs): migrate from `doc_auto_cfg` to `doc_cfg`](https://github.com/rust-osdev/endian-num/pull/6)


### [`fuse-abi`](https://github.com/rust-osdev/fuse-abi)
<span class="maintainers">Maintained by [@mkroening](https://github.com/mkroening)</span>

The `fuse-abi` crate provides bindings to FUSE devices. In motivation similar to that of `virtio-spec`, this project aims to provide correct foundational definitions for the FUSE kernel ABI.

We merged the following changes this month:

- [fix(docsrs): migrate from `doc_auto_cfg` to `doc_cfg`](https://github.com/rust-osdev/fuse-abi/pull/1)

<!-- - [chore: upgrade bindgen-cli to 0.72.1](https://github.com/rust-osdev/fuse-abi/pull/2) -->


### [`spinning_top`](https://github.com/rust-osdev/spinning_top)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `spinning_top` crate provides a simple spinlock implementation based on the abstractions of the [`lock_api`](https://docs.rs/lock_api/0.4.1/lock_api/) crate.

We merged the following changes this month:

- [fix(docsrs): migrate from `doc_auto_cfg` to `doc_cfg`](https://github.com/rust-osdev/spinning_top/pull/27)


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [uefi: Add device path generation for discovered devices in a PciTree](https://github.com/rust-osdev/uefi-rs/pull/1831)
- [uefi: significantly improve ergonomics of Handle (device path and component2 protocols)](https://github.com/rust-osdev/uefi-rs/pull/1858)
- [uefi-raw & uefi: serial: add support for protocol revision 1.1](https://github.com/rust-osdev/uefi-rs/pull/1873)
- [Improve docs of `OpenProtocolAttributes`](https://github.com/rust-osdev/uefi-rs/pull/1891)
- [uefi: Add `handle_protocol` doc alias to open_protocol functions](https://github.com/rust-osdev/uefi-rs/pull/1893)
- [uefi: serial: improve documentation and correctness of read() and write()](https://github.com/rust-osdev/uefi-rs/pull/1900)

<!-- - [chore(deps): update crate-ci/typos action to v1.42.3](https://github.com/rust-osdev/uefi-rs/pull/1885) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1889) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1890) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1895) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1904) -->

Thanks to [@seijikun](https://github.com/seijikun) for their contributions!


### [`virtio-spec-rs`](https://github.com/rust-osdev/virtio-spec-rs)
<span class="maintainers">Maintained by [@mkroening](https://github.com/mkroening)</span>

The `virtio-spec` crate provides definitions from the Virtual I/O Device (VIRTIO) specification.
This project aims to be unopinionated regarding actual VIRTIO drivers that are implemented on top of this crate.

We merged the following PRs this month:

- [build(deps): upgrade bitfield-structs to 0.12](https://github.com/rust-osdev/virtio-spec-rs/pull/17)
- [build(deps): update allocator-api2 to 0.4](https://github.com/rust-osdev/virtio-spec-rs/pull/18)
- [fix(docsrs): migrate from `doc_auto_cfg` to `doc_cfg`](https://github.com/rust-osdev/virtio-spec-rs/pull/21)
- [fix: rust-2018-idioms](https://github.com/rust-osdev/virtio-spec-rs/pull/22)
- [fix: upgrade to Rust 2024](https://github.com/rust-osdev/virtio-spec-rs/pull/23)


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

### [`metta-systems/ram-map-viewer`](https://github.com/metta-systems/ram-map-viewer)
<span class="maintainers">(Section written by [@berkus](https://github.com/berkus))</span>

Added a little GUI for visualizing memory maps. The application itself currently supports a map format that my initialisation code emits, but
it has a MemorySource trait that you can implement to consume any format.

<video src="https://raw.githubusercontent.com/metta-systems/ram-map-viewer/video/video/ram-map-viewer.mp4" controls></video>

### [`metta-systems/vesper`](https://github.com/metta-systems/vesper)
<span class="maintainers">(Section written by [@berkus](https://github.com/berkus))</span>

Vesper has learned to put nucleus into higher-half memory and added a kernel syscall API with a single syscall - a capability invocation. I have a list of capabilities I want to implement, and first one implemented is Debug Console so you can output execution trace from your domains.

Init thread implementation in progress, now parsing DTB and preparing to launch a user-space "init" process. The video above shows memory map produced by the init thread after parsing a RasPi3 B+ 1Gb DTB.


<!-- <span class="gray">No projects updates were submitted this month.</span> -->



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
