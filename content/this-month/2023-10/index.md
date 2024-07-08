+++
title = "This Month in Rust OSDev: October 2023"
date = 2023-11-06

[extra]
month = "October 2023"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (October 2023)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Announcements, News, and Blog Posts

Here we collect news, blog posts, etc. related to OS development in Rust.

- [On the Challenge of Sound Code for Operating Systems](https://doi.org/10.1145/3623759.3624554)
  - An open access paper on common patterns of unsound abstractions in operating systems when the codebase or the programmers are more familiar with C than with Rust
- [Bare-metal Rust in Android](https://security.googleblog.com/2023/10/bare-metal-rust-in-android.html?m=1)
- [The Binder Linux driver is being rewritten in Rust](https://www.reddit.com/r/rust/comments/17lzdwt/the_binder_linux_driver_is_being_rewritten_in_rust/)
- [Microsoft is planning to make Rust a 1st class language across their engineering systems](https://nitter.net/dwizzzleMSFT/status/1720134540822520268?s=20)
- [Vivo Unveils BlueOS, Based on Rust Language](https://en.wikipedia.org/wiki/BlueOS)

<!--
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->


## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

There weren't really any OS-related infrastructure updates this month, but there was some great progress on several upcoming language and tooling features that may also be of interest to OS development:

- [Stabilize `async fn` and return-position `impl Trait` in trait](https://github.com/rust-lang/rust/pull/115822)
- [Distribute `cg_clif` as rustup component on the nightly channel](https://github.com/rust-lang/rust/pull/81746)
  - Faster debug builds using [cranelift](https://cranelift.dev/)
- [Implement `gen` blocks in the 2024 edition](https://github.com/rust-lang/rust/pull/116447)
  - Allows creating iterators through generators
- [Stabilize `[const_]pointer_byte_offsets`](https://github.com/rust-lang/rust/pull/116205)
- [Stabilize Ratified RISC-V Target Features](https://github.com/rust-lang/rust/pull/116485)
- [Guarantee that `char` has the same size and alignment as `u32`](https://github.com/rust-lang/rust/pull/116894)
- [feat: implement RFC 3127 `-Ztrim-paths`](https://github.com/rust-lang/cargo/pull/12625)
  - Allows sanitizing file system paths used in panic messages → can reduce binary size
  - See [RFC 3127](https://rust-lang.github.io/rfcs/3127-trim-paths.html) for details

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

### [`spinning_top`](https://github.com/rust-osdev/spinning_top)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `spinning_top` crate provides a simple spinlock implementation based on the abstractions of the [`lock_api`](https://docs.rs/lock_api/0.4.1/lock_api/) crate.

We merged the following changes this month:

#### Features

- [perf: inline everything](https://github.com/rust-osdev/spinning_top/pull/17)
- [feat: add backoff feature](https://github.com/rust-osdev/spinning_top/pull/16)
- [feat: add `RwSpinlock` readers-writer lock](https://github.com/rust-osdev/spinning_top/pull/18)
- [feat: add `arc_lock` feature and typedefs](https://github.com/rust-osdev/spinning_top/pull/25)
- [Prepare for v0.3.0 release](https://github.com/rust-osdev/spinning_top/pull/26)

#### Other

- [ci: build with all features](https://github.com/rust-osdev/spinning_top/pull/19)
- [docs: fix typo](https://github.com/rust-osdev/spinning_top/pull/23)
- [test: don't ignore statics example](https://github.com/rust-osdev/spinning_top/pull/22)
- [chore: remove `const_spinlock` function](https://github.com/rust-osdev/spinning_top/pull/20)
- [chore: remove deprecated `nightly` feature](https://github.com/rust-osdev/spinning_top/pull/21)

Thanks to [@mkroening](https://github.com/mkroening) for their contributions!

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

The `uefi-rs` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS. We merged the following PRs this month:

- [Release](https://github.com/rust-osdev/uefi-rs/pull/958)
- [Release uefi-services-0.22.0](https://github.com/rust-osdev/uefi-rs/pull/960)
- [uefi-raw: Use workspace dependency for uguid](https://github.com/rust-osdev/uefi-rs/pull/967)
- [uefi/gop: fix memory leak](https://github.com/rust-osdev/uefi-rs/pull/969)
- [Allow indexing of `MemoryMap`.](https://github.com/rust-osdev/uefi-rs/pull/966)
- [uefi-services: Return event in init](https://github.com/rust-osdev/uefi-rs/pull/920)
- [Remove unused UefiRegularFileHandle type alias](https://github.com/rust-osdev/uefi-rs/pull/975)
- [uefi-services: Remove NonNull wrapper from system_table](https://github.com/rust-osdev/uefi-rs/pull/974)
- [Mark free_pages and free_pool as unsafe](https://github.com/rust-osdev/uefi-rs/pull/973)
- [uefi: Change IMAGE_HANDLE to an atomic pointer](https://github.com/rust-osdev/uefi-rs/pull/976)
- [uefi-services: Change SYSTEM_TABLE to an atomic pointer](https://github.com/rust-osdev/uefi-rs/pull/977)
- [Change Logger to use an atomic pointer internally](https://github.com/rust-osdev/uefi-rs/pull/978)
- [uefi: Use atomics instead of `static mut` in allocator](https://github.com/rust-osdev/uefi-rs/pull/979)
- [Use const interface pointers in protocol management functions](https://github.com/rust-osdev/uefi-rs/pull/981)

<!-- - [build(deps): bump crate-ci/typos from 1.16.15 to 1.16.16](https://github.com/rust-osdev/uefi-rs/pull/950) -->
<!-- - [xtask: Temporarily disable a false-positive clippy lint](https://github.com/rust-osdev/uefi-rs/pull/953) -->
<!-- - [build(deps): bump ureq from 2.7.0 to 2.8.0](https://github.com/rust-osdev/uefi-rs/pull/951) -->
<!-- - [build(deps): bump crate-ci/typos from 1.16.16 to 1.16.17](https://github.com/rust-osdev/uefi-rs/pull/956) -->
<!-- - [build(deps): bump uguid from 2.0.0 to 2.1.0](https://github.com/rust-osdev/uefi-rs/pull/957) -->
<!-- - [build(deps): bump regex from 1.9.0 to 1.10.2](https://github.com/rust-osdev/uefi-rs/pull/963) -->
<!-- - [build(deps): bump crate-ci/typos from 1.16.17 to 1.16.19](https://github.com/rust-osdev/uefi-rs/pull/964) -->
<!-- - [build(deps): bump crate-ci/typos from 1.16.19 to 1.16.20](https://github.com/rust-osdev/uefi-rs/pull/971) -->
<!-- - [build(deps): bump rustix from 0.37.19 to 0.37.26](https://github.com/rust-osdev/uefi-rs/pull/972) -->
<!-- - [build(deps): bump crate-ci/typos from 1.16.20 to 1.16.21](https://github.com/rust-osdev/uefi-rs/pull/984) -->

Thanks to [@JohnAZoidberg](https://github.com/JohnAZoidberg) and [@JarlEvanson](https://github.com/JarlEvanson) for their contributions!


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following changes this month:

- [fix(interrupts): replace compiler fences with potentially-synchronizing assembly](https://github.com/rust-osdev/x86_64/pull/440)
- [add `from_slice`` to VirtAddr](https://github.com/rust-osdev/x86_64/pull/442)
- [Enable dependabot to update actions](https://github.com/rust-osdev/x86_64/pull/420)
- [Bump actions/checkout from 3 to 4](https://github.com/rust-osdev/x86_64/pull/441)

Thanks to [@Wasabi375](https://github.com/Wasabi375), [@joycebrum](https://github.com/joycebrum), and [@mkroening](https://github.com/mkroening) for their contributions!

### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS. We merged the following changes this month:

- [aml: Implement DefOr, DefSubtract and DefLNot opcodes](https://github.com/rust-osdev/acpi/pull/199)
- [Compile for aarch64 and i686 targets on CI in addition to x86_64](https://github.com/rust-osdev/acpi/pull/201)

Thanks to [@alnyan](https://github.com/alnyan) for their contribution!

### [`ovmf-prebuilt`](https://github.com/rust-osdev/ovmf-prebuilt)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@nicholasbishop](https://github.com/nicholasbishop)</span>

The `ovmf-prebuilt` project provides pre-built [edk2](https://github.com/tianocore/edk2) releases to make it easier to set up OVMF. We merged the following improvement this month:

- [Dockerfile: add riscv64 with QemuBuild.py](https://github.com/rust-osdev/ovmf-prebuilt/pull/3)

Thanks to [@Firenezz](https://github.com/Firenezz) for their contribution!

### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following PRs:

- [Update `rustix` dependency](https://github.com/rust-osdev/bootloader/pull/398)
- [Add an additional MB of space to the generated FAT partition](https://github.com/rust-osdev/bootloader/pull/397)

Thanks to [@kennystrawnmusic](https://github.com/kennystrawnmusic) for their contribution!

### [`linked-list-allocator`](https://github.com/rust-osdev/linked-list-allocator)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@jamesmunns](https://github.com/jamesmunns)</span>

The `linked-list-allocator` crate provides a basic `no_std` allocator that builds a linked list from freed memory blocks and thus needs no additional data structures. We merged the following PR this month:

- [Fix potential panic due to huge layout](https://github.com/rust-osdev/linked-list-allocator/pull/79)

Thanks to [@00xc](https://github.com/00xc) for their contribution!


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [RavnOS](https://github.com/ShyanJMC/RavnOS)

RanvOS, (from norwegian; raven), is a operative system programmed in Rust. Aims to be; minimal, stable, secure and modern ( this maybe you know as; state-of-art ).

The objective of RavnOS is make an operative system minimalist, self hosted (no external crates, all programmed by me), stable and secure.

- October updates;
   - [Minor code changes to follow good practices](https://github.com/ShyanJMC/RavnOS/commit/048cf546b0488fca9b4dabc2f9b38b6d93e373e1)
- September updates;
   - [ Rune - Added "du" as built-in. Right now is functional but in the future I will add results sorted by size.](https://github.com/ShyanJMC/RavnOS/commit/abc8d639d51b39c712158b9b3a769b88a0b05b66)
   - [ Rune - Now "ls" built-in recognize and show where points the symbolinc link](https://github.com/ShyanJMC/RavnOS/commit/625f2899fa70b6932cbc7899f84f905ebfecd429)
   - [ Rune - Added show as built-in, expanded "info" built-in with more information. Show - Deleted because now is Rune's built-in](https://github.com/ShyanJMC/RavnOS/commit/b8e016d78943f6c64ae1e77072d5dba343268bca)

### [`mkroening/interrupt-mutex`](https://github.com/mkroening/interrupt-mutex)
<span class="maintainers">(Section written by [@mkroening](https://github.com/mkroening))</span>

Building upon [last month's `interrupts` crate](@/this-month/2023-09/index.md#mkroening-interrupts), I created a mutex for sharing data with interrupt handlers or signal handlers.

`RawInterruptMutex` wraps any [`lock_api::RawMutex`](https://docs.rs/lock_api/0.4.10/lock_api/trait.RawMutex.html), be it a [`parking_lot::RawMutex`](https://docs.rs/parking_lot/0.12.1/parking_lot/struct.RawMutex.html) on Unix or a [`spinning_top::RawSpinlock`](https://docs.rs/spinning_top/0.2.5/spinning_top/struct.RawSpinlock.html) on bare metal.
When such an `InterruptMutex` is locked, interrupts are disabled.
When the `InterruptMutex` is unlocked again, the previous interrupt state is restored.
This does not completely rule out deadlocks, since you can just enable interrupts manually when you should not.
Still, it is very convenient to just change the mutex type of data that is shared with interrupt handlers instead of disabling and enabling interrupts manually on every access.

```rust
// Make a mutex of your choice into an `InterruptMutex`.
type InterruptSpinlock<T> = interrupt_mutex::InterruptMutex<spinning_top::RawSpinlock, T>;

static X: InterruptSpinlock<Vec<i32>> = InterruptSpinlock::new(Vec::new());

fn interrupt_handler() {
    X.lock().push(1);
}

let v = X.lock();
// Raise an interrupt
raise_interrupt();
assert_eq!(*v, vec![]);
drop(v);

// The interrupt handler runs

let v = X.lock();
assert_eq!(*v, vec![1]);
drop(v);
```

### [`mkroening/interrupt-ref-cell`](https://github.com/mkroening/interrupt-ref-cell)
<span class="maintainers">(Section written by [@mkroening](https://github.com/mkroening))</span>

Also building upon [last month's `interrupts` crate](@/this-month/2023-09/index.md#mkroening-interrupts), I created a `RefCell` for sharing data with interrupt handlers or signal handlers on the same thread.

On the same thread (software thread or hardware thread (core)), a compiler fence is sufficient for synchronization with signal handlers (on Unix) and interrupt handlers (on bare metal).
In these cases, the new `InterruptRefCell` allows easy sharing without the overhead of mutexes and without the deadlock potential of mutexes.
Similar to `InterruptMutex`, this is helpful for disabling interrupts on accesses but does not protect you from manually enabling interrupts while holding a reference.

```rust
use interrupt_ref_cell::{InterruptRefCell, LocalKeyExt};
 
thread_local! {
    static X: InterruptRefCell<Vec<i32>> = InterruptRefCell::new(Vec::new());
}
 
fn interrupt_handler() {
    X.with_borrow_mut(|v| v.push(1));
}

X.with_borrow(|v| {
    // Raise an interrupt
    raise_interrupt();
    assert_eq!(*v, vec![]);
});
 
// The interrupt handler runs
 
X.with_borrow(|v| assert_eq!(*v, vec![1]));
```

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
