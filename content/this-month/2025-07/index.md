+++
title = "This Month in Rust OSDev: July 2025"
date = 2025-08-08

[extra]
month = "July 2025"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (July 2025)" post.
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

- [You Are The BIOS Now: Building A Hypervisor In Rust With KVM](https://yeet.cx/blog/you-are-the-bios-now)
- [How to write Rust in the kernel: part 3](https://lwn.net/Articles/1026694/)
- [Vivo BlueOS Kernel open-sourced](https://github.com/vivoblueos/kernel)
  - Vivo is a Chinese company selling smartphones and [smartwatches](https://www.vivo.com/en/products/watch3)
  - BlueOS appears to be their smartwatch OS, now its kernel is open source (Apache license)
  - POSIX-compatible, targets ARM and RISCV, supports Rust's `std` library
  - Chinese homepage: https://blueos.vivo.com/kernel
- [This Month in Redox - July 2025](https://www.redox-os.org/news/this-month-250731/)
- [Writing a Rust GPU kernel driver: a brief introduction on how GPU drivers work](https://www.collabora.com/news-and-blog/blog/2025/08/06/writing-a-rust-gpu-kernel-driver-a-brief-introduction-on-how-gpu-drivers-work/)
- [`bitpiece`: a crate for defining and manipulating bitfields with procedural macros](https://github.com/roeeshoshani/bitpiece)
- [A Clone of the Linux Kernel's Red-Black Tree in Rust](https://www.reddit.com/r/rust/comments/1lyad9b/rougenoir_a_clone_of_the_linux_kernels_redblack/)
- [Rex: Closing the language-verifier gap with safe and usable kernel extensions](https://www.usenix.org/conference/atc25/presentation/jia)
- [Dynamic Indirect Syscalls via JOP or ROP in Rust](https://kirchware.com/Dynamic-Indirect-Syscalls-via-JOP-or-ROP-in-Rust)
- [Practicing Linux Syscalls with Rust and x86_64 Assembly](https://www.reddit.com/r/rust/comments/1lyxyoa/practicing_linux_syscalls_with_rust_and_x86_64/)

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [Allow volatile access to non-Rust memory, including address 0](https://github.com/rust-lang/rust/pull/141260)


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

### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following fix:

- [Fixes the type of target-c-int-width in target jsons.](https://github.com/rust-osdev/bootloader/pull/509)

Thanks to [@OmegaMetor](https://github.com/OmegaMetor) for their contribution!


### [`uart_16550`](https://github.com/rust-osdev/uart_16550)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `uart_16550` crate provides basic support for serial port I/O for 16550-compatible UARTs. We merged the following change this month:

- [Update port.rs to do \r\n](https://github.com/rust-osdev/uart_16550/pull/40)

Thanks to [@rsahwe](https://github.com/rsahwe) for their contribution!


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [doc: fix broken links](https://github.com/rust-osdev/uefi-rs/pull/1716)
- [uefi: Add HiiKeywordHandler and HiiConfigAccess protocol](https://github.com/rust-osdev/uefi-rs/pull/1684)
- [protocols: Add ACPI Table protocol](https://github.com/rust-osdev/uefi-rs/pull/1731)
- [feat: `uefi-raw` IoMmu Protocol Impl](https://github.com/rust-osdev/uefi-rs/pull/1732)

Thanks to [@seijikun](https://github.com/seijikun), [@Jonathas-Conceicao](https://github.com/Jonathas-Conceicao) and [@PelleKrab](https://github.com/PelleKrab) for their contributions!

<!-- - [chore(deps): update crate-ci/typos action to v1.34.0](https://github.com/rust-osdev/uefi-rs/pull/1717)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1718)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1729)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1730) -->

### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PRs this month:

- [docs(offset_page_table): fix typo](https://github.com/rust-osdev/x86_64/pull/555)
- [feat: add forwarding `impl PageTableFrameMapping for &P`](https://github.com/rust-osdev/x86_64/pull/556)

Thanks to [@mkroening](https://github.com/mkroening) for their contributions!



## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


<!-- <span class="gray">No projects updates were submitted this month.</span> -->

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)
<span class="maintainers">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

We merged the following changes to the [_Writing an OS in Rust_](https://os.phil-opp.com/) blog this month:

- [Fix: `target-c-int-width` field now expects an integer](https://github.com/phil-opp/blog_os/pull/1425)
  - [Update blog for `target-c-int-width` change](https://github.com/phil-opp/blog_os/pull/1426)
- [Add `[[bin]]` section with `test=false` to Cargo.toml](https://github.com/phil-opp/blog_os/pull/1412) (thanks to [@tigeryant](https://github.com/tigeryant))
  - [Update first post to set `test=false` for binary](https://github.com/phil-opp/blog_os/pull/1427)

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
