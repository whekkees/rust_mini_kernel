# Averion OS

A small educational 32-bit operating system written in Rust for the x86 architecture. 

The project is built entirely from scratch to explore and understand how an operating system interacts with hardware at the lowest level.

---

## 🚀 Features

*   **Custom Rust Kernel**: Built with `#![no_std]` and `#![no_main]`.
*   **Target Architecture**: x86 (32-bit protected mode).
*   **Bootloader**: Multiboot-compliant GRUB bootloader.
*   **Hardware Control**:
    *   Direct x86 I/O port manipulation (`inb` / `outb`).
    *   Programmable Interval Timer (PIT) implementation.
    *   PC Speaker driver.
    *   CMOS Real-Time Clock (RTC) support.
    *   PS/2 controller and Keyboard drivers.
*   **Interrupt Handling**: Custom Interrupt Descriptor Table (IDT) and exception handlers.
*   **Visual Output**: VGA text-mode buffer driver for printing to the screen.

---

## 📂 Project Structure

```text
averion_os/
├── .cargo/
├── iso/
├── src/
│   ├── drivers/
│   │   ├── io/
│   │   │   ├── inb.rs
│   │   │   ├── mod.rs
│   │   │   └── outb.rs
│   │   ├── keyboard.rs
│   │   ├── mod.rs
│   │   ├── pit.rs
│   │   ├── ps2.rs
│   │   ├── rtc.rs
│   │   ├── speaker.rs
│   │   └── vga_buffer.rs
│   ├── interrupts/
│   │   ├── handlers.rs
│   │   ├── idt.rs
│   │   └── mod.rs
│   ├── panic/
│   │   └── mod.rs
│   └── main.rs
├── boot.asm
├── i686-averion.json
├── linker.ld
└── run.ps1
```

---

## 🎯 Project Goals

Averion OS is primarily a personal learning playground. The main objectives are to understand:

1.  How the CPU boot sequence transitions into executing kernel code.
2.  How GRUB detects, parses, and loads the kernel binary via Multiboot.
3.  Low-level CPU mechanics (memory segmentation, registers, GDT/IDT).
4.  How Rust code communicates with physical hardware via assembly wrappers.
5.  How basic kernel-space drivers are designed and implemented.

---

## 🛠️ Building & Running

The project targets the `i686-averion` custom JSON target and relies on **GRUB** (to create the ISO) and **QEMU** (for emulation).

To build the kernel, generate a bootable ISO image, and launch it inside the QEMU emulator, simply run:

```bash
cargo run
```

> **Note**: For Windows environments, you can use the provided PowerShell automation script: `./run.ps1`.

---

## 👤 Author

*   **whekkees** — *Initial work and architecture design.*

---
*Averion OS — building a deeper understanding of hardware, one byte at a time.*