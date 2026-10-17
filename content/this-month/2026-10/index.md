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

<span class="gray">No project updates were submitted this month.</span>



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way to get in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
