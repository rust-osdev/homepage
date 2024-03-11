+++
title = "This Month in Rust OSDev: October 2022"
date = 2022-11-07

[extra]
month = "October 2022"
editors = ["phil-opp", "IsaacWoods"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (October 2022)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
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

### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

- [Change `Event` to be FFI-safe using `NonNull`](https://github.com/rust-osdev/uefi-rs/pull/507)
- [Add `guid!` macro](https://github.com/rust-osdev/uefi-rs/pull/536)
- [Add Guid methods to convert to/from byte arrays](https://github.com/rust-osdev/uefi-rs/pull/535)
- [Fixes and improvements for `Revision`](https://github.com/rust-osdev/uefi-rs/pull/529)
- [Replace `UnalignedCStr16` with `UnalignedSlice`](https://github.com/rust-osdev/uefi-rs/pull/539)
- [Add new `DeviceSubType` values](https://github.com/rust-osdev/uefi-rs/pull/537)
- [Fix clippy warnings](https://github.com/rust-osdev/uefi-rs/pull/538)
- [uefi-services: Change panic handler log message.](https://github.com/rust-osdev/uefi-rs/pull/526)

Thanks to [@timrobertsdev](https://github.com/timrobertsdev) and [@raccog](https://github.com/raccog) for their contributions!

<!--
- [Copy license file to uefi-macros and uefi-services](https://github.com/rust-osdev/uefi-rs/pull/520)
- [Fix clippy lint by removing unnecessary cast](https://github.com/rust-osdev/uefi-rs/pull/525)
- [Update documentation section of readme](https://github.com/rust-osdev/uefi-rs/pull/519)
- [Fix warning from unneeded `macro_use`](https://github.com/rust-osdev/uefi-rs/pull/527)
- [Add option to disable network tests](https://github.com/rust-osdev/uefi-rs/pull/528)
- [Tweak pull request template wrapping](https://github.com/rust-osdev/uefi-rs/pull/533)
- [ci: Simplify some jobs with ubuntu-22.04 runner](https://github.com/rust-osdev/uefi-rs/pull/532)
-->


### [`linked-list-allocator`](https://github.com/rust-osdev/linked-list-allocator)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@jamesmunns](https://github.com/jamesmunns)</span>

- [Consider regions that lead to very small back paddings as unsuitable](https://github.com/rust-osdev/linked-list-allocator/pull/71)
  - Fixes a potential [memory leak](https://github.com/rust-osdev/linked-list-allocator/issues/66) that was introduced with `v0.10`.
- [Add a random action fuzzer](https://github.com/rust-osdev/linked-list-allocator/pull/69)
  - Runs randomized tests against the `allocate_first_fit`, `deallocate`, and `extend` methods
  - This is the fuzzer that was used to discover the [possible out-of-bounds writes in versions `<=0.10.1`](https://github.com/rust-osdev/linked-list-allocator/security/advisories/GHSA-xg8p-34w2-j49j)
- [Run new cargo-fuzz job on CI with time limit](https://github.com/rust-osdev/linked-list-allocator/pull/72)
  - Fuzzes each commit and PR for 5 minutes as a guard against regressions
  - The CI job is also run on schedule every day to test against the latest Rust nightly and to increase the chance of finding improbably bugs over time.

Thanks to [@evanrichter](https://github.com/evanrichter) for their contribution!

### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@rybot666](https://github.com/rybot666), and [@64](https://github.com/64)</span>

We merged the following changes this month:

- [Update `noto-sans-mono-bitmap` to `v0.2.0`](https://github.com/rust-osdev/bootloader/pull/267)
  - now supports more unicode ranges, including the fallback character `�`
  - code ranges are optional via cargo features to minimize binary size
- [implement read-only relocations](https://github.com/rust-osdev/bootloader/pull/269)
  - implements `GNU_RELRO` program header handling to make relocations read-only
- [allow booting without a UEFI graphics output](https://github.com/rust-osdev/bootloader/pull/268)
  - don't error if no UEFI framebuffer is detected
  - this was merged into the `next` branch for the upcoming `v0.11` release

Thanks to [@phip1611](https://github.com/phip1611), and [@Freax13](https://github.com/Freax13) for their contributions!

### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

This month, we began work to remove `acpi`'s dependence on the `alloc` crate for allocations, instead directly
using the [`core::alloc::Allocator`](https://doc.rust-lang.org/beta/core/alloc/trait.Allocator.html) API. This
improves support for bootloaders and kernels that want to treat memory allocation as fallible, allowing them to
more gracefully recover from out-of-memory situations. In the future, we hope that this support will be brought
into `alloc`, so that the library can return to using the standard containers.

These changes are not yet finished or released, but a new major version will likely be out in December with these
changes, as well as better support for PCI topologies with multiple segment groups, and a few other changes. If
you'd like to make use of these new features as they're developed, they're already in `main`, so you can use a Git
dependency.

- [Rework crate to work without allocations](https://github.com/rust-osdev/acpi/pull/131)
- [Add Debug implementations](https://github.com/rust-osdev/acpi/pull/128)

Thanks to [@semiviral](https://github.com/semiviral), and [@mattfbacon](https://github.com/mattfbacon) for their contributions!

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

### [`d-sonuga/blog-blasterball`](https://github.com/d-sonuga/blog-blasterball/)
<span class="maintainers">(Section written by [@d-sonuga](https://github.com/d-sonuga))</span>

[Demilade Sonuga's Blog](https://d-sonuga.netlify.app/) is a new blog on writing a
simple blasterball game with no OS, no engine and minimal external crates. The blog's aim is to take a Rust
beginner from starting the project to finishing it and understanding everything involved in the project's
creation.

The project is still in it's early stages, so lots of feedback is very much needed.

If you're interested, please take a look, and send your thoughts to [@d_sonuga](https://twitter.com/d_sonuga/)
on Twitter.

### Talk on Windows Linked Lists in safe and idiomatic Rust
<span class="maintainers">(Section written by [@ColinFinck](https://github.com/ColinFinck))</span>

At the first ever EuroRust conference from 13th to 14th October, Colin Finck gave a talk about the challenges of developing his [`nt-list`](https://github.com/ColinFinck/nt-list) crate.
The crate provides a safe and idiomatic Rust wrapper around the Windows variant of Linked Lists.
It was presented on this blog [in July](/this-month/2022-07/#nt-list-windows-linked-lists-in-idiomatic-rust).

A [recording](https://www.youtube.com/watch?v=IxhZIyXOIw8) of the talk has just been uploaded.
Check it out, and also check out the [other videos](https://www.youtube.com/channel/UCR3gXcme1HMK7_TrUaNZOqw/videos) from that conference for some great takes on Rust!

[![Preview of video: Windows linked lists in safe and idiomatic Rust - Colin Finck - EuroRust 2022](nt-list-video.jpg "Preview of video: Windows linked lists in safe and idiomatic Rust - Colin Finck - EuroRust 2022")](https://www.youtube.com/watch?v=IxhZIyXOIw8)

### [`MaderNoob/galloc`](https://github.com/MaderNoob/galloc)
<span class="maintainers">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

The new [`good_memory_allocator`](https://crates.io/crates/good_memory_allocator) crate implements a `no_std`-compatible linked list allocator, inspired by the [`dlmalloc`](https://gee.cs.oswego.edu/dl/html/malloc.html) algorithm. It stores an additional `usize` of metadata per allocation, which decreases memory efficiency, but increased runtime efficiency. The `README` includes promising [benchmark results](https://github.com/MaderNoob/galloc#benchmarks) that compare the crate against the [`linked-list-allocator`](https://github.com/rust-osdev/linked-list-allocator) and [`simple-chunk-allocator`](https://github.com/phip1611/simple-chunk-allocator) crates.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)
<span class="maintainers">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

We merged the following changes to the [_Writing an OS in Rust_](https://os.phil-opp.com/) blog this month:

- [Korean translation of _Testing_ post](https://github.com/phil-opp/blog_os/pull/1135)
  - Read it at <https://os.phil-opp.com/ko/testing/>
  - Thanks to [@JOE1994](https://github.com/JOE1994) for creating the translation and to [@SNOOPYOF](https://github.com/SNOOPYOF) and [@dalinaum](https://github.com/dalinaum) for reviewing!
- [Korean translation of _CPU Exceptions_ post](https://github.com/phil-opp/blog_os/pull/1162)
  - Read it at <https://os.phil-opp.com/ko/cpu-exceptions/>
  - Thanks to [@JOE1994](https://github.com/JOE1994) for creating the translation and to [@KimWang906](https://github.com/KimWang906) for reviewing!
- [French translation of _A Minimal Rust Kernel_ post](https://github.com/phil-opp/blog_os/pull/1144)
  - Read it at <https://os.phil-opp.com/fr/minimal-rust-kernel/>
  - Thanks to [@TheMimiCodes](https://github.com/TheMimiCodes) and [@maximevaillancourt](https://github.com/maximevaillancourt) for creating the translation and to [@alaincao](https://github.com/alaincao) for reviewing (and for [fixing links](https://github.com/phil-opp/blog_os/pull/1166))!
- [Update zola to v0.16.1](https://github.com/phil-opp/blog_os/pull/1147)
- [Fixes bad URL from `post-09` address calculation section](https://github.com/phil-opp/blog_os/pull/1146) <span class="gray">(thanks to [@seewishnew](https://github.com/seewishnew))</span>
- [Remove warning output from QEMU command](https://github.com/phil-opp/blog_os/pull/1151) <span class="gray">(thanks to [@lovemeforareason](https://github.com/lovemeforareason))</span>

## Other News

- [Rust UEFI Firmware Targets Promoted To Tier-2 Status](https://www.phoronix.com/news/Rust-UEFI-Promoted-Tier-2)
- [Microsoft seems to ship Rust code in Windows Font Parsing (dwrite)](https://twitter.com/dwizzzleMSFT/status/1578532292662005760)
- Announcement of [COSMIC Text](https://github.com/pop-os/cosmic-text), a pure Rust library for font shaping, layout, and rendering
  - Based on [`rustybuzz`](https://github.com/RazrFalcon/rustybuzz) and [`swash`](https://github.com/dfrg/swash), with custom layout and font fallback implementations
  - Developed for the upcoming Rust-based desktop environment for [Pop_OS](https://pop.system76.com/) and also usable for [Redox OS](https://www.redox-os.org/)

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
