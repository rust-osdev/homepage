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
- [FOSDEM 2024](https://fosdem.org/2024/)
  - [News from the Hermit Crab — From Soundness Foundations to GPU Virtualization](https://fosdem.org/2024/schedule/event/fosdem-2024-3375-news-from-the-hermit-crab-from-soundness-foundations-to-gpu-virtualization/)
  - [Making VirtIO sing - implementing virtio-sound in rust-vmm project](https://fosdem.org/2024/schedule/event/fosdem-2024-1910-making-virtio-sing-implementing-virtio-sound-in-rust-vmm-project/)
  - [The case for a virtual Rust stateless codec driver](https://fosdem.org/2024/schedule/event/fosdem-2024-2985-the-case-for-a-virtual-rust-stateless-codec-driver/)
  - [An open-source, open-hardware offline finding system](https://fosdem.org/2024/schedule/event/fosdem-2024-3264-an-open-source-open-hardware-offline-finding-system/)
- [Making an RISC-V OS (Part 1): Project Setup](https://traxys.me/riscv_os_setup.html)
- [Looking for people to help out with CharlotteOS (Beginners are welcome!)](https://www.reddit.com/r/osdev/comments/1aeffha/looking_for_people_to_help_out_with_charlotteos/)
- [The Linux kernel now contains the first useful component written in Rust](https://fosstodon.org/@kernellogger/111741507899977461)
- [BPF Opens Door to Linux Extensible Scheduling (Maybe with Rust!)](https://thenewstack.io/bpf-opens-a-door-to-linux-dynamic-scheduling-maybe-with-rust/)

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


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PR this month:

- [expose `Cr3::write_raw`](https://github.com/rust-osdev/x86_64/pull/445)

We also merged the following changes into the `next` branch, which will be released as `v0.15` soon:

- [Update `next` branch with latest changes from `master`](https://github.com/rust-osdev/x86_64/pull/447)
- [remove deprecated from_bits_unchecked functions](https://github.com/rust-osdev/x86_64/pull/449)
- [make `HandlerFuncType` unsafe](https://github.com/rust-osdev/x86_64/pull/450)
- [Update docs to clarify new `set_handler_fn` behavior](https://github.com/rust-osdev/x86_64/pull/451)

### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following nightly fixes:

- [Fix data layout for stage 3 target](https://github.com/rust-osdev/bootloader/pull/413)
- [Release `v0.11.6`](https://github.com/rust-osdev/bootloader/pull/414)
- [[v0.9] Fix data layout for `x86_64-bootloader` target](https://github.com/rust-osdev/bootloader/pull/415)
  - Released as v0.9.24


### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS. We merged the following changes this month:

- [Add `Debug` impl for `PhysicalMapping` even when `T` is not `Debug`](https://github.com/rust-osdev/acpi/pull/206)

Thanks to [@Spartan2909](https://github.com/Spartan2909) for their contribution!


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

The `uefi-rs` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. We merged the following PRs this month:

- [Fix broken UEFI spec link in uefi-raw README.md](https://github.com/rust-osdev/uefi-rs/pull/1046)
- [book: Add a page on building drivers](https://github.com/rust-osdev/uefi-rs/pull/1047)
- [book: Add a link to rust's UEFI target docs](https://github.com/rust-osdev/uefi-rs/pull/1048)
- [uefi-raw: Add LoadFileProtocol and LoadFile2Protocol](https://github.com/rust-osdev/uefi-rs/pull/1022)

<!--
- [chore(deps): update crate-ci/typos action to v1.16.26](https://github.com/rust-osdev/uefi-rs/pull/1038)
- [fix(deps): update rust crate anyhow to v1.0.78](https://github.com/rust-osdev/uefi-rs/pull/1039)
- [fix(deps): update rust crate clap to v4.4.12](https://github.com/rust-osdev/uefi-rs/pull/1040)
- [fix(deps): update rust crate proc-macro2 to v1.0.73](https://github.com/rust-osdev/uefi-rs/pull/1041)
- [fix(deps): update rust crate anyhow to v1.0.79](https://github.com/rust-osdev/uefi-rs/pull/1043)
- [chore(deps): update rust crate trybuild to v1.0.88](https://github.com/rust-osdev/uefi-rs/pull/1042)
- [fix(deps): update rust crate clap to v4.4.13](https://github.com/rust-osdev/uefi-rs/pull/1044)
-->

<!--
- [chore(deps): update crate-ci/typos action to v1.17.1](https://github.com/rust-osdev/uefi-rs/pull/1051)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1053)
- [chore(deps): update cachix/install-nix-action action to v25](https://github.com/rust-osdev/uefi-rs/pull/1054)
-->
<!-- - [fix(deps): update rust crate clap to v4.4.17](https://github.com/rust-osdev/uefi-rs/pull/1050)
- [fix(deps): update rust crate crates-index to v2.5.0](https://github.com/rust-osdev/uefi-rs/pull/1052)
- [chore(deps): update crate-ci/typos action to v1.17.2](https://github.com/rust-osdev/uefi-rs/pull/1055)
- [chore(deps): update rust crate bitflags to v2.4.2](https://github.com/rust-osdev/uefi-rs/pull/1056)
- [fix(deps): update rust crate clap to v4.4.18](https://github.com/rust-osdev/uefi-rs/pull/1057)
- [fix(deps): update rust crate proc-macro2 to v1.0.78](https://github.com/rust-osdev/uefi-rs/pull/1058)
- [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1062) -->

Thanks to [@gurry](https://github.com/gurry) for their contribution!


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

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)
<span class="maintainers">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

We merged the following changes to the [_Writing an OS in Rust_](https://os.phil-opp.com/) blog this month:

- [Update data layouts in target specifications](https://github.com/phil-opp/blog_os/pull/1271)
- [change some format errors for chinese edition minimal kernel](https://github.com/phil-opp/blog_os/pull/1274)
- [fix testing and paging introduction chapter errors in zh-CN translation](https://github.com/phil-opp/blog_os/pull/1262)
- [[Edition 3] fix a lil typo](https://github.com/phil-opp/blog_os/pull/1270)
- [[Edition 3] Fix typos in code for `embedded_graphics` crate in chapter 3](https://github.com/phil-opp/blog_os/pull/1269)
- [[Edition 3] Fix embedded_graphics code + typo in chapter 3](https://github.com/phil-opp/blog_os/pull/1276)
- [[Edition 3] Fix typo in chapter 2](https://github.com/phil-opp/blog_os/pull/1265)
- [[Edition 3] Fix formatting in chapter 2](https://github.com/phil-opp/blog_os/pull/1266)

Thanks to [@acyanbird](https://github.com/acyanbird), [@proudmuslim-dev](https://github.com/proudmuslim-dev), and [@lachsdachs](https://github.com/lachsdachs) for their contributions!

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
