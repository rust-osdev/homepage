+++
title = "This Month in Rust OSDev: April 2022"
date = 0000-01-01

[extra]
month = "April 2022"
authors = [
    "phil-opp",
    "toku-sa-n",
    "phip1611",
    "andre-richter",
    "berkus"
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (April 2022)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

### [`x86_64`](https://github.com/rust-osdev/x86_64)

<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13), and [@rybot666](https://github.com/orgs/rust-osdev/people/rybot666)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

In April, …

### [`xhci`](https://github.com/rust-osdev/xhci)

The `xhci` crate provides types of xHCI structires, such as Registers and TRBs.

In this month, we released a new version 0.8.3 which includes [a bug fix](https://github.com/rust-osdev/xhci/pull/132) reported and committed by [@Yuna-Tomi](https://github.com/Yuna-Tomi). The bug was that `EventRingDequeuePointerRegister::event_ring_dequeue_pointer()` did not return the correct address. Thanks for the contribution!.

### [`multiboot2`](https://github.com/rust-osdev/multiboot2)

The `multiboot2` crate provides abstraction types for the multiboot information structure (MBI) of multiboot2 
bootloaders. The latest release of the `multiboot2`-crate is now `v.0.13.2` (was `v0.13.1`). It contains minor 
improvements, such as that `TagType` implements `Ord`.

## Call for Participation

Want to contribute to a Rust OSDev project, but don't know where to start? Pick up one of these outstanding
issues in one of our projects and get started!

[(`multiboot2`) Missing Tags](https://github.com/rust-osdev/multiboot2/issues/100)

<!--
Please use the following template for adding items:
- [(`repo_name`) Issue Description](https://example.com/link-to-issue)
-->

If you maintain a Rust project related to operating system development and are looking for contributors, especially for tasks suited to people getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the `next` branch with the tasks you want to include in the next issue.

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`metta-systems/vesper`](https://github.com/metta-systems/vesper)

<span class="gray">(Section written by [@berkus](https://github.com/berkus))</span>

Vesper is a capability-based single-address-space nanokernel. This means it is aiming to be small, to provide only isolation primitives; at the same time SAS makes it a lot easier to perform cross-process operations (because all addresses are the same across all processes). It uses capabilities to provide security for such operations, so that unauthorized processes will not be able to intervene in legitimate traffic.

The kernel is in very early stages of development. This time I will update on the progress of tooling and my next steps. As usual, I will link directly to my blog for more details. [Read the full article here](https://metta.systems/blog/osdev-tooling-3/).

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

This month, ...

### [`rust-embedded/rust-raspberrypi-OS-tutorials`](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)

<span class="gray">(Section written by [@andre-richter](https://github.com/andre-richter))</span>

The [Operating System development tutorials in Rust on the Raspberry Pi](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials) project
saw two more tutorial releases:

- [Tutorial 17 - `Kernel Symbols`](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/tree/master/17_kernel_symbols)
- [Tutorial 18 - `Backtracing`](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/tree/master/18_backtrace)

The two tutorials implement the generation of backtraces that show address and symbol information. Here is an example of the of a backtrace generated as part of  the kernel's panic handler:

```console
[    0.002782] Writing to bottom of address space to address 1 GiB...
[    0.004623] Kernel panic!

Panic location:
      File 'kernel/src/_arch/aarch64/exception.rs', line 59, column 5

[...]

Backtrace:
      ----------------------------------------------------------
          Address            Function containing address
      ----------------------------------------------------------
       1. ffffffffc0001294 | core::fmt::write
       2. ffffffffc0005560 | libkernel::panic_wait::_panic_print
       3. ffffffffc00054a0 | rust_begin_unwind
       4. ffffffffc0002950 | core::panicking::panic_fmt
       5. ffffffffc0004898 | current_elx_synchronous
       6. ffffffffc0000a74 | __vector_current_elx_synchronous
       7. ffffffffc000111c | kernel_init
      -----------------------------------------------------------
```

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).


<!--
TODO: Update publication date
-->
