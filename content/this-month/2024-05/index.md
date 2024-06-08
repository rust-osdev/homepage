+++
title = "This Month in Rust OSDev: May 2024"
date = 2024-06-08

[extra]
month = "May 2024"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (May 2024)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Announcements, News, and Blog Posts

Here we collect news, blog posts, etc. related to OS development in Rust.

- [This Month in Redox - May 2024](https://www.redox-os.org/news/this-month-240531/): Redox is running COSMIC file manager, editor, and terminal now
- Bachelor's Thesis: [Writing an NVMe Driver in Rust](https://db.in.tum.de/~ellmann/theses/finished/24/pirhonen_writing_an_nvme_driver_in_rust.pdf) (PDF)
- [Building an Async Runtime for the Windows Kernel](https://github.com/carlos-al/windows-kernel-rs)
- [Ferrocene 24.05.0 now available for purchase](https://ferrous-systems.com/blog/ferrocene-24-05-0/)
- [Rust 1.78: Performance Impact of the 128-bit Memory Alignment Fix](https://codspeed.io/blog/rust-1-78-performance-impact-of-the-128-bit-memory-alignment-fix)
- [GxHash - an extremely fast hardware-accelerated non-cryptographic hashing algorithm](https://github.com/ogxd/gxhash) (zero dependencies, no_std compatible)
- The Embedded Rustacean [Issue #19](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-19), [Issue #20](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-20), and [Issue #21](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-21)

<!--
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->


## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

- [Stabilize `LazyCell` and `LazyLock`](https://github.com/rust-lang/rust/pull/121377) (`LazyCell` is available in `no_std`)
- [Stabilize `error_in_core`](https://github.com/rust-lang/rust/pull/125951)
- [Add `x86_64-unknown-linux-none` target](https://github.com/rust-lang/rust/pull/125023) (freestanding linux binaries without `libc` dependency)
- [Add `opt-for-size` core lib feature flag](https://github.com/rust-lang/rust/pull/125011)
- [Implement feature `integer_sign_cast`](https://github.com/rust-lang/rust/pull/125884)

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

### [`endian-num`](https://github.com/rust-osdev/endian-num) (new project!)
<span class="maintainers">Maintained by [@mkroening](https://github.com/mkroening)</span>

The `endian-num` crate provides the `Be` (big-endian) and `Le` (little-endian) byte-order-aware numeric types.

- [initial implementation](https://github.com/rust-osdev/endian-num/commit/000f86f5470401e4d0d8824ec976738fb8a35bb7)
- [docs: elaborate on differences to other crates](https://github.com/rust-osdev/endian-num/pull/1)
- [docs: refer to related crates via docs.rs](https://github.com/rust-osdev/endian-num/pull/2)

Thanks to [@mkroening](https://github.com/mkroening) for creating and maintaining this crate!


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

The `uefi-rs` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. We merged the following PRs this month:

- [Fix risc target_arch cfg](https://github.com/rust-osdev/uefi-rs/pull/1159)
- [Match MaximumCapsuleSize to UEFI spec](https://github.com/rust-osdev/uefi-rs/pull/1161)
- [Add RuntimeServices::update_capsule](https://github.com/rust-osdev/uefi-rs/pull/1162)
- [Add RuntimeServices::query_capsule_capabilities](https://github.com/rust-osdev/uefi-rs/pull/1166)
- [Note about feature flags for uefi book](https://github.com/rust-osdev/uefi-rs/pull/1168)
- [uefi-raw: misc](https://github.com/rust-osdev/uefi-rs/pull/1173)
- [mem: clarify confusion around MemoryDescriptor](https://github.com/rust-osdev/uefi-rs/pull/1174)
- [uefi/helpers: logger logs to debugcon device](https://github.com/rust-osdev/uefi-rs/pull/1172)
- [Add basic API for a global system table](https://github.com/rust-osdev/uefi-rs/pull/1156)
- [uefi: BootServices::allocate_pool now returns NonZero<u8> instead of *mut u8](https://github.com/rust-osdev/uefi-rs/pull/1176)
- [Fix uefi-macros trybuild test](https://github.com/rust-osdev/uefi-rs/pull/1183)

<!-- - [chore(deps): update crate-ci/typos action to v1.21.0](https://github.com/rust-osdev/uefi-rs/pull/1158) -->
<!-- - [chore(deps): update rust crate trybuild to v1.0.93](https://github.com/rust-osdev/uefi-rs/pull/1157) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1165) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1171) -->
<!-- - [fix(deps): update rust crate anyhow to v1.0.86](https://github.com/rust-osdev/uefi-rs/pull/1164) -->
<!-- - [fix(deps): update rust crate itertools to 0.13.0](https://github.com/rust-osdev/uefi-rs/pull/1179) -->
<!-- - [fix(deps): update rust crate nix to 0.29.0](https://github.com/rust-osdev/uefi-rs/pull/1180) -->
<!-- - [chore(deps): update cachix/install-nix-action action to v27](https://github.com/rust-osdev/uefi-rs/pull/1181) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1182) -->

Thanks to [@stillinbeta](https://github.com/stillinbeta) and [@andre-braga](https://github.com/andre-braga) for their contributions!


### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following improvements:

- [Fix doc comment and error message only referencing the BIOS but used for UEFI](https://github.com/rust-osdev/bootloader/pull/439)
- [Ensure all page table frames are mapped as writable](https://github.com/rust-osdev/bootloader/pull/444)

Thanks to [@fmckeogh](https://github.com/fmckeogh) and [@Wasabi375](https://github.com/Wasabi375) for their contributions!


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PRs this month:

- [fix cfg related warnings](https://github.com/rust-osdev/x86_64/pull/485)
- [merge master into next](https://github.com/rust-osdev/x86_64/pull/486)
- [add Mapper::clear to clear any page table entry regardless of present flag](https://github.com/rust-osdev/x86_64/pull/484)
- [fix warnings](https://github.com/rust-osdev/x86_64/pull/488)

Thanks to [@Wasabi375](https://github.com/Wasabi375) for their contribution!


### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

The `multiboot2` crate provides abstraction types for the multiboot information structure (MBI) of multiboot2 bootloaders. We merged the following changes this month:

- [multiboot2: builder: Allow to specify SMBIOS tag multiple times](https://github.com/rust-osdev/multiboot2/pull/210)
- [dev: misc improvements](https://github.com/rust-osdev/multiboot2/pull/213)
- [release](https://github.com/rust-osdev/multiboot2/pull/214)
- [multiboot2: fix handling of efi memory map](https://github.com/rust-osdev/multiboot2/pull/216)

<!-- - [build(deps): bump crate-ci/typos from 1.19.0 to 1.21.0](https://github.com/rust-osdev/multiboot2/pull/211) -->

Thanks to [@YtvwlD](https://github.com/YtvwlD) for their contribution!


### [`linked-list-allocator`](https://github.com/rust-osdev/linked-list-allocator)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@jamesmunns](https://github.com/jamesmunns)</span>

The `linked-list-allocator` crate provides a basic `no_std` allocator that builds a linked list from freed memory blocks and thus needs no additional data structures. We merged the following PR this month:

- [Fix warnings about `cfg(fuzzing)`](https://github.com/rust-osdev/linked-list-allocator/pull/82)


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


### [`mkroening/free-list`](https://github.com/mkroening/free-list)
<span class="maintainers">(Section written by [@mkroening](https://github.com/mkroening))</span>

The `free-list` crate provides the `FreeList` type for managing virtual or physical memory.
Opposed to normal memory allocators, `FreeList` does not use pointers but page ranges.
It operates by keeping a list of free page ranges (hence the name) and allows allocating at user-provided ranges.
Instead of operating directly on the unallocated memory through a linked list, this free list uses statically allocated memory before dynamically allocating more memory to hold its elements.

```rust
use free_list::{FreeList, PageLayout};

let mut free_list = FreeList::<16>::new();

unsafe {
    free_list.deallocate((0x1000..0x5000).try_into().unwrap()).unwrap();
}
assert_eq!(free_list.free_space(), 0x4000);

let layout = PageLayout::from_size(0x4000).unwrap();
assert_eq!(free_list.allocate(layout).unwrap(), (0x1000..0x5000).try_into().unwrap());
```


## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
