+++
title = "This Month in Rust OSDev: April 2026"
date = 2026-05-05

[extra]
month = "April 2026"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

Please submit interesting posts and projects for the next issue [on Zulip](https://rust-osdev.zulipchat.com/#narrow/channel/435142-newsletter/topic/Content.20suggestions/with/580172810) or via a PR [on GitHub](https://github.com/rust-osdev/homepage/).

<span class="gray">
Disclaimer: Automated scripts and AI assistance were used for collecting and categorizing links.
Everything was proofread and checked manually and there were many manual tweaks.
</span>


<!--
    This is a draft for the upcoming "This Month in Rust OSDev (April 2026)" post.
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

### [`willamhou/hypervisor`](https://github.com/willamhou/hypervisor)
<span class="maintainers">(Section written by [@willamhou](https://github.com/willamhou))</span>

An ARM64 Type-1 bare-metal hypervisor written in Rust (`no_std`). It runs at EL2 and manages guest VMs at EL1, targeting the QEMU `virt` machine. The project boots Linux 6.12.12 to a BusyBox shell with 4 vCPUs, virtio-blk storage, and virtio-net inter-VM networking.

Key features:

- **S-EL2 SPMC**: Runs as BL32 in TF-A boot chain, replacing Hafnium. Manages multiple Secure Partitions (SPs) at S-EL1 with per-SP Secure Stage-2 page tables.
- **FF-A v1.1**: Full proxy implementation including DIRECT_REQ/RESP messaging, memory sharing (MEM_SHARE/LEND/DONATE/RETRIEVE/RELINQUISH/RECLAIM), descriptor fragmentation, PARTITION_INFO_GET, notifications, indirect messaging, and CONSOLE_LOG.
- **pKVM integration**: Coexists with Android pKVM at NS-EL2 — our SPMC at S-EL2, pKVM at NS-EL2. 35/35 `ffa_test.ko` tests pass including SP-to-SP DIRECT_REQ relay and SP-to-SP memory sharing through the real SPMD chain.
- **Multi-VM**: 2 Linux VMs time-sliced with VMID-tagged TLBs and per-VM Stage-2 page tables.
- **SMP**: Both 4-vCPU-on-1-pCPU (round-robin scheduler) and 4-vCPU-on-4-pCPU (1:1 affinity) modes.
- **SP-to-SP**: CallStack cycle detection, recursive dispatch, chain preemption, secure virtual interrupt injection via HCR_EL2.VI.
- **34 test suites** with ~457 assertions running on QEMU, plus 20/20 BL33 E2E integration tests.



## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
