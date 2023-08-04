+++
title = "This Month in Rust OSDev: June 2023"
date = 2023-07-06

[extra]
month = "June 2023"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (June 2023)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Announcements, News, and Blog Posts

Here we collect news, blog posts, etc. related to OS development in Rust.

- [Redox OS elects its first Board of Directors](https://www.redox-os.org/news/board-meeting-2023-06-21/)
- [Frida 16.1.0 Released](https://frida.re/news/2023/06/23/frida-16-1-0-released/)
- [Intercepting Allocations with the Global Allocator](https://bd103.github.io/blog/2023-06-27-global-allocators/)
- [Building an out-of-tree Rust Kernel Module Part Three](https://blog.rnstlr.ch/building-an-out-of-tree-rust-kernel-module-part-three.html)
- Discussion: [How hard would it be to port the Rust toolchain to a new non-POSIX OS written in Rust and get it to host its own development?](https://www.reddit.com/r/rust/comments/14qvu98/how_hard_would_it_be_to_port_the_rust_toolchain/)
- Discussion: [Will there be fewer jailbreaks in the future if operating systems like iOS start adopting rust?](https://www.reddit.com/r/rust/comments/14k49ho/will_there_be_fewer_jailbreaks_in_the_future_if/)

<!--
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->


## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

    ### Summary
    <span class="maintainers">(Section written by [@author](https://github.com/author))</span>

    <text>
-->

_No updates were proposed for this section this month._


## `rust-osdev` Projects

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

<!--
    Please use the following template:

    ### [`repo_name`](https://github.com/rust-osdev/repo_name)
    <span class="maintainers">Maintained by [@maintainer_1](https://github.com/maintainer_1)</span>

    The `repo_name` crate ...<<short introduction>>...

    We merged the following changes this month:
    <<changelog, either in list or text form>>
-->


### [`volatile`](https://github.com/rust-osdev/volatile)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `volatile` crate provides a safe wrapper type for implementing volatile read and write operations. This is useful for accessing memory regions that have side-effects, such as memory-mapped hardware registers or framebuffers.

Unfortunately, the design crate had a [soundness issue](https://github.com/rust-osdev/volatile/pull/13#issuecomment-842455552) because it used reference types for accessing the volatile memory. This is problematic because the Rust compiler marks references are _"dereferencable"_, which allows LLVM to insert spurious read operations. This is fine for accessing "normal" memory, but it can lead to correctness issues when used in combination with memory-mapped device registers. These registers look like normal memory, but they are actually accessing some device-specific registers, which might change at any time. So they might change between spurious reads, which violates the assumptions of LLVM and can lead to undefined behavior.

To solve this issue, we started [a full redesign of the crate](https://github.com/rust-osdev/volatile/pull/22) that uses raw pointers only. This solves the issue because raw pointers are not considered _"dereferencable"_, so LLVM is not allowed to insert spurious reads. While we started working on the new design more than 2 years ago, we didn't merge it until this month because we weren't sure about the implementation details. The main discussion point was whether we should treat the proposed `VolatilePointer` type like Rust's reference types or like Rust's raw pointer types. The difference is that raw pointers implement the `Copy` trait, but are not `Send` to prevent aliasing. References, on the other hand, can safely implement `Send` because they're only `Copy` if the reference is read-only.

After a lot of back and forth, we decided to [provide both options](https://github.com/rust-osdev/volatile/pull/29) and finally publish [**`volatile v0.5`**](https://docs.rs/volatile/0.5.1/volatile/index.html). So the new design has two wrapper types, [`VolatilePtr`](https://docs.rs/volatile/0.5.1/volatile/struct.VolatilePtr.html) (behaves like a raw pointer) and [`VolatileRef`](https://docs.rs/volatile/0.5.1/volatile/struct.VolatileRef.html) (behaves like a reference). We hope that we support most use cases this way!

Note that there is also some ongoing discussion [about a potential `VolatileCell` type](https://github.com/rust-osdev/volatile/issues/31) to wrap values in-place. Most implementations of such a type would require support from the Rust compiler, which [needs an RFC](https://github.com/rust-lang/unsafe-code-guidelines/issues/411). However, there is one [promising design based on zero-sized types and proc-macros](https://github.com/rust-osdev/volatile/issues/31#issuecomment-1606044353) by [@Freax13](https://github.com/Freax13) that should not require any new language features. We will continue to investigate.

### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

The [multiboot2](https://crates.io/crates/multiboot2) and the [multiboot2-header](https://crates.io/crates/multiboot2)
crates got a big overhaul. The list of new features includes but is not limited to:
- use DSTs for tags where applicable ([#134](https://github.com/rust-osdev/multiboot2/pull/134))
- model the MBI as DST ([#155](https://github.com/rust-osdev/multiboot2/pull/155))
- add a runtime builder for an MBI ([#133](https://github.com/rust-osdev/multiboot2/pull/133)) \
  Huge thanks to [YtvwlD / Niklas](https://github.com/YtvwlD) for this greate external contribution
- added an integration test including a multiboot2 chainloader for better test coverage ([#129](https://github.com/rust-osdev/multiboot2/pull/129))
- added miri to the CI for more memory safety ([#128](https://github.com/rust-osdev/multiboot2/pull/128))
- several fixes and small improvements

`multiboot2` was updated from `0.15.1` to `0.16.0` and `multiboot2-header` was updated from `0.2.0` to `0.3.0`. Both 
releases come with a large amount of [breaking changes](https://github.com/rust-osdev/multiboot2/blob/main/Changelog.md). 
However, after a sensible consideration, they are all worth it for a more streamlined API and more memory safety.

[`rust-osdev`]: https://github.com/rust-osdev/about

<details>

<summary>Merged pull requests:</summary>

- [Add a builder to multiboot2](https://github.com/rust-osdev/multiboot2/pull/133)
- [treewide: code improvements and other stuff](https://github.com/rust-osdev/multiboot2/pull/141)
- [various fixes + cleanup + bump crate versions](https://github.com/rust-osdev/multiboot2/pull/143)
- [streamline default derives](https://github.com/rust-osdev/multiboot2/pull/144)
- [tree-wide: rename builder structs (remove Multiboot2 prefix)](https://github.com/rust-osdev/multiboot2/pull/146)
- [tree-wide: streamline default derives (also Hash everywhere)](https://github.com/rust-osdev/multiboot2/pull/147)
- [workspace: use workspace dependencies](https://github.com/rust-osdev/multiboot2/pull/148)
- [fix memory issue in memory-map](https://github.com/rust-osdev/multiboot2/pull/149)
- [multiboot2: memory-map: derive Eq + uefi-raw@0.3.0](https://github.com/rust-osdev/multiboot2/pull/150)
- [multiboot2: more tag name streamlining](https://github.com/rust-osdev/multiboot2/pull/151)
- [multiboot2: fix memory issue in boxed_dst_tag](https://github.com/rust-osdev/multiboot2/pull/152)
- [multiboot2: builder: add terminating null-bytes to tags that hold a string](https://github.com/rust-osdev/multiboot2/pull/153)
- [multiboot2: load: remove odd offset thingy](https://github.com/rust-osdev/multiboot2/pull/142)
- [various cleanups](https://github.com/rust-osdev/multiboot2/pull/154)
- [multiboot2: model MBI as DST](https://github.com/rust-osdev/multiboot2/pull/155)
- [various fixes](https://github.com/rust-osdev/multiboot2/pull/157)
- [builder: use new abstraction to guarantee alignment](https://github.com/rust-osdev/multiboot2/pull/156)
- [Add Integration Test](https://github.com/rust-osdev/multiboot2/pull/129)
- [multiboot2: create DSTs: hopefully better memory fix](https://github.com/rust-osdev/multiboot2/pull/158)
- [integration-test: cargo update](https://github.com/rust-osdev/multiboot2/pull/159)
- [ci: run miri + adjustments for miri](https://github.com/rust-osdev/multiboot2/pull/128)
- [ci: integrationtest: use magic nix cache](https://github.com/rust-osdev/multiboot2/pull/161)
- [Make InformationBuilder adhere the API guidelines.](https://github.com/rust-osdev/multiboot2/pull/162)

Thanks to [@YtvwlD](https://github.com/YtvwlD) and [@scholzp](https://github.com/scholzp) for their contributions!

</details>

### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following changes this month:

- [Change Star::write() to use checked subtractions](https://github.com/rust-osdev/x86_64/pull/422)
- [Bump bitflags to 2.3.2](https://github.com/rust-osdev/x86_64/pull/426)
- [add workaround for recursive page tables with recursive index 511](https://github.com/rust-osdev/x86_64/pull/425)
- [Fix off-by-one in documentation](https://github.com/rust-osdev/x86_64/pull/427)

Thanks to [@Qix-](https://github.com/Qix-), [@grant0417](https://github.com/grant0417), and [@Egggggg](https://github.com/Egggggg) for their contributions!


### [`vga`](https://github.com/rust-osdev/vga)
<span class="maintainers">Maintained by [@RKennedy9064](https://github.com/RKennedy9064)</span>

The work-in-progress `vga` crate allows the configuration of the VGA hardware, e.g. switching from text-based mode to a pixel-based graphics mode. This month, we merged the following pull request:

- [implement a draw_rect function](https://github.com/rust-osdev/vga/pull/35)

Thanks to [@tsatke](https://github.com/tsatke) for their contribution!


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), 
the successor to the BIOS firmware. It empowers everyone to write EFI-
applications with Rust in a convenient way, such as your own bootloader.

In June, we added multiple small improvements for developers using 
[Nix(OS)](https://nixos.org) and simplifications for working with device 
paths. Additionally, we moved more definitions to the new `uefi-raw` crate.

Furthermore, we'd like to mention the new high-level [File-System API](https://docs.rs/uefi/0.24.0/uefi/fs/index.html). 
It was [merged in April](https://github.com/rust-osdev/uefi-rs/issues/747) 
actually, but not mentioned in this newsletter so far. Feel free to give it a 
try!

We merged the following PRs this month:

- [Release uefi-raw-0.2.0, uefi-0.22.0, and uefi-services-0.19.0](https://github.com/rust-osdev/uefi-rs/pull/840)
- [Fix various CI failures](https://github.com/rust-osdev/uefi-rs/pull/844)
- [uefi: Fix wrong install_configuration_table() signature](https://github.com/rust-osdev/uefi-rs/pull/843)
- [Release uefi-0.23.0 and uefi-services-0.20.0](https://github.com/rust-osdev/uefi-rs/pull/846)
- [build(deps): bump tempfile from 3.5.0 to 3.6.0](https://github.com/rust-osdev/uefi-rs/pull/848)
- [device_path: add more convenience (part 2)](https://github.com/rust-osdev/uefi-rs/pull/849)
- [Make UEFI shell protocols testable](https://github.com/rust-osdev/uefi-rs/pull/793)
- [Add DevicePathProtocol to uefi-raw](https://github.com/rust-osdev/uefi-rs/pull/850)
- [Integration test for boot service function load_image](https://github.com/rust-osdev/uefi-rs/pull/826)
- [cargo: Use `[workspace.package]` to deduplicate metadata](https://github.com/rust-osdev/uefi-rs/pull/853)
- [uefi: Update MSRV policy language](https://github.com/rust-osdev/uefi-rs/pull/852)
- [uefi: Remove some uses of MaybeUninit in BootServices](https://github.com/rust-osdev/uefi-rs/pull/854)
- [nix: add more convenience for Nix/NixOS users](https://github.com/rust-osdev/uefi-rs/pull/828)
- [ci: Fix developer_productivity push error](https://github.com/rust-osdev/uefi-rs/pull/855)
- [uefi-raw: Add BootServices table](https://github.com/rust-osdev/uefi-rs/pull/856)
- [uefi: Add raw pointer Event/Handle methods](https://github.com/rust-osdev/uefi-rs/pull/858)
- [uefi: Make BootServices fn ptrs unsafe](https://github.com/rust-osdev/uefi-rs/pull/857)
- [xtask: add "cargo xtask fmt" (and formatting for nix and yml files)](https://github.com/rust-osdev/uefi-rs/pull/757)
- [build(deps): bump ureq from 2.6.2 to 2.7.0](https://github.com/rust-osdev/uefi-rs/pull/860)
- [uefi-raw: Fix arg type in connect_controller](https://github.com/rust-osdev/uefi-rs/pull/862)
- [uefi: Improve support for null protocol interfaces](https://github.com/rust-osdev/uefi-rs/pull/861)
- [uefi: Make `BootServices` a wrapper around `uefi_raw::table::boot::BootServices`](https://github.com/rust-osdev/uefi-rs/pull/863)
- [uefi-raw: Add common derives to `EventType`](https://github.com/rust-osdev/uefi-rs/pull/866)
- [xtask: Improve an error location in check-raw](https://github.com/rust-osdev/uefi-rs/pull/867)
- [uefi-raw: Add ConfigurationTable](https://github.com/rust-osdev/uefi-rs/pull/868)
- [uefi-raw: Add SimpleTextInputProtocol and SimpleTextOutputProtocol](https://github.com/rust-osdev/uefi-rs/pull/869)
- [uefi-raw: Add SystemTable](https://github.com/rust-osdev/uefi-rs/pull/870)
- [uefi-raw: Fill in a few more BootServices function pointers](https://github.com/rust-osdev/uefi-rs/pull/865)
- [uefi-raw: derive all the things](https://github.com/rust-osdev/uefi-rs/pull/871)
- [device_path: add more convenience (part 1)](https://github.com/rust-osdev/uefi-rs/pull/827)
- [Release uefi-raw-0.3.0, uefi-0.24.0, uefi-services-0.21.0](https://github.com/rust-osdev/uefi-rs/pull/873)
- [uefi-raw: Add LoadedImageProtocol](https://github.com/rust-osdev/uefi-rs/pull/874)
- [uefi: Check for null pointer in config_table](https://github.com/rust-osdev/uefi-rs/pull/875)
- [test-runner: Simplify an iterator chain](https://github.com/rust-osdev/uefi-rs/pull/878)
- [build(deps): bump itertools from 0.10.5 to 0.11.0](https://github.com/rust-osdev/uefi-rs/pull/876)
- [uefi: Use uefi_raw's `SimpleTextInputProtocol` in `Input`](https://github.com/rust-osdev/uefi-rs/pull/879)
- [uefi: Use uefi_raw's `SimpleTextOutputProtocol` in `Output`](https://github.com/rust-osdev/uefi-rs/pull/881)
- [uefi: Use uefi_raw's `LoadedImageProtocol` to implement `LoadedImage`](https://github.com/rust-osdev/uefi-rs/pull/882)
- [uefi: Fix warnings when compiling without the alloc feature](https://github.com/rust-osdev/uefi-rs/pull/880)
- [uefi-raw: Add `SimplePointerProtocol`](https://github.com/rust-osdev/uefi-rs/pull/884)
- [uefi: Use uefi_raw's SystemTable to implement SystemTable](https://github.com/rust-osdev/uefi-rs/pull/883)

Thanks to [@medhefgo](https://github.com/medhefgo) for their contribution!


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`hermitcore/rusty-hermit`](https://github.com/hermitcore/rusty-hermit)
<span class="maintainers">(Section written by [@mkroening](https://github.com/mkroening))</span>

The Hermit library operating system allows you to bundle a whole OS directly with your application, creating a freestanding, bootable _Unikernel_ image.
This month, we achieved several milestones for reaching more users with Hermit:

- Stable Rust support.

  You can now compile your programs for Hermit using the stable Rust toolchain! 🥳
  
  While the Hermit targets (`x86_64-unknown-hermit` and `aarch64-unknown-hermit`) are still at tier 3, we now distribute pre-built artifacts of the Rust standard library for use with stable toolchains!
  This means, no more `-Zbuild-std`, resulting in faster builds, and the bliss of the stable Rust compiler.
  
  See [`hermitcore/rust-std-hermit`](https://github.com/hermitcore/rust-std-hermit) for details on our `rust-std` artifacts.

- Windows support.

  Thanks to Rust's awesome cross-compilation capabilities, you can now compile Hermit applications from anywhere! 😎

  We have resolved a longstanding issue when building Hermit applications on Windows ([hermitcore/rusty-hermit#431](https://github.com/hermitcore/rusty-hermit/pull/431)).
  The issue is all sorted out now and Windows, macOS, and Linux are tested and verified by our CI.
  

- AArch64 (ARM64) support.

  You can now run real applications on AArch64, with scheduling, network and everything! 🤯

  - PCI now works on AArch64, which allows us to use the network devices ([hermitcore/libhermit-rs#748](https://github.com/hermitcore/libhermit-rs/pull/748)).

  - We merged scheduling support for AArch64 ([hermitcore/libhermit-rs#765](https://github.com/hermitcore/libhermit-rs/pull/765)).

  - You can also now chainload Hermit apps for AArch64 using our loader ([hermitcore/rusty-loader#201](https://github.com/hermitcore/rusty-loader/pull/201)).
    
    This means, you don't have to statically compile the full application into the final loader image anymore.

We'd love if you gave Hermit a try. Just start with our "Hello, World!" application template: [`hermitcore/rusty-demo`](https://github.com/hermitcore/rusty-demo).

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
