+++
title = "This Month in Rust OSDev: July 2023"
date = 2023-08-06

[extra]
month = "July 2023"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (July 2023)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

## Announcements, News, and Blog Posts

Here we collect news, blog posts, etc. related to OS development in Rust.

<!--
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->


## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

    ### Summary
    <span class="maintainers">(Section written by [@author](https://github.com/author))</span>

    <text>
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

### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@phip1611](https://github.com/phip1611)</span>

The [multiboot2](https://crates.io/crates/multiboot2) was bumped from `0.16.0` 
to `0.17.0`. The new release includes the builder pattern for the MBI builder 
and the ability to use custom memory types in the memory map in addition to 
pre-defined ones. For more info, look [here](https://docs.rs/multiboot2/0.17.0/multiboot2/struct.MemoryAreaTypeId.html).


## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.


<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


### [`SFBdragon/talc`](https://github.com/SFBdragon/talc)
<span class="maintainers">(Section written by [@SFBdragon](https://github.com/SFBdragon))</span>

`Talc` is a recently published, fast, and flexible `no-std` memory allocator. 
* It's the fastest allocator I've tested as of yet (galloc falls short, buddy_alloc is close but lacks heap efficiency).
* It features a OOM-handling component with dynamic arena resizing.

By the time you're seeing this, hopefully v2 should be out or coming soon:
* The OOM handler system has been made more powerful.
* `lock_api` is used to allow for custom allocator synchronization.
* The internals and API has been improved to pass miri's stacked borrows validation.
* You can now move the allocator struct around freely.
* And more :3

I hope you find it useful!

### [`vinc/moros`](https://github.com/vinc/moros)
<span class="maintainers">(Section written by [@vinc](https://github.com/vinc))</span>

[MOROS](http://moros.cc) is a text-based hobby operating system targeting computers with a x86-64 architecture and a BIOS.

Since last month's [release](https://github.com/vinc/moros/releases/tag/v0.10.0), I focused on adding new syscalls to interact with [network sockets](https://github.com/vinc/moros/pull/512) from userspace. The DNS and HTTP clients are now using the new UDP and TCP sockets.

I also added another syscall to poll multiple handles at the same time, to read from the console and a socket, improving the main network tool that can now be used as a simple chat program.

The VGA driver, the filesystem, and the editor got a few significant [improvements](https://github.com/vinc/moros/blob/trunk/CHANGELOG.md) as well, to support downloading and reading larger files.

### [`valibali/cluu`](https://github.com/valibali/cluu)
<span class="maintainers">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

There is a new project featured in our [Showcase](@/showcase/_index.md) series:

- [**CLUU (Compact Lightweight Unix Utopia)**](@/showcase/cluu/index.md) by [@valibali](https://github.com/valibali)



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
