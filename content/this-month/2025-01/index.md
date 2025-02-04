+++
title = "This Month in Rust OSDev: January 2025"
date = 2025-02-04

[extra]
month = "January 2025"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (January 2025)" post.
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

- Podcast: [Rust in Production: Volvo Ships Memory-Safe ECUs in Production Cars](https://www.reddit.com/r/rust/comments/1i88zmb/rust_in_production_volvo_ships_memorysafe_ecus_in/)
- [The VEKOS operating system is now able to execute programs](https://www.reddit.com/r/rust/comments/1ieetqt/the_vekos_operating_system_is_now_able_to_execute/)
- Video: [Windows Kernel Programming with Rust - Matthias Heiden | EuroRust 2024](https://www.youtube.com/watch?v=NfBXDEgm6VY)
- [Understanding the Microsoft Pluton security processor](https://techcommunity.microsoft.com/blog/windows-itpro-blog/understanding-the-microsoft-pluton-security-processor/4370413) (uses Rust and [TockOS](https://tockos.org/))
- Linux: [Resistance to Rust abstractions for DMA mapping](https://lwn.net/SubscriberLink/1006805/f75d238e25728afe/)

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

- [Insert null checks for pointer dereferences when debug assertions are enabled](https://github.com/rust-lang/rust/pull/134424)
- [Make missing_abi lint warn-by-default](https://github.com/rust-lang/rust/pull/132397)
- [show linker output even if the linker succeeds](https://github.com/rust-lang/rust/pull/119286)

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

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [uefi-raw: Add FirmwareVolume{,Block}2Protocol](https://github.com/rust-osdev/uefi-rs/pull/1503)
- [uefi-raw: hii: Add Database Protocol](https://github.com/rust-osdev/uefi-rs/pull/1510)
- [uefi-raw: Add ScsiIoProtocol](https://github.com/rust-osdev/uefi-rs/pull/1517)
- [Add missing type/subtype checks to `TryFrom<&DevicePathNode>`](https://github.com/rust-osdev/uefi-rs/pull/1516)
- [uefi-raw: Add common impls for http types](https://github.com/rust-osdev/uefi-rs/pull/1518)
- [relicensing: Rewrite allocator, configuration table, and image unload PRs](https://github.com/rust-osdev/uefi-rs/pull/1523)
- [relicensing: Rewrite set_timer PR](https://github.com/rust-osdev/uefi-rs/pull/1524)
- [Fix memory leaks in DevicePathFromText](https://github.com/rust-osdev/uefi-rs/pull/1525)
- [Add warning to custom memory types](https://github.com/rust-osdev/uefi-rs/pull/1526)
- [test-runner: Clean up device path tests](https://github.com/rust-osdev/uefi-rs/pull/1527)

<!-- - [chore(deps): update crate-ci/typos action to v1.29.4](https://github.com/rust-osdev/uefi-rs/pull/1512) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1515) -->
<!-- - [fix(deps): update rust crate itertools to 0.14.0](https://github.com/rust-osdev/uefi-rs/pull/1513) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1522) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1530) -->

Thanks to [@crawfxrd](https://github.com/crawfxrd) and [@hannahfluch](https://github.com/hannahfluch) for their contributions!


### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following improvements:

- [use threads instead of futures in build.rs](https://github.com/rust-osdev/bootloader/pull/484)
- [Move test kernels to a separate workspace](https://github.com/rust-osdev/bootloader/pull/486)
- [fix condition for running bootloader common tests](https://github.com/rust-osdev/bootloader/pull/487)

### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PRs this month:

- [fix warnings & remove broken CI job](https://github.com/rust-osdev/x86_64/pull/530)
- [Add page attribute table support](https://github.com/rust-osdev/x86_64/pull/529)
- [use default python again](https://github.com/rust-osdev/x86_64/pull/533)
- [feat(msr): add IA32_APIC_BASE support](https://github.com/rust-osdev/x86_64/pull/532)

Thanks to [@hannahfluch](https://github.com/hannahfluch) and [@adavis628](https://github.com/adavis628) for their contributions!


### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS. We merged the following changes this month:

- [acpi: Remove Clone Copy traits for MADT](https://github.com/rust-osdev/acpi/pull/238)
- [acpi: Clone impl for PlatformInfo and ManagedSlice](https://github.com/rust-osdev/acpi/pull/239)
- [aml: fix clippy warnings and run clippy in CI](https://github.com/rust-osdev/acpi/pull/237)
- [Fix unsoundness in our representation of the MADT](https://github.com/rust-osdev/acpi/pull/223)

Thanks to [@IsaacWoods](https://github.com/IsaacWoods), [@mrjbom](https://github.com/mrjbom), and [@00xc](https://github.com/00xc) for their contributions!


### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

_Convenient and safe parsing of Multiboot2 Boot Information (MBI) structures and
the contained information tags. Usable in no_std environments, such as a kernel.
An optional builder feature also allows the construction of the corresponding
structures._

We merged the following PRs this month:

- [fix: typos](https://github.com/rust-osdev/multiboot2/pull/253)
- [misc improvements](https://github.com/rust-osdev/multiboot2/pull/254)

<!-- - [build(deps): bump crate-ci/typos from 1.28.1 to 1.29.0](https://github.com/rust-osdev/multiboot2/pull/252)
- [build(deps): bump bitflags from 2.6.0 to 2.7.0](https://github.com/rust-osdev/multiboot2/pull/255)
- [build(deps): bump bitflags from 2.7.0 to 2.8.0](https://github.com/rust-osdev/multiboot2/pull/256)
- [build(deps): bump crate-ci/typos from 1.29.0 to 1.29.4](https://github.com/rust-osdev/multiboot2/pull/257) -->

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

### [`roeeshoshani/genesis`](https://github.com/roeeshoshani/genesis)
<span class="maintainers">(Section written by [@roeeshoshani](https://github.com/roeeshoshani))</span>

`genesis` is a bare metal firmware implementation for mips. it implements everything from the bottom up, from
initializing the cpu caches, to configuring pci devices and the interrupt controller.

this month, the core async executor was implemented.

this means that we can implement blocking operations (for example reading a byte from the UART) as rust futures, and we then
`.await` them.

this makes our lives much easier when writing code that needs to block until some I/O events happen. instead of using callbacks,
and having to pass our state all over the place, we can just `.await` the blocking future, and write our code using async functions,
which is much more ergonomic.

##### example

currently, there is only one blocking operation implemented, the operation of reading a byte from the UART.

this allows code like the following to be written:
```rust
loop {
    let byte = uart_read_byte().await;
    println!("received uart byte: {}", byte);
}
```

which is a huge improvement over the previous implementation of putting the code inside the UART interrupt handler.

##### how does it work?

the way this works is that the core kernel's main loop looks roughly like the following:
```rust
loop {
    poll_tasks();
    wait_for_interrupt();
}
```

then, the interrupt handler is responsible for waking up the relevant tasks.

futures that need interrupt handlers to wake them up should somehow register themselves, and the interrupt hanlers will then
wake the registered tasks.

then, in the next iteration, the tasks that were woken up will be polled again, which completes the loop.


## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
