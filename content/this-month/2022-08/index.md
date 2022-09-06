+++
title = "This Month in Rust OSDev: August 2022"
date = 0000-01-01

[extra]
month = "August 2022"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (August 2022)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## `rust-osdev` Projects

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

<!--
    Please use the following template:

    ### [`repo_name`](https://github.com/rust-osdev/repo_name)
    <span class="maintainers">Maintained by [@maintainer_1](https://github.com/maintainer_1)</span>

    The `repo_name` crate ...<<short introduction>>...

    We merged the following changes this month:
    <<changelog, either in list or text form>>
-->

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS.

We merged the following changes in August:

- [Add `DiskIo` and `DiskIo2` protocols](https://github.com/rust-osdev/uefi-rs/pull/467)
- [Added macros `print!` and `println!`](https://github.com/rust-osdev/uefi-rs/pull/430)
- [Use `doc_auto_cfg` to show feature requirements on docs.rs](https://github.com/rust-osdev/uefi-rs/pull/487)
- [add `Ord` and `PartialOrd` for `Guid` struct](https://github.com/rust-osdev/uefi-rs/pull/493)
- [QEMU/OVMF improvements](https://github.com/rust-osdev/uefi-rs/pull/474)
- [Update `nix` requirement from 0.24.1 to 0.25.0](https://github.com/rust-osdev/uefi-rs/pull/480)
- [Update deprecation version in `ScopedProtocol::interface`](https://github.com/rust-osdev/uefi-rs/pull/485)
- [Relax version requirements for various deps](https://github.com/rust-osdev/uefi-rs/pull/482)

We also published a subset of the above changes as a new `v0.16.1` release:

- [Cherry-pick fixes to the `version-0.16` branch](https://github.com/rust-osdev/uefi-rs/pull/490)
- [Update changelog for 0.16.1](https://github.com/rust-osdev/uefi-rs/pull/492)

Thanks to [@kendase3](https://github.com/kendase3), [@JonahPlusPlus](https://github.com/JonahPlusPlus), and [@e820](https://github.com/e820) for their contributions!

### [`bootloader`](https://github.com/rust-osdev/bootloader)

<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@rybot666](https://github.com/rybot666), and [@64](https://github.com/64)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables.

This month, we finally finished the new BIOS boot implementation for the [upcoming `v0.11` release](https://github.com/rust-osdev/bootloader/pull/232). It's now almost completely written in Rust (instead of assembly), which should make further improvements much easier.

Some selected commits that might be interesting:

- [Load a third stage](https://github.com/rust-osdev/bootloader/pull/232/commits/6492eab051b71a19b58a4f70185e7898fabb2c46)
- [Load the kernel into buffer memory](https://github.com/rust-osdev/bootloader/pull/232/commits/84eea29b69ac75c0d7ce9c36e0d2a1369052dc2b)
- [Copy kernel to protected mode](https://github.com/rust-osdev/bootloader/pull/232/commits/52ad3e2afab4f902ef5acb3241d7bc34c2a16e94)
- [Jump to third stage](https://github.com/rust-osdev/bootloader/pull/232/commits/7cf073eae6a5c6e30684a67cdac66016c7dcfdf2)
- [Set up paging and enter long mode (compatibility mode)](https://github.com/rust-osdev/bootloader/pull/232/commits/824786b0498a8f02e4d79ca8c8477ed68dae0068)
- [Create prototype for long mode stage 4 and load it](https://github.com/rust-osdev/bootloader/pull/232/commits/39ba5269eada8ad40963d2ec7c92c4a6410060ab)
- [Load long mode `GDT` and jump to 4th stage](https://github.com/rust-osdev/bootloader/pull/232/commits/05130d1d356e7e1566f7e576245580fc542184e6)
- [Query vesa modes and filter by resolution](https://github.com/rust-osdev/bootloader/pull/232/commits/d8931970365a2ac26088320678cc8ce6ae60150d)
- [Enable VESA framebuffer and update screen writer in stages 3 and 4](https://github.com/rust-osdev/bootloader/pull/232/commits/557c03427f5d143ee814fb908b05c584ec37b87a)
- [Load `E820` memory map and put everything together](https://github.com/rust-osdev/bootloader/pull/232/commits/48cd6dcd109778032ce586735f5be1f1dac67117)

All the tests are passing now, so we only need to do some cleanup and write proper documentation, then we should be ready to publish an alpha release for testing.

### [`linked-list-allocator`](https://github.com/rust-osdev/linked-list-allocator)

<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@jamesmunns](https://github.com/jamesmunns)</span>

The `linked-list-allocator` crate provides a basic `no_std` allocator that builds a linked list from freed memory blocks and thus needs no additional data structures.

In August, [Evan Richter](https://github.com/evanrichter) discovered a _vulnerability in `Heap::extend`_ that could lead to out-of-bound writes. The issue occured when `extend` was called with a size smaller than `size_of::<usize> * 2`, i.e., a size too small to store the metadata for the new memory region.

Upon investigating this issue, we found several similar issues:

- Initializing a heap with a size smaller than `size_of::<usize> * 3` could result in an out-of-bounds write too.
- Calling `extend` on an uninitialized heap could also result in an out-of-bounds write.
- Calling `extend` on a heap whose size is not a multiple of the size of two `usize`s resulted in unaligned writes.

We created a [security advisory](https://github.com/rust-osdev/linked-list-allocator/security/advisories/GHSA-xg8p-34w2-j49j) with more details and released a fix in `v0.10.2`, with the following changes:

- The initialization functions now panic if the given size is not large enough to store the necessary metadata. Depending on the alignment of the heap bottom pointer, the minimum size is between `2 * size_of::<usize>` and `3 * size_of::<usize>`.
- The `extend` method now panics when trying to extend an unitialized heap.
- Extend calls with a size smaller than `size_of::<usize>() * 2` are now buffered internally and not added to the list directly. The buffered region will be merged with future `extend` calls.
- The `size()` method now returns the _usable_ size of the heap, which might be slightly smaller than the `top() - bottom()` difference because of alignment constraints.

Thanks to [@evanrichter](https://github.com/evanrichter) for reporting this vulnerability and working with us on a fix.

### [`xhci`](https://github.com/rust-osdev/xhci)

<span class="maintainers">Maintained by [@toku-sa-n](https://github.com/toku-sa-n)</span>

The `xhci` crate provides types of xHCI structires, such as Registers and TRBs.

We merged the following changes this month:

- [Rename `InterruptRegisterSet` to `InterrupterRegisterSet`](https://github.com/rust-osdev/xhci/pull/143) <span class="gray">([published as `0.8.7`](https://github.com/rust-osdev/xhci/pull/144))</span>
- [Allow updating single fields of InterrupterRegisterSet](https://github.com/rust-osdev/xhci/pull/142) <span class="gray">([published as `0.9.0`](https://github.com/rust-osdev/xhci/pull/145))</span>

Thanks to [@Demindiro](https://github.com/Demindiro) for their contribution!


### [`pci_types`](https://github.com/rust-osdev/pci_types)

<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>


The `pci_types` library provides types for accessing and configuring PCI devices from Rust operating systems.

We merged the following change in August:

- [64-bit `BAR` clarification](https://github.com/rust-osdev/pci_types/pull/6)

Thanks to [@0Killian](https://github.com/0Killian) for this contribution!


### [`acpi`](https://github.com/rust-osdev/acpi)

<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods), [@Restioson](https://github.com/Restioson), and [@Gegy](https://github.com/Gegy)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS.

We merged the following changes this month:


- [fix `HpetInfo::num_comparators`](https://github.com/rust-osdev/acpi/pull/121)
- [Various comments, fix `PM_TMR_BLK` checking of `PM_TMR_LEN` to match spec](https://github.com/rust-osdev/acpi/pull/123)

Thanks to [@semiviral](https://github.com/semiviral), and [@Freax13](https://github.com/Freax13) for their contributions!


## Call for Participation

Want to contribute to a Rust OSDev project, but don't know where to start? Help with one of these outstanding issues!

<!--
    Please use the following template for adding items:
    - [(`repo_name`) Issue Description](https://example.com/link-to-issue)
-->

If you maintain a Rust project related to operating system development and are looking for contributors, especially for tasks suited to people getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the `next` branch with the tasks you want to include in the next issue.

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`cdrzewiecki/celos`](https://gitlab.com/cdrzewiecki/celos)
<span class="maintainers">(Section written by [@drzewiec](https://github.com/drzewiec))</span>

It's been a while since my last project update! That's partly because life has been busy, but also because this update concerns a huge feature. I'm very pleased to report that I have been able to add preemptive multitasking to CelOS.

This was feature that took a good bit of foundation to be able to implement (hence why it took me so long). I had to spend a good bit of time getting memory allocation (both physical and virtual) into a happier place, as well as add support for ACPI and the APIC. And, of course, there were many snags along the way as I learned (at least some of) the traps that are easy to step into when doing something as delicate as context switching.

Now that I have finished this key feature, I plan to work on setting up the other infrastructure needed to begin writing services in userspace (such as message passing and synchronization primitives). And, hopefully soon, finally make the jump into ring 3!

As always, many thanks to [@phil-opp](https://github.com/phil-opp) for his hard work on supporting the Rust osdev community, and for writing the [apic](https://github.com/rust-osdev/apic) crate which helped serve as a sanity check while I wrote my own driver for the IOAPIC and LAPIC. Thanks as well to the maintainers of the excellent [acpi](https://github.com/rust-osdev/acpi) crate, you guys are doing incredible work out there!

### Blog Post: GNU ld Discards Section Containing Code – Section Flags in Assembly are Important

<span class="maintainers">(Section written by [@phip1611](https://github.com/phip1611))</span>

In late August/early September, I encountered problems when building my Rust kernel. I faced
unintuitive interaction between my global assembly code and the linker. I specified a custom
section in assembly with executable code with `.section .bootcode`. The linker never linked
the code where I specified it in my linker script. It's address was not what it is supposed to be.
`readelf` didn't show the section inside the binary either. The section was discarded no matter
how hard I tried to modify the linker, thus, `KEEP((.bootcode));` also didn't work. An experienced
colleague ensured me that my linker script is correct.

Section names such as `.init` or `.text.bootcode` worked by the way. Only my custom name was
rejected somehow. In the end, I figured out writing `.section .bootcode, "ax"` does the trick. The
difference is small, but the impact to the object file and final executable of those section flags
is big. I could find the answer in the ELF specification. A section needs to be allocatable
(`a`-flag) so that it can be properly placed in a LOAD segment/program header. The section names
`.init` and `.text.*` have this pre-configured but my custom section name `.bootcode` has not.

I traced it down to a minimal reproducible example that can be found [on GitHub](https://github.com/phip1611/gnu-linker-discards-code-section-that-is-not-in-text-section).
A comprehensive write-up can be found on my website [Phip's Blog](https://phip1611.de/blog/gnu-ld-discards-section-containing-code/).

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="maintainers">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

This month, we merged a pull request that fixes numerous grammar and punctuation errors across all posts of the [_"Writing an OS in Rust"_](https://os.phil-opp.com/) series. Thanks to [@hecatia-elegua](https://github.com/hecatia-elegua) for this contribution!

We also received multiple pull requests to add and update translations:

- [Fix Japanese translation of "Double Faults"](https://github.com/phil-opp/blog_os/pull/1127)
- [Update the chinese translation](https://github.com/phil-opp/blog_os/pull/1131)
- [[Translation][Korean] post-04](https://github.com/phil-opp/blog_os/pull/1135)

We are still looking for reviewers for the last two PRs. If you speak Chinese or Korean, it would be great if you could take a look!

Thanks to [@ykomatsu](https://github.com/ykomatsu), [@liuyuran](https://github.com/liuyuran), and [@JOE1994](https://github.com/JOE1994) for contributing these translations!

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).



<!--
TODO: Update publication date
-->
