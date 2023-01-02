+++
title = "This Month in Rust OSDev: December 2022"
date = 2023-01-06

[extra]
month = "December 2022"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (December 2022)" post.
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

### [Experimental feature gate proposal `interoperable_abi`](https://github.com/rust-lang/rust/pull/105586)

This Rust language proposal suggests to create a new `extern "interop"` ABI as a strict superset of the C ABI.
The goal of this new ABI is to "define a standard way to make calls across high-level languages, passing high-level data types, without dropping to the lowest common denominator of C".
For example, it will define a specific memory representation for strings, tuples, and common standard library types such as `Option` and `Result`.

This new ABI would be very useful for operating system development because there are often multiple executables that need to communicate with each other using a stable ABI.
For example, user-space programs communicate with the kernel using [system calls](https://en.wikipedia.org/wiki/System_call), and with each other porgrams using different forms of [inter-process communication](https://en.wikipedia.org/wiki/Inter-process_communication).
With new `extern "interop"` ABI, these communication boundaries could use safe, higher-level types when both sides are written in Rust.

## Announcements, News, and Blog Posts

<!--
Here we collect news, blog posts, etc. related to OS development in Rust.
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [Kernel/Boot: Cope with a Relocation by a Bootloader in 32-bit x86 Assembly Code](https://phip1611.de/blog/kernel-boot-cope-with-a-relocation-by-a-bootloader-in-32-bit-x86-assembly-code/) \
  In this blogpost, [@phip1611](https://github.com/phip1611) shows you how you can cope with a
  relocation by a bootloader in x86 32-bit assembly code.
- [How Does the “File Size is Smaller Than Mem Size” Optimization Work in GNU ld for the .bss Section?](https://phip1611.de/blog/how-does-the-file-size-is-smaller-than-mem-size-optimization-work-in-gnu-ld/) \
  In this blogpost, [@phip1611](https://github.com/phip1611) explains what properties bring the GNU 
  linker ld to save disk space, as symbols in the `.bss` section do not need to be statically 
  allocated inside the ELF. Symbols in the `.bss` segment are expected to be initialized to all 
  zeroes.
- [GNU ld: Linking .bss into .data to Ensure that Mem Size Equals File Size For Each LOAD Segment (.bss in a PROGBITS Section)](https://phip1611.de/blog/linking-bss-into-data-to-ensure-the-mem-size-equals-file-size-for-each-load-segment-bss-in-a-progbits-section/) \
  In this blogpost, [@phip1611](https://github.com/phip1611) explains what steps you have to do
  that the GNU linker put's all symbols of the `.bss` section "as they are" into the binary
  so that they occupy zeroed memory in the ELF. This is relevant for some very rudimentary 
  ELF loaders that are found in some microkernels to bootstrap their initial process.
- [The Probably Simplest x86 Driver Written in Assembly – Printing to QEMU’s debugcon-Device)](https://phip1611.de/blog/the-probably-simplest-x86-driver-written-in-assembly-printing-to-qemus-debugcon-device/) \
  In this blogpost, [@phip1611](https://github.com/phip1611) codes with you probably the simplest
  driver that one can write in assembly. *This blogpost is from September but wasn't mentioned here 
  earlier.*

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

### [`Andy-Python-Programmer/Aero`](https://github.com/Andy-Python-Programmer/aero)

<span class="maintainers">(Section written by [@Andy-Python-Programmer](https://github.com/Andy-Python-Programmer))</span>

Aero is a new modern, experimental, unix-like operating system following the monolithic kernel design. Supporting modern PC features such as long mode, 5-level paging, and SMP (multicore), to name a few. 

This month, Aero successfully managed to get [`mesa-demos`](https://github.com/freedesktop/mesa-demos) and [`alacritty`](https://github.com/alacritty/alacritty) running.

![Aero](aero.png)

In addition, `deps.sh` script was added to automate the process of installing the required dependencies on the host to build the sysroot.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
