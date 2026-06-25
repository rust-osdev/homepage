+++
title = "This Month in Rust OSDev: June 2026"
date = 2026-07-05

[extra]
month = "June 2026"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

Please submit interesting posts and projects for the next issue [on Zulip](https://rust-osdev.zulipchat.com/#narrow/channel/435142-newsletter/topic/Content.20suggestions/with/580172810) or via a PR [on GitHub](https://github.com/rust-osdev/homepage/).

<span class="gray">
Disclaimer: Automated scripts and AI assistance were used for collecting and categorizing links.
Everything was proofread and checked manually, with many manual tweaks.
</span>


<!--
    This is a draft for the upcoming "This Month in Rust OSDev (June 2026)" post.
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
- [Announcing Asterinas 0.18.0](https://asterinas.github.io/2026/06/04/announcing-asterinas-0.18.0.html)

<span class="gray">No content was submitted for this section this month.</span>

## Infrastructure and Tooling

In this section, we collect recent updates to `rustc`, `cargo`, and other tooling that are relevant to Rust OS development.

<!--
    Please use the following template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

<span class="gray">No content was submitted for this section this month.</span>

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

<span class="gray">No projects updates were submitted this month.</span>

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->

### [`mkroening/elf-symbols`](https://github.com/mkroening/elf-symbols)
<span class="maintainers">(Section written by [@mkroening](https://github.com/mkroening))</span>

When developing an OS, you often need some information about the loaded kernel image:

- Where has the loader loaded the kernel to?
- How large is the loaded kernel?
- Where do the text segment and the data segment end?
- How do I get the kernel TLS image?

These questions can be answered by building non-relocatable images, by writing custom linker scripts, or by having a custom loader that provides this information somehow.

But there is another way!
In fact, the main ELF linkers that I am aware of ([BFD], [gold], [LLD], [mold], and [Wild]) all have built-in ELF symbols that answer these questions when not using custom linker scripts.
Unfortunately, many are poorly documented, so I created the [elf-symbols] crate that exposes and documents them.
All symbols are tested on the aforementioned linkers.
The documentation shows when each linker gained support for the respective symbol.

The following symbols are straightforward:

- `__executable_start` (`executable_start()`) is the start of the executable.
- `_etext` (`text_end()`) is the end of the text segment.
- `_edata` (`data_end()`) is the end of the data segment.
- `_end` (`executable_end()`) is the end of the executable.

`__ehdr_start` (`elf_header()`) is especially interesting.
It allows programs to examine themselves by reading their ELF headers (file headers and program headers).
This can be used to get TLS image information, for example.

Note that these symbols are ELF specific, though, so they cannot be used when linking to something else, such as a PE32+ UEFI executable.

#### Examples

```rust
println!("Executable start: {:p}", elf_symbols::executable_start());
println!("ELF header:       {:p}", elf_symbols::elf_header());
println!("Text segment end: {:p}", elf_symbols::text_end());
println!("Data segment end: {:p}", elf_symbols::data_end());
println!("Executable end:   {:p}", elf_symbols::executable_end());
```

[BFD]: https://sourceware.org/git/?p=binutils-gdb.git;a=tree;f=ld;h=b1662159cdd15bb857e04e42bd26361c0d406099;hb=5e56594815854de5eca35c7c04b11705d0f19c02
[gold]: https://sourceware.org/git/?p=binutils-gdb.git;a=tree;f=gold;h=ac6272c7bb3ad02524b2ca86a2cf9b68e9ca30ca;hb=5e56594815854de5eca35c7c04b11705d0f19c02
[LLD]: https://github.com/llvm/llvm-project/tree/llvmorg-22.1.8/lld
[mold]: https://github.com/rui314/mold/tree/v2.41.0
[Wild]: https://github.com/wild-linker/wild/tree/0.9.0
[elf-symbols]: https://crates.io/crates/elf-symbols

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way to get in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
