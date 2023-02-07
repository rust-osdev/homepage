+++
title = "This Month in Rust OSDev: January 2023"
date = 2023-02-07

[extra]
month = "January 2023"
editors = ["phil-opp", "berkus"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (January 2023)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

    ### Summary
    <span class="maintainers">(Section written by [@author](https://github.com/author))</span>

    <text>
-->

- [Stabilize `abi_efiapi` feature](https://github.com/rust-lang/rust/pull/105795)
- [Add checks for the signature of the `start` lang item](https://github.com/rust-lang/rust/pull/106092)
- [default OOM handler: use non-unwinding panic, to match std handler](https://github.com/rust-lang/rust/pull/106045)

## Announcements, News, and Blog Posts

<!--
Here we collect news, blog posts, etc. related to OS development in Rust.
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [A GUI installer for redox is coming soon, written in iced!](https://fosstodon.org/@soller/109674396246472281)
  - [Discussion on Reddit](https://www.reddit.com/r/rust/comments/109qm9j/media_a_gui_installer_for_redox_is_coming_soon/)
- [cargo-show-asm 0.2.10](https://www.reddit.com/r/rust/comments/107h9ay/cargoshowasm_0210_new_and_improved_all_over_the/)
- [RustyHermit @ FOSDEM 2023: A Rust-Based, modular Unikernel for MicroVMs](https://fosdem.org/2023/schedule/event/rustunikernel/)

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

### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@rybot666](https://github.com/rybot666)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables.

We merged lots of improvements this month:

#### Features

- [Load ramdisk feature](https://github.com/rust-osdev/bootloader/pull/302)
- [Add support for a boot configuration file](https://github.com/rust-osdev/bootloader/pull/326)
- [Make log level configurable](https://github.com/rust-osdev/bootloader/pull/303)
- [Add support for logging to serial port (configurable)](https://github.com/rust-osdev/bootloader/pull/314)
- [Add `bios` and `uefi` cargo features](https://github.com/rust-osdev/bootloader/pull/304)
- [Add a `FrameBuffer::into_buffer` method for taking ownership](https://github.com/rust-osdev/bootloader/pull/319)
- [Implement faster bios builds](https://github.com/rust-osdev/bootloader/pull/324)
- [Support higher half position independent kernels](https://github.com/rust-osdev/bootloader/pull/289)

#### Fixes

- [Correctly allocate last frame in memory descriptor](https://github.com/rust-osdev/bootloader/pull/316)
- [Fix: treat `kernel_slice_end` as an exclusive bound when checking for overlaps](https://github.com/rust-osdev/bootloader/pull/334)
- [Map BIOS stage-4 at lower address to avoid conflicts with the kernel](https://github.com/rust-osdev/bootloader/pull/337)
- [Correctness fixes for stage2](https://github.com/rust-osdev/bootloader/pull/328)
- [Fix loading of boot configuration](https://github.com/rust-osdev/bootloader/pull/342)

#### Docs

- [Fix Create Disk Image Example](https://github.com/rust-osdev/bootloader/pull/300)
- [Make a link in the documentation clickable](https://github.com/rust-osdev/bootloader/pull/341)

#### Other

- [Fix spelling and add a check](https://github.com/rust-osdev/bootloader/pull/340)
- [Check for breaking changes on CI](https://github.com/rust-osdev/bootloader/pull/325)
- [Cancel in progress PR builds when a new commit is pushed for that PR](https://github.com/rust-osdev/bootloader/pull/322)
- [Remove dependency on `time` crate](https://github.com/rust-osdev/bootloader/pull/332)
- [[test runner] Print QEMU output directly instead of waiting until it finishes](https://github.com/rust-osdev/bootloader/pull/333)
- [Fix warnings from Clippy](https://github.com/rust-osdev/bootloader/pull/336)
- [Test framework: Don't inherit `stdin` when spawning QEMU](https://github.com/rust-osdev/bootloader/pull/339)

Thanks to [@jasoncouture](https://github.com/jasoncouture), [@Stary2001](https://github.com/Stary2001), [@AlexJMohr](https://github.com/AlexJMohr), [@Freax13](https://github.com/Freax13), [@tsoutsman](https://github.com/tsoutsman), and [@asensio-project](https://github.com/asensio-project) for their contributions!


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS.

We merged the following changes last month:

#### Features

- [Implement the rest of the TPM v1 interface](https://github.com/rust-osdev/uefi-rs/pull/631)
- [Implement the rest of the TPM v2 interface](https://github.com/rust-osdev/uefi-rs/pull/634)
- [Release uefi-macros-0.10.0, uefi-0.19.0, and uefi-services-0.16.0](https://github.com/rust-osdev/uefi-rs/pull/642)
- [Add PAGE_SIZE constant and update MemoryProtection protocol docs](https://github.com/rust-osdev/uefi-rs/pull/645)

#### Fixes

- [PXE: Fix BaseCode::discover optional argument](https://ygithub.com/rust-osdev/uefi-rs/pull/630)
- [Fix warnings from `abi_efiapi` stabilization](https://github.com/rust-osdev/uefi-rs/pull/635)
- [uefi: Fix protocol functions to work with unsized protocols](https://github.com/rust-osdev/uefi-rs/pull/643)
- [Fix new lints related to derives on a packed struct](https://github.com/rust-osdev/uefi-rs/pull/646)

#### Docs

- [uefi: Update MSRV in the readme](https://github.com/rust-osdev/uefi-rs/pull/626)
- [book: Fix link to handles page](https://github.com/rust-osdev/uefi-rs/pull/627)
- [changelog: Move some macro-related changes to correct section](https://github.com/rust-osdev/uefi-rs/pull/639)

#### Other

- [Minor alloc-related cleanups](https://github.com/rust-osdev/uefi-rs/pull/623)
- [uefi: Remove useless padding field](https://github.com/rust-osdev/uefi-rs/pull/629)
- [Move some util code from TCG to a new top-level module](https://github.com/rust-osdev/uefi-rs/pull/640)
- [media/test: add integration test for creating a directory](https://github.com/rust-osdev/uefi-rs/pull/625)
- [ci: Update checkout action to latest version](https://github.com/rust-osdev/uefi-rs/pull/633)
- [test-runner: Simplify and slightly refactor the disk test](https://github.com/rust-osdev/uefi-rs/pull/641)
- [ci: fix book/deploy in forks](https://github.com/rust-osdev/uefi-rs/pull/644)

Thanks to [@nsemmel](https://github.com/nsemmel) and [@liferooter](https://github.com/liferooter) for their contributions!


### [`volatile`](https://github.com/rust-osdev/volatile)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `volatile` crate provides a safe wrapper type for implementing volatile read and write operations. This is useful for accessing memory regions that have side-effects, such as memory-mapped hardware registers or framebuffers.

We merged the following PRs this month:

- [Fix UB in slice methods when `Deref` returns different references](https://github.com/rust-osdev/volatile/pull/27) <span class="gray">(published as `v0.4.6`)</span>
- [various improvements for the new design](https://github.com/rust-osdev/volatile/pull/28)

Thanks to [@Freax13](https://github.com/Freax13) for their contributions!

We are still working on a new, safer design. This month, we opened PR [#29](https://github.com/rust-osdev/volatile/pull/29) to compare and discuss two alternative designs. The main question is whether the provided `VolatilePtr` type should implement `Copy` or `Send`. Only one of these trait implementations is possible, otherwise there could be data races that lead to undefined behavior. Since both variants have valid use cases, the latest proposal is to implement two different types with conversion methods between them. We haven't reached a decision yet, so if anyone has more input on this, please join the discussion.

### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13), and [@rybot666](https://github.com/orgs/rust-osdev/people/rybot666)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following changes in January:

- [Set `repr` to transparent for various types](https://github.com/rust-osdev/x86_64/pull/402)
- [Check for breaking changes on CI](https://github.com/rust-osdev/x86_64/pull/401)


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

### [`metta-systems/vesper`](https://github.com/metta-systems/vesper)

<span class="maintainers">(Section written by [@berkus](https://github.com/berkus))</span>

Vesper is a capability-based single-address-space nanokernel. This means it is aiming to be small, to provide only isolation primitives; at the same time SAS makes it a lot easier to perform cross-process operations (because all addresses are the same across all processes). It uses capabilities to provide security for such operations, so that unauthorized processes will not be able to intervene in legitimate traffic.

The kernel is in very early stages of development...

And for the past few months I've been on and off involved in deciphering a mystical miscompilation problem. I've managed to summarize it in a [ticket to rustc](https://github.com/rust-lang/rust/issues/102849).

The most minimal reproduction scenario triggers when I use a format_args!() macro in my code and a nightly version later than 2022-08-12.

Here's a key [snippet](https://github.com/metta-systems/vesper/blob/85fe40f603e500518af96d6aa922f7ecfa57f0c5/src/main.rs#L117-L119):

```
// if you keep this line, it works when compiled
// via rustc 2022-08-12 and breaks on 2022-08-13 and all the versions past that.
// if you comment this line out, on 2022-08-13 everything else starts to work.
uart.write_fmt(format_args_nl!("Lets {}!", "go")).ok();
```

There's also a [QEMU-only reproduction code](https://github.com/metta-systems/vesper/blob/c06d89a23c5c2b0cd3ed5d270aec10054216ea5a/src/main.rs), which is much smaller, as it reuses `armv8a_semihosting` and `armv8a_panic_semihosting` crates for pretty much everything.

The behavior is [the same](https://github.com/metta-systems/vesper/blob/c06d89a23c5c2b0cd3ed5d270aec10054216ea5a/src/main.rs#L29-L31) though:
```
// if you don't comment it out, it works on 08-12 and breaks on 08-13.
// if you comment this line out on 08-13 everything else starts to work.
armv8a_semihosting::hprintln!("Lets {}!", "go").ok();
```

"Working" here means the kernel boots and runs through to the last panic line:

```
[cargo-make] INFO - Running Task: qemu
🚜 Run QEMU -M raspi3b -semihosting with vesper/target/nucleus.bin
Letsgo!
Lets go!
Lets go 2!
panicked at 'Off you go!', src/main.rs:34:5
```

And "not working" is the kernel either panicing on boot in arch64, for which I've extracted panic message but I have low confidence this is what actually happens - panic was in once_cell detecting it is initializing a second time, which I discounted as potentially just a bug in linker script layout and not an actual code generation bug. This code is even completely removed in the latest reproduction (no once_cells) but the kernel still crashes before it even can write anything to serial.

In the qemu repro it's even weirder:

```
🚜 Run QEMU -M raspi3b -semihosting with vesper/target/nucleus.bin

```

No output - because hstdout_str() function from armv8a_semihosting crate fails to open semihosting stdout handle - syscall returns -1. Why would that happen simply by a compiler version change - is the biggest question.

I'm yet to find what am I missing here - assuming this is my own mistake and not a compiler fault, because it would've been noticed by everybody else then? But who knows, maybe you can spot something - drop me a line if you see anything suspicious, I'm pretty much out of ideas here.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)
<span class="maintainers">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

We merged the following changes to the [_Writing an OS in Rust_](https://os.phil-opp.com/) blog this month:

- [Fix broken links](https://github.com/phil-opp/blog_os/pull/1188)

Thanks a lot to [@kennystrawnmusic](https://github.com/kennystrawnmusic) for this contribution!

We also have an open pull request for a [Chinese translation of the _Paging Implementation_ post](https://github.com/phil-opp/blog_os/pull/1189).
We're still looking for a reviewer, so if you're speaking Chinese we would appreciate if you could take a look.
Thanks!

Since I mostly worked on the `bootloader` crate this month, I haven't made much process on the upcoming third edition of the blog yet.
I'll do my best to have something ready soon!

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
