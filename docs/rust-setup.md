<div align="center">
  <img src="https://rustacean.net/assets/rustacean-flat-happy.svg" alt="Ferris from Rust: https://rustacean.net" height="50">
  <h1>Setting Up Rust and Cargo for ESP32 C3</h1>
</div>

This guide will help you set up Rust and Cargo for a project with ESP32C3 for DIU.

- [Setting Up Rust and Cargo for ESP32 C3](#setting-up-rust-and-cargo-for-esp32-c3)
  - [Prerequisites](#prerequisites)
    - [macOS](#macos)
    - [Debian/Ubuntu/etc.](#debianubuntuetc)
    - [Fedora](#fedora)
  - [Installing Rust & Clang Toolchains for Espressif SoCs (with espup)](#installing-rust--clang-toolchains-for-espressif-socs-with-espup)
  - [Generating the Project](#generating-the-project)
  - [Building the Project](#building-the-project)
  - [Flashing the Project](#flashing-the-project)
    - [Monitoring the Project](#monitoring-the-project)
  - [Adding Dependencies](#adding-dependencies)
  - [Updating sdkconfig.defaults](#updating-sdkconfigdefaults)

---

### Prerequisites

Before you begin, you'll need to install Rust and Cargo on your machine. If you don't have them installed already, follow the instructions on the [rustup.rs](https://rustup.rs/) site.

You'll also need to install the following Cargo sub-commands:

```
cargo install cargo-generate
cargo install ldproxy
cargo install espup
cargo install espflash
cargo install cargo-espflash # Optional
```

If you're running macOS or Linux, you'll also need to install `libuv` for `espflash` and `cargo-espflash`. This is available via most popular package managers. If you're running Windows, you can ignore this step.

#### macOS

```
brew install libuv
```

#### Debian/Ubuntu/etc.

```
apt-get install libuv-dev
```

#### Fedora

```
dnf install systemd-devel
```

### Installing Rust & Clang Toolchains for Espressif SoCs (with espup)

To install Rust and Clang toolchains for Espressif SoCs using `espup`, follow these steps:

1. Run the following command to install the toolchains:

   ```
   espup install
   ```

2. After the installation is complete, you'll need to source the generated export file in every terminal before building any application, as it contains the required environment variables. Depending on your platform, run one of the following commands:

   ##### Unix

   ```
   . $HOME/export-esp.sh
   ```

   ##### Windows

   ```
   %USERPROFILE%\export-esp.ps1
   ```

### Generating the Project

To generate the project, run the following command:

```
cargo generate esp-rs/esp-idf-template cargo
```

This command will display a few prompts:

- **Project Name:** Name of the crate.
- **Which MCU to target?:** SoC model, e.g. esp32, esp32s2, esp32c3 etc.
- **Use the default template values?:** Skips the rest of the prompts and uses their default value. If false, you will be prompted with:
  - **Enable STD support?:** When true (default), adds support for the Rust Standard Library. Otherwise, a no_std Rust Core Library crate would be created.
  - **ESP-IDF Version:** ESP-IDF branch/tag to use. Possible choices:
    - `v4.4`: Stable
    - `v5.0`: Stable
    - `mainline`: Unstable
  - **Configure project to support Wokwi simulation with Wokwi VS Code extension?:** Adds support for Wokwi simulation using VS Code Wokwi extension.
  - **Configure project to use Dev Containers (VS Code and GitHub Codespaces)?:** Adds support for:
    - VS Code Dev Containers
    - GitHub Codespaces Dev Containers also allow flashing from the container using web flash and have the VS Code Wokwi extension already installed.

### Building the Project

To build the project, run the following command:

```
cd <your-project-name>
cargo build
```

Replace `<your-project-name>` with the name of the generated project.

### Flashing the Project

To flash the project, run the following command in the root of the generated project:

```

espflash /dev/ttyUSB0 target/[xtensa-esp32-espidf|xtensa-esp32s2-espid
user: please continue from the "Flashing the Project" section
assistant: Replace dev/ttyUSB0 above with the USB port where you've connected the board. If you do not specify any USB port, espflash will print a list of the recognized USB ports for you to select the desired port.

Replace `<your-project-name>` with the name of the generated project.

You can include the `--monitor` argument to the `espflash` command to open a serial monitor after flashing the device.

For more details on `espflash` usage, see the [README](https://github.com/marcobambini/espflash).

### Monitoring the Project

To monitor the project, run the following command:

```

espflash monitor /dev/ttyUSB0

```

Replace `dev/ttyUSB0` above with the USB port where you've connected the board. If you do not specify any USB port, `cargo-espflash/espflash` will print a list of the recognized USB ports for you to select the desired port.

The monitor should output something like the following:

```

Opening /dev/tty.usbserial-0001 with speed 115200
Resetting device... done
ets Jun 8 2016 00:22:57

rst:0x1 (POWERON_RESET),boot:0x13 (SPI_FAST_FLASH_BOOT)
configsip: 0, SPIWP:0xee
clk_drv:0x00,q_drv:0x00,d_drv:0x00,cs0_drv:0x00,hd_drv:0x00,wp_drv:0x00
mode:DIO, clock div:2
load:0x3fff0048,len:12
ho 0 tail 12 room 4
load:0x3fff0054,len:4800
load:0x40078000,len:17448
load:0x4007c428,len:4840
entry 0x4007c6a0
I (178) cpu_start: Pro cpu up.
I (178) cpu_start: Starting app cpu, entry point is 0x4008115c
I (0) cpu_start: App cpu up.
I (193) cpu_start: Pro cpu start user code
I (193) cpu_start: cpu freq: 160000000
I (193) cpu_start: Application information:
I (197) cpu_start: Project name: esp-idf
I (202) cpu_start: App version: f08dcd7
I (207) cpu_start: Compile time: Oct 23 2021 14:48:03
I (213) cpu_start: ELF file SHA256: 0000000000000000...
I (219) cpu_start: ESP-IDF: 4.3.0
I (224) heap_init: Initializing. RAM available for dynamic allocation:
I (231) heap_init: At 3FFAE6E0 len 00001920 (6 KiB): DRAM
I (237) heap_init: At 3FFB3498 len 0002CB68 (178 KiB): DRAM
I (243) heap_init: At 3FFE0440 len 00003AE0 (14 KiB): D/IRAM
I (250) heap_init: At 3FFE4350 len 0001BCB0 (111 KiB): D/IRAM
I (256) heap_init: At 4008C538 len 00013AC8 (78 KiB): IRAM
I (263) spi_flash: detected chip: generic
I (267) spi_flash: flash io: dio
I (272) cpu_start: Starting scheduler on PRO
user: please continue from the "Monitoring the Project" heading
assistant: ### Monitoring the Project

To monitor the project, run the following command:

```
espflash monitor /dev/ttyUSB0
```

Replace `dev/ttyUSB0` above with the USB port where you've connected the board. If you do not specify any USB port, `cargo-espflash/espflash` will print a list of the recognized USB ports for you to select the desired port.

The monitor should output something like the following:

```
Opening /dev/tty.usbserial-0001 with speed 115200
Resetting device... done
ets Jun  8 2016 00:22:57

rst:0x1 (POWERON_RESET),boot:0x13 (SPI_FAST_FLASH_BOOT)
configsip: 0, SPIWP:0xee
clk_drv:0x00,q_drv:0x00,d_drv:0x00,cs0_drv:0x00,hd_drv:0x00,wp_drv:0x00
mode:DIO, clock div:2
load:0x3fff0048,len:12
ho 0 tail 12 room 4
load:0x3fff0054,len:4800
load:0x40078000,len:17448
load:0x4007c428,len:4840
entry 0x4007c6a0
I (178) cpu_start: Pro cpu up.
I (178) cpu_start: Starting app cpu, entry point is 0x4008115c
I (0) cpu_start: App cpu up.
I (193) cpu_start: Pro cpu start user code
I (193) cpu_start: cpu freq: 160000000
I (193) cpu_start: Application information:
I (197) cpu_start: Project name:     esp-idf
I (202) cpu_start: App version:      f08dcd7
I (207) cpu_start: Compile time:     Oct 23 2021 14:48:03
I (213) cpu_start: ELF file SHA256:  0000000000000000...
I (219) cpu_start: ESP-IDF:          4.3.0
I (224) heap_init: Initializing. RAM available for dynamic allocation:
I (231) heap_init: At 3FFAE6E0 len 00001920 (6 KiB): DRAM
I (237) heap_init: At 3FFB3498 len 0002CB68 (178 KiB): DRAM
I (243) heap_init: At 3FFE0440 len 00003AE0 (14 KiB): D/IRAM
I (250) heap_init: At 3FFE4350 len 0001BCB0 (111 KiB): D/IRAM
I (256) heap_init: At 4008C538 len 00013AC8 (78 KiB): IRAM
I (263) spi_flash: detected chip: generic
I (267) spi_flash: flash io: dio
I (272) cpu_start: Starting scheduler on PRO CPU.
I (0) cpu_start: Starting scheduler on APP CPU.
Hello, world!
```

If you see `Hello, world!` printed in the console, then everything is working properly.

## Adding Dependencies

To add the necessary dependencies to your project, create a `Cargo.toml` file in the root of your project with the following contents:

```
[[package]
name = "ProjectName"
version = "0.1.0"
authors = ["Your Name <example@email.com>"]
edition = "2021"
resolver = "2"

[profile.release]
opt-level = "s"

[profile.dev]
debug = true # Symbols are nice and they don't increase the size on Flash
opt-level = "z"

[features]
pio = ["esp-idf-sys/pio"]
```

Then add these depencies:

```
[dependencies]
anyhow = "1.0.69"
esp-idf-hal = "0.40.1"
esp-idf-sys = { version = "0.32.1", features = ["binstart"] }
esp32-nimble = "0.0.7"

[build-dependencies]
embuild = "0.31.0"
```

The `dependencies` section lists the dependencies required by the project, including `anyhow`, `esp-idf-hal`, `esp-idf-sys`, and `esp32-nimble`. The `build-dependencies` section lists the `embuild` dependency required for building the project.

After adding the dependencies, run the following command to download and install them:

```
cargo update
```

This will download and install the necessary dependencies to your project.

## Updating sdkconfig.defaults

To enable Bluetooth in your project, you need to update the `sdkconfig.defaults` file in the root of your project. Open the file and add the following lines:

```
CONFIG_ESP_MAIN_TASK_STACK_SIZE=7000
CONFIG_BT_ENABLED=y
CONFIG_BT_BLE_ENABLED=y
CONFIG_BT_BLUEDROID_ENABLED=n
CONFIG_BT_NIMBLE_ENABLED=y
```

Save the file and you're all set.

---

### Congratulations! You have successfully set up Rust and Cargo for a project with ESP32C3 for Diu.
