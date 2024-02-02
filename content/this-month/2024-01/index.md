+++
title = "This Month in Rust OSDev: January 2024"
date = 2024-01-05

[extra]
month = "January 2024"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (January 2024)" post.
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

- [This Month in Redox](https://redox-os.org/news/this-month-240131/)
- [x86 Kernel Development & Relocatable Binaries – What I learned about Toolchains and Relocatable Code](https://phip1611.de/blog/x86-kernel-development-relocatable-binaries/)
    - This post is roughly a summary of the obscure knowledge Philipp learned 
      about toolchains and relocatable code in the last couple of years with a 
      focus on relocatable x86_64 multiboot2 kernels for legacy BIOS boot.
- The Embedded Rustacean [Issue 11](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-11) and [Issue 12](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-12)

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

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

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


### [`phip1611/phipsboot`](https://github.com/phip1611/phipsboot)
<span class="maintainers">(Section written by [@phip1611](https://github.com/phip1611))</span>

I'd like to announce my project PhipsBoot. 🎉 PhipsBoot is a relocatable x86_64 
bootloader for legacy x86_64 boot written in Rust and assembly. It is intended 
to be loaded by GRUB via Multiboot2, where it uncovers its main benefit: It is 
relocatable in physical memory without having relocation information in the 
ELF! It outsources a lot of complexity to GRUB which also better fits into
the ecosystem and makes it easier usable. The README contains more background 
about why I have chosen to use GRUB instead of writing my own stage 1 
bootloader.

This project combines a lot of toolchain and binary knowledge and experience I 
collected and gained in recent years about legacy x86_64 boot. **The main 
contribution IMHO is how the binary is assembled and that the thing boots
with all the properties described in the README, but not the high-level 
functionality itself.**

I am especially proud of the well-commented structure of the assembly files.
For example the whole page-table mappings are done IMHO very nicely even tho
it is assembly language. Also, I think it turned out quite cool how I configured
the linker script. I hope this can be a learning resource for others!

TL;DR: It is a learning ground and a reference for how to solve the relocation
problem with Multiboot2 and GRUB, as GRUB is not able to load DYN ELFs.

You have multiple options for testing it out:

- `$ cloud-hypervisor --debug-console file=log.txt --kernel ./build/phipsboot.elf64` (using Xen PVH)
- `$ qemu-system-x86_64 -kernel ./build/phipsboot.elf32 -debugcon stdio` (using Multiboot 1)

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
