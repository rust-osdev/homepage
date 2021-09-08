+++
title = "This Month in Rust OSDev (August 2021)"
date = 0000-01-01

[extra]
month = "August 2021"
authors = [
    "phil-opp",
    "IsaacWoods",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (August 2021)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

### [`acpi`](https://github.com/rust-osdev/acpi)

The `acpi` repository contains crates for parsing the ACPI tables – data structures that the firmware of modern computers use to relay information about the hardware to the OS.

There was a fair amount of progress this month, including adding support for a major AML feature: buffer fields.
These are objects that represent bit-level slices of buffer objects, allowing a section of a buffer to be read and
written. This requires us to support multiple namespace objects operating on the same underlying data -
currently this is handled simply with spinlocks, but we're looking to improve this situation in the future.
With this, we grew support for the `DefCreateBitField`, `DefCreateByteField`, `DefCreateWordField`,
`DefCreateDWordField`, `DefCreateQWordField`, and `DefCreateField` opcodes, as well as logic for reading and
writing `Integer`s and `Buffer`s out of them. <span class="gray">(published as `aml v0.16.0`)</span>

The `acpi` crate also saw improvement, and unfortunately some more breakage. We now support X2APIC entries
appearing in the MADT, and so have changed the representation of Processor UIDs and Local APIC IDs in
`acpi::platform::Processor` to `u32`, to support the wider IDs that X2APIC uses to support more processors. On
older platforms that only support APIC, the upper bits will simply always be zeroed, and so these values can be
cast down to `u8` when needed. Note that for complex legacy reasons, you cannot rely upon ACPI for detecting
whether a platform has X2APIC support or not, and so whether the MADT actually contained X2APIC entries is not exposed
by this interface - this is by design. <span class="gray">(published as `acpi v4.0.0`)</span>

Other changes were:
- [`aml`: the contents of the `namespace` module were made public](https://github.com/rust-osdev/acpi/pull/107).
  Thanks to [@Andy-Python-Programmer](https://github.com/Andy-Python-Programmer) for this contribution!
- [`aml`: store locals and method arguments in arrays](https://github.com/rust-osdev/acpi/commit/c1597aba3d39344834292637fb81e2f2971d6c04). If you're manually calling AML methods
  that take arguments, you will need to update how you create them.
- [`aml`: support the `DefFatal` opcode](https://github.com/rust-osdev/acpi/commit/514e55df07acbca93dfd4eb2db3cdd6fdea5aaf5). `DefFatal` will cause a panic by default, but this behaviour can be overridden by implementing `Handler::handle_fatal_error`.
- [`aml`: implement the `DefWhile` opcode](https://github.com/rust-osdev/acpi/commit/06409b360ef30b3b08b56865f3ee380315751f14)
- [`aml`: implement the `DefBreak` opcode](https://github.com/rust-osdev/acpi/commit/da5f5cec8096d2ebd5697212e282abbeaed6edb7)
- [`aml`: implement the `DefContinue` opcode](https://github.com/rust-osdev/acpi/commit/ed0400092e18598c73ca6048fb96b2522237808d)
- [`aml`: implement the `DefIncrement` and `DefDecrement` opcodes](https://github.com/rust-osdev/acpi/commit/b854d5491e48e5a4f332ff259ce185cb357261d0)
- [`aml`: implement the `ToInteger` opcode](https://github.com/rust-osdev/acpi/commit/00a61d8b7471dae725283296f4ee9c0c20013156)

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)

The `uefi` crate provides safe and performant wrappers for [UEFI](https://en.wikipedia.org/wiki/Unified_Extensible_Firmware_Interface), the successor to the BIOS.

In August, [@nicholasbishop](https://github.com/nicholasbishop) joined as a co-maintainer of the crate. Welcome!

Many improvements were merged this month:

- [Add `BootServices::exit` method; Make `PointerMode`'s fields public for checking against support of a pointer device](https://github.com/rust-osdev/uefi-rs/pull/261)
- [Implement `PartialEq`+`Eq` for `DevicePath`](https://github.com/rust-osdev/uefi-rs/pull/265)
- [System table can get debug-formatted](https://github.com/rust-osdev/uefi-rs/pull/248)
- [loaded image: add set_image function](https://github.com/rust-osdev/uefi-rs/pull/266)
- [Better `CStr16` to `String` and `str` conversions](https://github.com/rust-osdev/uefi-rs/pull/249)
- [Add a minimal UEFI application template](https://github.com/rust-osdev/uefi-rs/pull/268)
- [Use the built-in `aarch64-unknown-uefi` target](https://github.com/rust-osdev/uefi-rs/pull/269)
- [Make `GraphicsOutput.query_mode(...)` public](https://github.com/rust-osdev/uefi-rs/pull/270)
- [Publish new versions of all the crates](https://github.com/rust-osdev/uefi-rs/pull/276) <span class="gray">(published `uefi v0.12.0`, `uefi-macros v0.4.0`, and `uefi-services v0.9.0`)</span>
- [Add newtype enum for variable vendors and add `IMAGE_SECURITY_DATABASE` to it](https://github.com/rust-osdev/uefi-rs/pull/273)
- [Fix doc link and make CI lints stricter](https://github.com/rust-osdev/uefi-rs/pull/272)
- [Add num_bytes method to `CStr16`](https://github.com/rust-osdev/uefi-rs/pull/274)
- [Add `CString16`](https://github.com/rust-osdev/uefi-rs/pull/275)
- [Enhance `Guid::from_values` and `Guid::fmt`](https://github.com/rust-osdev/uefi-rs/pull/280)
- [Handle panics by unwinding the stack and implement check_event method](https://github.com/rust-osdev/uefi-rs/pull/282)

Thanks to [@HTG-YT](https://github.com/HTG-YT), [@phip1611](https://github.com/phip1611), [@Andy-Python-Programmer](https://github.com/Andy-Python-Programmer), and [@timrobertsdev](https://github.com/timrobertsdev) for their contributions!

### [`x86_64`](https://github.com/rust-osdev/x86_64)

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

In August, we merged the following changes:

- [Rename `XCr0` and `CR4` flags](https://github.com/rust-osdev/x86_64/pull/275)
- [Fix CI tests on Windows](https://github.com/rust-osdev/x86_64/pull/297)
- [Use `#[cfg(doc)]` instead of docs.rs-specific cfg flag](https://github.com/rust-osdev/x86_64/pull/287)
- [Expose `MapperFlush::new` and `MapperFlushAll::new`](https://github.com/rust-osdev/x86_64/pull/296)
- [docs: Update segment register references in `GDT::load*` method to non-deprecated methods](https://github.com/rust-osdev/x86_64/pull/301)
- [docs(idt): remove a panic note](https://github.com/rust-osdev/x86_64/pull/300)
- [Fix typo and doc links in GDT docs](https://github.com/rust-osdev/x86_64/pull/302)

These changes were not published yet, but a pull request for a new version is open already: [Release version 0.14.5](https://github.com/rust-osdev/x86_64/pull/304).

Thanks to [@toku-sa-n](https://github.com/toku-sa-n) and [@ncatelli](https://github.com/ncatelli) for their contributions!

### [`bootloader`](https://github.com/rust-osdev/bootloader)

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables.

This month, we merged the following changes:

- [[v0.9] Set `relocation-model: static` and `panic-strategy: abort` and fix `.intel_syntax` warnings](https://github.com/rust-osdev/bootloader/pull/185) <span class="gray">(published as `v0.9.19`)</span>
- [Fix relocation-model field name in the target spec json](https://github.com/rust-osdev/bootloader/pull/186) <span class="gray">(published as `v0.10.7`)</span>
- [Pad uefi fat file length](https://github.com/rust-osdev/bootloader/pull/180)
- [Also check cfg gated target field for bootloader dependency](https://github.com/rust-osdev/bootloader/pull/182) <span class="gray">(published as `v0.10.8`)</span>
- [Fix typo in link in changelog](https://github.com/rust-osdev/bootloader/pull/194)

Thanks to [@vinc](https://github.com/vinc), [@bjorn3](https://github.com/bjorn3), [@Freax13](https://github.com/Freax13), [@yusdacra](https://github.com/yusdacra), and [@martica](https://github.com/martica) for their contributions!

### [`multboot2`](https://github.com/rust-osdev/multiboot2)

The `multiboot2` crate provides abstraction types for the boot information of multiboot2 bootloaders.

The following changes were merged this month:

- [Unpublic tags bugfix](https://github.com/rust-osdev/multiboot2/pull/89) <span class="gray">(published as `v0.12.1`)</span>
- [Fix `ModuleTag` cmdline](https://github.com/rust-osdev/multiboot2/pull/91)
- [Stricter typing + better split of modules + bugfix](https://github.com/rust-osdev/multiboot2/pull/90)

### [`pic_8259`](https://github.com/rust-osdev/pic8259)

The `pic_8259` crate provides abstractions for 8259 and 8259A Programmable Interrupt Controllers (PICs).

In August, we added cargo features to make the crate buildable on stable:

- [Add `nightly` and `stable` feature flags to enable compilation on stable Rust](https://github.com/rust-osdev/pic8259/pull/1) <span class="gray">(published as `v0.10.2`)</span>

Thanks to [@toku-sa-n](https://github.com/toku-sa-n) for this contribution!

## Call for Participation

Want to contribute to a Rust OSDev project, but don't know where to start? Pick up one of these outstanding
issues in one of our projects and get started!

<!--
Please use the following template for adding items:
- [(`repo_name`) Issue Description](https://example.com/link-to-issue)
-->

<span class="gray">

_No tasks were proposed for this section._

</span>

If you maintain a Rust OSDev project and are looking for contributors, especially for tasks suited to people
getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the
`next` branch with the tasks you want to include in the next issue.


## Personal Projects

In this section, we describe updates to personal projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

We merged the following improvements to the [_Writing an OS in Rust_](https://os.phil-opp.com) blog:

- [Update crate versions across translations](https://github.com/phil-opp/blog_os/pull/1031)
- [Replace MS doc link that linked to a German page](https://github.com/phil-opp/blog_os/pull/1034)
- [Fix link: The const_fn unstable feature no longer exists](https://github.com/phil-opp/blog_os/commit/c1e6a66e356653c166426adbcdbb158792bc408c)
- [Replace fathom analytics with goatcounter](https://github.com/phil-opp/blog_os/commit/27ab4518acbb132e327ed4f4f0508393e9d4d684)
- [fix: typo](https://github.com/phil-opp/blog_os/pull/1040)
- [Fix links in Russian translation](https://github.com/phil-opp/blog_os/pull/1046)

Thanks to [@Foo-x](https://github.com/Foo-x), [@adi-g15](https://github.com/adi-g15), [@Kalbiq](https://github.com/Kalbiq), and [@non-descriptive](https://github.com/non-descriptive) for their contributions!

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).


<!--
TODO: Update publication date
-->
