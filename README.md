# Bare‑Metal Rust on nRF52840DK → TouchDesigner  
### HC‑SR04 Distance Measurement with Real‑Time Output

This repository contains a bare‑metal Rust implementation for the nRF52840DK.  
The firmware reads distance values from an HC‑SR04 ultrasonic sensor with microsecond precision and processes the data for use in real‑time interactive systems.

This project was originally developed as part of a thesis exploring real‑time human‑interface systems using Rust and OSC.

---

## Features
- Bare‑metal Rust (`no_std`)
- Precise 10 µs trigger‑pulse generation
- Direct register access (P0.14 = TRIG, P0.15 = ECHO)
- Microsecond‑accurate echo timing
- Outlier filtering and timeout handling
- FPU‑optimized distance calculation
- Designed to forward processed values to a host system for OSC transmission

---

## Limitations & Current Status
Due to **APPROTECT** being enabled on newer revisions of the nRF52840, the firmware could not be fully deployed in a workflow where the microcontroller streams data directly to TouchDesigner via OSC.

### Why OSC output was not completed
- APPROTECT locks the debug interface on every reboot.
- Unlocking requires Nordic’s Command Line Tools, which only run on **x86 systems**.
- Raspberry Pi (ARM) cannot run these tools.
- As a result, the firmware could be flashed only once, and continuous development/debugging was blocked.

Because of this, the OSC transmission stage was not implemented on the microcontroller itself.  
Instead, the system was tested by forwarding sensor data to a host machine, which then handled OSC output.

---

## Future Development
This project can be extended in several ways:

### 1. **Use Zephyr RTOS**
Zephyr includes built‑in support for:
- automatic APPROTECT unlocking  
- device tree configuration  
- stable drivers for GPIO, timers, and networking  

Using Zephyr would allow:
- reliable flashing  
- easier debugging  
- native UDP/OSC support on the nRF52840 (via Zephyr networking stack)

### 2. **Implement OSC directly on the microcontroller**
Once APPROTECT is handled, the next steps would be:
- add a UDP stack (either Zephyr or embassy‑net)
- encode OSC messages (e.g., via a Rust OSC crate)
- send packets directly to TouchDesigner

### 3. **Use a dual‑device architecture**
A stable alternative is:
- nRF52840 handles real‑time sensing  
- Raspberry Pi or PC handles OSC transmission  

This avoids APPROTECT issues entirely.

---

## Requirements
- nRF52840DK  
- Rust toolchain (`no_std`, `thumbv7em-none-eabihf`)  
- `probe-rs` for flashing  
- Host system (Raspberry Pi 5 or PC)  
- TouchDesigner for visualization  
