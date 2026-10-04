+++
title = "This Month in Rust OSDev: September 2026"
date = 2026-10-01

[extra]
month = "September 2026"
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
    This is a draft for the upcoming "This Month in Rust OSDev (September 2026)" post.
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

<span class="gray">No content was submitted for this section this month.</span>

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

<span class="gray">No content was submitted for this section this month.</span>

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

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@nicholasbishop](https://github.com/nicholasbishop) and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We continued last month's **soundness and specification compliance** work. The
common theme: the crates no longer blindly trust what the firmware reports. For
example, `boot::memory_map` trusted the map size reported by the firmware, so
sorting a map larger than its buffer read out of bounds, and several `boot` and
protocol functions turned null pointers returned by the firmware into handles or
references. In `uefi-raw`, we audited the pointer mutability of the protocol
definitions against the UEFI and PI specifications. This is breaking for the raw
bindings, but it lets the high-level `uefi` crate hand out `&`/`&mut` without
lying to the compiler.

Not everything was about fixing things. `uefi-raw` gained HII Internal Forms
Representation (IFR) types and `uefi` gained the `AbsolutePointer` protocol,
`CString16::extend`, and allocation-free iteration over the components of a
`Path`.

All of this was released as `uefi v0.41.0`, `uefi-raw v0.17.0`, and
`uefi-macros v0.20.0`.

As mentioned last month, [Anthropic](https://www.anthropic.com/) sponsors
@phip1611 with a Max plan as part of their open source program. Much of the
auditing above happened under that sponsorship, and we are happy that it helps
us improve the security, robustness, and reliability of the ecosystem.

Thanks to [@crawfxrd](https://github.com/crawfxrd),
[@cwize1](https://github.com/cwize1) and
[@the-shank](https://github.com/the-shank) for their contributions!

We merged the following PRs this month:

- [release: uefi-macros-0.20.0, uefi-raw-0.17.0, uefi-0.41.0](https://github.com/rust-osdev/uefi-rs/pull/2092)
- [Various Fixes of Potential Undefined Behavior (LLM Assisted)](https://github.com/rust-osdev/uefi-rs/pull/2077)
- [Various Fixes of Potential Undefined Behavior (LLM Assisted) (2/N)](https://github.com/rust-osdev/uefi-rs/pull/2078)
- [Various Fixes of Potential Undefined Behavior (LLM Assisted) (3/N)](https://github.com/rust-osdev/uefi-rs/pull/2079)
- [Various Fixes of Potential Undefined Behavior (LLM Assisted) (4/N)](https://github.com/rust-osdev/uefi-rs/pull/2080)
- [Various Fixes of Potential Undefined Behavior (LLM Assisted) (5/N)](https://github.com/rust-osdev/uefi-rs/pull/2083)
- [Various Fixes of Potential Undefined Behavior and Panics (LLM Assisted) (6/N)](https://github.com/rust-osdev/uefi-rs/pull/2084)
- [Various Fixes of Potential Undefined Behavior and Panics (LLM Assisted) (8/N)](https://github.com/rust-osdev/uefi-rs/pull/2085)
- [uefi: fix aliasing UB in PciRootBridgeIo I/O accessors](https://github.com/rust-osdev/uefi-rs/pull/2074)
- [uefi: mem: fix the allocation handling of make_boxed](https://github.com/rust-osdev/uefi-rs/pull/2081)
- [uefi: media: validate the file info returned by firmware](https://github.com/rust-osdev/uefi-rs/pull/2082)
- [uefi-raw: spec violation fixes: outputs behind read-only pointers (1/5)](https://github.com/rust-osdev/uefi-rs/pull/2086)
- [uefi-raw: spec violation fixes: read-only inputs (2/5)](https://github.com/rust-osdev/uefi-rs/pull/2087)
- [uefi-raw: spec compliance: event notification context (3/5)](https://github.com/rust-osdev/uefi-rs/pull/2088)
- [uefi-raw: spec compliance: caller-owned outputs (4/5)](https://github.com/rust-osdev/uefi-rs/pull/2089)
- [uefi-raw: spec compliance: mode pointers and docs (5/5)](https://github.com/rust-osdev/uefi-rs/pull/2090)
- [uefi-raw: a few more spec adjustments regarding api_guidelines.md](https://github.com/rust-osdev/uefi-rs/pull/2093)
- [A few small Spec Fixes/Adjustments](https://github.com/rust-osdev/uefi-rs/pull/2063)
- [uefi-raw: Add HII IFR bindings](https://github.com/rust-osdev/uefi-rs/pull/2064)
- [uefi: Add AbsolutePointerProtocol](https://github.com/rust-osdev/uefi-rs/pull/2061)
- [Add EdidDiscovered protocol](https://github.com/rust-osdev/uefi-rs/pull/2073)
- [uefi: some string and path improvements](https://github.com/rust-osdev/uefi-rs/pull/2066)
- [uefi: Improve the debug format of `CStr8`, `CStr16`, and `CString16`.](https://github.com/rust-osdev/uefi-rs/pull/2099)
- [uefi-raw: Use efiapi for C variadic functions](https://github.com/rust-osdev/uefi-rs/pull/2098)
- [pxe: use PxeBaseCodeBootType in PxeBaseCodeSrvlist](https://github.com/rust-osdev/uefi-rs/pull/2058)
- [data_types: export FromSliceUntilNulError](https://github.com/rust-osdev/uefi-rs/pull/2059)
- [uefi-raw: improve documentation how to model UEFI types](https://github.com/rust-osdev/uefi-rs/pull/2091)
- [Various Small Release Preparation Fixes](https://github.com/rust-osdev/uefi-rs/pull/2096)
- [Some Small Fixes](https://github.com/rust-osdev/uefi-rs/pull/2100)

<!-- Chore and dependency PRs: -->
<!-- - [chore(deps): update crate-ci/typos action to v1.50.1](https://github.com/rust-osdev/uefi-rs/pull/2067) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/2069) -->
<!-- - [cargo: fix warnings from latest nightly (v1.100.0)](https://github.com/rust-osdev/uefi-rs/pull/2070) -->
<!-- - [Streamline Lints](https://github.com/rust-osdev/uefi-rs/pull/2071) -->
<!-- - [Update crate-ci/typos action to v1.50.2](https://github.com/rust-osdev/uefi-rs/pull/2094) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.50.3](https://github.com/rust-osdev/uefi-rs/pull/2101) -->

### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers uses to relay information about the hardware to the OS.

We merged the following changes this month:

- [Breaking: Improve pm1 enable registers control ](https://github.com/rust-osdev/acpi/pull/326)
- [Add functions to get and clear pending events for Pm1EventRegisterBlock](https://github.com/rust-osdev/acpi/pull/325)
- [Add I2C and GPIO resource descriptors](https://github.com/rust-osdev/acpi/pull/333)
- [Add `set_interrupt_model_used` method for calling `\_PIC`](https://github.com/rust-osdev/acpi/pull/357)
- [Create `pci_routing::Pin::from_pci_interrupt_pin` convenience fn.](https://github.com/rust-osdev/acpi/pull/359)
- [Make RegionHandlers be Send + Sync by default](https://github.com/rust-osdev/acpi/pull/318)
- [feat: Don't take explicit references to `Handler`](https://github.com/rust-osdev/acpi/pull/345)
- [Use updated PhysicalMapping interface](https://github.com/rust-osdev/acpi/pull/360)
- [feat(aml): make `spinning_top`, `byteorder`, and `smallvec` optional](https://github.com/rust-osdev/acpi/pull/340)
- [Keep a copy of block stream instead of raw pointer](https://github.com/rust-osdev/acpi/pull/310)
- [Ensure that Locals passed to Methods are preserved](https://github.com/rust-osdev/acpi/pull/335)
- [Correctly unwind stack when handling `Continue`](https://github.com/rust-osdev/acpi/pull/363)
- [Prevent some infinite recursion and loops](https://github.com/rust-osdev/acpi/pull/352)
- [Fix a couple issues with end tags](https://github.com/rust-osdev/acpi/pull/332)
- [fix: Fix compilation with only the `alloc` feature](https://github.com/rust-osdev/acpi/pull/342)
- [Add basic test for ConcatenateRestTemplate](https://github.com/rust-osdev/acpi/pull/334)

<!-- Chore PRs: -->
<!-- - [Add $FEATURES to build flags](https://github.com/rust-osdev/acpi/pull/343) -->
<!-- - [fix(aml): Fix Clippy lints](https://github.com/rust-osdev/acpi/pull/339) -->
<!-- - [ci: Update workflow actions](https://github.com/rust-osdev/acpi/pull/344) -->
<!-- - [fix: Fix Rust lints without default features](https://github.com/rust-osdev/acpi/pull/341) -->
<!-- - [fix(tools): Fix `cargo::non_kebab_case_bins`](https://github.com/rust-osdev/acpi/pull/337) -->
<!-- - [fix: Fix exported_private_dependencies](https://github.com/rust-osdev/acpi/pull/338) -->
<!-- - [fix(aml): Fix clippy::unnecessary_cast](https://github.com/rust-osdev/acpi/pull/350) -->

Thanks to [@dewyatt](https://github.com/dewyatt), [@martin-hughes](https://github.com/martin-hughes), [@mkroening](https://github.com/mkroening), and [@ChocolateLoverRaj](https://github.com/ChocolateLoverRaj) for their contributions!

### [`virtio-spec-rs`](https://github.com/rust-osdev/virtio-spec-rs)
<span class="maintainers">Maintained by [@mkroening](https://github.com/mkroening)</span>

The `virtio-spec` crate provides definitions from the Virtual I/O Device (VIRTIO) specification.
This project aims to be unopinionated regarding actual VIRTIO drivers that are implemented on top of this crate.

This month, the crate was upgraded to version 1.4 of the VIRTIO specification and gained virtio-blk definitions, released as `v0.4.0`:

- [chore: Release version 0.4.0](https://github.com/rust-osdev/virtio-spec-rs/pull/46)
- [feat: add virtio-blk definitions](https://github.com/rust-osdev/virtio-spec-rs/pull/33)
- [feat: update `virtio::Id` to spec 1.4](https://github.com/rust-osdev/virtio-spec-rs/pull/26)
- [feat: add `DeviceStatus::SUSPEND` from spec 1.4](https://github.com/rust-osdev/virtio-spec-rs/pull/27)
- [feat: update features to spec 1.4](https://github.com/rust-osdev/virtio-spec-rs/pull/35)
- [feat(driver_notifications): upgrade to spec 1.4](https://github.com/rust-osdev/virtio-spec-rs/pull/36)
- [feat(net): upgrade to spec 1.4](https://github.com/rust-osdev/virtio-spec-rs/pull/40)
- [feat(pci): upgrade to spec 1.4](https://github.com/rust-osdev/virtio-spec-rs/pull/41)
- [feat(mmio): upgrade to spec 1.4](https://github.com/rust-osdev/virtio-spec-rs/pull/37)
- [feat: make enums ABI-compatible](https://github.com/rust-osdev/virtio-spec-rs/pull/30)

<!-- Chore PRs: -->
<!-- - [build(deps): Upgrade bitfield-struct to 0.13](https://github.com/rust-osdev/virtio-spec-rs/pull/42) -->
<!-- - [ci: Upgrade to actions/checkout@v7](https://github.com/rust-osdev/virtio-spec-rs/pull/45) -->
<!-- - [chore(Cargo.toml): Remove deprecated `package.authors` field](https://github.com/rust-osdev/virtio-spec-rs/pull/43) -->
<!-- - [ci: Use Cargo `build.warnings` instead of `RUSTFLAGS=-Dwarnings`](https://github.com/rust-osdev/virtio-spec-rs/pull/44) -->

### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We published a first release candidate for `v0.16.0` this month. We merged the following changes:

- [release 0.16.0-rc.0](https://github.com/rust-osdev/x86_64/pull/608)
- [increase the Minimum Supported Rust Version to 1.98](https://github.com/rust-osdev/x86_64/pull/604)
- [make memory encryption bit an upper limit for physical address bits](https://github.com/rust-osdev/x86_64/pull/603)
- [feat(paging): Implement `IntoIterator` and `Copy` for ranges](https://github.com/rust-osdev/x86_64/pull/609)
- [fix: Add `#[track_caller]` to many address, page, and frame methods](https://github.com/rust-osdev/x86_64/pull/615)
- [fix(paging): Fix panic on displaying a page table with the last physical frame](https://github.com/rust-osdev/x86_64/pull/614)
- [clean up features](https://github.com/rust-osdev/x86_64/pull/607)

<!-- Chore PRs: -->
<!-- - [Fix some failing CI jobs](https://github.com/rust-osdev/x86_64/pull/606) -->

Thanks to [@mkroening](https://github.com/mkroening) for their contributions!

### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

_Convenient and safe parsing of Multiboot2 Boot Information (MBI) structures and
the contained information tags. Usable in no_std environments, such as a kernel.
An optional builder feature also allows the construction of the corresponding
structures._

The `raw_type!` macro we announced last month landed. For users, this means that
values unknown to the specification - an unknown header tag type, architecture,
or memory area type - no longer produce undefined behavior. They now arrive as a
`Custom` variant that can simply be ignored.

On the builder side, structures built on the stack could leak uninitialized
padding bytes into the output. Built structures are now byte-wise identical to
before, except that the padding between tags is guaranteed to be zeroed - so
what a bootloader hands to a kernel no longer depends on whatever was on the
stack.

Released as `multiboot2 v0.27.0` and `v0.28.0`, `multiboot2-header v0.11.0`, and
`multiboot2-common v0.6.0` and `v0.7.0`. These come with a few breaking changes,
most notably that `Header` and `MaybeDynSized` are now `unsafe` traits, which
affects users implementing custom tags.

We merged the following PRs this month:

- [Various small-ish UB and Safety Fixes.](https://github.com/rust-osdev/multiboot2/pull/319)
- [Various UB fixes and Code Improvements](https://github.com/rust-osdev/multiboot2/pull/318)
- [Various UB Fixes](https://github.com/rust-osdev/multiboot2/pull/317)

<!-- Chore PRs: -->
<!-- - [build(deps): bump crate-ci/typos from 1.48.0 to 1.50.0](https://github.com/rust-osdev/multiboot2/pull/316) -->

### [`uart_16550`](https://github.com/rust-osdev/uart_16550)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

_Simple yet highly configurable low-level driver for 16550 UART devices,
typically known and used as serial ports or COM ports._

[`v0.8.1`](https://github.com/rust-osdev/uart_16550/commit/1d220e8615dbd1fbe3abef0446ffe90d70265be9) adds the public method [`Uart16550::check_present()`](https://github.com/rust-osdev/uart_16550/commit/a563618731b579fcc0984a85d7b2bfe7300e5c08), which probes for a
device through the scratch register. `init()` now delegates its existing
presence check to it, so users can run the same probe on their own before
touching the device.

The repository also gained a `real-hw-test` crate member that builds a bootable
EFI image. This makes it much easier to verify the driver on real hardware
rather than only in virtual machines - which is exactly what this crate was
rewritten for.

We merged the following PRs this month:

- [real-hw-test: init](https://github.com/rust-osdev/uart_16550/pull/71)

<!-- Chore PRs: -->
<!-- - [build(deps): bump crate-ci/typos from 1.48.0 to 1.50.0](https://github.com/rust-osdev/uart_16550/pull/72) -->
<!-- - [build(deps): bump crate-ci/typos from 1.50.0 to 1.50.2](https://github.com/rust-osdev/uart_16550/pull/73) -->


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

<span class="gray">No project updates were submitted this month.</span>



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way to get in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
