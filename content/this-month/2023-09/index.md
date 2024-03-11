+++
title = "This Month in Rust OSDev: September 2023"
date = 2023-10-05

[extra]
month = "September 2023"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (September 2023)" post.
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

- [Toolchain Upgrade - Redox OS](https://www.redox-os.org/news/toolchain-upgrade-1/)
- [Open Sourcing Ferrocene](https://ferrous-systems.com/blog/ferrocene-open-source/)
- [ESP32 Standard Library Embedded Rust: GPIO Interrupts](https://apollolabsblog.hashnode.dev/esp32-standard-library-embedded-rust-gpio-interrupts)
- [The Embedded Rust ESP Development Ecosystem](https://apollolabsblog.hashnode.dev/the-embedded-rust-esp-development-ecosystem)
- [ESP Embedded Rust: Multithreading with FreeRTOS Bindings](https://apollolabsblog.hashnode.dev/esp-embedded-rust-multithreading-with-freertos-bindings)
- [How Rust can build an elegant API around raw memory](https://litchipi.site/post/14762501311625827021)
- [Redox: Development Priorities for 2023/24](https://redox-os.org/news/development-priorities-2023-09/)


## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [Add Minimal Std implementation for UEFI](https://github.com/rust-lang/rust/pull/105861)
- [Stdio support for UEFI](https://github.com/rust-lang/rust/pull/116207)
- [Promote loongarch64-unknown-none* to Tier 2](https://github.com/rust-lang/rust/pull/115368)
- [Add initial libstd support for Xous](https://github.com/rust-lang/rust/pull/104101)
- [added support for GNU/Hurd](https://github.com/rust-lang/rust/pull/115230)
- [MCP661: Move wasm32-wasi-preview1-threads target to Tier 2](https://github.com/rust-lang/rust/pull/115345)
- [add notes about non-compliant FP behavior on 32bit x86 targets](https://github.com/rust-lang/rust/pull/113053)
- [add more explicit I/O safety documentation](https://github.com/rust-lang/rust/pull/114780)
- [Change `unsafe_op_in_unsafe_fn` to be `warn-by-default` from edition 2024](https://github.com/rust-lang/rust/pull/112038)
- [cargo: Stabilize lint table](https://github.com/rust-lang/cargo/pull/12648)
- Final comments period
  - [Stabilize `atomic_from_ptr`](https://github.com/rust-lang/rust/pull/115719)
  - [Tracking Issue for `pointer_bytes_offsets`](https://github.com/rust-lang/rust/issues/96283)
  - [Add `f16` and `f128` float types](https://github.com/rust-lang/rfcs/pull/3453)
  - [RFC: Remove implicit features in a new edition](https://github.com/rust-lang/rfcs/pull/3491)

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


### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

The [multiboot2](https://crates.io/crates/multiboot2) was bumped from `0.18.1` 
to `0.19.0`. The new release includes the ability to add custom tags to the MBI 
builder and a bugfix when parsing Multiboot strings, such as the command line 
from a Module tag.

For more details, please have a look at the [changelog](https://github.com/rust-osdev/multiboot2/releases/tag/multiboot2-v0.19.0).

Thanks to [@A0lson](https://github.com/A0lson) for their [contribution](https://github.com/rust-osdev/multiboot2/pull/172)
that helped to fix the string parsing bug.

### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS.

This month, a new major version of `acpi` was published, offering greater control over how the crate allocates
memory. Specifically, the new `allocator_api` and `alloc` features allow you to opt-out of allocation altogether
(allowing the crate to be used from slimmer environments like bootloaders), or to provide your own allocator using
the new (and still unstable) [`core::alloc::Allocator` API](https://doc.rust-lang.org/beta/core/alloc/trait.Allocator.html).
Enabling both features makes the crate behave very similarly to before, so migration should be relatively easy.

Because the `acpi` crate can now be used from environements without allocation, the `rsdp` crate has been
deprecated, and all functionality moved into `acpi`. The `rsdp` crate will continue to work, but will not receive
further updates. This should not affect users using `rsdp` to simply find the address of the RSDP, but is a
breaking change as types that have been moved to `acpi` will no longer be usable across the crate boundary.

Some improvements were also made to the `aml` crate this month, adding functionality and improving our correctness
- many thanks to our contributors!

We merged the following changes this month:

- [AML: Implement OpReg-relative PkgLength parser](https://github.com/rust-osdev/acpi/pull/191)
- [AML: Fix DefPackage len less than NumElements failing](https://github.com/rust-osdev/acpi/pull/192)
- [Prepare new version of `acpi`](https://github.com/rust-osdev/acpi/pull/197)
- [AML: add support for the `DefSleep` opcode](https://github.com/rust-osdev/acpi/commit/133001e59d3f56056d371954eb52a79ee5a2b377)
- [AML: add support for the `DefStall` opcode](https://github.com/rust-osdev/acpi/commit/133001e59d3f56056d371954eb52a79ee5a2b377)

Thanks to [@alnyan](https://github.com/alnyan) for their contributions!

### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PR this month:

- [fix(interrupts): add compiler fences to enable and disable](https://github.com/rust-osdev/x86_64/pull/436)
- [add `flush_broadcast` and `tlbsync` functions](https://github.com/rust-osdev/x86_64/pull/403)
- [Release v0.14.11](https://github.com/rust-osdev/x86_64/pull/437)
- [Add `HandlerFuncType` trait](https://github.com/rust-osdev/x86_64/pull/439)

Thanks to [@brandonchinn178](https://github.com/brandonchinn178) and [@mkroening](https://github.com/mkroening) for their contributions!

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

The `uefi-rs` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. We merged the following PRs this month:

- [Add ShellParams protocol](https://github.com/rust-osdev/uefi-rs/pull/772)
- [github: Change dependabot interval to weekly](https://github.com/rust-osdev/uefi-rs/pull/949)

<!--
- [build(deps): bump actions/checkout from 3 to 4](https://github.com/rust-osdev/uefi-rs/pull/934)
- [build(deps): bump cachix/install-nix-action from 22 to 23](https://github.com/rust-osdev/uefi-rs/pull/933)
- [build(deps): bump crate-ci/typos from 1.16.9 to 1.16.10](https://github.com/rust-osdev/uefi-rs/pull/932)
- [build(deps): bump walkdir from 2.3.3 to 2.4.0](https://github.com/rust-osdev/uefi-rs/pull/940)
- [build(deps): bump crate-ci/typos from 1.16.10 to 1.16.11](https://github.com/rust-osdev/uefi-rs/pull/941)
- [build(deps): bump crate-ci/typos from 1.16.11 to 1.16.12](https://github.com/rust-osdev/uefi-rs/pull/944)
- [build(deps): bump crate-ci/typos from 1.16.12 to 1.16.13](https://github.com/rust-osdev/uefi-rs/pull/946)
- [build(deps): bump crate-ci/typos from 1.16.13 to 1.16.14](https://github.com/rust-osdev/uefi-rs/pull/947)
- [build(deps): bump crate-ci/typos from 1.16.14 to 1.16.15](https://github.com/rust-osdev/uefi-rs/pull/948)
-->

Thanks to [@JohnAZoidberg](https://github.com/JohnAZoidberg) for their contribution!


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`mkroening/interrupts`](https://github.com/mkroening/interrupts)
<span class="maintainers">(Section written by [@mkroening](https://github.com/mkroening))</span>

I created a dependency-free `interrupts` crate, allowing you to temporarily disable interrupts on AArch64, 64-bit RISC-V, and x86-64.
Two different paradigms allow you to run code without interrupts and synchronize with interrupt handlers running on the same hardware thread (core):

Use [`disable`] to disable interrupts with a guard:

```rust
// interrupts may or may not be enabled
let guard = interrupts::disable();
// interrupts are disabled
drop(guard);
// interrupts are restored to the previous state
```

Use [`without`] (similar to [`x86_64::instructions::interrupts::without_interrupts`]) to run a closure with disabled interrupts:

```rust
// interrupts may or may not be enabled
interrupts::without(|| {
    // interrupts are disabled
});
// interrupts are restored to the previous state
```

I would appreciate you dropping by and giving it a try. :)

[`disable`]: https://docs.rs/interrupts/latest/interrupts/fn.disable.html
[`without`]: https://docs.rs/interrupts/latest/interrupts/fn.without.html
[`x86_64::instructions::interrupts::without_interrupts`]: https://docs.rs/x86_64/latest/x86_64/instructions/interrupts/fn.without_interrupts.html

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
