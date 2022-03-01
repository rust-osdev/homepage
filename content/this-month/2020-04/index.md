+++
title = "This Month in Rust OSDev (April 2020)"
date = 2020-05-06

[extra]
month = "April 2020"
authors = [
    "phil-opp",
    "IsaacWoods",
]
+++

Welcome to the first issue of _"This Month in Rust OSDev"_. In these posts, we will give a regular overview of notable changes in the Rust operating system development community.

<!-- more -->

These posts are the successor of the [_"Status Update"_ posts](https://os.phil-opp.com/status-update/) on the _"Writing an OS in Rust"_ blog. Instead of only focusing on the updates to the blog and the directly related crates, we try to give an overview of the full Rust OSDev ecosystem in this new series. This includes all the projects under the [`rust-osdev`] GitHub organization, relevant projects of other organizations, and also personal OS projects.

[`rust-osdev`]: https://github.com/rust-osdev/about

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new).

## News and Blog Posts

We try to collect posts that are relevant to Rust-based OS development each month. Please create pull requests for any posts that you want linked in the next issue.

- [Rust/WinRT Public Preview](https://blogs.windows.com/windowsdeveloper/2020/04/30/rust-winrt-public-preview/)
- [Georgia Tech CS-3210: Write an OS for the Raspberry Pi in Rust](https://web.archive.org/web/20210303034834/https://tc.gts3.org/cs3210/2020/spring/lab.html)
- [Rust-Written Redox OS Booting The 128-Thread AMD Ryzen Threadripper 3990X](https://www.phoronix.com/scan.php?page=news_item&px=Redox-OS-On-Threadripper-3990X)

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

### `x86_64`

The [`x86_64` crate](https://github.com/rust-osdev/x86_64) provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

This month, we released version `0.10.0` of `x86_64`, which includes some breaking changes:

- _Breaking:_ [Make `map_to` and `update_flags` unsafe](https://github.com/rust-osdev/x86_64/pull/135)
- _Breaking:_ [Make FrameDeallocator::deallocate_frame unsafe](https://github.com/rust-osdev/x86_64/pull/146)
- _Breaking:_ [Don't pass small trivially copyable types by reference](https://github.com/rust-osdev/x86_64/pull/147)
- [Use `#[inline]` everywhere](https://github.com/rust-osdev/x86_64/pull/145)
- [Add `{PhysFrame,Page}::from_start_address_unchecked`](https://github.com/rust-osdev/x86_64/pull/142)

After `v0.10.0`, the following changes were merged:

- [Add `InterruptDescriptorTable::load_unsafe`](https://github.com/rust-osdev/x86_64/pull/137) <span class="gray">(published as `v0.10.1`)</span>
- [Use `llvm_asm!` instead of deprecated `asm!` macro](https://github.com/rust-osdev/x86_64/pull/151)
    - This might break older nightlies. Given that version `0.10.x` is so recent, we still decided to release only a patch version. Note that this isn't a breaking change in the `semver` sense since we only guarantee compatibility with recent nightlies.
- [Return the correct RPL from `GDT::add_entry()`](https://github.com/rust-osdev/x86_64/pull/153) <span class="gray">(published as `v0.10.2`)</span>

Thanks to [@m-ou-se](https://github.com/m-ou-se), [@tomaka](https://github.com/tomaka), [@haraldh](https://github.com/haraldh), and [@imtsuki](https://github.com/imtsuki) for their contributions!

### `acpi` and `aml`

The [`acpi` repository](https://github.com/rust-osdev/acpi) contains crates for parsing the [ACPI](https://en.wikipedia.org/wiki/Advanced_Configuration_and_Power_Interface) tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS. The crate for parsing the static tables ([`acpi`](https://crates.io/crates/acpi)) was stabilised this month – while it needs more work to support the entire spec, it is now in the state where it can be integrated into a Rust kernel to provide useful information, and its overall architecture is unlikely to change in the near future.

The full set of changes is:

- [Remove two nightly features in the ACPI crate](https://github.com/rust-osdev/acpi/pull/69)
- [Make local NMI line table optional](https://github.com/rust-osdev/acpi/pull/68) <span class="gray">(published as v0.8.0)</span>
- [Prepare for stabilisation of the `acpi` crate](https://github.com/rust-osdev/acpi/pull/70) (🎉 published as v1.0.0 🎉)

Thanks to [@tomaka](https://github.com/tomaka) for their contribution and congratulations to the creator and maintainer [@IsaacWoods](https://github.com/IsaacWoods) on the `1.0` release!

### `bootloader`

The [`bootloader` crate](https://github.com/rust-osdev/bootloader) implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. It received the following updates this month:

- [SSE feature: remove inline assembly + don't set reserved bits](https://github.com/rust-osdev/bootloader/pull/105) <span class="gray">(published as `v0.9.1`)</span>
- [Use `llvm_asm!` instead of deprecated `asm!`](https://github.com/rust-osdev/bootloader/pull/108) <span class="gray">(published as `v0.9.2`)</span>

Thanks to [@Freax13](https://github.com/Freax13) and [@realKennyStrawn93](https://github.com/realKennyStrawn93) for these changes!

Right now, [@rybot666](https://github.com/rybot666) and [@phil-opp](https://github.com/phil-opp) are working on a rewrite of the bootloader. The goal is to port the 16-bit and 32-bit stages from assembly to Rust, which should make the crate safer, more composable, and easier to understand for outsiders. Our progress is tracked in the [`Rewrite` milestone](https://github.com/rust-osdev/bootloader/milestone/1).

### `bootimage`

The [`bootimage` tool](https://github.com/rust-osdev/bootimage) allows the creation of bootable disk images for `bootloader`-based kernels. It also provides a runner executable for `cargo` to make `cargo xrun` and `cargo xtest` work using QEMU. In April, the crate received the following updates:

- [Set empty RUSTFLAGS to ensure that no .cargo/config applies](https://github.com/rust-osdev/bootimage/pull/51) <span class="gray">(published as v0.7.9)</span>
- [Add support for doctests](https://github.com/rust-osdev/bootimage/pull/52) <span class="gray">(published as v0.7.10)</span>

Thanks to [@Freax13](https://github.com/Freax13) for their contribution!

### `uart_16550`

The [`uart_16550` crate](https://github.com/rust-osdev/uart_16550) provides basic support for serial port I/O for 16550-compatible UARTs. This month, the crate received support for serial input:

- [Support receiving bytes from serial ports](https://github.com/rust-osdev/uart_16550/pull/8)

Thanks to [@imtsuki](https://github.com/imtsuki) for their contribution!

### `cargo-xbuild`

The [`cargo-xbuild`](https://github.com/rust-osdev/cargo-xbuild) project provides `cargo` command wrappers to cross-compile the sysroot crates `core` and `alloc`. This month, we added a new environment variable to help debugging build errors:

- [Add an environment variable to keep the temp dir](https://github.com/rust-osdev/cargo-xbuild/pull/67) <span class="gray">(published as v0.5.29)</span>

### `uefi`

The [`uefi` crate](https://github.com/rust-osdev/uefi-rs) provides abstractions for the [`UEFI`](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface) standard that replaces the traditional BIOS on modern systems. This month, the crate received a new cargo feature to ignore logger errors:

- [Add cargo feature for ignoring logger errors](https://github.com/rust-osdev/uefi-rs/pull/132)

## New Projects

There are a number of new projects in the `rust-osdev` organization:

- [**`homepage`:**](https://github.com/rust-osdev/homepage) As you might have noticed by now, we have a new organization-level homepage at <https://rust-osdev.com/>. The `homepage` repository contains the source code for this website. Right now, it is still a work-in-progress and only contains the very minimum to host this post, but we plan to add more content soon.

  Note that we will create a branch for the upcoming May issue of _"This Month in Rust OSDev"_. Please open pull requests for any content that you would like to see next month.
- [**`vga`:**](https://github.com/rust-osdev/vga) The goal of the `vga` crate is to allow configuration of the VGA hardware. It already makes it possible to switch from a text-based buffer to a pixel-based framebuffer, which enables drawing of lines, geometric shapes, and even images. The library is created by [@RKennedy9064](https://github.com/RKennedy9064).
- [**`ps2-mouse`:**](https://github.com/rust-osdev/ps2-mouse) The library provides a basic interface for interacting with a PS/2 mouse. It is also created by [@RKennedy9064](https://github.com/RKennedy9064).

## Personal Projects

In this section, we describe updates to personal projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### `IsaacWoods/pebble`

<span class="gray">(Section written by [@IsaacWoods](ttps://github.com/IsaacWoods))</span>

[Pebble](https://github.com/IsaacWoods/pebble) has been undergoing a bit of a reorganisation, in the interests of supporting a second architecture (ARM64). A hardware abstraction layer (HAL) has been introduced that abstracts away the platform-specifics of managing page tables, creating processes etc. so that the main `kernel` crate is now completely platform-independent.

I also wrote a [little TFTP server](https://github.com/IsaacWoods/pebble/blob/master/tools/tftp_serve/src/main.rs) for netbooting a Raspberry Pi 4 from a development machine. This makes iterating the kernel a lot easier because there's no need to write it to an SD card after every compile. In the next few weeks, I hope to clean this code up and publish it for use as both a library and CLI application, and maybe write a blog-post on the intricacies of netbooting the Pi.

### `rust-embedded/rust-raspberrypi-OS-tutorials`

<span class="gray">(Section written by [@andre-richter](https://github.com/andre-richter))</span>

The [_Operating System development tutorials in Rust on the Raspberry Pi_](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials) project received the following updates recently:

- [Add `tutorial 14`](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/tree/master/14_exceptions_part2_peripheral_IRQs): Exceptions Part 2: Peripheral IRQs.
    - We write `device drivers` for the two interrupt controllers on the _Raspberry Pi 3_ (Broadcom
      custom controller) and _Pi 4_ (ARM Generic Interrupt Controller v2, `GICv2`).
    - Modularity is ensured by interfacing everything through a trait named `IRQManager`.
    - Handling for our first peripheral IRQ is implemented: The `UART`'s receive IRQ - one IRQ per
      received character.
- [Set chainloader relocation address to `32 MiB`](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/commit/3a794448adc26bcd318df47ae1a81ab56203364a).
    - Enables booting on Pi3 A+ devices.
- [Run the chainloader natively on non-Linux Unix systems](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/commit/089565762a60462c527a907fba2caeee583fa58d).
    - Adds experimental support for `macOS` and other Unix systems which have `Ruby` available.

### `phil-opp/blog_os`

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

Apart from a few [minor fixes](https://github.com/phil-opp/blog_os/pulls?q=is%3Apr+is%3Aclosed+merged%3A2020-04-02..2020-05-01) (thanks for all the contributions!), April was a quiet month for the [_"Writing an OS in Rust"_](https://os.phil-opp.com/) project. I focused my work this month on the `x86_64` library, the rewrite of the bootloader, and my job search instead. In this regard, I'm excited to announce that I have decided to do Rust-related freelance work for now, which will allow me to continue dedicating some of my time to open-source work.

<!-- Maybe next month

## Call for Participation

-->

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
