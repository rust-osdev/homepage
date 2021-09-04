+++
title = "This Month in Rust OSDev (August 2021)"
date = 0000-01-01

[extra]
month = "August 2021"
authors = [
    "phil-opp",
    "IsaacWoods",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (August 2021)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

### [`x86_64`](https://github.com/rust-osdev/x86_64)

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

In August, …

### [`acpi`](https://github.com/rust-osdev/acpi)

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS.

There was a fair amount of progress this month, including adding support for a major AML feature: buffer fields.
These are objects that represent bit-level slices of buffer objects, allowing a section of a buffer to be read and
written. This requires us to support multiple namespace objects operating on the same underlying data -
currently this is handled simply with spinlocks, but we're looking to improve this situation in the future.
With this, we grew support for the `DefCreateBitField`, `DefCreateByteField`, `DefCreateWordField`,
`DefCreateDWordField`, `DefCreateQWordField`, and `DefCreateField` opcodes, as well as logic for reading and
writing `Integer`s and `Buffer`s out of them. <span class="gray">(published as `aml v0.16.0`)</span>

The `acpi` crate also saw improvement, and unfortunately some more breakage. We now support X2APIC entries
appearing in the MADT, and so have changed the representation of Processor UIDs and Local APIC IDs in
`acpi::platform::Processor` to `u32`, to support the wider IDs that X2APIC uses to support more processors. On
older platforms that only support APIC, the upper bits will simply always be zeroed, and so these values can be
cast down to `u8` when needed. Note that for complex legacy reasons, you cannot rely upon ACPI for detecting
whether a platform has X2APIC support or not, and so whether the MADT actually contained X2APIC entries is not exposed
by this interface - this is by design. <span class="gray">(published as `acpi v4.0.0`)</span>

Other changes were:
- [`aml`: the contents of the `namespace` module were made public](https://github.com/rust-osdev/acpi/pull/107).
  Thanks to [@Andy-Python-Programmer](https://github.com/Andy-Python-Programmer) for this contribution!
- [`aml`: store locals and method arguments in arrays](https://github.com/rust-osdev/acpi/commit/c1597aba3d39344834292637fb81e2f2971d6c04). If you're manually calling AML methods
  that take arguments, you will need to update how you create them.
- [`aml`: support the `DefFatal` opcode](https://github.com/rust-osdev/acpi/commit/514e55df07acbca93dfd4eb2db3cdd6fdea5aaf5). `DefFatal` will cause a panic by default, but this behaviour can be overridden by implementing `Handler::handle_fatal_error`.
- [`aml`: implement the `DefWhile` opcode](https://github.com/rust-osdev/acpi/commit/06409b360ef30b3b08b56865f3ee380315751f14)
- [`aml`: implement the `DefBreak` opcode](https://github.com/rust-osdev/acpi/commit/da5f5cec8096d2ebd5697212e282abbeaed6edb7)
- [`aml`: implement the `DefContinue` opcode](https://github.com/rust-osdev/acpi/commit/ed0400092e18598c73ca6048fb96b2522237808d)
- [`aml`: implement the `DefIncrement` and `DefDecrement` opcodes](https://github.com/rust-osdev/acpi/commit/b854d5491e48e5a4f332ff259ce185cb357261d0)
- [`aml`: implement the `ToInteger` opcode](https://github.com/rust-osdev/acpi/commit/00a61d8b7471dae725283296f4ee9c0c20013156)

## Call for Participation

Want to contribute to a Rust OSDev project, but don't know where to start? Pick up one of these outstanding
issues in one of our projects and get started!

<!--
Please use the following template for adding items:
- [(`repo_name`) Issue Description](https://example.com/link-to-issue)
-->

<span class="gray">

_No tasks were proposed for this section._

</span>

If you maintain a Rust OSDev project and are looking for contributors, especially for tasks suited to people
getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the
`next` branch with the tasks you want to include in the next issue.


## Personal Projects

In this section, we describe updates to personal projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

This month, ...

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).


<!--
TODO: Update publication date
-->
