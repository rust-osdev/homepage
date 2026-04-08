+++
title = "This Month in Rust OSDev: March 2026"
date = 2026-04-08

[extra]
month = "March 2026"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

Please submit interesting posts and projects for the next issue [on Zulip](https://rust-osdev.zulipchat.com/#narrow/channel/435142-newsletter/topic/Content.20suggestions/with/580172810) or via a PR [on GitHub](https://github.com/rust-osdev/homepage/).

<span class="gray">
Disclaimer: Automated scripts and AI assistence were used for collecting and categorizing links.
Everything was proofread and checked manually and there were many manual tweaks.
</span>

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (March 2026)" post.
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

- [Redox OS: Capability-based Security — Namespace and CWD](https://www.redox-os.org/news/nlnet-cap-nsmgr-cwd/)
- [Ferrous Systems: "Accessing Hardware in Rust"](https://ferrous-systems.com/blog/hardware-access-rust/)
  - Compares four Rust hardware abstraction approaches (svd2rust, tock-registers, safe-mmio, derive-mmio) for bare-metal programming.
- [How to use storytelling to fit inline assembly into Rust](https://www.ralfj.de/blog/2026/03/13/inline-asm.html)
  - Proposes a "storytelling" approach where inline assembly blocks must correspond to equivalent Rust code, ensuring soundness and allowing compiler optimizations.
- Video: [Writing a Linux GPU Kernel Driver in Rust with Daniel Almeida](https://youtu.be/rgjTPBRae6I)
  - Interview about writing the Tyr GPU kernel driver for Arm Mali hardware in Rust.
- [Ariel OS v0.4.0](https://github.com/ariel-os/ariel-os/blob/main/CHANGELOG.md#040---2026-03-18)
  - New release of this embedded Rust RTOS adds BLE on ESP32, USB CDC-NCM Ethernet, GNSS sensor support, and stack usage measurement.
- [Elfina: Multi-architecture ELF Loader in Rust](https://github.com/iss4cf0ng/Elfina)
  - Educational ELF loader supporting x86 and x86-64 Linux binaries, with mmap and memfd execution modes.
- [VectorWare: "Rust threads on the GPU"](https://www.vectorware.com/blog/threads-on-gpu)
  - Maps Rust's `std::thread` API to GPU warps, enabling familiar concurrent Rust code to compile and run on GPU hardware.

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [ACP: Introduce `alloc::io`](https://github.com/rust-lang/libs-team/issues/755)
  - Approved proposal to move OS-independent parts of `std::io` to `alloc`, making IO traits available in `no_std + alloc` environments. The [implementation PR](https://github.com/rust-lang/rust/pull/152918) is in progress.
- [Make atomic primitives type aliases of `Atomic<T>`](https://github.com/rust-lang/rust/pull/153015)
  - Refactors all atomic types (`AtomicI32`, `AtomicBool`, etc.) into type aliases of a generic `Atomic<T>`.
  - Only types that implement the sealed `AtomicPrimitive` trait can be used with `Atomic<T>`
- [Implement `MaybeDangling` compiler support](https://github.com/rust-lang/rust/pull/150447)
  - Adds compiler-level support for the `MaybeDangling` wrapper type, which relaxes aliasing assumptions on references. Important for unsafe code dealing with self-referential structures and raw memory.
- [Constify `Vec::{into, from}_raw_parts{_in|_alloc}`](https://github.com/rust-lang/rust/pull/153399)
  - Makes Vec raw parts methods usable in const contexts.
- [Target specs: stricter checks for LLVM ABI values, and correlate with `cfg(target_abi)`](https://github.com/rust-lang/rust/pull/153769)
  - Tightens ABI validation in target specs. Note: out-of-tree JSON custom targets may need updating.
- [`-Zbranch-protection` is a target modifier](https://github.com/rust-lang/rust/pull/152909)
  - Makes `-Zbranch-protection` a target modifier, enforcing consistent application across the entire crate graph. Important for AArch64 kernel security (PAC/BTI).
- [Defer codegen for the VaList Drop impl to actual uses](https://github.com/rust-lang/rust/pull/154133)
  - Allows compiling libcore with codegen backends that don't implement VaList (like Cranelift). Unblocks alternative backends for bare-metal work.

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


### [`uart_16550`](https://github.com/rust-osdev/uart_16550)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

Simple yet highly configurable low-level driver for 16550 UART devices,
typically known and used as serial ports or COM ports.

We've just released `v0.5.0` - a **complete rewrite** and fresh start of the crate, implemented by
[@phip1611](https://github.com/phip1611).
The main motivation is to make the crate fit for working on real hardware,
clean up technical debt, and streamline the code paths for x86 port I/O and
MMIO. [@phip1611](https://github.com/phip1611) is also taking over maintenance of the crate, thanks a lot!

More info:

- <https://github.com/rust-osdev/uart_16550/releases/tag/v0.5.0>
- <https://docs.rs/uart_16550/0.5.0/uart_16550/>
- <https://github.com/rust-osdev/uart_16550/pull/41>

Special Thanks to Philipp Oppermann (@phil-opp) and Martin Kröning (@mkroening)
for their very valuable review on the new crate!

After the rewrite, we merged the following changes this month:

- [feat: implement embedded-io traits](https://github.com/rust-osdev/uart_16550/pull/42)
- [Rename byte transfer functions and update IO trait implementations](https://github.com/rust-osdev/uart_16550/pull/49)
- [Enhance MmioAddress safety and fix AArch64 instruction emission](https://github.com/rust-osdev/uart_16550/pull/50)
- [Improve documentation examples and streamline architecture support](https://github.com/rust-osdev/uart_16550/pull/53)
- [Add API test and improve memory safety in MMIO constructors](https://github.com/rust-osdev/uart_16550/pull/54)

<!-- - [treewide: nuke old repo and replace with rewrite from @phip1611](https://github.com/rust-osdev/uart_16550/pull/41) -->
<!-- - [Bump crate-ci/typos from 1.41.0 to 1.44.0](https://github.com/rust-osdev/uart_16550/pull/43) -->
<!-- - [Bump bitflags from 2.10.0 to 2.11.0](https://github.com/rust-osdev/uart_16550/pull/44) -->
<!-- - [Bump assert2 from 0.3.16 to 0.4.0](https://github.com/rust-osdev/uart_16550/pull/45) -->
<!-- - [doc: update changelog](https://github.com/rust-osdev/uart_16550/pull/46) -->

Thanks to [@mkroening](https://github.com/mkroening) for their contributions!

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [add `join` on `Path`](https://github.com/rust-osdev/uefi-rs/pull/1903)
- [uefi: serial: add read_exact() and write_exact() + fix core::fmt::Write for Serial](https://github.com/rust-osdev/uefi-rs/pull/1875)
- [runtime: add `CStr16::from_bytes_with_nul` for UCS-2 strings in UEFI Variables](https://github.com/rust-osdev/uefi-rs/pull/1915)
- [Replace a couple `as` casts with `ptr::from_ref`](https://github.com/rust-osdev/uefi-rs/pull/1917)
- [release: uefi-raw-0.14.0, uefi-0.37](https://github.com/rust-osdev/uefi-rs/pull/1918)

<!-- - [Fix new clippy lints](https://github.com/rust-osdev/uefi-rs/pull/1908) -->
<!-- - [chore(deps): update rust crate quote to v1.0.45](https://github.com/rust-osdev/uefi-rs/pull/1910) -->
<!-- - [chore(deps): update rust crate ovmf-prebuilt to v0.2.8](https://github.com/rust-osdev/uefi-rs/pull/1909) -->
<!-- - [test-runner: rename cpy -> copy to make `typos` happy](https://github.com/rust-osdev/uefi-rs/pull/1912) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.44.0](https://github.com/rust-osdev/uefi-rs/pull/1894) -->
<!-- - [chore(deps): update rust crate tempfile to v3.27.0](https://github.com/rust-osdev/uefi-rs/pull/1914) -->
<!-- - [chore(deps): update rust crate clap to v4.6.0](https://github.com/rust-osdev/uefi-rs/pull/1913) -->
<!-- - [cargo: update deps (fix GitHub Security Alerts)](https://github.com/rust-osdev/uefi-rs/pull/1916) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1922) -->
<!-- - [chore(deps): update codecov/codecov-action action to v5.5.3](https://github.com/rust-osdev/uefi-rs/pull/1919) -->

Thanks to [@the-shank](https://github.com/the-shank) for their contributions!




### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS.

We merged the following changes this month:

- [Improve handling of top-level names with new `ResolveBehaviour` system](https://github.com/rust-osdev/acpi/pull/270)
- [For discussion: A way of improving regression/integration testing for ASL snippets](https://github.com/rust-osdev/acpi/pull/269)
- [Use correct offset for PCI I/O](https://github.com/rust-osdev/acpi/pull/280)
- [Allow `aml_tester` to continue after an interpreter panic](https://github.com/rust-osdev/acpi/pull/277)
- [Apply search rules to lookups](https://github.com/rust-osdev/acpi/pull/283)

<!-- - [Add a toolchain file](https://github.com/rust-osdev/acpi/pull/268) -->
<!-- - [Fix aml_tester files argument](https://github.com/rust-osdev/acpi/pull/267) -->
<!-- - [Typo - update PCI config location](https://github.com/rust-osdev/acpi/pull/279) -->

Thanks to [@martin-hughes](https://github.com/martin-hughes) for their contributions!


### [`ovmf-prebuilt`](https://github.com/rust-osdev/ovmf-prebuilt)
<span class="maintainers">Maintained by [@nicholasbishop](https://github.com/nicholasbishop) and [@phil-opp](https://github.com/phil-opp)</span>

The `ovmf-prebuilt` project provides pre-built [edk2](https://github.com/tianocore/edk2) releases to make it easier to set up OVMF. We merged the following changes this month:

- [feat: IOMMU support via edk2-platforms and IntelVTdDxe](https://github.com/rust-osdev/ovmf-prebuilt/pull/275)
- [Rename build-edk2 to xtask](https://github.com/rust-osdev/ovmf-prebuilt/pull/277)
- [Add build-edk2 action to xtask CLI](https://github.com/rust-osdev/ovmf-prebuilt/pull/278)
- [xtask: Add update-sources action](https://github.com/rust-osdev/ovmf-prebuilt/pull/280)
- [release: 0.2.8](https://github.com/rust-osdev/ovmf-prebuilt/pull/282)
- [Update to 2024 edition](https://github.com/rust-osdev/ovmf-prebuilt/pull/283)

<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/276) -->
<!-- - [Reformat ureq dependency to one line](https://github.com/rust-osdev/ovmf-prebuilt/pull/279) -->
<!-- - [Ignore files generated during ovmf-prebuilt test](https://github.com/rust-osdev/ovmf-prebuilt/pull/281) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/284) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/286) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/289) -->

Thanks to [@PelleKrab](https://github.com/PelleKrab) for their contributions!


### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following changes:

- [don't use BPB_TotSec16 to determine FAT type](https://github.com/rust-osdev/bootloader/pull/547)
- [speed up BIOS bootloader](https://github.com/rust-osdev/bootloader/pull/550)

<!-- - [Fix doc link](https://github.com/rust-osdev/bootloader/pull/548) -->
<!-- - [Bump tar from 0.4.44 to 0.4.45 in /examples/basic](https://github.com/rust-osdev/bootloader/pull/553) -->
<!-- - [Bump rustls-webpki from 0.103.8 to 0.103.10 in /examples/basic](https://github.com/rust-osdev/bootloader/pull/554) -->

Thanks to [@ic3w1ne](https://github.com/ic3w1ne) for their contribution!


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PRs this month:

- [don't enable abi_x86_interrupt if not needed](https://github.com/rust-osdev/x86_64/pull/585)


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

- [[Translation] translated post 10 into russian](https://github.com/phil-opp/blog_os/pull/1469)

Thanks to [@TakiMoysha](https://github.com/TakiMoysha) for their contribution!



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
