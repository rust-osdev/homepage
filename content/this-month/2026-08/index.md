+++
title = "This Month in Rust OSDev: August 2026"
date = 2026-09-01

[extra]
month = "August 2026"
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
    This is a draft for the upcoming "This Month in Rust OSDev (August 2026)" post.
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

This month was all about **specification compliance and soundness**. We audited
large parts of `uefi-raw` and `uefi` against the UEFI and PI specifications.
Users now get correct data where the crates previously returned garbage or read
out of bounds, for example:

- `boot::set_watchdog_timer` passed the watchdog data size in characters instead
  of bytes, so firmware only saw half of the data.
- `ProcessorInformation` was 24 bytes too small, so firmware could write past
  its end.
- `UsbIo::supported_languages` reported twice the actual number of language IDs,
  where the second half was an out-of-bounds read.

`MemoryDescriptor` is now portable across x86 targets, so kernels and
bootloaders built for a generic i686 target can finally parse a UEFI memory map.
To keep such bugs away, our ABI tests are now `const` assertions evaluated for
the actual target, instead of unit tests that only ever check the host.

The new `char16!()` macro builds a `Char16` from a character literal in `const`
context - no `unsafe` needed, and a compile error if the character is not valid
in UCS-2.

All of this is available in `uefi-raw v0.16.0` and `uefi v0.40.0`. We also
refreshed our `CONTRIBUTING.md`, which now documents our expectations regarding
code style, commit style, and AI/LLM-assisted contributions.

#### Sponsorship by Anthropic

We are glad to announce that [Anthropic](https://www.anthropic.com/) sponsors @phip1611 for six months 
as part of their open source program. The sponsorship covers `uefi-rs` and 
related crates in the `rust-osdev` space, with a focus on security issues, 
undefined behavior, and specification compliance. Thank you!

We merged the following PRs this month:

- [UEFI Spec Compliance: Various repr/packed fixes](https://github.com/rust-osdev/uefi-rs/pull/2040)
- [Spec Fixes: Various Smaller Fixes or Additions](https://github.com/rust-osdev/uefi-rs/pull/2041)
- [Spec Fixes: Address various smaller size/buffer mismatches](https://github.com/rust-osdev/uefi-rs/pull/2044)
- [Spec Fixes: various smaller fixes regarding protocols](https://github.com/rust-osdev/uefi-rs/pull/2048)
- [uefi: various UB fixes](https://github.com/rust-osdev/uefi-rs/pull/2052)
- [uefi-raw: make MemoryDescriptor layout portable across x86 targets](https://github.com/rust-osdev/uefi-rs/pull/2036)
- [uefi-raw: fix MemoryDescriptor layout on 32-bit](https://github.com/rust-osdev/uefi-rs/pull/2035)
- [uefi: add convenient char16!() macro](https://github.com/rust-osdev/uefi-rs/pull/2031)
- [replace ABI-related unit tests with const checks](https://github.com/rust-osdev/uefi-rs/pull/2039)
- [contributing: streamline contribution guidance + AI/LLM Policy](https://github.com/rust-osdev/uefi-rs/pull/2029)
- [docs: streamline rustdoc in uefi-raw and uefi](https://github.com/rust-osdev/uefi-rs/pull/2026)
- [various small doc improvements](https://github.com/rust-osdev/uefi-rs/pull/2025)
- [release: uefi-raw-0.16.0 and uefi-0.40.0](https://github.com/rust-osdev/uefi-rs/pull/2054)

<!--
    `Make memory maps repr(C)` (#2007) was reverted again in the same month
    (#2024), so both are omitted here.
-->
<!-- - [Make memory maps repr(C)](https://github.com/rust-osdev/uefi-rs/pull/2007) -->
<!-- - [Revert "Make memory maps repr(C)"](https://github.com/rust-osdev/uefi-rs/pull/2024) -->

<!-- Chore and dependency PRs: -->
<!-- - [xtask: Update to syn-3](https://github.com/rust-osdev/uefi-rs/pull/2019) -->
<!-- - [uefi-macros: Update to syn-3](https://github.com/rust-osdev/uefi-rs/pull/2020) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/2021) -->
<!-- - [chore(deps): update rust crate time to v0.3.47 [security]](https://github.com/rust-osdev/uefi-rs/pull/2022) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.49.0](https://github.com/rust-osdev/uefi-rs/pull/2023) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/2033) -->
<!-- - [chore(deps): update rust crate time to v0.3.47 [security]](https://github.com/rust-osdev/uefi-rs/pull/2034) -->
<!-- - [clippy: adjust latest nightly findings](https://github.com/rust-osdev/uefi-rs/pull/2038) -->
<!-- - [cargo: update deps](https://github.com/rust-osdev/uefi-rs/pull/2045) -->
<!-- - [chore(deps): update rust crate time to v0.3.55](https://github.com/rust-osdev/uefi-rs/pull/2056) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.50.0](https://github.com/rust-osdev/uefi-rs/pull/2057) -->

Thanks to [@cwize1](https://github.com/cwize1) and [@SpecificProtagonist](https://github.com/SpecificProtagonist) for their contributions!

### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

_Convenient and safe parsing of Multiboot2 Boot Information (MBI) structures and
the contained information tags. Usable in no_std environments, such as a kernel.
An optional builder feature also allows the construction of the corresponding
structures._

We removed a whole class of undefined behavior. Parsing a structure with a value
unknown to the specification - an unknown framebuffer type, VBE memory model, or
header tag type - used to construct an invalid Rust enum. The new `raw_type!`
macro generates an ABI-safe newtype plus an open-set enum with a `Custom`
variant, so unknown values now pass through safely. `multiboot2-common` got
further soundness fixes around size and alignment validation.

Users also benefit from `BootInformation::get_tags`, which iterates over _all_
occurrences of a tag. Network and SMBIOS tags may legitimately appear multiple
times, but our API only exposed the first one. The builder gained `add_network`
- and it turned out that `Builder::network` never included the tag at all.

Released as `multiboot2 v0.26.1`, `multiboot2-header v0.10.0`, and
`multiboot2-common v0.5.0`. The `raw_type!` work follows in the next release.

We merged the following PRs this month:

- [Various Subtle UB Fixes](https://github.com/rust-osdev/multiboot2/pull/312)
- [add raw_type! macro for ABI-safe raw newtypes](https://github.com/rust-osdev/multiboot2/pull/314)
- [multiboot2: improve tag getters](https://github.com/rust-osdev/multiboot2/pull/310)
- [treewide: clarify Multiboot2 header APIs and debug output](https://github.com/rust-osdev/multiboot2/pull/309)
- [docs: treewide documentation fixes](https://github.com/rust-osdev/multiboot2/pull/307)

### [`uart_16550`](https://github.com/rust-osdev/uart_16550)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

_Simple yet highly configurable low-level driver for 16550 UART devices,
typically known and used as serial ports or COM ports._

Two releases, `v0.7.0` and `v0.8.0`, make the driver behave better on real
hardware. Sending no longer waits for the `MSR::CTS` line by default, as modern
hardware tends to leave that pin disconnected - which previously meant no output
at all. Those who need hardware flow control can re-enable the check via
`Config::check_cts_before_sending`.

Further, `Config::default()` now disables _all_ interrupts, and `init()` enables
the configured ones only at the very end. This way, a driver does not receive
interrupts before it is ready to handle them.

We merged the following PRs this month:

- [Allow configuring whether to care about the CTS line](https://github.com/rust-osdev/uart_16550/pull/66)
- [config: disable interrupts by default + document interrupt behavior of init()](https://github.com/rust-osdev/uart_16550/pull/69)
- [doc: tighten cts/hw control flow wording](https://github.com/rust-osdev/uart_16550/pull/68)

Thanks to [@meithecatte](https://github.com/meithecatte) for this contribution!

### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers uses to relay information about the hardware to the OS.

We merged the following changes this month:

- [Resolve names used as package elements lazily](https://github.com/rust-osdev/acpi/pull/320)
- [Fix mutex references and the global lock](https://github.com/rust-osdev/acpi/pull/322)
- [Add Support for Multiple IRQs in IrqDescriptor (2)](https://github.com/rust-osdev/acpi/pull/331)
- [Fix acpi_dumper manifest](https://github.com/rust-osdev/acpi/pull/321)

Thanks to [@martin-hughes](https://github.com/martin-hughes), [@ArthurHeymans](https://github.com/ArthurHeymans), [@ChocolateLoverRaj](https://github.com/ChocolateLoverRaj), and [@hustlerone](https://github.com/hustlerone) for their contributions!

### [`virtio-spec-rs`](https://github.com/rust-osdev/virtio-spec-rs)
<span class="maintainers">Maintained by [@mkroening](https://github.com/mkroening)</span>

The `virtio-spec` crate provides definitions from the Virtual I/O Device (VIRTIO) specification.
This project aims to be unopinionated regarding actual VIRTIO drivers that are implemented on top of this crate.

We merged the following PRs this month:

- [feat: don't derive zerocopy traits for volatile structs](https://github.com/rust-osdev/virtio-spec-rs/pull/31)
- [feat(features): remove `FeatureBits` bounds that are not elaborated and seal `FeatureBits`](https://github.com/rust-osdev/virtio-spec-rs/pull/20)
- [feat(features): remove `requirements()` and `recommendations()`](https://github.com/rust-osdev/virtio-spec-rs/pull/34)
- [docs: mark Entropy Device as supported](https://github.com/rust-osdev/virtio-spec-rs/pull/25)
- [docs(balloon): add module-level doc comment](https://github.com/rust-osdev/virtio-spec-rs/pull/32)

<!-- - [style: remove trailing spaces from doc comment](https://github.com/rust-osdev/virtio-spec-rs/pull/29) -->

### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables.

We merged the following changes this month:

- [deps: bump uart_16550 to 0.8.0 + fix UEFI weirdness](https://github.com/rust-osdev/bootloader/pull/580)
  - Picks up the `uart_16550` changes described above. On the UEFI path, the bootloader now explicitly disconnects the UEFI console from the serial device before setting up its logger, so that the logger has exclusive ownership of the UART. Previously, UEFI kept driving the device, which duplicated console output on the serial port and fired interrupts before `init()` had finished.

Thanks to [@phip1611](https://github.com/phip1611) for this contribution!

<!--
    `bootimage` #112 and #113 merged in early August, but were already reported
    in the July 2026 issue, so they are not repeated here.
    `ovmf-prebuilt` only had renovate lock file maintenance this month.
-->

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

### [`open-nexus-OS/open-nexus-OS`](https://github.com/open-nexus-OS/open-nexus-OS)
<span class="maintainers">(Section written by [@jenningschaefer](https://github.com/jenningschaefer))</span>

[Open Nexus OS](https://open-nexus-os.io/) is a capability-based microkernel operating system written in Rust and targeting RISC-V.

This month, the Open Nexus graphical desktop stack reached a major milestone: its declarative UI DSL is now driving a working desktop interface. The `.nx` UI definitions and `.nxtheme` design tokens are used to implement the actual interface, allowing the desktop UI to be built from the same declarative system used to define its design.

The desktop stack includes a compositor, window manager, launcher, and UI components running on top of the Open Nexus userspace architecture. The project also includes a boot-to-desktop demonstration showing the graphical environment running in QEMU.

[Website](https://open-nexus-os.io/) · [Repository](https://github.com/open-nexus-OS/open-nexus-OS) · [Demo video](https://www.youtube.com/watch?v=Vrf6Z1sAY5I)


### [`phip1611/tar-no-std`](https://github.com/phip1611/tar-no-std)
<span class="maintainers">(Section written by [@phip1611](https://github.com/phip1611))</span>

[`tar-no-std`](https://github.com/phip1611/tar-no-std) supports a relevant
subset of Tar archives to extract multiple files from a single Tar archive in
`no_std` environments with zero allocations. A typical use case is a kernel
reading an initial ramdisk.

The new `v0.5.0` release stops trusting the input. `TarArchive[Ref]::new` now
rejects invalid headers, checksums, payload sizes, and missing archive
termination, so a malformed archive fails right away instead of producing
garbage entries. Numeric fields with invalid UTF-8 bytes no longer silently
parse as zero, and `CorruptDataError` became an enum that names the violated
invariant. Additionally, there is now limited support for POSIX PAX archives
that use extended records only for optional metadata, such as high-precision
timestamps.

To keep it that way, the repository gained `cargo-fuzz` infrastructure,
including structure-aware fuzzing with checksum-valid archives.

Thanks to [@internetisalie](https://github.com/internetisalie) and
[@fogti](https://github.com/fogti) for their contributions!



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way to get in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
