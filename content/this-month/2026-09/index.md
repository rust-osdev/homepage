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

All of this is unreleased so far and will ship in the next `uefi` and
`uefi-raw` releases.

As mentioned last month, [Anthropic](https://www.anthropic.com/) sponsors
@phip1611 with a Max plan as part of their open source program. Much of the
auditing above happened under that sponsorship, and we are happy that it helps
us improve the security, robustness, and reliability of the ecosystem.

Thanks to [@crawfxrd](https://github.com/crawfxrd),
[@cwize1](https://github.com/cwize1) and
[@the-shank](https://github.com/the-shank) for their contributions!

We merged the following PRs this month:
<!-- TODO -->

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
<!-- TODO -->

### [`uart_16550`](https://github.com/rust-osdev/uart_16550)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

_Simple yet highly configurable low-level driver for 16550 UART devices,
typically known and used as serial ports or COM ports._

`v0.8.1` adds the public method `Uart16550::check_present()`, which probes for a
device through the scratch register. `init()` now delegates its existing
presence check to it, so users can run the same probe on their own before
touching the device.

The repository also gained a `real-hw-test` crate member that builds a bootable
EFI image. This makes it much easier to verify the driver on real hardware
rather than only in virtual machines - which is exactly what this crate was
rewritten for.

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
