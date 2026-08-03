+++
title = "This Month in Rust OSDev: July 2026"
date = 2026-08-01

[extra]
month = "July 2026"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

Please submit interesting posts and projects for the next issue by commenting on the [draft pull request](https://github.com/rust-osdev/homepage/pulls) or via a PR [on GitHub](https://github.com/rust-osdev/homepage/).

<span class="gray">
Disclaimer: Automated scripts and AI assistance were used for collecting and categorizing links.
Everything was proofread and checked manually, with many manual tweaks.
</span>


<!--
    This is a draft for the upcoming "This Month in Rust OSDev (July 2026)" post.
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

- [This Month in Redox - June 2026](https://www.redox-os.org/news/this-month-260630/)
- [SafaOS: another hobby OS written in rust, now has Audio, GUI and a bunch of software ports](https://old.reddit.com/r/rust/comments/1v9zh2r/safaos_another_hobby_os_written_in_rust_now_has/)
  - The hobby OS SafaOS gained UDP networking, a GUI, an Intel HDA audio stack, ports of SDL2/SDL2_Mixer/ffplay, and now runs Doom, Quake 2, and Celeste Classic.
- [sermuns/efimux: EFI application for booting other EFI applications](https://github.com/sermuns/efimux)
  - A rudimentary UEFI boot menu written in Rust that scans filesystems for `.efi` files and presents them in a Ratatui TUI, built on top of the author's `ratatuefi` `no_std` Ratatui backend for UEFI.
- ['Rust makes coding fun again': Why Linux is moving away from C, according to Greg Kroah-Hartman](https://www.zdnet.com/article/greg-kroah-hartman-linux-kernel-rust/)
- [Progress toward compiling Linux with gccrs](https://lwn.net/SubscriberLink/1083202/f1ba926cd57ac5c5/)
- [Rust MEMS drivers: 3 reasons to try and adopt our new sensor drivers written in Rust](https://blog.st.com/rust-mems-drivers/)
  - STMicroelectronics announces official Rust drivers for its MEMS sensor line, one of the first vendor-blessed embedded Rust driver crates from a major microcontroller/sensor manufacturer.
- [nasa/spacewasm: A flight-compliant WebAssembly interpreter](https://github.com/nasa/spacewasm)
  - NASA released a WebAssembly 1.0 interpreter designed for safety-critical spacecraft: fixed-size memory blocks, no dynamic deallocation after allocation, deterministic memory usage, and streaming binary decoding, intended for on-board sequencing and sandboxing of low-confidence code.
- [The Embedded Rustacean Issue #76](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-76)

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [stabilize `feature(atomic_from_mut)`](https://github.com/rust-lang/rust/pull/158573)
  - Stabilizes converting `&mut [Atomic*]` to and from `&mut [primitive]`, useful for building lock-free data structures without going through `unsafe` transmutes.
- [Stabilize c-variadic function definitions](https://github.com/rust-lang/rust/pull/155697)
  - Rust code can now *define* C-style variadic functions, not just call them, which is needed when implementing C-compatible interfaces such as a libc from scratch.
- [allow `Allocator`s to be used as `#[global_allocator]`s](https://github.com/rust-lang/rust/pull/157153)
  - Adds a `GlobalAllocator` marker trait so that types implementing the newer `Allocator` trait can be installed as the global allocator, without implementing `GlobalAlloc` directly and risking infinite recursion.
- [make volatile operations const](https://github.com/rust-lang/rust/pull/159092)
  - Volatile reads and writes are now usable in `const fn`, which allows sharing code paths between MMIO-style special memory and regular memory.
- [Implement `ptr::{read,write}_unaligned` via `repr(packed)`](https://github.com/rust-lang/rust/pull/158427)
  - Reimplements the unaligned pointer accessors on top of `repr(packed)` fields instead of `memcpy` and intrinsics, producing much simpler MIR and enabling further optimizations.
- The move of `std::io` into `core` and `alloc` continued at a fast pace this month. After `io::Error` landed in `core` in June, most of the remaining building blocks followed, so that `Read`, `Write`, `Seek`, and the buffered wrappers are now usable without `std`:
  - [Move `SizeHint` and `IoHandle` to `core::io`](https://github.com/rust-lang/rust/pull/158539)
  - [Move `std::io::Seek` to `core::io`](https://github.com/rust-lang/rust/pull/158540)
  - [Move `std::io::Write` to `core::io`](https://github.com/rust-lang/rust/pull/158541)
  - [Move `std::io::Read` to `alloc::io`](https://github.com/rust-lang/rust/pull/158544)
  - [Move `std::io::read_to_string` to `alloc::io`](https://github.com/rust-lang/rust/pull/158545)
  - [Move `std::io::BufRead` to `alloc::io`](https://github.com/rust-lang/rust/pull/158546)
  - [Move `std::io::buffered` to `alloc::io`](https://github.com/rust-lang/rust/pull/158547)
  - See the [tracking issue for `alloc::io` and `core::io`](https://github.com/rust-lang/rust/issues/154046) for the remaining work.
- [Add `riscv32imfc-unknown-none-elf` bare-metal target](https://github.com/rust-lang/rust/pull/158473)
  - A new `no_std` target for RISC-V32 cores with the `I`, `M`, `F`, and `C` extensions.
- [rustc_target: Add ARMv8-M related target features](https://github.com/rust-lang/rust/pull/158405)
  - Adds target features for ARMv8-M (Cortex-M23/M33/M35P/M55 and similar), relevant for embedded/bare-metal development on those chips.
- [rustc_target/asm: add LoongArch LSX/LASX inline asm register support](https://github.com/rust-lang/rust/pull/158364)
  - Extends `asm!` with LoongArch SIMD register classes.
- [Convert `-Ctarget-cpu` into a target-modifier for AVR, AMDGCN and NVPTX](https://github.com/rust-lang/rust/pull/150732)
  - Also merged this month: [fix: emit diagnostic for AVR target without target-cpu](https://github.com/rust-lang/rust/pull/158776), which errors instead of silently miscompiling when no AVR `target-cpu` is set.
- [rustc_target: Add OpenEmbedded/Yocto Linux base targets](https://github.com/rust-lang/rust/pull/157650)
  - New base targets for embedded Linux systems built with the Yocto/OpenEmbedded toolchain.
- [Add documentation for the `no_std` attribute](https://github.com/rust-lang/rust/pull/158886)

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
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables.

We merged the following changes this month:

- [Revert `uart_16550` version bump to 0.6.0](https://github.com/rust-osdev/bootloader/pull/575)
- [Fix workspace.exclude in Cargo.toml](https://github.com/rust-osdev/bootloader/pull/571)
- [Make release script more robust and send user agent](https://github.com/rust-osdev/bootloader/pull/577)
- [Release 0.11.16](https://github.com/rust-osdev/bootloader/pull/570)
- [Release v0.11.17](https://github.com/rust-osdev/bootloader/pull/576)

<!--
    The `softfloat` target-ABI fixes (bootloader #568 and #569) and the
    `uefi` bump (#566) merged in early July, but were already reported in the
    June 2026 issue, so they are not repeated here.
-->
<!-- - [(v0.9) Change `rustc-abi` in custom targets from `x86-softfloat` to `softfloat`](https://github.com/rust-osdev/bootloader/pull/568) -->
<!-- - [Change `rustc-abi` in custom targets from `x86-softfloat` to `softfloat`](https://github.com/rust-osdev/bootloader/pull/569) -->
<!-- - [uefi: bump from 0.20 to 0.38](https://github.com/rust-osdev/bootloader/pull/566) -->
<!-- - [update Cargo.lock for basic example](https://github.com/rust-osdev/bootloader/pull/572) -->

Thanks to [@Wasabi375](https://github.com/Wasabi375) for this contribution!

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [uefi: add driver helper](https://github.com/rust-osdev/uefi-rs/pull/1989)
- [doc: add showcases to README](https://github.com/rust-osdev/uefi-rs/pull/1996)
- [release: uefi-raw-0.15.1 and uefi-0.39.0](https://github.com/rust-osdev/uefi-rs/pull/2004)

<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1999) -->
<!-- - [chore(deps): update rust crate time to v0.3.47 [security]](https://github.com/rust-osdev/uefi-rs/pull/2000) -->
<!-- - [fix(deps): update rust crate itertools to 0.15.0](https://github.com/rust-osdev/uefi-rs/pull/2001) -->
<!-- - [chore(deps): update actions/checkout action to v7](https://github.com/rust-osdev/uefi-rs/pull/2002) -->
<!-- - [chore(deps): update codecov/codecov-action action to v7](https://github.com/rust-osdev/uefi-rs/pull/2003) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/2012) -->
<!-- - [chore(deps): update rust crate time to v0.3.47 [security]](https://github.com/rust-osdev/uefi-rs/pull/2013) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.48.0](https://github.com/rust-osdev/uefi-rs/pull/2015) -->

Thanks to [@reynoldsbd](https://github.com/reynoldsbd) and [@sermuns](https://github.com/sermuns) for these contributions!

### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following changes this month:

- [feat: Added the "from_pfn" in PhysFrame](https://github.com/rust-osdev/x86_64/pull/593)
- [add missing {forward,backward}_overflowing impls](https://github.com/rust-osdev/x86_64/pull/595)
- [release 0.15.5](https://github.com/rust-osdev/x86_64/pull/596)

<!-- - [Bump actions/cache from 5 to 6](https://github.com/rust-osdev/x86_64/pull/591) -->
<!-- - [chore: merge master into next](https://github.com/rust-osdev/x86_64/pull/598) -->

Thanks to [@zhangxuan2011](https://github.com/zhangxuan2011) and [@tpdenk](https://github.com/tpdenk) for these contributions!

### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers uses to relay information about the hardware to the OS.

We merged the following changes this month:

- [Demonstrate that less-than-all features will build](https://github.com/rust-osdev/acpi/pull/314)
- [Skip unsupported _CRS resource descriptors](https://github.com/rust-osdev/acpi/pull/316)
- [Handle DerefOf buffer fields](https://github.com/rust-osdev/acpi/pull/317)

Thanks to [@martin-hughes](https://github.com/martin-hughes) and [@ArthurHeymans](https://github.com/ArthurHeymans) for these contributions!

### [`ps2-mouse`](https://github.com/rust-osdev/ps2-mouse)

The `ps2-mouse` library provides a basic interface for interacting with a PS/2 mouse. A Rust nightly change broke the `Step` trait implementation in `x86_64` this month (see [rust-osdev/x86_64#594](https://github.com/rust-osdev/x86_64/issues/594)), so `ps2-mouse` needed a dependency update too:

- [Update x86_64 to fix build failure caused by rust-osdev/x86_64#594](https://github.com/rust-osdev/ps2-mouse/pull/6)

Thanks to [@forestaa](https://github.com/forestaa) for this contribution!

### [`bootimage`](https://github.com/rust-osdev/bootimage)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `bootimage` tool allows the creation of bootable disk images for `bootloader`-based kernels. It also provides a runner executable for `cargo` to make `cargo run` and `cargo test` work using QEMU.

We merged the following changes this month:

- [Fix example build on latest nightly](https://github.com/rust-osdev/bootimage/pull/111)
- [Adjust test detection for new `build-dir` layout of cargo](https://github.com/rust-osdev/bootimage/pull/112)
  - Cargo's new `build-dir` layout v2 (stabilized on nightly in July) broke `bootimage`'s test-binary detection. This PR adjusts the runner accordingly, so `cargo test` works again on recent nightlies.
- [Release version 0.10.5](https://github.com/rust-osdev/bootimage/pull/113)

<!--
    #112 and #113 merged in early August 2026, but they land the recent-nightly
    fix for cargo's build-dir layout v2 change (see the "Infrastructure and
    Tooling" section) and we want it visible for readers hitting the breakage
    now.
-->

<!-- `multiboot2`, `uart_16550`, and `ovmf-prebuilt` only had chore/dependency-update PRs merged this month, so they are omitted here. -->

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

<span class="gray">No projects updates were submitted this month.</span>



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way to get in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
