+++
title = "This Month in Rust OSDev (February 2021)"
date = 0000-01-01

[extra]
month = "February 2021"
authors = [
    "phil-opp",
    # add yourself here
]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we will give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (February 2021)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Project Updates

In this section, we give an overview of notable changes to the projects hosted under the [`rust-osdev`] organization.

[`rust-osdev`]: https://github.com/rust-osdev/about

### [`x86_64`](https://github.com/rust-osdev/x86_64)

The `x86_64` crate provides various abstractions for `x86_64` systems, including wrappers for CPU instructions, access to processor-specific registers, and abstraction types for architecture-specific structures such as page tables and descriptor tables.

In February, …

## Personal Projects

In this section, we describe updates to personal projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)

<span class="gray">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

This month, ...

### [`cdrzewiecki/celos`](https://gitlab.com/cdrzewiecki/celos)

<span class="gray">(Section written by [@drzewiec](https://github.com/drzewiec))</span>

I have been working on an OS following along with @phil-opp's tutorial series for a while, but recently decided I would rework my OS based on the first edition of the blog (since I preferred to use GRUB as my bootloader). This is the first progress I have to share on CelOS, and indeed the first time I've published one of these updates in general.

In February, I made a lot of great progress on CelOS. I have the complete physical memory (plus the framebuffer provided by GRUB) mapped to virtual memory, and a pixel-based framebuffer working with text output. Things are not very optimized right now (for one thing I'm stretching the `font8x8` font into 8x12), but this is a great first step that I can build on. Next planned steps are:

* Move the kernel in virtual memory so that it occupies the higher half of the 48-bit address space
* Create some page fault interrupt handling so that the kernel can at least attempt to handle page faults (rather than triple faulting as it does now)
* Set up memory allocation for the kernel, to get heap allocation
* Once heap allocation is in place, utilize some existing crate to handle TrueType fonts so that text will look a bit nicer on screen

I probably won't get all of that done in March, but those are my planned next steps. Thanks to this great community and to @phil-opp for being so helpful in the osdev journey!

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).


<!--
TODO: Update publication date
-->
