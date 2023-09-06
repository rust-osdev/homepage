+++
title = "This Month in Rust OSDev: August 2023"
date = 2023-09-05

[extra]
month = "August 2023"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (August 2023)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Announcements, News, and Blog Posts

Here we collect news, blog posts, etc. related to OS development in Rust.

- [Redox Summer Of Code: VirtIO drivers - 1](https://www.redox-os.org/news/rsoc-virtio-1/)
- [Redox Summer Of Code: On-demand paging](https://www.redox-os.org/news/kernel-8/)
- [Redox Summer Of Code: On-demand paging II](https://www.redox-os.org/news/kernel-9/)
- [Redox Summer Of Code: Apps and Driver support in Redox OS](https://www.redox-os.org/news/rsoc-2023-eny-1/)

<!--
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->


## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

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


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.


### [`mkroening/take-static`](https://github.com/mkroening/take-static)
<span class="maintainers">(Section written by [@mkroening](https://github.com/mkroening))</span>

I published a tiny new crate, allowing you to get a mutable reference to static items safely (only once, though):

```rust
use take_static::take_static;

take_static! {
    static NUMBER: usize = 5;
}

assert_eq!(NUMBER.take(), Some(&mut 5));
assert_eq!(NUMBER.take(), None);
```

This allows you to easily use statically allocated memory before dynamic memory allocators may be available.
Compared to [`cortex_m::singleton`], `take_static` is thread-safe.
Compared to [`takecell::TakeCell`], `take_static` also supports `!Send` types.

[`cortex_m::singleton`]: https://docs.rs/cortex-m/0.7.7/cortex_m/macro.singleton.html
[`takecell::TakeCell`]: https://docs.rs/takecell/0.1.1/takecell/index.html


### [`hermit-os/kernel`](https://github.com/hermit-os/kernel)
<span class="maintainers">(Section written by [@mkroening](https://github.com/mkroening))</span>

<img width="128" align="right" src="https://raw.githubusercontent.com/hermit-os/.github/47a27de62e8cfc658ddbccb3f00743c7538281ae/img/Hermit.svg" />

The Hermit unikernel project allows you to bundle your Rust application with our library operating system to create a bootable unikernel image.
Hermit is a single-address-space operating system.
Since there is only one application running in the virtual machine, no isolation between applications or between user space and kernel space is necessary.
This reduces system call overhead immensely, since every system call is just a library call.
For more information, see <https://rust-osdev.com/showcase/hermit/>.

The RustyHermit project has been renamed.
We have renamed our GitHub organization from [@hermitcore](https://github.com/hermitcore) to [@hermit-os](https://github.com/hermit-os) and reserved the <http://hermit-os.org> domain.

We have also renamed some of our core projects to reduce confusion:
- [hermit-os/kernel](https://github.com/hermit-os/kernel) is the Hermit kernel.
- [hermit-os/hermit-rs](https://github.com/hermit-os/hermit-rs) provides Rust application support.
- [hermit-os/uhyve](https://github.com/hermit-os/uhyve) is a specialized hypervisor for Hermit.
- [hermit-os/loader](https://github.com/hermit-os/loader) is a bootloader for other platforms, such as QEMU.

We have a new logo!
As hermit crabs occupy empty shells produced by other organisms, the original HermitCore occupied one or several cores on a computer.
Because we migrated to Rust in 2018, our new logo of a hermit crab occupying the Rust logo's [bike gear](https://bugzilla.mozilla.org/show_bug.cgi?id=680521) fits quite nicely with the Rust logo as well as Rust's [Ferris](https://rustacean.net/).

And as always, please come and try Hermit! :)


<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
