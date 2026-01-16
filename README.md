# BatMetal
**Bare Metal Kernel for Raspberry Pi 5 Sonar Mapping**

BatMetal is a lightweight, Rust-based bare metal kernel designed for the **Raspberry Pi 5**. Its primary mission is to provide a real-time, low-latency platform for ultrasonic sonar mapping operations, bypassing the overhead of a traditional operating system.

Currently, it serves as a robust hardware abstraction layer verifying RP1 (Southbridge) access, UART communication, and GPIO control.

---

## Hardware Configuration

### Pinout & GPIOs
| GPIO Pin | Function | Description |
| :--- | :--- | :--- |
| **GPIO 14** | UART0 TX | Serial Console Transmit (Connected to Adapter RX) |
| **GPIO 15** | UART0 RX | Serial Console Receive (Connected to Adapter TX) |
| **GPIO 18** | SERVO PWM | 9g Servo Motor Control (50Hz Software PWM) |
| **GPIO 6** | `ALIVE_LED` | Alive Status Indicator (Blinks during normal operation) |
| **GPIO 5** | `ERROR_LED` | Panic/Error Indicator (Blinked by Panic Handler) |
| **ACT LED** | Onboard | Activity LED (Green) - Used for Boot/Panic status |

### Memory Map (Crucial for Pi 5)
The Raspberry Pi 5 uses a split architecture with the **RP1 I/O Controller** connected via PCIe.
*   **RP1 Base Address (CPU View):** `0x1f00000000` (BAR0)
*   **UART0 Offset:** `0x30000`
*   *Note:* The Firmware initializes UART0 at `0x1c...`, but the Kernel must access it via the PCIe BAR0 mapping at `0x1f...`.

---

## Getting Started

### Prerequisites
*   **Rust Toolchain** (Nightly recommended for `no_std`)
*   **Cross-Compiler Tools:** `aarch64-linux-gnu-binutils` (for `objcopy`)
*   **Hardware:** Raspberry Pi 5, USB-to-UART Adapter (3.3V logic)

### Build Instructions

1.  **Install Target:**
    ```bash
    rustup target add aarch64-unknown-none
    ```

2.  **Compile & Link:**
    ```bash
    make
    ```
    This produces `kernel8.img`.

3.  **Flash:**
    *   Format an SD card with FAT32.
    *   Copy standard Pi 5 boot files (`bcm2712-rpi-5-b.dtb`, `config.txt`, etc.).
    *   Copy `kernel8.img` to the root.
    *   Ensure `config.txt` contains:
        ```ini
        os_prefix=
        check_syslin_rom=0
        enable_uart=1
        ```

4.  **Run:**
    Connect UART adapter and power on.
    ```bash
    sudo minicom -D /dev/ttyUSB0 -b 115200
    ```

---

## Roadmap: Project "Sonar"

The ultimate goal of BatMetal is **Autonomous Sonar Mapping**.
Future implementations will include:

*   [ ] **HC-SR04 Driver:** Precise microsecond timing for ultrasonic triggering and echo measurement.
*   [x] **Servo Control:** PWM generation to sweep the sonar sensor (Radar style).
*   [ ] **Point Cloud Storage:** In-memory mapping of detected obstacles.
*   [ ] **Visualization:** Exporting polar coordinate data via UART for external rendering.

---

## 🛠 Tech Stack
*   **Language:** Rust 🦀
*   **Architecture:** AArch64 (ARM64)
*   **Platform:** Raspberry Pi 5 (BCM2712 + RP1)
*   **Approach:** Bare Metal (No OS)

---

> *"In the silence of bare metal, every echo counts."*
