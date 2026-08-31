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

We merged the following PRs this month:

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
