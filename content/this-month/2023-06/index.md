+++
title = "This Month in Rust OSDev: June 2023"
date = 2023-07-05

[extra]
month = "June 2023"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (June 2023)" post.
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


## Announcements, News, and Blog Posts

Here we collect news, blog posts, etc. related to OS development in Rust.

- [Redox OS elects its first Board of Directors](https://www.redox-os.org/news/board-meeting-2023-06-21/)

<!--
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

## `rust-osdev` Projects

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

<!--
    Please use the following template:

    ### [`repo_name`](https://github.com/rust-osdev/repo_name)
    <span class="maintainers">Maintained by [@maintainer_1](https://github.com/maintainer_1)</span>

    The `repo_name` crate ...<<short introduction>>...

    We merged the following changes this month:
    <<changelog, either in list or text form>>
-->


### [`volatile`](https://github.com/rust-osdev/volatile)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

The `volatile` crate provides a safe wrapper type for implementing volatile read and write operations. This is useful for accessing memory regions that have side-effects, such as memory-mapped hardware registers or framebuffers.

Unfortunately, the design crate had a [soundness issue](https://github.com/rust-osdev/volatile/pull/13#issuecomment-842455552) because it used reference types for accessing the volatile memory. This is problematic because the Rust compiler marks references are _"dereferencable"_, which allows LLVM to insert spurious read operations. This is fine for accessing "normal" memory, but it can lead to correctness issues when used in combination with memory-mapped device registers. These registers look like normal memory, but they are actually accessing some device-specific registers, which might change at any time. So they might change between spurious reads, which violates the assumptions of LLVM and can lead to undefined behavior.

To solve this issue, we started [a full redesign of the crate](https://github.com/rust-osdev/volatile/pull/22) that uses raw pointers only. This solves the issue because raw pointers are not considered _"dereferencable"_, so LLVM is not allowed to insert spurious reads. While we started working on the new design more than 2 years ago, we didn't merge it until this month because we weren't sure about the implementation details. The main discussion point was whether we should treat the proposed `VolatilePointer` type like Rust's reference types or like Rust's raw pointer types. The difference is that raw pointers implement the `Copy` trait, but are not `Send` to prevent aliasing. References, on the other hand, can safely implement `Send` because they're only `Copy` if the reference is read-only.

After a lot of back and forth, we decided to [provide both options](https://github.com/rust-osdev/volatile/pull/29) and finally publish [**`volatile v0.5`**](https://docs.rs/volatile/0.5.1/volatile/index.html). So the new design has two wrapper types, [`VolatilePtr`](https://docs.rs/volatile/0.5.1/volatile/struct.VolatilePtr.html) (behaves like a raw pointer) and [`VolatileRef`](https://docs.rs/volatile/0.5.1/volatile/struct.VolatileRef.html) (behaves like a reference). We hope that we support most use cases this way!

Note that there is also some ongoing discussion [about a potential `VolatileCell` type](https://github.com/rust-osdev/volatile/issues/31) to wrap values in-place. Most implementations of such a type would require support from the Rust compiler, which [needs an RFC](https://github.com/rust-lang/unsafe-code-guidelines/issues/411). However, there is one [promising design based on zero-sized types and proc-macros](https://github.com/rust-osdev/volatile/issues/31#issuecomment-1606044353) by [@Freax13](https://github.com/Freax13) that should not require any new language features. We will continue to investigate.

### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods), [@phip1611](https://github.com/phip1611), [@robert-w-gries](https://github.com/robert-w-gries), and [@ahmedcharles](https://github.com/ahmedcharles)</span>

The [multiboot2](https://crates.io/crates/multiboot2) and the [multiboot2-header](https://crates.io/crates/multiboot2)
crates got a big overhaul. The list of new features includes but is not limited to:
- use DSTs for tags where applicable ([#134](https://github.com/rust-osdev/multiboot2/pull/134))
- model the MBI as DST ([#155](https://github.com/rust-osdev/multiboot2/pull/155))
- add a runtime builder for an MBI ([#133](https://github.com/rust-osdev/multiboot2/pull/133)) \
  Huge thanks to [YtvwlD / Niklas](https://github.com/YtvwlD) for this greate external contribution
- added an integration test including a multiboot2 chainloader for better test coverage ([#129](https://github.com/rust-osdev/multiboot2/pull/129))
- added miri to the CI for more memory safety ([#128](https://github.com/rust-osdev/multiboot2/pull/128))
- several fixes and small improvements

`multiboot2` was updated from `0.15.1` to `0.16.0` and `multiboot2-header` was updated from `0.2.0` to `0.3.0`. Both 
releases come with a large amount of [breaking changes](https://github.com/rust-osdev/multiboot2/blob/main/Changelog.md). 
However, after a sensible consideration, they are all worth it for a more streamlined API and more memory safety.

[`rust-osdev`]: https://github.com/rust-osdev/about

<!--
    Please use the following template:

    ### [`repo_name`](https://github.com/rust-osdev/repo_name)
    <span class="maintainers">Maintained by [@maintainer_1](https://github.com/maintainer_1)</span>

    The `repo_name` crate ...<<short introduction>>...

    We merged the following changes this month:
    <<changelog, either in list or text form>>
-->


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`hermitcore/rusty-hermit`](https://github.com/hermitcore/rusty-hermit)
<span class="maintainers">(Section written by [@mkroening](https://github.com/mkroening))</span>

The Hermit library operating system allows you to bundle a whole OS directly with your application, creating a freestanding, bootable _Unikernel_ image.
This month, we achieved several milestones for reaching more users with Hermit:

- Stable Rust support.

  You can now compile your programs for Hermit using the stable Rust toolchain! 🥳
  
  While the Hermit targets (`x86_64-unknown-hermit` and `aarch64-unknown-hermit`) are still at tier 3, we now distribute pre-built artifacts of the Rust standard library for use with stable toolchains!
  This means, no more `-Zbuild-std`, resulting in faster builds, and the bliss of the stable Rust compiler.
  
  See [`hermitcore/rust-std-hermit`](https://github.com/hermitcore/rust-std-hermit) for details on our `rust-std` artifacts.

- Windows support.

  Thanks to Rust's awesome cross-compilation capabilities, you can now compile Hermit applications from anywhere! 😎

  We have resolved a longstanding issue when building Hermit applications on Windows ([hermitcore/rusty-hermit#431](https://github.com/hermitcore/rusty-hermit/pull/431)).
  The issue is all sorted out now and Windows, macOS, and Linux are tested and verified by our CI.
  

- AArch64 (ARM64) support.

  You can now run real applications on AArch64, with scheduling, network and everything! 🤯

  - PCI now works on AArch64, which allows us to use the network devices ([hermitcore/libhermit-rs#748](https://github.com/hermitcore/libhermit-rs/pull/748)).

  - We merged scheduling support for AArch64 ([hermitcore/libhermit-rs#765](https://github.com/hermitcore/libhermit-rs/pull/765)).

  - You can also now chainload Hermit apps for AArch64 using our loader ([hermitcore/rusty-loader#201](https://github.com/hermitcore/rusty-loader/pull/201)).
    
    This means, you don't have to statically compile the full application into the final loader image anymore.

We'd love if you gave Hermit a try. Just start with our "Hello, World!" application template: [`hermitcore/rusty-demo`](https://github.com/hermitcore/rusty-demo).

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
