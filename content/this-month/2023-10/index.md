+++
title = "This Month in Rust OSDev: October 2023"
date = 2023-11-05

[extra]
month = "October 2023"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (October 2023)" post.
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

### [`mkroening/interrupt-mutex`](https://github.com/mkroening/interrupt-mutex)
<span class="maintainers">(Section written by [@mkroening](https://github.com/mkroening))</span>

Building upon [last month's `interrupts` crate](@/this-month/2023-09/#mkroening-interrupts), I created a mutex for sharing data with interrupt handlers or signal handlers.

`RawInterruptMutex` wraps any [`lock_api::RawMutex`](https://docs.rs/lock_api/0.4.10/lock_api/trait.RawMutex.html), be it a [`parking_lot::RawMutex`](https://docs.rs/parking_lot/0.12.1/parking_lot/struct.RawMutex.html) on Unix or a [`spinning_top::RawSpinlock`](https://docs.rs/spinning_top/0.2.5/spinning_top/struct.RawSpinlock.html) on bare metal.
When such an `InterruptMutex` is locked, interrupts are disabled.
When the `InterruptMutex` is unlocked again, the previous interrupt state is restored.
This does not completely rule out deadlocks, since you can just enable interrupts manually when you should not.
Still, it is very convenient to just change the mutex type of data that is shared with interrupt handlers instead of disabling and enabling interrupts manually on every access.

```rust
// Make a mutex of your choice into an `InterruptMutex`.
type InterruptSpinlock<T> = interrupt_mutex::InterruptMutex<spinning_top::RawSpinlock, T>;

static X: InterruptSpinlock<Vec<i32>> = InterruptSpinlock::new(Vec::new());

fn interrupt_handler() {
    X.lock().push(1);
}

let v = X.lock();
// Raise an interrupt
raise_interrupt();
assert_eq!(*v, vec![]);
drop(v);

// The interrupt handler runs

let v = X.lock();
assert_eq!(*v, vec![1]);
drop(v);
```

### [`mkroening/interrupt-ref-cell`](https://github.com/mkroening/interrupt-ref-cell)
<span class="maintainers">(Section written by [@mkroening](https://github.com/mkroening))</span>

Also building upon [last month's `interrupts` crate](@/this-month/2023-09/#mkroening-interrupts), I created a `RefCell` for sharing data with interrupt handlers or signal handlers on the same thread.

On the same thread (software thread or hardware thread (core)), a compiler fence is sufficient for synchronization with signal handlers (on Unix) and interrupt handlers (on bare metal).
In these cases, the new `InterruptRefCell` allows easy sharing without the overhead of mutexes and without the deadlock potential of mutexes.
Similar to `InterruptMutex`, this is helpful for disabling interrupts on accesses but does not protect you from manually enabling interrupts while holding a reference.

```rust
use interrupt_ref_cell::{InterruptRefCell, LocalKeyExt};
 
thread_local! {
    static X: InterruptRefCell<Vec<i32>> = InterruptRefCell::new(Vec::new());
}
 
fn interrupt_handler() {
    X.with_borrow_mut(|v| v.push(1));
}

X.with_borrow(|v| {
    // Raise an interrupt
    raise_interrupt();
    assert_eq!(*v, vec![]);
});
 
// The interrupt handler runs
 
X.with_borrow(|v| assert_eq!(*v, vec![1]));
```

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
