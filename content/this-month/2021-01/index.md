+++
title = "This Month in Rust OSDev (January 2021)"
date = 0000-01-01

[extra]
month = "January 2021"
authors = [
    "phil-opp",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we will give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (January 2021)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Showcase

We started a new [_Showcase_](https://rust-osdev.com/showcase/) section this month, where we introduce and present interesting Rust OSDev projects. The first post of this section is:

- [The `RustyHermit` Unikernel](https://rust-osdev.com/showcase/rusty-hermit/) written by [@stlankes](https://github.com/stlankes)

If you like to present your project too, just let us know!

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

### [`uart_16550`](https://github.com/rust-osdev/uart_16550)

The `uart_16550` crate provides basic support for serial port I/O for 16550-compatible UARTs. The crate received the following maintenance update in January:

- [Use stabilized `hint::spin_loop` instead of deprecated `atomic::spin_loop_hint`](https://github.com/rust-osdev/uart_16550/commit/cd497a98dabc66ba151218451d07f856950d443d)


### [`cargo-xbuild`](https://github.com/rust-osdev/cargo-xbuild)

The `cargo-xbuild` project provides `cargo` command wrappers to cross-compile the sysroot crates `core` and `alloc`. This month, we fixed an error that occured in combination with the `XARGO_RUST_SRC` environment variable:

- [Ensure copied Cargo.lock is writable](https://github.com/rust-osdev/cargo-xbuild/pull/98) <span class="gray">(published as `v0.6.5`)</span>

Thanks to [@astro](https://github.com/astro) for this contribution!

Even though we still maintain the `cargo-xbuild` crate, we recommend switching to cargo's own `build-std` feature that is always up-to-date with the latest Rust/Cargo changes. We wrote a short guide on how to switch to it, which is available [in our Readme](https://github.com/rust-osdev/cargo-xbuild#alternative-the-build-std-feature-of-cargo).

## Personal Projects

In this section, we describe updates to personal projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

This month, ...

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).


<!--
TODO: Update publication date
-->