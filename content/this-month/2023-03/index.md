+++
title = "This Month in Rust OSDev: March 2023"
date = 2023-04-06

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


## Announcements, News, and Blog Posts

Here we collect news, blog posts, etc. related to OS development in Rust.

<!--
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

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

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
