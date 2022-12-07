+++
title = "This Month in Rust OSDev: November 2022"
date = 2022-12-07

[extra]
month = "November 2022"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

We have some new sections this month, we hope you like the content!

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (November 2022)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

### UEFI Targets are now Tier 2
<span class="maintainers">(Section written by [@nicholasbishop](https://github.com/nicholasbishop))</span>

The [MCP to raise the three UEFI targets to tier 2](https://github.com/rust-lang/compiler-team/issues/555) by [@dvdhrm](https://github.com/dvdhrm) was recently approved.
Following that we merged a PR to [dist builds of the UEFI targets](https://github.com/rust-lang/rust/pull/103933) so that you can install them via rustup (e.g. `rustup target add --toolchain nightly x86_64-unknown-uefi`), and a PR to [add an initial QEMU test](
https://github.com/rust-lang/rust/pull/101703) for the x86_64 UEFI target to help prevent regressions from landing.

The initial nightlies containing the prebuilt UEFI targets revealed some issues in `compiler_builtins` which we fixed and are in the [0.1.84](https://github.com/rust-lang/compiler-builtins/compare/0.1.83...0.1.84) release.
Finally, we [changed the C compiler for the UEFI targets from gcc to clang](https://github.com/rust-lang/rust/pull/104622), which resolved some linker problems.
As of the 2022-11-22 nightly, the three UEFI targets should be fully usable, which means you no longer need to use the unstable `-Zbuild-std` feature.

## Announcements, News, and Blog Posts

- [Redox OS 0.8.0](https://www.redox-os.org/news/release-0.8.0/)
- <span class="gray">\[Talk\]</span> [FerrOS: Rust-y unikernels on seL4](https://www.youtube.com/watch?v=osepBlSQjY8)
- [Aero, a new modern OS made in rust and is now able to run Xorg](https://www.reddit.com/r/rust/comments/ytrpss/aero_a_new_modern_os_made_in_rust_and_is_now_able/)
- [Tales of the M1 GPU](https://asahilinux.org/2022/11/tales-of-the-m1-gpu/)
  - Asahi Lina shares her experience with writing a Linux kernel driver for the Apple M1 GPU in Rust.
- [Memory Safe Languages in Android 13](https://security.googleblog.com/2022/12/memory-safe-languages-in-android-13.html)
  - There are now ~1.5 million lines of Rust code in AOSP.
  - "Based on this historical vulnerability density, it’s likely that using Rust has already prevented hundreds of vulnerabilities from reaching production."
- [Rust Developers Move Ahead With Preparing To Upstream More Code Into The Linux Kernel](https://www.phoronix.com/news/More-Rust-Upstream-Prep-Linux)

<!--
Here we collect news, blog posts, etc. related to OS development in Rust.
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->


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
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

Just a few days ago, we finally [released](https://github.com/rust-osdev/bootloader/pull/293) version `0.11.0` of the the `bootloader` crate. This release is a [major rewrite](https://github.com/rust-osdev/bootloader/pull/232) with various new features and also breaking changes:

- **Separate API crate:** The bootloader is now split into two parts: A [`bootloader_api`](https://docs.rs/bootloader_api/0.11.0/bootloader_api/) crate to make kernels loadable by the bootloader and the actual bootloader implementation. This makes the build process for kernels much easier and faster.
- **New config system:** Instead of configuring the bootloader via a special table in the `Cargo.toml`, the configuration now happens through a normal Rust struct, which is part of the [`entry_point!` macro](https://docs.rs/bootloader_api/0.11.0/bootloader_api/macro.entry_point.html). The macro then serializes the config struct at compile time and places it in a special ELF output section. The compile time serialization happens through a manually implemented `const fn` of the config struct.
- **Load the kernel at runtime:** Up to version `0.10`, the bootloader used to link the kernel at compile time, which required recompiling the bootloader whenever the kernel was modified. In `v0.11`, we now load both the kernel and the configuration at runtime, so no rebuilding of the bootloader is needed anymore.
- **Split into sub-crates:** Since the bootloader build process does not need access to the kernel executable or its `Cargo.toml` anymore, we can build the different parts of the bootloader independently. For example, the BIOS boot sector is now a separate crate, and the UEFI bootloader is too. (We plan to make them proper [artifact dependencies](https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies) as soon as they're allowed on crates.io.)
- **Library to create disk images:** To create an abstraction the complex build steps of the different bootloader executables, we compile them inside cargo build scripts. At the top level, we provide a [`bootloader`](https://docs.rs/bootloader/0.11.0/bootloader/) _library_ crate, which compiles everything as part of its build script. This library includes functions for creating BIOS and UEFI disk images for a given kernel. These functions can be used e.g. from a builder crate or a build script of the downstream operating system.

See our [README](https://github.com/rust-osdev/bootloader/blob/main/README.md) for detailed usage instructions. We also created [migration guides](https://github.com/rust-osdev/bootloader/tree/main/docs/migration) that explain how to update from `v0.9` and `v0.10`.

Thanks a lot to the numerous people that tested our beta releases and reported issues!


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

#### Features

- [Make the `cstr16!` macro usable in const contexts](https://github.com/rust-osdev/uefi-rs/pull/544)
- [const fn for trivial getters where possible](https://github.com/rust-osdev/uefi-rs/pull/545)
- [Support ISA-specific `MemoryAttribute`s in `MemoryDescriptor`s](https://github.com/rust-osdev/uefi-rs/pull/549)
- [Allow uefi-services to work when the "logger" feature is disabled in uefi](https://github.com/rust-osdev/uefi-rs/pull/552)
- [Unicode collation](https://github.com/rust-osdev/uefi-rs/pull/551)
- [Add structs for all device path node types and a new `DevicePathBuilder`](https://github.com/rust-osdev/uefi-rs/pull/547)
- [Implement additional `BootServices` functions](https://github.com/rust-osdev/uefi-rs/pull/550)
- [Publish new releases](https://github.com/rust-osdev/uefi-rs/pull/571)
- [const fn wherever possible](https://github.com/rust-osdev/uefi-rs/pull/546)
- [rename crate features](https://github.com/rust-osdev/uefi-rs/pull/561)
- [Several small improvements to EqStrUntilNul](https://github.com/rust-osdev/uefi-rs/pull/580)
- [`TryFrom<core::ffi::CStr>` implementation for `CStr8`](https://github.com/rust-osdev/uefi-rs/pull/581)
- [`Directory::read_entry_boxed` plus common abstraction `make_boxed`](https://github.com/rust-osdev/uefi-rs/pull/559)
- [Implement Error trait for `uefi::Error`](https://github.com/rust-osdev/uefi-rs/pull/587)
- [Added `read_entry_boxed_in` and `get_boxed_info_in` that use the `allocator_api`](https://github.com/rust-osdev/uefi-rs/pull/584)

#### Docs

- [Add uefi/README.md](https://github.com/rust-osdev/uefi-rs/pull/577)
- [Install UEFI targets via rustup](https://github.com/rust-osdev/uefi-rs/pull/555)
- [book: Add a graphics example](https://github.com/rust-osdev/uefi-rs/pull/586)
- [documentation and code improvements for Status, Error, and read()](https://github.com/rust-osdev/uefi-rs/pull/556)
- [Clean up crate feature list documentation](https://github.com/rust-osdev/uefi-rs/pull/589)
- [Add `unstable` feature and improve CI for feature flags](https://github.com/rust-osdev/uefi-rs/pull/590)
- [Doc updates: add "Running on Hardware" to book and drop BUILDING.md](https://github.com/rust-osdev/uefi-rs/pull/588)

#### Internal

- [Update `mbrman` to 0.5.0](https://github.com/rust-osdev/uefi-rs/pull/543)
- [Don't explicitly require compiler-builtins(-mem)](https://github.com/rust-osdev/uefi-rs/pull/534)
- [Add temporary workaround for yanked dependency](https://github.com/rust-osdev/uefi-rs/pull/574)
- [workspace: uefi (main library) is in a dedicated directory now](https://github.com/rust-osdev/uefi-rs/pull/566)
- [Copy LICENSE to the new uefi directory](https://github.com/rust-osdev/uefi-rs/pull/576)
- [Improve clap help for `--target`](https://github.com/rust-osdev/uefi-rs/pull/578)
- [several unrelated cleanups](https://github.com/rust-osdev/uefi-rs/pull/562)
- [uefi-test-runner: Assume that we're running in the special QEMU env](https://github.com/rust-osdev/uefi-rs/pull/579)
- [Simplify serial usage in test-runner and check that tests completed](https://github.com/rust-osdev/uefi-rs/pull/582)
- [clippy: require must_use_candidate lint](https://github.com/rust-osdev/uefi-rs/pull/592)
- [doc: add '--document-private-items' to `cargo xtask doc`](https://github.com/rust-osdev/uefi-rs/pull/569)
- [test-runner: Make some tests stricter](https://github.com/rust-osdev/uefi-rs/pull/595)
- [test-runner: Open serial device in exclusive mode](https://github.com/rust-osdev/uefi-rs/pull/598)
- [Make GOP test work on aarch64](https://github.com/rust-osdev/uefi-rs/pull/599)

<!--
#### Other, less notable changes
- [Fix `cargo xtask test` help text](https://github.com/rust-osdev/uefi-rs/pull/542)
- [Fix location of a changelog item](https://github.com/rust-osdev/uefi-rs/pull/548)
- [fs-tests: doc improvements](https://github.com/rust-osdev/uefi-rs/pull/558)
- [changelog: add publish date to all versions](https://github.com/rust-osdev/uefi-rs/pull/567)
- [update PUBLISHING.md](https://github.com/rust-osdev/uefi-rs/pull/568)
- [build(deps): update nix requirement from 0.25.0 to 0.26.1](https://github.com/rust-osdev/uefi-rs/pull/597)
-->

Thanks to [@blitz](https://github.com/blitz), [@YtvwlD](https://github.com/YtvwlD), [@timrobertsdev](https://github.com/timrobertsdev), [@NathanRoyer](https://github.com/NathanRoyer), and [@d-sonuga](https://github.com/d-sonuga) for their contributions!



### [`linked-list-allocator`](https://github.com/rust-osdev/linked-list-allocator)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@jamesmunns](https://github.com/jamesmunns)</span>

We merged the following tooling changes this month to make the crate more reliable:

- [Detect fragmentation when fuzzing](https://github.com/rust-osdev/linked-list-allocator/pull/73)
- [Fix miri test failures caused by address unleaking of heap data in tests](https://github.com/rust-osdev/linked-list-allocator/pull/75)

Thanks to [@evanrichter](https://github.com/evanrichter) for their contribution!

### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

- [Improve `PciConfigRegions` API](https://github.com/rust-osdev/acpi/pull/132)

Thanks to [@semiviral](https://github.com/semiviral) for their contribution!


## Call for Participation

Want to contribute to a Rust OSDev project, but don't know where to start? Help with one of these outstanding issues!

<!--
    Please use the following template for adding items:
    - [(`repo_name`) Issue Description](https://example.com/link-to-issue)
-->

<span class="gray">

_No tasks were proposed for this section this month._

</span>

If you maintain a Rust project related to operating system development and are looking for contributors, especially for tasks suited to people getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the `next` branch with the tasks you want to include in the next issue.

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

### [`bendudson/EuraliOS`](https://github.com/bendudson/EuraliOS)
<span class="maintainers">(Section written by [@bendudson](https://github.com/bendudson))</span>

EuraliOS is a hobby multitasking operating system written in
Rust. It's based on a microkernel ("Merriwig") that provides on-demand
paging, stack and heap memory management for multi-threaded user
processes. Drivers run in Ring 3 and communication between processes
is by Rendezvous message passing. Each process can have its own
virtual file system, enabling multiple users to be isolated from each
other.

This still has many rough edges and doesn't have many drivers:
EuraliOS only has ramdisk storage, but does have a TCP stack thanks to
the [smoltcp](https://github.com/smoltcp-rs/smoltcp) crate. The only
user programs are a simple shell and a
[Gopher](https://en.wikipedia.org/wiki/Gopher_(protocol)) client; I'm
trying to port the [kibi](https://github.com/ilai-deutel/kibi) text
editor but have a lot of work to do on the standard library.

This was based on [Phil's blog](https://os.phil-opp.com/) and uses
many rust-osdev crates including
[x86_64](https://github.com/rust-osdev/x86_64),
[bootloader](https://github.com/rust-osdev/bootloader) and
[vga](https://github.com/rust-osdev/vga). Thanks to Phil and
Rust-OSdev contributors for all their work supporting this community!

I've tried to
[document](https://github.com/bendudson/EuraliOS#documentation) the
development steps and hope these are useful for others, particularly
the sections on getting into Ring 3, implementing syscalls and
switching stacks with `swapgs`. Suggestions for improvement welcome!

### [`hermitcore/hermit-sync`](https://github.com/hermitcore/hermit-sync)
<span class="maintainers">(Section written by [@mkroening](https://github.com/mkroening))</span>

While working on the [`libhermit-rs`] kernel, I moved the synchronization primitives targeted at kernel development into a separate crate: [`hermit-sync`].
Whilst I was there, I also enhanced the code and made it independent from libhermit, so the code can now be used in other projects as well; at least x86_64 and aarch64 ones for the moment.

The crate contains the following features:

* a function for running a closure without interrupts
* a simple spinlock with exponential backoff
* a fair ticket lock with exponential backoff
* `OnceCell` and `Lazy` based on [`generic_once_cell`] (another recent project of mine)

All of these primitives are available with and without interrupt safety.
See the [API docs] for more details.

Any feedback is very welcome of course.

[`libhermit-rs`]: https://github.com/hermitcore/libhermit-rs
[`hermit-sync`]: https://github.com/hermitcore/hermit-sync
[`generic_once_cell`]: https://github.com/mkroening/generic_once_cell
[API docs]: https://docs.rs/hermit-sync

### [`phip1611/paging-calculator`](https://github.com/phip1611/paging-calculator)

<span class="maintainers">(Section written by [@phip1611](https://github.com/phip1611))</span>

I worked on setting up page tables in a low-level project and noticed that there is lots of room for
confusion. For example, x86 has at least four different modes of paging: 32-bit paging, 32-bit
paging with PAE, 64-bit 4-level paging, and 64-bit 5-level paging. 32-bit without PAE paging uses 10
bits to index into a page table while 64-bit paging uses 9 bits to index into the table of each
level. To demystify the magic a little, I created a CLI utility called `paging-calculator` that
takes a virtual address as input and shows you the indices the hardware will use. You can find the
utility on [crates.io](https://crates.io/crates/paging-calculator).


## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
