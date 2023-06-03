+++
title = "This Month in Rust OSDev: May 2023"
date = 2023-05-05

[extra]
month = "May 2023"
editors = ["phil-opp"]
+++

Welcome to a new issue of _"This Month in Rust OSDev"_. In these posts, we give a regular overview of notable changes in the Rust operating system development ecosystem.

<!-- more -->

This series is openly developed [on GitHub](https://github.com/rust-osdev/homepage/). Feel free to open pull requests there with content you would like to see in the next issue. If you find some issues on this page, please report them by [creating an issue](https://github.com/rust-osdev/homepage/issues/new) or using our <a href="#comment-form">_comment form_</a> at the bottom of this page.

<!--
    This is a draft for the upcoming "This Month in Rust OSDev (May 2023)" post.
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


## Announcements, News, and Blog Posts

Here we collect news, blog posts, etc. related to OS development in Rust.

<!--
Please follow this template:

- [Title](https://example.com)
  - (optional) Some additional context
-->

- [RTIC (The hardware accelerated Rust RTOS) releases v2.0.0!](https://www.reddit.com/r/rust/comments/13xp6q9/rtic_the_hardware_accelerated_rust_rtos_releases/)
- [My Rust OS for microcontrollers now has a dir command](https://www.reddit.com/r/rust/comments/13aittv/media_my_rust_os_for_microcontrollers_now_has_a/)
- [First Rust Code Shows Up in the Windows 11 Kernel](https://www.thurrott.com/windows/windows-11/282995/first-rust-code-shows-up-in-the-windows-11-kernel)
- [May Flowers Spring COSMIC Showers](https://blog.system76.com/post/may-flowers-spring-cosmic-showers/)
- [Ironclad-Shell: a "shell" made in rust and for OS classes](https://www.reddit.com/r/rust/comments/13h1ii0/ironcladshell_a_shell_made_in_rust_and_for_os/)


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

- [uefi: Switch Rng repr from C to transparent](https://github.com/rust-osdev/uefi-rs/pull/784)
- [uefi: Make MEMORY_DESCRIPTOR_VERSION an associated constant](https://github.com/rust-osdev/uefi-rs/pull/785)
- [macros: allow to create empty cstr8/16 slices](https://github.com/rust-osdev/uefi-rs/pull/786)
- [Add Revision to uefi-raw and use it from uefi](https://github.com/rust-osdev/uefi-rs/pull/783)
- [Add Tpl to uefi-raw and use it from uefi](https://github.com/rust-osdev/uefi-rs/pull/790)
- [Add PhysicalAddress/VirtualAddress to uefi-raw and use them from uefi](https://github.com/rust-osdev/uefi-rs/pull/789)
- [cstr16: add method to get the underlying bytes](https://github.com/rust-osdev/uefi-rs/pull/788)
- [Move various memory types to uefi-raw and re-export in uefi](https://github.com/rust-osdev/uefi-rs/pull/791)
- [uefi/fs: add path and pathbuf abstraction](https://github.com/rust-osdev/uefi-rs/pull/771)
- [uefi: Clear the Vec in DevicePathBuilder::with_vec](https://github.com/rust-osdev/uefi-rs/pull/794)
- [uefi-macros: Restore entry macro tests](https://github.com/rust-osdev/uefi-rs/pull/796)
- [Add LoadedImageDevicePath protocol](https://github.com/rust-osdev/uefi-rs/pull/795)
- [fs: add new error type](https://github.com/rust-osdev/uefi-rs/pull/792)
- [deps: bump bitflags from 1.0 to 2.1](https://github.com/rust-osdev/uefi-rs/pull/714)
- [uefi-macros: Fix entry macro with docstring](https://github.com/rust-osdev/uefi-rs/pull/797)
- [fs: implement remove_dir_all](https://github.com/rust-osdev/uefi-rs/pull/799)
- [uefi-raw: Drop dep on uefi-macros](https://github.com/rust-osdev/uefi-rs/pull/802)
- [Disable some tests on aarch64](https://github.com/rust-osdev/uefi-rs/pull/803)
- [allocator: Use appropriate memory types](https://github.com/rust-osdev/uefi-rs/pull/804)
- [Release branch](https://github.com/rust-osdev/uefi-rs/pull/806)
- [uefi: Rename FileSystemIOErrorContext to IoErrorContext](https://github.com/rust-osdev/uefi-rs/pull/807)
- [uefi: Improve fs error Display impls, drop derive_more](https://github.com/rust-osdev/uefi-rs/pull/808)
- [Move InterfaceType to uefi-raw](https://github.com/rust-osdev/uefi-rs/pull/811)
- [uefi-raw: Add table Header struct](https://github.com/rust-osdev/uefi-rs/pull/810)
- [build(deps): bump bitflags from 2.2.1 to 2.3.1](https://github.com/rust-osdev/uefi-rs/pull/814)
- [Relax bitflags dep version to 2.0.0](https://github.com/rust-osdev/uefi-rs/pull/815)
- [xtask: Drop an unused test file](https://github.com/rust-osdev/uefi-rs/pull/812)
- [xtask: Improve QEMU boot time](https://github.com/rust-osdev/uefi-rs/pull/805)
- [Move some runtime services types to uefi-raw](https://github.com/rust-osdev/uefi-rs/pull/813)
- [Fix and enable uefi-raw unit/doc tests](https://github.com/rust-osdev/uefi-rs/pull/816)
- [Add Time to uefi-raw and use it from uefi](https://github.com/rust-osdev/uefi-rs/pull/817)
- [uefi-raw: Add Char8/Char16](https://github.com/rust-osdev/uefi-rs/pull/809)
- [uefi: Fix use of tuples in pointer structs](https://github.com/rust-osdev/uefi-rs/pull/822)
- [build(deps): bump clap from 4.2.7 to 4.3.0](https://github.com/rust-osdev/uefi-rs/pull/823)
- [uefi(boot_services): add `install_configuration_table`](https://github.com/rust-osdev/uefi-rs/pull/821)
- [xtask: Add check-raw](https://github.com/rust-osdev/uefi-rs/pull/819)
- [uefi-raw: Add Update Capsule types](https://github.com/rust-osdev/uefi-rs/pull/818)
- [Download OVMF files rather than relying on distro package](https://github.com/rust-osdev/uefi-rs/pull/798)
- [Export FileInfoCreationError](https://github.com/rust-osdev/uefi-rs/pull/831)
- [Move EventType to uefi-raw](https://github.com/rust-osdev/uefi-rs/pull/833)
- [uefi: Derive Eq for Handle](https://github.com/rust-osdev/uefi-rs/pull/836)
- [Add `RuntimeServices` definition to `uefi-raw` and use it from `uefi`](https://github.com/rust-osdev/uefi-rs/pull/832)
- [loaded_image: Add set_unload](https://github.com/rust-osdev/uefi-rs/pull/835)
- [uefi: Change file reading to read in 1 MiB chunks](https://github.com/rust-osdev/uefi-rs/pull/834)

Thanks to [@RaitoBezarius](https://github.com/RaitoBezarius) and [@medhefgo](https://github.com/medhefgo) for their contributions!


### [`multiboot2`](https://github.com/rust-osdev/multiboot2)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods), [@phip1611](https://github.com/phip1611), [@robert-w-gries](https://github.com/robert-w-gries), [@ahmedcharles](https://github.com/ahmedcharles), and [@Caduser2020](https://github.com/Caduser2020)</span>

- [add missing functionality in multiboot2-header (finding the header, getting tags)](https://github.com/rust-osdev/multiboot2/pull/136)
- [multiboot2: properly type DST tags](https://github.com/rust-osdev/multiboot2/pull/134)
- [Add BasicMemoryInfo and SMBIOS tags](https://github.com/rust-osdev/multiboot2/pull/137)
- [multiboot2: Add the end tag](https://github.com/rust-osdev/multiboot2/pull/138)

Thanks to [@YtvwlD](https://github.com/YtvwlD) for their contributions!


### [`vga`](https://github.com/rust-osdev/vga)
<span class="maintainers">Maintained by [@RKennedy9064](https://github.com/RKennedy9064)</span>

- [implement Graphics1280x800x256](https://github.com/rust-osdev/vga/pull/32)

Thanks to [@tsatke](https://github.com/tsatke) for their contribution!


### [`ovmf-prebuilt`](https://github.com/rust-osdev/ovmf-prebuilt)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

- [Rework the repo to build edk2 release tags](https://github.com/rust-osdev/ovmf-prebuilt/pull/1)

Thanks to [@nicholasbishop](https://github.com/nicholasbishop) for this contribution!


### [`acpi`](https://github.com/rust-osdev/acpi)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

- [rsdp: Make whole RSDP readable](https://github.com/rust-osdev/acpi/pull/179)
- [Clean up allocator_api feature](https://github.com/rust-osdev/acpi/pull/177)
- [acpi: fix length of "extended_area_bytes" in RSDP search](https://github.com/rust-osdev/acpi/pull/164)

Thanks to [@YtvwlD](https://github.com/YtvwlD), [@semiviral](https://github.com/semiviral), and [@A0lson](https://github.com/A0lson) for their contributions!


### [`pci_types`](https://github.com/rust-osdev/pci_types)
<span class="maintainers">Maintained by [@IsaacWoods](https://github.com/IsaacWoods)</span>

- [add function to determine the inteerupt pin and interrupt line](https://github.com/rust-osdev/pci_types/pull/9)

Thanks to [@stlankes](https://github.com/stlankes) for their contributions!


### [`bootloader`](https://github.com/rust-osdev/bootloader)
<span class="maintainers">Maintained by [@phil-opp](https://github.com/phil-opp)</span>

- [Use `cargo-semver-checks-action`](https://github.com/rust-osdev/bootloader/pull/369)

Thanks to [@mgr0dzicki](https://github.com/mgr0dzicki) for their contributions!


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

- [Fix panics caused by misaligned pointer dereferences in "Double Faults" & "Introduction to Paging"](https://github.com/phil-opp/blog_os/pull/1226)
- [Correcting typos and errors in French translation of Post 1](https://github.com/phil-opp/blog_os/pull/1219)
- [posts/06-double-faults/index: fixup deprecated fn](https://github.com/phil-opp/blog_os/pull/1218)

Thanks to [@SPuntte](https://github.com/SPuntte), [@Firenezz](https://github.com/Firenezz), and [@twilfredo](https://github.com/twilfredo) for these contributions!

There wasn't much progress on the upcoming third edition this month, as I've been on vacation for the past few weeks. I still have a lot of catching up to do, but I'll try my best to continue writing soon :).

## Join Us?

Are you interested in Rust-based operating system development? Our `rust-osdev` organization is always open to new members and new projects. Just let us know if you want to join! A good way for getting in touch is our [gitter channel](https://gitter.im/rust-osdev/Lobby).
