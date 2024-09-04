+++
title = "This Month in Rust OSDev: August 2024"
date = 2024-09-04

[extra]
month = "August 2024"
editors = ["phil-opp", "phip1611"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (August 2024)" post.
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

- [Asahi Lina on how Rust helps with API docs in the Linux kernel](https://vt.social/@lina/113056457969145576)
- [You can kick the alpha tires on System76’s Cosmic, a new Linux desktop](https://arstechnica.com/gadgets/2024/08/you-can-kick-the-alpha-tires-on-system76s-cosmic-a-new-linux-desktop/)
- [WhenFS: a Rust FUSE filesystem for your Google Calendar](https://github.com/lvkv/whenfs)
- [Status update on CharlotteOS: a post-unix operating system written in Rust and assembly language](https://www.reddit.com/r/rust/comments/1epkro0/status_update_on_charlotteos_a_postunix_operating/)


## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [add Box::as_ptr and Box::as_mut_ptr methods](https://github.com/rust-lang/rust/pull/129091)
- [Stabilize unsafe extern blocks (RFC 3484)](https://github.com/rust-lang/rust/pull/127921)
- [Stabilize `asm_const`](https://github.com/rust-lang/rust/pull/128570)
- [Stabilize `unsafe_attributes`](https://github.com/rust-lang/rust/pull/128771)
- [Stabilize raw_ref_op (RFC 2582)](https://github.com/rust-lang/rust/pull/127679)
- [Promote riscv64gc-unknown-linux-musl to tier 2](https://github.com/rust-lang/rust/pull/122049)
- [float types: document NaN bit pattern guarantees](https://github.com/rust-lang/rust/pull/129559)
- [Partially stabilize feature(new_uninit)](https://github.com/rust-lang/rust/pull/129401)

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


### [`multiboot2`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

_Convenient and safe parsing of Multiboot2 Boot Information (MBI) structures and
the contained information tags. Usable in no_std environments, such as a kernel. 
An optional builder feature also allows the construction of the corresponding 
structures._

🎉 We are happy to announce release `v0.22` of the `multiboot2` crate. This is a
**major step forward** as all Undefined Behaviour (UB) and a bunch of safety and
memory issues have been removed / fixed. 🎉 Now, all unit tests (and there are
many) **pass Miri**! This was possible due to massive internal refactorings done
in [#226](https://github.com/rust-osdev/multiboot2/pull/226) and [#227](https://github.com/rust-osdev/multiboot2/pull/227).

The quite complex logic to parse all these structures **safe and correctly**
has been moved to a new [`multiboot2-common`](https://docs.rs/multiboot2-common)
crate. This way, the [`multiboot2-header`](https://crates.io/crates/multiboot2-header)
crate can also leverage the new abstractions. A comprehensive writeup about why 
the parsing of Multiboot2 structures and typing them correctly in Rust is much
more complex than one initially might think is written down in 
[`lib.rs` of `multiboot2-common`](https://docs.rs/multiboot2-common).

`multiboot2-header` also got a new release `v0.5` and can now be considered 
safe and free of UB when parsing the corresponding structures. The internal
refactorings only slightly leak to the public interfaces of `multiboot2`
and `multiboot2-header`.

Read more:
- [Changelog of `multiboot2`](https://github.com/rust-osdev/multiboot2/blob/b95b73508925b5484a0cf531a45c9c9ce555d560/multiboot2/Changelog.md)
- [Changelog of `multiboot2-header`](https://github.com/rust-osdev/multiboot2/blob/b95b73508925b5484a0cf531a45c9c9ce555d560/multiboot2-header/Changelog.md)

<details>
<summary>Background - Read More</summary>

The crate grew historically without a centralized design or approach how to work
with memory and pointers. Therefore, many UB way unintentionally created. We 
are happy to get lack of the technical debt. Nevertheless, thanks to everyone
who contributed over the years - sorry if we had to rewrite your code in this 
one! :)
</details>

### [`uefi`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality. Recently, we released 
version `v0.31` with some notable changes:

1. We introduced a completely new API to access boot and runtime services
2. We reorganized the MemoryMap-related types.
3. We reorganized and improved the documentation, and also added documentation
   about how the `std` implementation for UEFI targets compares to `uefi` and
   how both can be used together.

#### 1. New API: Freestanding Functions API Change

We are planning a massive API change and introduced a new replacement API in
release `v0.31`. The old API co-exists and will be marked as 
`#[deprecated]` in `0.32`. The new API will make it easier to call boot or 
runtime services without having to pass `&BootServices` around all the time.

The typical use-case of our library users is to write a UEFI bootloader
application that heavily leverages boot services. Then control is handed over to
the next stage, which is typically the kernel of an OS. Boot services are
usually exited either just prior to launching the kernel or by the kernel
itself. In any case, the UEFI application spends almost all of its time with
boot services active.

Although the old API design ensures via the type system that no boot
services can be called after they have been exited, the test of time has proven
that this adds unjustified complexity without bringing much real value add.

Instead, with the new API, which we provide **additionally** at this point,
one can use freestanding functions which are behind the new modules:

- `uefi::system`: is a new module that provides freestanding functions for
  accessing fields of the global system table.
- `uefi::boot`:
  is a new module that provides freestanding functions for boot services using
  the global system table.
- `uefi::runtime`: is a new module that provides freestanding functions for
  runtime services using the global system table.

The freestanding functions itself are close to the originals ones but without
`&BootServices` or `&RuntimeServices` parameters, as you can see for example
[here](https://github.com/rust-osdev/uefi-rs/pull/1344/files#diff-46f1e3c04d719fff471faf35c4d218430e1d664ac0a0fab9d2c15870c2d0f066).

The new API design solves API inconsistencies and restrictions already existing
so far, and makes the overall handling a lot easier. This comes with the costs
that functions may panic, if the boot services were exited but one tries to use
them. However, the massive API simplification justifies this.

Find more and follow the progress and discussions on:
- [the GitHub Issue](https://github.com/rust-osdev/uefi-rs/issues/893#issuecomment-2241610633).
- [The function Migration Guide](https://github.com/rust-osdev/uefi-rs/blob/main/docs/funcs_migration.md)

#### 2. Memory Map Refactoring

TL;DR: What used to return `MemoryMap<'buf>` in the API, now returns
`MemoryMapOwned`. Additionally, you can parse a chunk of memory using
`MemoryMapRef` or `MemoryMapRefMut`.

We put significant effort into refactoring our abstractions for the UEFI memory 
map. These started in release v0.29 and were finalized in release v0.31. 
Instead of one `MemoryMap<'buf>` type, we now have the traits `MemoryMap` and 
`MemoryMapMut` along with the implementations `MemoryMapRef`, `MemoryMapRefMut`, 
and `MemoryMapOwned`. It is recommended to work with the specific 
implementations, as the main purpose for the traits is only to enforce a 
consistent API for these three implementations. This gives users the 
flexibility to cover all possible use cases one can have with an UEFI memory 
map. Read more:
- [#1175](https://github.com/rust-osdev/uefi-rs/pull/1175)
- [#1251](https://github.com/rust-osdev/uefi-rs/pull/1251)
- [#1240](https://github.com/rust-osdev/uefi-rs/pull/1240)
- [#1263](https://github.com/rust-osdev/uefi-rs/pull/1263)

In any case, obtaining the memory map from UEFI is hidden behind the
public API of the crate, but we made it very easy to read/parse it in all
possible scenarios!

#### 3. Documentation Reorganization

We put notable work into our README and `lib.rs` files to improve the
structure of our documentation. The main value-add and improvement is that we 
clearly, directly, and precisely talk about:

- What is `uefi`?
- Which problems does it solve?
- How can it be used?
- How does it compare to the ecosystem, such as `std` for Rust targets?

To see an example how `uefi` and `std` can be used together, please head to
[our repository](https://github.com/rust-osdev/uefi-rs/tree/main/uefi-std-example).

#### Other

There were also a ton of other interesting fixes, changes, and additions! 
Check out our [Changelog](https://github.com/rust-osdev/uefi-rs/blob/main/uefi/CHANGELOG.md).

We merged the following PRs this month:

<details><summary>Click to expand</summary>


- [boot: Add freestanding version of raise_tpl](https://github.com/rust-osdev/uefi-rs/pull/1276)
- [boot: Add freestanding connect_controller and disconnect_controller](https://github.com/rust-osdev/uefi-rs/pull/1278)
- [nix/niv: update dependencies](https://github.com/rust-osdev/uefi-rs/pull/1279)
- [boot: Add freestanding exit function](https://github.com/rust-osdev/uefi-rs/pull/1283)
- [uefi: Fix lifetimes in device_path TryFrom<&[u8]> impls](https://github.com/rust-osdev/uefi-rs/pull/1282)
- [release: uefi-0.30.0](https://github.com/rust-osdev/uefi-rs/pull/1285)
- [uefi: Add release data to 0.30.0 release](https://github.com/rust-osdev/uefi-rs/pull/1286)
- [doc: unified catchy new Crate introduction [doc: 1/N]](https://github.com/rust-osdev/uefi-rs/pull/1284)
- [boot: Add freestanding version of `create_event`](https://github.com/rust-osdev/uefi-rs/pull/1280)
- [Merge release-v0.30 branch into main](https://github.com/rust-osdev/uefi-rs/pull/1289)
- [boot: Add freestanding stall](https://github.com/rust-osdev/uefi-rs/pull/1296)
- [boot: Add freestanding check_event](https://github.com/rust-osdev/uefi-rs/pull/1295)
- [doc: move misc stuff from README to lib.rs [doc: 2/N] ](https://github.com/rust-osdev/uefi-rs/pull/1290)
- [Restore some memory exports to uefi::table::boot](https://github.com/rust-osdev/uefi-rs/pull/1299)
- [Remove dead_code workarounds](https://github.com/rust-osdev/uefi-rs/pull/1302)
- [runtime: Add freestanding set_virtual_address_map](https://github.com/rust-osdev/uefi-rs/pull/1301)
- [Add freestanding set_timer and wait_for_event](https://github.com/rust-osdev/uefi-rs/pull/1298)
- [Add freestanding {install,reinstall,uninstall}_protocol_interface functions](https://github.com/rust-osdev/uefi-rs/pull/1300)
- [boot: Add freestanding close_event](https://github.com/rust-osdev/uefi-rs/pull/1304)
- [boot: Add freestanding install_configuration_table](https://github.com/rust-osdev/uefi-rs/pull/1306)
- [boot: Add freestanding version of protocols_per_handle](https://github.com/rust-osdev/uefi-rs/pull/1305)
- [misc small improvements](https://github.com/rust-osdev/uefi-rs/pull/1308)
- [boot: Add freestanding test_protocol](https://github.com/rust-osdev/uefi-rs/pull/1310)
- [boot: Add freestanding set_watchdog_timer](https://github.com/rust-osdev/uefi-rs/pull/1311)
- [boot: Add freestanding memory_map](https://github.com/rust-osdev/uefi-rs/pull/1312)
- [boot: Add freestanding create_event_ex](https://github.com/rust-osdev/uefi-rs/pull/1313)
- [boot: Add freestanding get_handle_for_protocol](https://github.com/rust-osdev/uefi-rs/pull/1314)
- [Add doc on freestanding function migration](https://github.com/rust-osdev/uefi-rs/pull/1315)
- [boot: Add freestanding locate_device_path](https://github.com/rust-osdev/uefi-rs/pull/1316)
- [boot: Add freestanding locate_handle and find_handles](https://github.com/rust-osdev/uefi-rs/pull/1321)
- [boot: Add freestanding get_image_file_system](https://github.com/rust-osdev/uefi-rs/pull/1322)
- [boot: Add freestanding exit_boot_services](https://github.com/rust-osdev/uefi-rs/pull/1325)
- [uefi: Add table::system_table_raw](https://github.com/rust-osdev/uefi-rs/pull/1323)
- [boot: Add freestanding register_protocol_notify](https://github.com/rust-osdev/uefi-rs/pull/1324)
- [doc: add comprehensive About section to lib.rs [doc: 3/N]](https://github.com/rust-osdev/uefi-rs/pull/1291)
- [uefi: Drop BootServices arg from device path <-> text conversions](https://github.com/rust-osdev/uefi-rs/pull/1327)
- [Revert "uefi: Drop BootServices arg from device path <-> text conversions"](https://github.com/rust-osdev/uefi-rs/pull/1328)
- [Update funcs_migration doc](https://github.com/rust-osdev/uefi-rs/pull/1329)
- [uefi: add BootPolicy type](https://github.com/rust-osdev/uefi-rs/pull/1326)
- [ci: cancel obsolete runs automatically + streamline](https://github.com/rust-osdev/uefi-rs/pull/1332)
- [LoadFileProtocol and LoadFile2Protocol](https://github.com/rust-osdev/uefi-rs/pull/1297)
- [doc: Comparison to Ecosystem (including Rust std impl) [doc: 4/N]](https://github.com/rust-osdev/uefi-rs/pull/1292)
- [release: uefi-macros-0.15.0, uefi-raw-0.7.0, uefi-0.31.0](https://github.com/rust-osdev/uefi-rs/pull/1330)
- [uefi: Inline the template example into lib.rs doc](https://github.com/rust-osdev/uefi-rs/pull/1338)
- [release: uefi-0.31.0](https://github.com/rust-osdev/uefi-rs/pull/1339)
- [doc: Talk about Documentation Organization/Overview [doc: 5/N]](https://github.com/rust-osdev/uefi-rs/pull/1293)
- [uefi std: add example and add book chapter](https://github.com/rust-osdev/uefi-rs/pull/1331)
- [uefi: Drop BootServices arg from device path <-> text conversions](https://github.com/rust-osdev/uefi-rs/pull/1340)
- [test-runner: Convert all examples to new style](https://github.com/rust-osdev/uefi-rs/pull/1342)
- [uefi: Drop BootServices arg from GraphicsOutput::modes](https://github.com/rust-osdev/uefi-rs/pull/1344)
- [Update the uefi::allocator module to use the global system table](https://github.com/rust-osdev/uefi-rs/pull/1343)
- [doc: final README streamlining [doc: 6/6]](https://github.com/rust-osdev/uefi-rs/pull/1294)
- [uefi: Drop BootServices arg from ComponentName::open](https://github.com/rust-osdev/uefi-rs/pull/1345)
- [ci: release package check](https://github.com/rust-osdev/uefi-rs/pull/1341)
- [book: Update protocols how-to to use the `boot` module](https://github.com/rust-osdev/uefi-rs/pull/1347)
- [uefi: Move various types to the `uefi::boot` module](https://github.com/rust-osdev/uefi-rs/pull/1346)
- [ci: Use `cargo xtask fmt --check`](https://github.com/rust-osdev/uefi-rs/pull/1348)
- [uefi: Move various types to the `uefi::runtime` module](https://github.com/rust-osdev/uefi-rs/pull/1349)
- [uefi: Make FileSystem work with both variants of ScopedProtocol](https://github.com/rust-osdev/uefi-rs/pull/1352)
- [uefi: Fix compilation of minimal example](https://github.com/rust-osdev/uefi-rs/pull/1353)
- [uefi-macros: Rename generated entry arguments](https://github.com/rust-osdev/uefi-rs/pull/1350)
- [book: Update boot_stages and tables](https://github.com/rust-osdev/uefi-rs/pull/1351)
- [uefi: Drop args from main in the example](https://github.com/rust-osdev/uefi-rs/pull/1355)
- [uefi-macros: Use uefi::boot::set_image_handle](https://github.com/rust-osdev/uefi-rs/pull/1354)
- [template: Drop args to main](https://github.com/rust-osdev/uefi-rs/pull/1356)
- [uefi-macros: Use raw pointer for system table when generating args](https://github.com/rust-osdev/uefi-rs/pull/1357)
- [uefi: Update logger to use the global system table](https://github.com/rust-osdev/uefi-rs/pull/1358)
- [uefi: Update panic handler to use the global system table](https://github.com/rust-osdev/uefi-rs/pull/1359)
- [uefi: Deprecate RuntimeServices](https://github.com/rust-osdev/uefi-rs/pull/1365)
- [uefi: Use global system table in MemoryMapBackingMemory](https://github.com/rust-osdev/uefi-rs/pull/1361)
- [uefi: Update println to use the global system table](https://github.com/rust-osdev/uefi-rs/pull/1360)
- [uefi: Update FS docstring example code](https://github.com/rust-osdev/uefi-rs/pull/1368)
- [uefi: Copy 'Accessing Protocols' docs to uefi::boot](https://github.com/rust-osdev/uefi-rs/pull/1369)
- [uefi: Deprecate BootServices](https://github.com/rust-osdev/uefi-rs/pull/1367)
- [uefi/mem: Update docs referring to BootServices](https://github.com/rust-osdev/uefi-rs/pull/1370)
- [uefi: Update input protocol docs](https://github.com/rust-osdev/uefi-rs/pull/1371)
- [uefi: Update ResetNotification protocol docs](https://github.com/rust-osdev/uefi-rs/pull/1372)
- [uefi: Update LoadedImage protocol docs](https://github.com/rust-osdev/uefi-rs/pull/1373)
- [uefi: Deprecate ancillary types in uefi::table::boot](https://github.com/rust-osdev/uefi-rs/pull/1374)
- [uefi: Update pointer protocol docs](https://github.com/rust-osdev/uefi-rs/pull/1376)
- [uefi: Deprecate table::{system_table_boot,system_table_runtime}](https://github.com/rust-osdev/uefi-rs/pull/1378)
- [uefi: Remove BootServices from more docstrings](https://github.com/rust-osdev/uefi-rs/pull/1377)
- [uefi: Drop references to SystemTable from docstrings](https://github.com/rust-osdev/uefi-rs/pull/1380)
- [docs: Update timeline for the API migration](https://github.com/rust-osdev/uefi-rs/pull/1382)
- [uefi: Deprecate SystemTable](https://github.com/rust-osdev/uefi-rs/pull/1379)
- [Revert "ci: check if crate can be packaged"](https://github.com/rust-osdev/uefi-rs/pull/1384)
- [uefi: Deprecate SystemTableView, Boot, and Runtime](https://github.com/rust-osdev/uefi-rs/pull/1385)
- [Move PAGE_SIZE to uefi-raw and reexport from uefi boot modules](https://github.com/rust-osdev/uefi-rs/pull/1383)
- [uefi: Clean up imports of uefi::table::runtime](https://github.com/rust-osdev/uefi-rs/pull/1386)


<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1277) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.23.6](https://github.com/rust-osdev/uefi-rs/pull/1287) -->
<!-- - [fix(deps): update rust crate regex to v1.10.6](https://github.com/rust-osdev/uefi-rs/pull/1288) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1319) -->
<!-- - [fix(deps): update rust crate serde_json to v1.0.124](https://github.com/rust-osdev/uefi-rs/pull/1318) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1333) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1336) -->
<!-- - [fix(deps): update rust crate syn to v2.0.76](https://github.com/rust-osdev/uefi-rs/pull/1335) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1364) -->
<!-- - [chore(deps): update crate-ci/typos action to v1.24.1](https://github.com/rust-osdev/uefi-rs/pull/1366) -->

</details>

### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following improvements:

- [Fixed "jc fail" instructions not working properly and updated README.md](https://github.com/rust-osdev/bootloader/pull/453)

Thanks to [@spencer3035](https://github.com/spencer3035) for their contributions!


### [`x86_64`](https://github.com/rust-osdev/x86_64)

<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

We merged the following PR this month:

- [remove `#![feature(asm_const)]`](https://github.com/rust-osdev/x86_64/pull/496)


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

<span class="gray">No projects updates were submitted this month.</span>

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
