+++
title = "This Month in Rust OSDev: December 2025"
date = 2026-01-06

[extra]
month = "December 2025"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (December 2025)" post.
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

- [Xous: A Pure-Rust Rethink of the Embedded Operating System](https://media.ccc.de/v/39c3-xous-a-pure-rust-rethink-of-the-embedded-operating-system)
  - Homepage: <https://xous.dev/>
  - Repo: <https://github.com/betrusted-io/xous-core>
- [Announcing Asterinas 0.17.0](https://asterinas.github.io/2025/12/19/announcing-asterinas-0.17.0.html)
- [The state of the kernel Rust experiment](https://lwn.net/SubscriberLink/1050174/6b6d55c90ce1100f/)
- [ratatui v0.30 released with `no-std` support](https://ratatui.rs/highlights/v030/#no_std-support)

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

### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp) and [@Freax13](https://github.com/orgs/rust-osdev/people/Freax13)</span>

The `bootloader` crate implements a custom Rust-based bootloader for easy loading of 64-bit ELF executables. This month, we merged the following changes:

- [fix buffer overflow in `DiskAccess::read_exact_into`](https://github.com/rust-osdev/bootloader/pull/524)
- [Code changes to update to 2024 edition](https://github.com/rust-osdev/bootloader/pull/526)
- [fix: some typo](https://github.com/rust-osdev/bootloader/pull/527)
- [Fix create-disk-image links](https://github.com/rust-osdev/bootloader/pull/528)

Thanks to [@spencer3035](https://github.com/spencer3035), [@Taxrosdev](https://github.com/Taxrosdev), and [@oxyzenQ](https://github.com/oxyzenQ) for their contributions!



### [`ovmf-prebuilt`](https://github.com/rust-osdev/ovmf-prebuilt)
<span class="maintainers">Maintained by [@nicholasbishop](https://github.com/nicholasbishop) and [@phil-opp](https://github.com/phil-opp)</span>

The `ovmf-prebuilt` project provides pre-built [edk2](https://github.com/tianocore/edk2) releases to make it easier to set up OVMF. We merged the following improvement this month:


- [release: 0.2.5](https://github.com/rust-osdev/ovmf-prebuilt/pull/240)
- [build-edk2: Install python3-dev](https://github.com/rust-osdev/ovmf-prebuilt/pull/247)
- [Upgrade loongarch64 toolchain](https://github.com/rust-osdev/ovmf-prebuilt/pull/248)
- [build-edk2: Drop IA32 support](https://github.com/rust-osdev/ovmf-prebuilt/pull/249)
- [build-edk2: Update aarch64 install path](https://github.com/rust-osdev/ovmf-prebuilt/pull/250)
- [Update readme to note ia32 is only available in older releases](https://github.com/rust-osdev/ovmf-prebuilt/pull/251)
- [release: 0.2.6 with edk2-stable202511-r1](https://github.com/rust-osdev/ovmf-prebuilt/pull/252)
- [ovmf-prebuilt: Add release availability info to Arch docstring](https://github.com/rust-osdev/ovmf-prebuilt/pull/253)

<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/237) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/238) -->
<!-- - [build(deps): get rid of unnecessary flate2 dependency](https://github.com/rust-osdev/ovmf-prebuilt/pull/239) -->
<!-- - [chore(deps): update actions/checkout action to v6](https://github.com/rust-osdev/ovmf-prebuilt/pull/233) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/241) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/242) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/243) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/ovmf-prebuilt/pull/255) -->

Thanks to [@fogti](https://github.com/fogti) for their contribution!


### [`uefi-rs`](https://github.com/rust-osdev/uefi-rs)
<span class="maintainers">Maintained by [@GabrielMajeri](https://github.com/GabrielMajeri), [@nicholasbishop](https://github.com/nicholasbishop), and [@phip1611](https://github.com/phip1611)</span>

`uefi` makes it easy to develop Rust software that leverages safe, convenient,
and performant abstractions for UEFI functionality.

We merged the following PRs this month:

- [Change SimpleNetwork::wait_for_packet to return `Option<Event>`](https://github.com/rust-osdev/uefi-rs/pull/1836)
- [Implement BlockIO2](https://github.com/rust-osdev/uefi-rs/pull/1841)
- [uefi: Refactor PciRootBridgeIo::enumerate() with tree-topology information](https://github.com/rust-osdev/uefi-rs/pull/1830)
- [uefi: http: fix integration test](https://github.com/rust-osdev/uefi-rs/pull/1850)

<!-- - [chore(deps): update crate-ci/typos action to v1.40.0](https://github.com/rust-osdev/uefi-rs/pull/1837) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1838) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1844) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1849) -->
<!-- - [chore(deps): lock file maintenance](https://github.com/rust-osdev/uefi-rs/pull/1853) -->

Thanks to [@seijikun](https://github.com/seijikun) and [@Virv12](https://github.com/Virv12) for their contributions!



## Other Projects

In this section, we describe updates to Rust OS projects that are not directly related to the `rust-osdev` organization. Feel free to [create a pull request](https://github.com/rust-osdev/homepage/pulls) with the updates of your OS project for the next post.

<!--
    Please use the following template:

    ### [`owner_name/repo_name`](https://github.com/rust-osdev/owner_name/repo_name)
    <span class="maintainers">(Section written by [@your_github_name](https://github.com/your_github_name))</span>

    ...<<your project updates>>...
-->


### [`valibali/cluu`](https://github.com/valibali/cluu)
<span class="maintainers">(Section written by [@valibali](https://github.com/valibali))</span>

    # CLUU Kernel Evolution: From a mess to Disciplined Microkernel

	---

	## Executive Summary of Changes

	Compared to the earlier kernel state, CLUU has undergone the following high-level transitions:

	* From **feature-oriented kernel** → **mechanism-only microkernel**
	* From **implicit privilege** → **explicit, token-based authority**
	* From **kernel-resident subsystems** → **userspace-managed policy**
	* From **best-effort safety** → **formally enforced correctness**
	* From **prototype scheduler and IPC** → **deterministic, tested primitives**
	* From **ad-hoc syscall growth** → **strictly minimal syscall surface**

	These changes are architectural, not cosmetic.

	---

	## Architectural Reorientation

	### Then: Hybrid / Transitional Kernel

	The earlier kernel combined multiple responsibilities:

	* Kernel-resident drivers (framebuffer, keyboard, serial)
	* Kernel-owned TTY and console subsystems
	* Implicit access to global kernel services
	* Feature growth driven by hardware enablement

	While still “microkernel-inspired,” the kernel acted as a **service provider**, not merely a mechanism layer.

	---

	### Now: Strict L4-Style Microkernel

	The current kernel is intentionally narrow:

	* Scheduler
	* Memory management (PMM + VMM)
	* IPC
	* Interrupt delivery
	* Token verification
	* Syscall dispatch

	Everything else — including filesystems, drivers, shells, and services — lives in **userspace**, communicating exclusively via IPC.

	The kernel no longer implements *policy*.

	---

	## Authority Model: The Single Biggest Change

	### Then: Implicit Trust Model

	In the earlier kernel:

	* Authority was largely implied by execution context
	* Kernel code could freely invoke subsystems
	* Userspace relied on convention and API discipline
	* No formal representation of “who is allowed to do what”

	This is typical of early-stage kernels.

	---

	### Now: Token-Based Explicit Authority

	All authority in CLUU is now represented by **cryptographically protected tokens**:

	* No ambient permissions
	* No global namespaces
	* No implicit access to resources
	* No operation without a valid token

	Key properties introduced:

	| Property             | Old Kernel | Current Kernel            |
	| -------------------- | ---------- | ------------------------- |
	| Authority visibility | Implicit   | Explicit                  |
	| Delegation           | Ad-hoc     | Cryptographically derived |
	| Expiration           | None       | Mandatory                 |
	| Enforcement          | Convention | Kernel-verified           |
	| Enumeration          | Possible   | Impossible                |

	This fundamentally changes how the system is reasoned about:
	**every operation has an auditable authority chain**.

	---

	## IPC: From Messaging to System Backbone

	### Then: IPC as a Feature

	Previously, IPC existed alongside other communication paths and kernel-managed services.

	---

	### Now: IPC as the Only Primitive

	IPC is now:

	* **Synchronous rendezvous**
	* Deterministic in blocking behavior
	* The sole mechanism for:

	  * Filesystem access
	  * Driver interaction
	  * Process management
	  * Service discovery

	Supported semantics are fully implemented and tested:

	* `send` / `recv`
	* `call` / `reply` / `replyrecv`
	* `copy` / `map` / `grant` buffers

	---

	## Scheduler and Execution Model

	### Then: Functional but Basic Scheduling

	Earlier scheduling focused on:

	* Correct task switching
	* Timer-driven preemption
	* Fairness as a secondary concern

	---

	### Now: Deterministic, O(1) Scheduler

	The scheduler is now:

	* Priority bitmap–based
	* O(1) selection cost
	* 256 fixed priority levels
	* FIFO fairness within a priority class
	* Cleanly separated policy hooks

	This enables predictable system behavior and future experimentation without kernel refactoring.

	---

	## Memory Management: Formalized and Verified

	### Then: Kernel-Centric Memory Control

	Earlier memory management emphasized:

	* Physical frame allocation
	* Kernel-controlled virtual mappings
	* Safety through Rust, not policy

	---

	### Now: Strict Address Spaces with Enforcement

	The current memory model introduces:

	* Per-process address spaces
	* Mandatory user-pointer validation
	* Lazy heap allocation via page faults
	* No implicit sharing
	* Explicit kernel/user boundary enforcement

	Memory faults are now **first-class control-flow events**, not error cases.

	---

	## Syscall Surface: Radical Reduction

	### Then: Expanding API Surface

	Earlier syscalls evolved organically with new features.

	---

	### Now: Exactly Seven Syscalls

	The syscall surface has been intentionally collapsed to:

	```
	Send
	Recv
	Call
	Reply
	Yield
	Invoke
	DebugPrint
	```

	All privileged operations are funneled through `Invoke`, guarded by tokens.

	This drastically simplifies:

	* Auditing
	* Reasoning
	* Formal verification potential

	---

	## Logging and Diagnostics

	### Then: Kernel Utility

	Logging previously behaved like a standard kernel service.

	---

	### Now: Diagnostic Instrument Only

	Logging is now:

	* IRQ-safe
	* Allocation-free
	* Zero-cost in release builds
	* UART-backed only

	It is explicitly **not** a runtime dependency.

	---

	## Userspace: From Demo to First-Class Citizen

	### Then: Kernel-Centric Demonstrations

	Earlier userspace programs existed mainly to exercise kernel features.

	---

	### Now: Policy Lives Entirely in Userspace

	Current userspace demonstrates:

	* IPC-based VFS
	* Userspace drivers
	* Token-governed service access
	* Zero-copy buffer passing
	* A working shell and utilities

	The kernel does not “know” what a filesystem or driver is.

	---

	## Test Coverage and Internal Confidence

	A critical maturity marker is test coverage:

	| Metric              | Old Kernel | Current Kernel |
	| ------------------- | ---------- | -------------- |
	| Kernel unit tests   | Minimal    | 145            |
	| Code coverage       | N/A        | ~90%           |
	| Passing rate        | N/A        | 100%           |
	| Subsystem isolation | Partial    | Complete       |
	| Internal invariants | Informal   | Enforced       |

	This reflects a shift from experimentation to **engineering discipline**.

	---

	## What Did *Not* Change

	Some things were preserved intentionally:

	* Rust as the implementation language
	* Inspiration (blog-os, Plan9, L4)
	* BOOTBOOT as the boot environment
	* x86_64 as the primary architecture
	* Clarity over convenience
	* Non-POSIX philosophy

	---

	Further work will happen almost entirely in **userspace**, exactly as intended.




## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [Zulip chat](https://rust-osdev.zulipchat.com).
