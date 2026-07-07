+++
title = "This Month in Rust OSDev: June 2026"
date = 2026-07-05

[extra]
month = "June 2026"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

Please submit interesting posts and projects for the next issue [on Zulip](https://rust-osdev.zulipchat.com/#narrow/channel/435142-newsletter/topic/Content.20suggestions/with/580172810) or via a PR [on GitHub](https://github.com/rust-osdev/homepage/).

<span class="gray">
Disclaimer: Automated scripts and AI assistance were used for collecting and categorizing links.
Everything was proofread and checked manually, with many manual tweaks.
</span>


<!--
    This is a draft for the upcoming "This Month in Rust OSDev (June 2026)" post.
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
- [Announcing Asterinas 0.18.0](https://asterinas.github.io/2026/06/04/announcing-asterinas-0.18.0.html)
- [This Month in Redox - May 2026](https://www.redox-os.org/news/this-month-260531/)
  - The latest Redox update covers an EEVDF scheduler, page flipping and plane support in the Intel graphics driver, a COSMIC monitor and XFCE port, and large I/O and RedoxFS performance improvements.
- [Zinnia](https://zinnia-os.org/)
  - A Unix-like kernel written almost entirely in Rust that boots on real x86_64 hardware and can run Wayland/X11 desktop sessions (Weston, XFCE). Drivers are loaded as modular Rust ELF libraries at boot.
- [LearnixOS](https://www.learnix-os.com/)
  - A book that teaches OS development from scratch in Rust, covering memory allocators, paging, filesystems, and kernel logic.
- [Hardware Is Asynchronous. Most of Our Operating Systems Still Aren't.](https://vorjdux.com/articles/hardware-is-async.html)
  - An article arguing that operating systems should treat asynchrony as the default rather than layering it on top of blocking abstractions, drawing on work from [CharlotteOS](https://github.com/charlotte-os/charlotte-os), an experimental modern OS written in Rust.
- [Building an AsyncIO executor for the 3DS (pt 1!)](https://blog.cat-girl.gay/3ds-async-part-one/)
  - Building a from-scratch async I/O executor for the Nintendo 3DS.


## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [Move `std::io::Error` into `core`](https://github.com/rust-lang/rust/pull/155625)
  - Makes I/O error handling available in `no_std`
  - Now the rest of `std::io` can be moved to `core`/`alloc` too, including the `Read` and `Write` traits. See the [open PR](https://github.com/rust-lang/rust/pull/156527) and the [tracking issue](https://github.com/rust-lang/rust/issues/154046) for details.
- [Stabilize `int_format_into` feature](https://github.com/rust-lang/rust/pull/152544)
  - Formats integers into fixed-size buffers with no allocation.
- [Stabilize `#![feature(box_as_ptr)]`](https://github.com/rust-lang/rust/pull/157876)
  - Stabilizes `Box::as_ptr` and `Box::as_mut_ptr`
  - These methods can be used to create multiple pointers to the same `Box` that don't invalidate each other.
- [staticlib: hide internal symbols](https://github.com/rust-lang/rust/pull/155338)
  - A new `-Zstaticlib-hide-internal-symbols` flag hides non-exported Rust symbols in static libraries, shrinking binaries by 5–12% in non-LTO builds.
- [`asm!` support for the Xtensa architecture](https://github.com/rust-lang/rust/pull/147302)
  - Inline assembly for Xtensa (ESP32 and related chips) lands in-tree after years in the esp-rs fork.

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
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [Feat: IoMmu protocol](https://github.com/rust-osdev/uefi-rs/pull/1728)
- [uefi: Add PciRootBridgeIo memory and I/O space access](https://github.com/rust-osdev/uefi-rs/pull/1958)
- [uefi: Add PciRootBridgeIo attribute manipulation](https://github.com/rust-osdev/uefi-rs/pull/1965)
- [uefi-raw: Add PciRootBridgeIoProtocolAttributes](https://github.com/rust-osdev/uefi-rs/pull/1957)
- [uefi: Add integration with `time` crate](https://github.com/rust-osdev/uefi-rs/pull/1955)
- [uefi: Add integration with `jiff` crate](https://github.com/rust-osdev/uefi-rs/pull/1956)
- [uefi: remove deprecated APIs](https://github.com/rust-osdev/uefi-rs/pull/1966)
- [uefi-raw: enhance Boolean type and make it more type safe](https://github.com/rust-osdev/uefi-rs/pull/1974)
- [uefi-raw: various spec fixes](https://github.com/rust-osdev/uefi-rs/pull/1962)
- [uefi: Fix `boot::exit` signature](https://github.com/rust-osdev/uefi-rs/pull/1959)
- [uefi: reject undersized device path nodes](https://github.com/rust-osdev/uefi-rs/pull/1979)
- [uefi: various smaller memory map related fixes and improvements](https://github.com/rust-osdev/uefi-rs/pull/1980)
- [uefi-raw: format negative time zones correctly](https://github.com/rust-osdev/uefi-rs/pull/1982)
- [release: uefi-raw-0.15.0, uefi-0.38.0](https://github.com/rust-osdev/uefi-rs/pull/1984)

<!-- - [chore(deps): update crate-ci/typos action to v1.47.2](https://github.com/rust-osdev/uefi-rs/pull/1961) -->
<!-- - [chore(deps): update codecov/codecov-action action to v6.0.2](https://github.com/rust-osdev/uefi-rs/pull/1960) -->
<!-- - [nix: simplify nix code & update](https://github.com/rust-osdev/uefi-rs/pull/1963) -->
<!-- - [uefi-raw: fix typo in PciRootBridgeIoProtocolAttributes](https://github.com/rust-osdev/uefi-rs/pull/1964) -->
<!-- - [ci: update QEMU for windows runners](https://github.com/rust-osdev/uefi-rs/pull/1969) -->
<!-- - [treewide: allow() -> expect() + remove unneeded allow()](https://github.com/rust-osdev/uefi-rs/pull/1968) -->
<!-- - [xtask: update OVMF from EDK2-STABLE202502 to EDK2-STABLE202605](https://github.com/rust-osdev/uefi-rs/pull/1970) -->
<!-- - [ci: use ubuntu-26.04 (to update QEMU)](https://github.com/rust-osdev/uefi-rs/pull/1976) -->
<!-- - [chore(deps): update rust crate time to v0.3.47 [security]](https://github.com/rust-osdev/uefi-rs/pull/1977) -->
<!-- - [Remove duplications in `uefi` and `uefi-raw`](https://github.com/rust-osdev/uefi-rs/pull/1967) -->
<!-- - [uefi: streamline + fix usize_from_u32()](https://github.com/rust-osdev/uefi-rs/pull/1981) -->
<!-- - [uefi: minor improvements to device path protocol code](https://github.com/rust-osdev/uefi-rs/pull/1983) -->
<!-- - [uefi: minor adjustments regarding previous commits](https://github.com/rust-osdev/uefi-rs/pull/1985) -->
<!-- - [treewide: enforce safety comments](https://github.com/rust-osdev/uefi-rs/pull/1988) -->
<!-- - [remove multilingual field from book.toml](https://github.com/rust-osdev/uefi-rs/pull/1992) -->

Thanks to [@JarlEvanson](https://github.com/JarlEvanson), [@mysteriouslyseeing](https://github.com/mysteriouslyseeing), and [@PelleKrab](https://github.com/PelleKrab) for their contributions!

### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

_Convenient and safe parsing of Multiboot2 Boot Information (MBI) structures and
the contained information tags. Usable in no_std environments, such as a kernel.
An optional builder feature also allows the construction of the corresponding
structures._

We merged the following PRs this month:

- [Rewrite: Add elf library for elf_sections.rs](https://github.com/rust-osdev/multiboot2/pull/292)
- [various safety and correctness improvements](https://github.com/rust-osdev/multiboot2/pull/301)
- [Various improvements and fixes](https://github.com/rust-osdev/multiboot2/pull/300)
- [modernize misc stuff](https://github.com/rust-osdev/multiboot2/pull/296)
- [elf: remove From impl](https://github.com/rust-osdev/multiboot2/pull/299)
- [chore: prepare new workspace releases](https://github.com/rust-osdev/multiboot2/pull/302)

<!-- - [build(deps): bump crate-ci/typos from 1.46.0 to 1.47.0](https://github.com/rust-osdev/multiboot2/pull/294) -->
<!-- - [build(deps): bump crate-ci/typos from 1.47.0 to 1.47.2](https://github.com/rust-osdev/multiboot2/pull/298) -->
<!-- - [build(deps): bump bitflags from 2.11.0 to 2.13.0](https://github.com/rust-osdev/multiboot2/pull/297) -->

Thanks to [@an-owl](https://github.com/an-owl) for their contribution!

### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. The following changes landed in early July, but we want to mention them already because they affect users building against recent Rust nightlies:

- Recent Rust nightlies renamed the `x86-softfloat` target ABI to `softfloat`. This update adjusts the bootloader's custom targets accordingly, on both the latest release and the `v0.9` branch.
  - [Change `rustc-abi` in custom targets from `x86-softfloat` to `softfloat`](https://github.com/rust-osdev/bootloader/pull/569)
  - [(v0.9) Change `rustc-abi` in custom targets from `x86-softfloat` to `softfloat`](https://github.com/rust-osdev/bootloader/pull/568)
- [uefi: bump from 0.20 to 0.38](https://github.com/rust-osdev/bootloader/pull/566)

<!-- - [deps: bump uart_16550 to 0.6.0](https://github.com/rust-osdev/bootloader/pull/565) -->
<!-- - [janked dependencies: bump critical-section, atomic-polyfill, heapless](https://github.com/rust-osdev/bootloader/pull/567) -->

Thanks to [@phip1611](https://github.com/phip1611) for their contribution!

### [`ovmf-prebuilt`](https://github.com/rust-osdev/ovmf-prebuilt)
<span class="maintainers">Maintained by [@nicholasbishop](https://github.com/nicholasbishop) and [@phil-opp](https://github.com/phil-opp)</span>

The `ovmf-prebuilt` project provides pre-built [edk2](https://github.com/tianocore/edk2) releases to make it easier to set up OVMF. We merged the following changes this month:

- [Fix build errors for edk2-stable202605](https://github.com/rust-osdev/ovmf-prebuilt/pull/312)
- [Update to sha2-0.11](https://github.com/rust-osdev/ovmf-prebuilt/pull/313)
- [release: 0.2.9 with edk2-stable202605-r1](https://github.com/rust-osdev/ovmf-prebuilt/pull/314)

<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/309) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/310) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/315) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/316) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/317) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/320) -->

### [`uart_16550`](https://github.com/rust-osdev/uart_16550)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

Simple yet highly configurable low-level driver for 16550 UART devices,
typically known and used as serial ports or COM ports.

We merged the following change this month:

- [lib: change signature of Uart16550::config() to be more flexible](https://github.com/rust-osdev/uart_16550/pull/61)

<!-- - [build(deps): bump crate-ci/typos from 1.46.0 to 1.47.1](https://github.com/rust-osdev/uart_16550/pull/59) -->
<!-- - [build(deps): bump bitflags from 2.11.0 to 2.12.1](https://github.com/rust-osdev/uart_16550/pull/60) -->

<!--
    x86_64 had only dependency/chore updates this month:
    - x86_64 #590 (Bump actions/checkout from 6 to 7)
-->

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

### [`mkroening/elf-symbols`](https://github.com/mkroening/elf-symbols)
<span class="maintainers">(Section written by [@mkroening](https://github.com/mkroening))</span>

When developing an OS, you often need some information about the loaded kernel image:

- Where has the loader loaded the kernel to?
- How large is the loaded kernel?
- Where do the text segment and the data segment end?
- How do I get the kernel TLS image?

These questions can be answered by building non-relocatable images, by writing custom linker scripts, or by having a custom loader that provides this information somehow.

But there is another way!
In fact, the main ELF linkers that I am aware of ([BFD], [gold], [LLD], [mold], and [Wild]) all have built-in ELF symbols that answer these questions when not using custom linker scripts.
Unfortunately, many are poorly documented, so I created the [elf-symbols] crate that exposes and documents them.
All symbols are tested on the aforementioned linkers.
The documentation shows when each linker gained support for the respective symbol.

The following symbols are straightforward:

- `__executable_start` (`executable_start()`) is the start of the executable.
- `_etext` (`text_end()`) is the end of the text segment.
- `_edata` (`data_end()`) is the end of the data segment.
- `_end` (`executable_end()`) is the end of the executable.

`__ehdr_start` (`elf_header()`) is especially interesting.
It allows programs to examine themselves by reading their ELF headers (file headers and program headers).
This can be used to get TLS image information, for example.

Note that these symbols are ELF specific, though, so they cannot be used when linking to something else, such as a PE32+ UEFI executable.

#### Examples

```rust
println!("Executable start: {:p}", elf_symbols::executable_start());
println!("ELF header:       {:p}", elf_symbols::elf_header());
println!("Text segment end: {:p}", elf_symbols::text_end());
println!("Data segment end: {:p}", elf_symbols::data_end());
println!("Executable end:   {:p}", elf_symbols::executable_end());
```

[BFD]: https://sourceware.org/git/?p=binutils-gdb.git;a=tree;f=ld;h=b1662159cdd15bb857e04e42bd26361c0d406099;hb=5e56594815854de5eca35c7c04b11705d0f19c02
[gold]: https://sourceware.org/git/?p=binutils-gdb.git;a=tree;f=gold;h=ac6272c7bb3ad02524b2ca86a2cf9b68e9ca30ca;hb=5e56594815854de5eca35c7c04b11705d0f19c02
[LLD]: https://github.com/llvm/llvm-project/tree/llvmorg-22.1.8/lld
[mold]: https://github.com/rui314/mold/tree/v2.41.0
[Wild]: https://github.com/wild-linker/wild/tree/0.9.0
[elf-symbols]: https://crates.io/crates/elf-symbols

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)
<span class="maintainers">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

We merged the following changes to the [_Writing an OS in Rust_](https://os.phil-opp.com/) blog. These landed in early July, but we mention them here since they keep the code building on the latest Rust nightly:

- [Fix target spec: use `softfloat` instead of `x86-softfloat`](https://github.com/phil-opp/blog_os/pull/1484)
- [Fix blog: the `x86-softfloat` feature was renamed to `softfloat`](https://github.com/phil-opp/blog_os/pull/1485)
- [Update for new Rust error messages](https://github.com/phil-opp/blog_os/pull/1486)
- [Update blog to zola 0.22.1](https://github.com/phil-opp/blog_os/pull/1487)

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way to get in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
