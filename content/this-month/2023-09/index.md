+++
title = "This Month in Rust OSDev: September 2023"
date = 2023-10-05

[extra]
month = "September 2023"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (September 2023)" post.
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

- [Toolchain Upgrade - Redox OS](https://www.redox-os.org/news/toolchain-upgrade-1/)

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

### [`mkroening/interrupts`](https://github.com/mkroening/interrupts)
<span class="maintainers">(Section written by [@mkroening](https://github.com/mkroening))</span>

I created a dependency-free `interrupts` crate, allowing you to temporarily disable interrupts on AArch64, 64-bit RISC-V, and x86-64.
Two different paradigms allow you to run code without interrupts and synchronize with interrupt handlers running on the same hardware thread (core):

Use [`disable`] to disable interrupts with a guard:

```rust
// interrupts may or may not be enabled
let guard = interrupts::disable();
// interrupts are disabled
drop(guard);
// interrupts are restored to the previous state
```

Use [`without`] (similar to [`x86_64::instructions::interrupts::without_interrupts`]) to run a closure with disabled interrupts:

```rust
// interrupts may or may not be enabled
interrupts::without(|| {
    // interrupts are disabled
});
// interrupts are restored to the previous state
```

I would appreciate you dropping by and giving it a try. :)

[`disable`]: https://docs.rs/interrupts/latest/interrupts/fn.disable.html
[`without`]: https://docs.rs/interrupts/latest/interrupts/fn.without.html
[`x86_64::instructions::interrupts::without_interrupts`]: https://docs.rs/x86_64/latest/x86_64/instructions/interrupts/fn.without_interrupts.html

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
