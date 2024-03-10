+++
title = "This Month in Rust OSDev: March 2023"
date = 2023-04-08

[extra]
month = "March 2023"
editors = ["phil-opp", "phip1611"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (March 2023)" post.
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

- [Partial stabilization of `once_cell`](https://github.com/rust-lang/rust/pull/105587)
- [Support for Fuchsia RISC-V target](https://github.com/rust-lang/rust/pull/108722)

## Announcements, News, and Blog Posts

Here we collect news, blog posts, etc. related to OS development in Rust.

<!--
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [Klint: Compile-time Detection of Atomic Context Violations for Kernel Rust Code](https://www.memorysafety.org/blog/gary-guo-klint-rust-tools/)
- [Writing a Linux Driver for QEMU’s Debugcon Device](https://phip1611.de/blog/writing-a-linux-driver-for-qemus-debugcon-device/) \
  In this blogpost, [@phip1611](https://github.com/phip1611) shows you can
  write a Linux driver for the QEMU debugcon device. Although, the driver
  still uses C, it is a wonderful example to demonstrate a minimal yet useful
  driver. Additionally, it is a good starting point for a rewrite in Rust, once
  the Rust tooling and API bindings in the kernel are more mature. Perhaps,
  the rewrite in Rust is your next learning project?


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

### [`multiboot2`](https://github.com/rust-osdev/multiboot2)

<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods), [@phip1611](https://github.com/phip1611), [@robert-w-gries](https://github.com/robert-w-gries), [@ahmedcharles](https://github.com/ahmedcharles), and [@Caduser2020](https://github.com/Caduser2020)</span>

The `multiboot2` crate provides abstraction types for the multiboot information
structure (MBI) of multiboot2 bootloaders. The latest release of the
`multiboot2`-crate is now `v0.15.0` (was `v0.14.0`), which fixed a
[bug](https://github.com/rust-osdev/multiboot2/pull/119). Furthermore, the
documentation was improved. However, the biggest change is that the library now
allows the parsing of custom multiboot tags, which are not prohibited by the
spec. For a full changelog, please refer to the
[GitHub repo](https://github.com/rust-osdev/multiboot2/blob/main/multiboot2/Changelog.md).

#### CI Refactoring
In the CI, we want to run many tests that cover a big portion of the cartesian
product of the following properties:
- rust version: stable, nightly, msrv
- type: build, test, style check
- target: default, no_std

As I (@phip1611) was annoyed by all the boilerplate configuration and
repetition, I've investigated new ways to improve that situation and created
a reusable workflow can be used like that:
```yaml
jobs:
  build_msrv:
    name: build (msrv)
    uses: ./.github/workflows/_build-rust.yml
    with:
      rust-version: 1.56.1
      do-style-check: false

  style_nightly:
    name: style (nightly)
    needs: build_nightly
    uses: ./.github/workflows/_build-rust.yml
    with:
      rust-version: nightly
      do-style-check: true
      do-test: false
```

The `./.github/workflows/_build-rust.yml` workflow abstracts setting up the
toolchain, setting up a cargo cache for a faster CI, and, depending on the
configuration, running `cargo test|clippy|doc|build|fmt`. I think that the
outcome is quite nice and might also help others. Feel free to check out the
corresponding [PR](https://github.com/rust-osdev/multiboot2/pull/126).



### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

#### Features

- [debug: add debug implementation for file-related structs](https://github.com/rust-osdev/uefi-rs/pull/624)
- [uefi: Add `delete_variable()` helper](https://github.com/rust-osdev/uefi-rs/pull/682)
- [uefi: Drop `'boot` lifetime from Output protocol](https://github.com/rust-osdev/uefi-rs/pull/689)
- [error: enable `core::error::Error` for all error payloads](https://github.com/rust-osdev/uefi-rs/pull/693)
- [Remove some more protocol lifetime parameters](https://github.com/rust-osdev/uefi-rs/pull/694)
- [uefi: Implement `Borrow`/`ToOwned` for `CString16`/`CStr16`](https://github.com/rust-osdev/uefi-rs/pull/695)
- [drop deprecations that are at least in one release](https://github.com/rust-osdev/uefi-rs/pull/692)
- [Add Sorted Iterator for the UEFI Memory Map (Issue#661)](https://github.com/rust-osdev/uefi-rs/pull/662)
- [Switch to the stable channel :tada: ](https://github.com/rust-osdev/uefi-rs/pull/688)
- [Merge changes from the 0.20 release](https://github.com/rust-osdev/uefi-rs/pull/701)
- [Rename `global_allocator` module and change scope of `global_allocator` feature](https://github.com/rust-osdev/uefi-rs/pull/705)
- [debug everywhere](https://github.com/rust-osdev/uefi-rs/pull/699)
- [cfg: Add ESRT GUID](https://github.com/rust-osdev/uefi-rs/pull/745)

#### Other Improvements

- [uefi-macros: Use a more precise error span](https://github.com/rust-osdev/uefi-rs/pull/679)
- [build(deps): update syn requirement from 1.0.74 to 2.0.4](https://github.com/rust-osdev/uefi-rs/pull/704)
- [Update pull request template](https://github.com/rust-osdev/uefi-rs/pull/709)
- [uefi: Remove static references from `SystemTable` implementation](https://github.com/rust-osdev/uefi-rs/pull/710)
- [Set rust-version = 1.68 in all public packages](https://github.com/rust-osdev/uefi-rs/pull/712)
- [uefi: Fill in some more RuntimeServices fn pointers](https://github.com/rust-osdev/uefi-rs/pull/717)
- [uefi: Add opaque_type macro](https://github.com/rust-osdev/uefi-rs/pull/718)
- [uefi: Consistently set `repr(transparent)` on bitflags](https://github.com/rust-osdev/uefi-rs/pull/719)

#### Docs

- [uefi-macros: Make `entry` example more compatible with stable](https://github.com/rust-osdev/uefi-rs/pull/678)
- [uefi: Update package docstring](https://github.com/rust-osdev/uefi-rs/pull/683)
- [Rework "Building UEFI programs" sections in the readmes](https://github.com/rust-osdev/uefi-rs/pull/680)
- [uefi: Improve clarity of global_allocator](https://github.com/rust-osdev/uefi-rs/pull/684)

#### CI & Testing

<details><summary><em>show changes</em></summary>

- [xtask: Add `--unstable` option to `cargo xtask doc`](https://github.com/rust-osdev/uefi-rs/pull/673)
- [ci: Increase Windows job timeout](https://github.com/rust-osdev/uefi-rs/pull/671)
- [xtask: Enable strict provenance checks in Miri](https://github.com/rust-osdev/uefi-rs/pull/666)
- [ci: Add a nightly_channel job](https://github.com/rust-osdev/uefi-rs/pull/674)
- [xtask: Add option to skip uefi-macros tests](https://github.com/rust-osdev/uefi-rs/pull/677)
- [ci: add spellcheck with "typos"](https://github.com/rust-osdev/uefi-rs/pull/687)
- [xtask: Add OVMF_CODE/OVMF_VARS env vars](https://github.com/rust-osdev/uefi-rs/pull/700)
- [OVMF: enable "cargo xtask run" under NixOS](https://github.com/rust-osdev/uefi-rs/pull/690)
- [add Cargo.lock](https://github.com/rust-osdev/uefi-rs/pull/707)
- [ci: MSRV fixes](https://github.com/rust-osdev/uefi-rs/pull/706)
- [ci: Enable caching](https://github.com/rust-osdev/uefi-rs/pull/702)
- [ci: Drop test_latest_release job](https://github.com/rust-osdev/uefi-rs/pull/708)
- [xtask: Turn off some unnecessary dep features](https://github.com/rust-osdev/uefi-rs/pull/711)
- [Add uefi to workspace members](https://github.com/rust-osdev/uefi-rs/pull/713)
- [dependabot: Ignore patch updates](https://github.com/rust-osdev/uefi-rs/pull/730)
- [dependabot: Fix config syntax](https://github.com/rust-osdev/uefi-rs/pull/732)
- [dependabot: Fix config syntax](https://github.com/rust-osdev/uefi-rs/pull/738)
- [ci: Increase Windows timeout to 10 minutes](https://github.com/rust-osdev/uefi-rs/pull/739)
- [Format `use` consistently](https://github.com/rust-osdev/uefi-rs/pull/743)
- [test-runner: Speculative fix for Windows CI timeout](https://github.com/rust-osdev/uefi-rs/pull/744)

</details>

#### Dependencies

<details><summary><em>show changes</em></summary>

- [xtask: Upgrade to syn 2.0](https://github.com/rust-osdev/uefi-rs/pull/698)
- [build(deps): bump clap from 4.0.26 to 4.1.13](https://github.com/rust-osdev/uefi-rs/pull/722)
- [build(deps): bump tempfile from 3.3.0 to 3.4.0](https://github.com/rust-osdev/uefi-rs/pull/728)
- [build(deps): bump clap from 4.1.13 to 4.2.0](https://github.com/rust-osdev/uefi-rs/pull/740)
- [build(deps): bump tempfile from 3.4.0 to 3.5.0](https://github.com/rust-osdev/uefi-rs/pull/742)

</details><ul></ul>


Thanks to [@hughsie](https://github.com/hughsie), [@nicholasbishop](https://github.com/nicholasbishop), [@JohnAZoidberg](https://github.com/JohnAZoidberg), [@phip1611](https://github.com/phip1611), [@JarlEvanson](https://github.com/JarlEvanson), and [@dependabot[bot]](https://github.com/apps/dependabot) for their contributions!


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

- [Remove unused `doc_cfg` feature](https://github.com/rust-osdev/x86_64/pull/408)
- [Enable `doc_auto_cfg` on `docs.rs` builds](https://github.com/rust-osdev/x86_64/pull/407)
- [run xtest bash shell](https://github.com/rust-osdev/x86_64/pull/409)
- [seal off the `PageSize` trait](https://github.com/rust-osdev/x86_64/pull/404)
- [Add `Descriptor::dpl` const method and use it in GDT construction](https://github.com/rust-osdev/x86_64/pull/410)
- [Set permissions to github workflows](https://github.com/rust-osdev/x86_64/pull/412)
- [Create a Security Policy](https://github.com/rust-osdev/x86_64/pull/415)

Thanks to [@joycebrum](https://github.com/joycebrum) for their contributions!


### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

- [acpi: Improve memory mapping usage](https://github.com/rust-osdev/acpi/pull/134)
- [aml: Add `l_and` parser and opcode](https://github.com/rust-osdev/acpi/pull/157)
- [acpi: Add missing and new fields to GIC CPU interface structure](https://github.com/rust-osdev/acpi/pull/156)
- [Add hack to get `aml` compiling on 32-bit platforms](https://github.com/rust-osdev/acpi/pull/159)
- [aml: Do not require unstable features from rustc that are not used](https://github.com/rust-osdev/acpi/pull/168)
- [acpi: Fix compile error when allocator_api is not enabled](https://github.com/rust-osdev/acpi/pull/160)

Thanks to [@A0lson](https://github.com/A0lson), [@rcerc](https://github.com/rcerc), and [@rw-vanc](https://github.com/rw-vanc) for their contributions!


### [`pci_types`](https://github.com/rust-osdev/pci_types)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

- [pci-pci bridge header](https://github.com/rust-osdev/pci_types/pull/7)
- [remove `Send` requirement from `ConfigRegionAccess`](https://github.com/rust-osdev/pci_types/pull/8)

Thanks to [@devsnek](https://github.com/devsnek) for their contributions!

### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

- [Run cargo update](https://github.com/rust-osdev/bootloader/pull/347)
- [Minor improvements to `BootConfig`](https://github.com/rust-osdev/bootloader/pull/349)
- [Simplified disk builder](https://github.com/rust-osdev/bootloader/pull/320)
- [Release version `0.11.1`](https://github.com/rust-osdev/bootloader/pull/350)
- [Release `v0.11.2`](https://github.com/rust-osdev/bootloader/pull/351)
- [Fix docs.rs build](https://github.com/rust-osdev/bootloader/pull/358)
- [Release `v0.11.3`](https://github.com/rust-osdev/bootloader/pull/359)

Thanks to [@jasoncouture](https://github.com/jasoncouture) for their contributions!

### [`vga`](https://github.com/rust-osdev/vga)
<span class="maintainers">Maintained by [@RKennedy9064](https://github.com/RKennedy9064)</span>

- [Access VGA memory at arbitrary address](https://github.com/rust-osdev/vga/pull/27)
- [Fix bug in `TextModeColor::set_foreground`](https://github.com/rust-osdev/vga/pull/28)

Thanks to [@bendudson](https://github.com/bendudson) for their contributions!

### [`pic8259`](https://github.com/rust-osdev/pic8259)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

- [Minor typo corrections](https://github.com/rust-osdev/pic8259/pull/2)

Thanks to [@Virux](https://github.com/Virux) for their contributions!


### [`volatile`](https://github.com/rust-osdev/volatile)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

- [Update build.yml to use minimal scoped permissions](https://github.com/rust-osdev/volatile/pull/33)
- [Create a Security Policy](https://github.com/rust-osdev/volatile/pull/35)

Thanks to [@joycebrum](https://github.com/joycebrum) for their contributions!

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

### [`phip1611/paging-calculator`](https://github.com/phip1611/paging-calculator)

<span class="maintainers">(Section written by [@phip1611](https://github.com/phip1611))</span>

In the November newsletter, I announced the initial release of my
`paging-calculator` CLI utility. Recently, I released a new version, which now
covers page table indices for x86, x86 with physical address extension (PAE),
x86_64, and x86_64 with 5 levels. For example, just type `$ paging-calculator 0xdeadbeef x86`
and `$ paging-calculator 0xdeadbeef x86 --pae`and compare the result. You can
install it from [crates.io](https://crates.io/crates/paging-calculator) or with
the [`pkgs.paging-calculator` attribute](https://github.com/NixOS/nixpkgs/blob/1a165401fe904b9cd89bd731e6e8372883652c7d/pkgs/development/tools/paging-calculator/default.nix),
if you are a [Nix](https://nixos.org/) user.

![Screenshot: Paging Calculator CLI Utility](screenshot-paging-calculator-x86-pae.png)

### [`xiaoyang-sde/rust-kernel-riscv`](https://github.com/xiaoyang-sde/rust-kernel-riscv)

<span class="maintainers">(Section written by [@xiaoyang-sde](https://github.com/xiaoyang-sde))</span>

[`rust-kernel-riscv`](https://github.com/xiaoyang-sde/rust-kernel-riscv) is an experimental operating system kernel built using Rust's asynchronous programming model to schedule threads in both kernel and user space. This approach allows for more efficient context switching and eliminates the need for allocating a separate kernel stack for each user process. In its current iteration, the kernel provides a basic shell capable of running several executables that demonstrate various kernel mechanisms.

The kernel provides a built-in executor, which manages the scheduling and execution of threads. Threads are executed for a time slice before an exception or interrupt occurs, and then the executor switches to another thread. To give you a better understanding, I included the `async` function that represents the lifetime of a user thread below, and I wrote a detailed [design document](https://github.com/xiaoyang-sde/rust-kernel-riscv#design-document).

```rs
async fn thread_loop(thread: Arc<Thread>) {
    loop {
        let trap_context = thread.state().lock().user_trap_context_mut();
        _enter_user_space(trap_context, thread.satp());

        // Invokes related methods to handle the exception or interrupt,
        // which returns a variant of the `ControlFlow` enum

        match control_flow {
            ControlFlow::Continue => continue,
            ControlFlow::Yield => yield_now().await,
            ControlFlow::Exit(exit_code) => {
                thread.exit(exit_code);
                break;
            }
        }
    }
}
```

The idea behind `rust-kernel-riscv` was inspired by Phil's recent [blog post](https://os.phil-opp.com/async-await/) on using `async/await` in the kernel. Thanks Phil for his invaluable support to the Rust community!

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
