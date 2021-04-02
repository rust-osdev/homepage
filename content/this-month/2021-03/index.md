+++
title = "This Month in Rust OSDev (March 2021)"
date = 0000-01-01

[extra]
month = "March 2021"
authors = [
    "phil-opp",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

### [`multiboot2`](https://github.com/rust-osdev/multiboot2-elf64)

The `multiboot2` crate provides abstraction types for the boot information of multiboot2 bootloaders. We merged the following updates this month:

- [Use `impl Iterator` as return type instead of named types](https://github.com/rust-osdev/multiboot2-elf64/pull/72)
- [Docs: Remove fragile `asm!` code example](https://github.com/rust-osdev/multiboot2-elf64/pull/73)
- [Apply `rustfmt`](https://github.com/rust-osdev/multiboot2-elf64/pull/74)

Thanks to [@toku-sa-n](https://github.com/toku-sa-n) for their contributions!

### [`volatile`](https://github.com/rust-osdev/volatile)

The `volatile` crate provides a safe wrapper type for implementing volatile read and write operations. This is useful for accessing memory regions that have side-effects, such as memory-mapped hardware registers or framebuffers. In March, we fixed a build error that was caused by a change in nightly Rust:

- [Replace feature `range_bounds_assert_len` with `slice_range`](https://github.com/rust-osdev/volatile/pull/21) <span class="gray">(published as `v0.4.4`)</span>
- [Add a test for `slice::as_chunks_mut` usage](https://github.com/rust-osdev/volatile/commit/15bbfac9c7cb42ff56698ac5c00daeddbcdb6a0d)
    - By using `as_chunks_mut`, it is possible read and write multiple slice elements through a single volatile operation. This allows the compiler to optimize the code better (compared to reading the elements one by one).

Thanks to [@KernelFreeze](https://github.com/KernelFreeze) for their contribution!

### [`x86_64`](https://github.com/rust-osdev/x86_64)

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

In March, …

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
