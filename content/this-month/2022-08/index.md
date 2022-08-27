+++
title = "This Month in Rust OSDev: August 2022"
date = 0000-01-01

[extra]
month = "August 2022"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our [_comment form_](#comment-form) at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (August 2022)" post.
    Feel free to create pull requests against the `next` branch to add your
    content here.
    Please take a look at the past posts on https://rust-osdev.com/ to see the
    general structure of these posts.
-->

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


## Call for Participation

Want to contribute to a Rust OSDev project, but don't know where to start? Help with one of these outstanding issues!

<!--
    Please use the following template for adding items:
    - [(`repo_name`) Issue Description](https://example.com/link-to-issue)
-->

If you maintain a Rust project related to operating system development and are looking for contributors, especially for tasks suited to people getting started in this space, please [create a PR](https://github.com/rust-osdev/homepage/pulls) against the `next` branch with the tasks you want to include in the next issue.

## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

### [`cdrzewiecki/celos`](https://gitlab.com/cdrzewiecki/celos)
<span class="maintainers">(Section written by [@drzewiec](https://github.com/drzewiec))</span>

It's been a while since my last project update! That's partly because life has been busy, but also because this update concerns a huge feature. I'm very pleased to report that I have been able to add preemptive multitasking to CelOS.

This was feature that took a good bit of foundation to be able to implement (hence why it took me so long). I had to spend a good bit of time getting memory allocation (both physical and virtual) into a happier place, as well as add support for ACPI and the APIC. And, of course, there were many snags along the way as I learned (at least some of) the traps that are easy to step into when doing something as delicate as context switching.

Now that I have finished this key feature, I plan to work on setting up the other infrastructure needed to begin writing services in userspace (such as message passing and synchronization primitives). And, hopefully soon, finally make the jump into ring 3!

As always, many thanks to [@phil-opp](https://github.com/phil-opp) for his hard work on supporting the Rust osdev community, and for writing the [apic](https://github.com/rust-osdev/apic) crate which helped serve as a sanity check while I wrote my own driver for the IOAPIC and LAPIC. Thanks as well to the maintainers of the excellent [acpi](https://github.com/rust-osdev/acpi) crate, you guys are doing incredible work out there!

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).



<!--
TODO: Update publication date
-->
