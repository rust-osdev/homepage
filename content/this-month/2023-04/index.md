+++
title = "This Month in Rust OSDev: April 2023"
date = 2023-05-03

[extra]
month = "April 2023"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (April 2023)" post.
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

_No updates were proposed for this section this month._

## Announcements, News, and Blog Posts

Here we collect news, blog posts, etc. related to OS development in Rust.

<!--
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [Rust Kernel Programming](https://coderjoshdk.github.io/posts/Rust-Kernel-Programming.html)
- [Linux Kernel Rust Modules](https://tomcat0x42.me/linux/rust/2023/04/07/linux-kernel-rust-modules.html)
- [Aero OS: A new modern operating system made in Rust, now able to run the Links browser, Alacritty and much more!](https://www.reddit.com/r/rust/comments/12p2rf7/aero_os_a_new_modern_operating_system_made_in/)
- [Felix, an x86 hobby OS written in Rust](https://www.reddit.com/r/rust/comments/12gxh8b/felix_an_x86_hobby_os_written_in_rust/)


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

#### Features

- [Introducing a high-level FS abstraction](https://github.com/rust-osdev/uefi-rs/pull/472)
- [uefi: debug: add DebugPort protocol](https://github.com/rust-osdev/uefi-rs/pull/755)
- [uefi: Panic if an error is created from Status::SUCCESS](https://github.com/rust-osdev/uefi-rs/pull/749)
- [uefi-macros: Drop !Send and !Sync from unsafe_protocol macro](https://github.com/rust-osdev/uefi-rs/pull/758)
- [cstr[ing]16: convenience functions](https://github.com/rust-osdev/uefi-rs/pull/751)
- [uefi: Clean up some Status -> Result conversions](https://github.com/rust-osdev/uefi-rs/pull/767)
- [Use the uguid crate to replace the `Guid` struct and `guid!` macro](https://github.com/rust-osdev/uefi-rs/pull/777)
- [gop: Derive PartialEq on ModeInfo](https://github.com/rust-osdev/uefi-rs/pull/773)
- [Add RngProtocol to `uefi-raw` and use it from `uefi`](https://github.com/rust-osdev/uefi-rs/pull/778)
- [uefi: Add get_variable_boxed](https://github.com/rust-osdev/uefi-rs/pull/779)

#### Docs

- [uefi: Update Status documentation](https://github.com/rust-osdev/uefi-rs/pull/748)
- [doc: build with --no-deps](https://github.com/rust-osdev/uefi-rs/pull/746)
- [uefi: Minor cleanups in the fs module doc](https://github.com/rust-osdev/uefi-rs/pull/753)

#### Other

- [ci: shorter job names](https://github.com/rust-osdev/uefi-rs/pull/750)
- [uefi: consistent use of crate:: over uefi::](https://github.com/rust-osdev/uefi-rs/pull/752)
- [Allow passing a constant's path into unsafe_protocol](https://github.com/rust-osdev/uefi-rs/pull/760)
- [uefi-raw: Add mostly-empty package](https://github.com/rust-osdev/uefi-rs/pull/761)
- [ci: Fix book token permission](https://github.com/rust-osdev/uefi-rs/pull/763)
- [Move newtype_enum macro to uefi-raw](https://github.com/rust-osdev/uefi-rs/pull/764)
- [uefi-macros: Fix compiler test for Rust 1.69](https://github.com/rust-osdev/uefi-rs/pull/765)
- [Move `Status` to `uefi-raw`, along with related API changes](https://github.com/rust-osdev/uefi-rs/pull/768)

Thanks to [@JohnAZoidberg](https://github.com/JohnAZoidberg) and [@felipebalbi](https://github.com/felipebalbi) for their contributions!

### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

- [Fixed bug stemming from treating an exclusive range as an inclusive ranges](https://github.com/rust-osdev/bootloader/pull/362)
- [Update `uefi` dependency to `v0.20`](https://github.com/rust-osdev/bootloader/pull/360)
- [Implemented sorting of uefi memory maps #315](https://github.com/rust-osdev/bootloader/pull/365)

Thanks to [@kennystrawnmusic](https://github.com/kennystrawnmusic) and [@JarlEvanson](https://github.com/JarlEvanson) for their contributions!


### [`pic8259`](https://github.com/rust-osdev/pic8259)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

- [Added `new_continguous` implementation for `ChainedPics`](https://github.com/rust-osdev/pic8259/pull/4)

Thanks to [@rasheedmhd](https://github.com/rasheedmhd) for their contributions!


### [`x86_64`](https://github.com/rust-osdev/x86_64)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp), [@josephlr](https://github.com/orgs/rust-osdev/people/josephlr), and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

- [Don't use third-party Python libraries in release workflow](https://github.com/rust-osdev/x86_64/pull/421)


### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

- [aml_tester: Add positional file arguments, in-order parsing and shared namespace](https://github.com/rust-osdev/acpi/pull/151)
- [AML: Add minimal CondRefOf support](https://github.com/rust-osdev/acpi/pull/170)
- [acpi: Allow Madt and Mcfg fields to be accessed without allocator_api](https://github.com/rust-osdev/acpi/pull/161)

Thanks to [@A0lson](https://github.com/A0lson), and [@rw-vanc](https://github.com/rw-vanc) for their contributions!


### [`ucs2-rs`](https://github.com/rust-osdev/ucs2-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

- [ci: Bring in various improvements from uefi-rs](https://github.com/rust-osdev/ucs2-rs/pull/14)
- [Switch to 2021 edition](https://github.com/rust-osdev/ucs2-rs/pull/15)



## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

### [`phil-opp/blog_os`](https://github.com/phil-opp/blog_os)
<span class="maintainers">(Section written by [@phil-opp](https://github.com/phil-opp))</span>

We merged the following changes to the [_Writing an OS in Rust_](https://os.phil-opp.com/) blog this month:

- [Update `zola` to `v0.17.2`](https://github.com/phil-opp/blog_os/pull/1209)
- [Fix link syntax](https://github.com/phil-opp/blog_os/pull/1210)
- [fix(translation zh-TW): typo](https://github.com/phil-opp/blog_os/pull/1211) <span class="gray">(thanks to [@GNITOAHC](https://github.com/GNITOAHC)!)</span>
- [The `#[alloc_error_handler]` attribute was removed](https://github.com/phil-opp/blog_os/pull/1216)
- [Update 'Heap Allocation' post to remove `alloc_error_handler`](https://github.com/phil-opp/blog_os/pull/1217)

I also continued writing on the upcoming third edition of the blog. I finished a draft of the second post, which is about booting using `v0.11` of the `bootloader` crate. You can already read it [on GitHub](https://github.com/phil-opp/blog_os/blob/edition-3/blog/content/edition-3/posts/02-booting/index.md) if you like.

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
