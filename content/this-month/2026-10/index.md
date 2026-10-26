+++
title = "This Month in Rust OSDev: October 2026"
date = 2026-11-01

[extra]
month = "October 2026"
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
    This is a draft for the upcoming "This Month in Rust OSDev (October 2026)" post.
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

- [Announcing Asterinas 0.19.0](https://asterinas.github.io/2026/10/13/announcing-asterinas-0.19.0.html)
  - [Asterinas](https://github.com/asterinas/asterinas) is a Linux ABI-compatible OS kernel written from scratch in Rust. 0.19.0 adds a DRM subsystem (Xorg and 3D games), AArch64 as its fourth architecture, Intel VMX hypervisor support in OSTD, a Rust-native device model, and self-hosting builds.
- [The Documentation I Wish I Had: How I build drivers for Open Nexus OS](https://open-nexus-os.io/blog/the-documentation-i-wish-i-had)
  - Bringing up the Rust microkernel OS [Open Nexus OS](https://github.com/open-nexus-OS/open-nexus-OS) on a Banana Pi BPI-F3: measuring the vendor's Linux system first and turning those measurements into pass/fail gates that drive the display, USB, and GPU driver work.

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

### [`uart_16550`](https://github.com/rust-osdev/uart_16550)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

_Simple yet highly configurable low-level driver for 16550 UART devices,
typically known and used as serial ports or COM ports._

[`v0.9.0`](https://github.com/rust-osdev/uart_16550/commit/492c16b2ed81020cb51e302ba7056481db69ac2c)
makes the driver considerably more robust on real hardware. The driver is now
tested against a model of a 16550A that follows the flows of the Linux 8250
driver. The model was created with LLM assistance and careful human review, and
it found 12 bugs, all fixed in this release. Most notably, devices without
working FIFOs, such as the 16450, overran the transmitter and hung the loopback
self-test that `Uart16550Tty` always runs. Also, internal register reads lost
line errors and modem status changes, which `lsr()` and `msr()` now report.

The `Backend` trait is no longer sealed, so users can bring their own register
access, e.g., port I/O via `tdcall` or the GHCB in confidential VMs. Together
with the new `const fn` constructors, devices can now live in a `static`. The
driver also gained `enable_interrupts()` and `disable_interrupts()`, e.g., for
interrupt-driven transmission.

The release comes with several breaking changes, most notably a new MSRV of
1.91, a smaller `Backend` trait with only `read()` and `write()`, and
`calc_divisor()` reporting divisors that don't fit into 16 bits instead of
panicking. The
[changelog](https://github.com/rust-osdev/uart_16550/blob/main/CHANGELOG.md)
has all details.

As mentioned in previous issues, [Anthropic](https://www.anthropic.com/)
sponsors @phip1611 with a Max plan as part of their open source program. The
16550A model and the testing above happened under that sponsorship, and we are
happy that it helps us improve the robustness and reliability of the ecosystem.

We merged the following PRs this month:
<!-- TODO -->

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

**Update: Open Nexus OS now runs on real hardware.** This month the OS booted on a Banana Pi BPI-F3 (SpacemiT K1) for the first time: it boots from eMMC through the vendor boot chain and shows the desktop at 1920x1080 on HDMI via the SoC's display controller. A new userspace USB host stack (a reactive xHCI driver plus HID parsers) lets a USB keyboard and mouse drive the desktop on the board, with the pointer riding the display controller's hardware layer.

Work then moved to the K1's GPU. The `gpud` service gained a Rust driver for it with multiple GPU contexts, a render queue, and a scanout contract, together with an in-house shader toolchain that compiles WGSL into the project's own IR and emits code for both the board's GPU and the virgl path on QEMU. The compositor now redraws only the damaged regions of a frame on the GPU, and since mid-October the board's display is no longer composited on the CPU at all.

On the desktop side, window tiling became the default model, a clipboard service with text editing and an IME landed, and the shell gained a screenshot tool. The bring-up method behind all of this is described in the blog post [The Documentation I Wish I Had](https://open-nexus-os.io/blog/the-documentation-i-wish-i-had).

[Website](https://open-nexus-os.io/) · [Repository](https://github.com/open-nexus-OS/open-nexus-OS) · [Changelog](https://github.com/open-nexus-OS/open-nexus-OS/blob/main/CHANGELOG.md)

### [`EmiBSD/Kernel`](https://github.com/EmiBSD/Kernel)
<span class="maintainers">(Section written by [@enavarre-cl](https://github.com/enavarre-cl))</span>

[EmiBSD](https://www.emibsd.org/) carries the OpenBSD kernel into Rust in two stages. [LandingZone](https://github.com/EmiBSD/LandingZone) ports it one C file to one Rust file, keeping the C's shape so each file can be checked against its origin; Kernel then redesigns what LandingZone ported, subsystem by subsystem. The `kernel` and `abi` crates are `#![forbid(unsafe_code)]`; only `hal` may hold `unsafe`, each site with a written proof. OpenBSD's behaviour is the specification: the system-call ABI stays byte-identical, so OpenBSD's own userland runs unmodified, and every change is checked against a real OpenBSD in QEMU. The code is written by AI coding agents under written rules; one maintainer reviews and approves every commit. Pre-alpha, arm64 in QEMU only, not affiliated with the OpenBSD project.

October was the month Kernel split from LandingZone and closed its first five milestones, N0 to N4: bootstrap, the leaf libraries, the `kernel`/`abi`/`hal` crates, memory (uvm into `kernel::vm`, the pmaps into `hal`), and processes and scheduling. At N4, processes, descriptors, kqueue, signals, the scheduler, terminals and the clock live in the safe `kernel` crate; the context switch, trap and system-call entry, user copies and the Spectre-BHB and KPTI code live in `hal`, which now has 184 `unsafe` sites against 9490 left in the inherited tree. 105 of OpenBSD's 331 system calls are handled natively, from 17 at the start of N4. The test suite passes on two and four CPUs, the installer runs end to end, and the comparison with a real OpenBSD found no new difference. The cost is measured too: under QEMU on arm64 the page fault is 41 % slower than at N3 and fork plus exec 24 %; those numbers wait for the performance milestone.

Next is N5, the VFS and the file systems.

[Website](https://www.emibsd.org/) · [Status, with every figure at a named commit](https://www.emibsd.org/status/) · [N4 post](https://www.emibsd.org/news/2026/10/24/n4-met/)



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way to get in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
