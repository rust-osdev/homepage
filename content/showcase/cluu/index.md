+++
title = "CLUU"
date = 2023-06-14

[extra]
authors = ["valibali"]
+++

# CLUU (Compact Lightweight Unix Utopia)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE) [![Documentation](https://img.shields.io/badge/documentation-RUSTDOCS-blue.svg)](https://valibali.github.io/cluu/)

CLUU is a hobby operating system being written in Rust - in active development phase - targeting x86_64 and with plans to support aarch64 in the future.
It's really at the starting phase, but it'll get there in time. No rush or whatsoever.

The project draws inspiration from the following Operating Systems:

- [Plan 9](https://github.com/plan9foundation/plan9): Plan 9 from Bell Labs is a distributed operating system developed at Bell Labs in the late 1980s.
- [BSD](https://github.com/freebsd/freebsd): FreeBSD is a modern, advanced operating system for x86 and ARM architectures.

As well as heavily influenced by the following projects:

- [RedoxOS](https://github.com/redox-os/redox): RedoxOS is an operating system written in Rust, aiming to bring the innovations of Rust to a modern microkernel and full set of applications. It's fair to say it is the most advanced of all Rust OS-es.
- [k4dos](https://github.com/clstatham/k4dos): k4dos is another hobby-os of that sort, it's fairly cool, with userspace, that can run FreeDoom for example. It has a nice shell implementation: kash
- [blog_os](https://os.phil-opp.com/): blog_os is a cutting-edge project by Philipp Oppermann that provides a detailed tutorial on building an operating system in Rust. It covers various aspects, including the bootloader, memory management, and device drivers.

## Project Information

CLUU (Compact Lightweight Unix Utopia) is a hobby operating system written in Rust. The project aims to achieve a microkernel design, emphasizing portability and memory safety. It draws inspiration from various operating systems and projects, combining the best practices and ideas to create a unique and efficient system.

### Bootloader: [BOOTBOOT](https://gitlab.com/bztsrc/bootboot)

One of the key elements in CLUU's design is the BOOTBOOT bootloader. BOOTBOOT provides a Level 2 bootloader implementation that facilitates the booting process and sets up the initial environment for the operating system. By leveraging BOOTBOOT, CLUU benefits from its features, such as high-half kernel, memory initialization, processor mode setup, init-ramdrive, early-uart-debug, a framebuffer, and parameter passing.

The main goals of CLUU include:

**Microkernel Design**: CLUU follows a microkernel architecture, aiming to keep the kernel minimal and provide most system services through user-level processes or servers. This design promotes modularity, extensibility, and flexibility.

**Portability**: CLUU is designed to be portable across different platforms and architectures. While initially targeting x86_64, future plans include support for aarch64.

**Memory Safety**: Rust is chosen as the programming language for CLUU due to its strong emphasis on memory safety. By leveraging Rust's ownership and borrowing model, CLUU aims to minimize common programming errors, such as null pointer dereferences, buffer overflows, and data races.

The project is open-source and welcomes contributions from the community. If you're interested in exploring the code, contributing enhancements, or reporting issues, please visit the GitHub repository.

## Prerequisites

To build and run the CLUU operating system, you need to install the following prerequisites:

1. **Operating System**: Linux, Windows, or macOS.
2. **QEMU Emulator**: QEMU is used to emulate the x86_64 architecture. Install QEMU by following the instructions for your operating system:

   - Linux: QEMU can usually be installed from the package manager. For example, on Ubuntu, run:
     ```shell
     sudo apt-get install qemu
     ```
   - Windows: Download the latest version of QEMU from the [QEMU website](https://www.qemu.org/download/) and install it.
   - macOS: Install QEMU using the Homebrew package manager by running:
     ```shell
     brew install qemu
     ```

3. **Terminal for COM port**
   This could be simply `telnet` - or any other tool that can communicate with UART_16550.
4. **Rust Programming Language**: CLUU is written in Rust. Install Rust by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

5. **Make Build System**: The build process for CLUU uses the Make build system. Make is commonly pre-installed on Linux and macOS. For Windows, you can install Make using the [GNUWin32 project](http://gnuwin32.sourceforge.net/packages/make.htm) or other alternatives.
6. VSCode for editing, and CodeLLDB plugin installed for Debugging.

## Building and Running

To build and run CLUU, follow these steps:

1. Clone the CLUU repository:

   ```shell
   git clone https://github.com/your-username/cluu.git
   ```

2. Build the kernel and bootboot image:

   ```shell
   cd cluu
   make all
   ```

3. Clean the build artifact:

   ```shell
   make clean
   ```

4. Run CLUU in QEMU (with UEFI):

   ```shell
   make qemu
   ```

   This will start QEMU, but will pause before boot. It also maps COM2 to the `localhost:4321`

   In another shell you can start the COM-terminal for debug messages:

   ```shell
   telnet localhost 4321
   ```

   If you are into debugging, hit F5 in VSCode (`launch.json` included), this will start the debugging session and attach to QEMU. The debugging session will start paused, just hit F5 again to continue.

5. Run CLUU in QEMU (without debugging symbols):
   ```shell
   make qemu_nodebug
   ```

## License

CLUU is licensed under the MIT License. See LICENSE for more information.
