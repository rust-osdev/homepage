+++
title = "This Month in Rust OSDev: August 2023"
date = 2023-09-06

[extra]
month = "August 2023"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (August 2023)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Announcements, News, and Blog Posts

Here we collect news, blog posts, etc. related to OS development in Rust.

- [Fomos: Experimental OS, built with Rust](https://github.com/Ruddle/Fomos)
  - Discussion on [HN](https://news.ycombinator.com/item?id=37316309), [reddit](https://www.reddit.com/r/rust/comments/164li1c/fomos_experimental_rust_os/), [lobste.rs](https://lobste.rs/s/eoyuf6/fomos_experimental_os_built_with_rust)
- [Interview with Rust and operating system Developer Andy Python](https://blog.rust.careers/post/andy-python-interview/)

### Redox Summer of Code

- [Redox Summer of Code Wrapup](https://redox-os.org/news/rsoc-2023-wrapup/)
- VirtIO drivers for Redox in a VM  - Andy-Python-Programmer: [Part 1](https://www.redox-os.org/news/rsoc-virtio-1/) and [Part 2](https://redox-os.org/news/rsoc-virtio-2/)
- On-Demand Paging for Redox - 4lDO2: [Part 1](https://www.redox-os.org/news/kernel-8/) and [Part 2](https://redox-os.org/news/kernel-9/)
- Using Linux drivers in a VM on Redox - Enygmator: [Overview](https://www.redox-os.org/news/rsoc-2023-eny-1/)

<!--
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->


## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [WASI threads, implementation of `wasm32-wasi-preview1-threads`` target](https://github.com/rust-lang/rust/pull/112922)
- [Re-enable atomic loads and stores for all RISC-V targets](https://github.com/rust-lang/rust/pull/98333)
- [Update to LLVM 17](https://github.com/rust-lang/rust/pull/114048)
- [feat: `riscv-interrupt-{m,s}` calling conventions](https://github.com/rust-lang/rust/pull/111891)
- [Add the `relocation_model` to the cfg](https://github.com/rust-lang/rust/pull/113966)
- [Default `relax_elf_relocations` to true](https://github.com/rust-lang/rust/pull/106511)
- [add `aarch64-unknown-teeos` target](https://github.com/rust-lang/rust/pull/113480)
- cargo: [Add support for `target.'cfg(..)'.linker`](https://github.com/rust-lang/cargo/pull/12535)
- cargo: [Support dependencies from registries for artifact dependencies, take 2](https://github.com/rust-lang/cargo/pull/12421)


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

The `uefi-rs` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. We merged the following PRs this month:

- [Drop `memmove` and `set_mem` from boot services](https://github.com/rust-osdev/uefi-rs/pull/906)
- [Add block I/O protocol to `uefi-raw` and use it in `uefi`](https://github.com/rust-osdev/uefi-rs/pull/909)
- [uefi-raw: Rename BlockIo to BlockIoProtocol](https://github.com/rust-osdev/uefi-rs/pull/911)
- [Add raw memory protection protocol and use in `uefi`](https://github.com/rust-osdev/uefi-rs/pull/896)
- [Use workspace.dependencies to declare shared dependencies](https://github.com/rust-osdev/uefi-rs/pull/913)
- [Add component name protocol to uefi-raw and use from uefi](https://github.com/rust-osdev/uefi-rs/pull/910)
- [Implement core::error::Error for all error types](https://github.com/rust-osdev/uefi-rs/pull/916)
- [Fix 1.72 lints](https://github.com/rust-osdev/uefi-rs/pull/928)
- [Derive `Debug` in more places (requires Rust 1.70)](https://github.com/rust-osdev/uefi-rs/pull/851)
- [Memory map change](https://github.com/rust-osdev/uefi-rs/pull/915)

<!---
- [build(deps): bump crate-ci/typos from 1.16.1 to 1.16.2](https://github.com/rust-osdev/uefi-rs/pull/908)
- [build(deps): bump crate-ci/typos from 1.16.2 to 1.16.3](https://github.com/rust-osdev/uefi-rs/pull/912)
- [build(deps): bump crate-ci/typos from 1.16.3 to 1.16.5](https://github.com/rust-osdev/uefi-rs/pull/919)
- [build(deps): bump crate-ci/typos from 1.16.5 to 1.16.7](https://github.com/rust-osdev/uefi-rs/pull/923)
- [build(deps): bump crate-ci/typos from 1.16.7 to 1.16.8](https://github.com/rust-osdev/uefi-rs/pull/924)
- [build(deps): bump rustls-webpki from 0.100.1 to 0.100.2](https://github.com/rust-osdev/uefi-rs/pull/926)
- [build(deps): bump clap from 4.3.0 to 4.4.0](https://github.com/rust-osdev/uefi-rs/pull/927)
- [build(deps): bump nix from 0.26.2 to 0.27.1](https://github.com/rust-osdev/uefi-rs/pull/929)
- [build(deps): bump crate-ci/typos from 1.16.8 to 1.16.9](https://github.com/rust-osdev/uefi-rs/pull/930)
--->

Thanks to [@cmoylan](https://github.com/cmoylan) and [@julic20s](https://github.com/julic20s) for their contributions!

### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following PRs:

- [kernel image fields & zero out rbp](https://github.com/rust-osdev/bootloader/pull/346)
- [`RacyCell<T>`: Data race allowed on T](https://github.com/rust-osdev/bootloader/pull/390)
- [Update license field following SPDX 2.1 license expression standard](https://github.com/rust-osdev/bootloader/pull/391)

Thanks to [@frisoft](https://github.com/frisoft), [@devsnek](https://github.com/devsnek), and [@kuzeyardabulut](https://github.com/kuzeyardabulut) for their contributions!



### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PR this month:

- [Add `inline` attribute to segment functions](https://github.com/rust-osdev/x86_64/pull/430)
- [Add the `iretq` function to the `InterruptStackFrameValue` struct.](https://github.com/rust-osdev/x86_64/pull/431)
- [Fix misc doc typos](https://github.com/rust-osdev/x86_64/pull/432)

Thanks to [@tsoutsman](https://github.com/tsoutsman), [@NathanKolpa](https://github.com/NathanKolpa), and [@xzmeng](https://github.com/xzmeng) for their contributions!


### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS. We merged the following changes this month:

- [AML: `DefSizeOf` implementation](https://github.com/rust-osdev/acpi/pull/189)
- [AML: Fix `DefIfElse` parser returning `UnexpectedEndOfStream` when `If (...) {}` is not followed by anything](https://github.com/rust-osdev/acpi/pull/190)

Thanks to [@alnyan](https://github.com/alnyan) for their contributions!


### [`uart_16550`](https://github.com/rust-osdev/uart_16550)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `uart_16550` crate provides basic support for serial port I/O for 16550-compatible UARTs. We merged the following change this month:

- [x86: crate finally builds with x86 and x86_64](https://github.com/rust-osdev/uart_16550/pull/29)

Thanks to [@phip1611](https://github.com/phip1611) for their contributions!


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.


### [`mkroening/take-static`](https://github.com/mkroening/take-static)
<span class="maintainers">(Section written by [@mkroening](https://github.com/mkroening))</span>

I published a tiny new crate, allowing you to get a mutable reference to static items safely (only once, though):

```rust
use take_static::take_static;

take_static! {
    static NUMBER: usize = 5;
}

assert_eq!(NUMBER.take(), Some(&mut 5));
assert_eq!(NUMBER.take(), None);
```

This allows you to easily use statically allocated memory before dynamic memory allocators may be available.
Compared to [`cortex_m::singleton`], `take_static` is thread-safe.
Compared to [`takecell::TakeCell`], `take_static` also supports `!Send` types.

[`cortex_m::singleton`]: https://docs.rs/cortex-m/0.7.7/cortex_m/macro.singleton.html
[`takecell::TakeCell`]: https://docs.rs/takecell/0.1.1/takecell/index.html


### [`hermit-os/kernel`](https://github.com/hermit-os/kernel)
<span class="maintainers">(Section written by [@mkroening](https://github.com/mkroening))</span>

<img width="128" align="right" src="https://raw.githubusercontent.com/hermit-os/.github/47a27de62e8cfc658ddbccb3f00743c7538281ae/img/Hermit.svg" />

The Hermit unikernel project allows you to bundle your Rust application with our library operating system to create a bootable unikernel image.
Hermit is a single-address-space operating system.
Since there is only one application running in the virtual machine, no isolation between applications or between user space and kernel space is necessary.
This reduces system call overhead immensely, since every system call is just a library call.
For more information, see our [_The Hermit Operating System_] showcase post.

[_The Hermit Operating System_]: @/showcase/hermit/index.md

The RustyHermit project has been renamed.
We have renamed our GitHub organization from [@hermitcore](https://github.com/hermitcore) to [@hermit-os](https://github.com/hermit-os) and reserved the <http://hermit-os.org> domain.

We have also renamed some of our core projects to reduce confusion:
- [hermit-os/kernel](https://github.com/hermit-os/kernel) is the Hermit kernel.
- [hermit-os/hermit-rs](https://github.com/hermit-os/hermit-rs) provides Rust application support.
- [hermit-os/uhyve](https://github.com/hermit-os/uhyve) is a specialized hypervisor for Hermit.
- [hermit-os/loader](https://github.com/hermit-os/loader) is a bootloader for other platforms, such as QEMU.

We have a new logo!
As hermit crabs occupy empty shells produced by other organisms, the original HermitCore occupied one or several cores on a computer.
Because we migrated to Rust in 2018, our new logo of a hermit crab occupying the Rust logo's [bike gear](https://bugzilla.mozilla.org/show_bug.cgi?id=680521) fits quite nicely with the Rust logo as well as Rust's [Ferris](https://rustacean.net/).

And as always, please come and try Hermit! :)

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)
<span class="maintainers">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

We merged the following changes to the [_Writing an OS in Rust_](https://os.phil-opp.com/) blog this month:

- [Update post 2 with beginner friendly cargo tips](https://github.com/phil-opp/blog_os/pull/1234)
- [Grammar fix](https://github.com/phil-opp/blog_os/pull/1235)
- [minimal-rust-kernel: fix missing .toml in zh-CN translation](https://github.com/phil-opp/blog_os/pull/1237)

Thanks to [@Connortsui20](https://github.com/Connortsui20) and [@xzmeng](https://github.com/xzmeng) for these contributions!


<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
