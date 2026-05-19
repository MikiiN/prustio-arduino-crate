# pRustIO Arduino Crate

This is the library crate for using the Arduino framework within [pRustIO](https://github.com/MikiiN/prustio) hybrid (Rust and C++) projects. 

It provides a safe Rust interface that wraps parts of the underlying Arduino C++ framework. By using `#[no_std]` and Foreign Function Interface (FFI) bindings, this crate bridges the gap between Rust and Arduino's core libraries.

> [!NOTE] 
> This crate is not designed to be usable alone. The embedded application itself must handle the entire compilation and linking process for the Arduino framework. It relies on pRustIO's build tools to compile the C/C++ dependencies correctly. But if you want it to use as a standalone crate, the build script exposes location of the C++ wrapper header and source files using variable `pio_source_path`, so you can use it in yours project build script. 

## Available Modules and Features

This crate wraps many of the familiar standard Arduino functions into safe, Rust-friendly APIs using types and enums (like `PinState`, `PinMode`, and `InterruptMode`).

### Core I/O
Interact directly with the pins on your board:
* **Digital I/O:** `pin_mode`, `digital_write`, `digital_read`.
* **Analog I/O:** `analog_write`, `analog_read`, `analog_reference`.
* Uses safe enumerations such as `PinMode::Input`, `PinMode::Output`, `PinMode::InputPullup` and `PinState::High`, `PinState::Low`.

### Time Control
Standard timing and delay functions are fully supported:
* `delay(ms: u32)` and `delay_microseconds(us: u32)`
* `millis()` and `micros()` tracking

### Advanced I/O
For more complex component interactions, you can use:
* `tone` and `no_tone` for generating sound.
* `shift_in` and `shift_out` for shifting data in and out of registers.
* `pulse_in` to read the length of a pulse on a pin.

### Communication & Hardware Interfaces
The crate exposes modules for communicating with other sensors and devices:
* **`serial`**: Provides a `Serial` struct to interact with UART (e.g., `begin`, `read`, `write_buffer`, `available`, `flush`). It also includes a `Printable` trait offering `print` and `println` functions. 
* **`wire`**: An implementation for the I2C (Wire) bus. It supports both master and slave device modes and includes functions like `begin_transmission`, `request_from`, `write_buffer`, and registering callback hooks (`on_receive`, `on_request`).
* **`spi`**: Access to the Serial Peripheral Interface module.

### Utility Modules
Additional functionality is categorized into smaller modules:
* `interrupts` (Supports `Change`, `Falling`, and `Rising` modes)
* `math`
* `random`