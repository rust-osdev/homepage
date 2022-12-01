+++
title = "This Month in Rust OSDev: November 2022"
date = 2022-12-06

[extra]
month = "November 2022"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (November 2022)" post.
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


## Call for Participation

Want to contribute to a Rust OSDev project, but don't know where to start? Help with one of these outstanding issues!

<!--
    Please use the following template for adding items:
    - [(`repo_name`) Issue Description](https://example.com/link-to-issue)
-->

<!-- <span class="gray">

_No tasks were proposed for this section this month._

</span> -->

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

### [`rust-lang/rust` UEFI news](https://github.com/rust-lang/rust)
<span class="maintainers">(Section written by [@nicholasbishop](https://github.com/nicholasbishop))</span>

The [MCP to raise the three UEFI targets to tier 2](https://github.com/rust-lang/compiler-team/issues/555) by [@dvdhrm](https://github.com/dvdhrm) was recently approved. 
Following that we merged a PR to [dist builds of the UEFI targets](https://github.com/rust-lang/rust/pull/103933) so that you can install them via rustup (e.g. `rustup target add --toolchain nightly x86_64-unknown-uefi`), and a PR to [add an initial QEMU test](
https://github.com/rust-lang/rust/pull/101703) for the x86_64 UEFI target to help prevent regressions from landing.

The initial nightlies containing the prebuilt UEFI targets revealed some issues in `compiler_builtins` which we fixed and are in the [0.1.84](https://github.com/rust-lang/compiler-builtins/compare/0.1.83...0.1.84) release.
Finally, we [changed the C compiler for the UEFI targets from gcc to clang](https://github.com/rust-lang/rust/pull/104622), which resolved some linker problems. 
As of the 2022-11-22 nightly, the three UEFI targets should be fully usable, which means you no longer need to use the unstable `-Zbuild-std` feature.

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

<span class="maintainer">(Section written by [@phip1611](https://github.com/phip1611))</span>

I worked on setting up page tables in a low-level project and noticed that there is lots of room for
confusion. For example, x86 has at least four different modes of paging: 32-bit paging, 32-bit 
paging with PAE, 64-bit 4-level paging, and 64-bit 5-level paging. 32-bit without PAE paging uses 10
bits to index into a page table while 64-bit paging uses 9 bits to index into the table of each
level. To demystify the magic a little, I created a CLI utility called `paging-calculator` that 
takes a virtual address as input and shows you the indices the hardware will use. You can find the
utility on [crates.io](https://crates.io/crates/paging-calculator).

## Other News

<!--
Here we collect other news, blog posts, etc. related to OS development in Rust. Follow this template:

- [Title](https://example.com)

-->


## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
