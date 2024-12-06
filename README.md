# Discoboard emulator

Emulates the discovery board/microbit used in COMP2300.

## Using

1. This is a Rust project, so you will need to [install Rust](https://www.rust-lang.org/tools/install). Check the tools work with
```
cargo --version
```

2. Once installed, clone this repo and navigate to the repo root. Run
```
cargo build
```

3. Follow one of the following to use this to debug a microbit project. Make sure to use the correct paths instead of the placeholders. Change `debug` to `release` in the emulator path if you want to use a release build made with `cargo build --release` (use this if you want the best performance).

    1. If using `cortex-debug` to debug, add the following to `.vscode/launch.json` configurations. 

    ```
    {
        "type": "cortex-debug",
        "request": "launch",
        "name": "Build & Debug (emulator)",
        "cwd": "${workspaceRoot}",
        "device": "STM32L476VG",
        "executable": "program.elf",
        "servertype": "qemu",
        "preLaunchTask": "Build",
        "serverpath": "/abs/path/to/comp2300-disco-emulator/target/debug/discoserver",
        "postLaunchCommands": [
            "-break-insert -t main"
        ]
    }
    ```

    ```
    {
        "type": "cortex-debug",
        "request": "launch",
        "name": "ARM Emulator Debug",
        "cwd": "${workspaceRoot}",
        "device": "STM32L476vg",
        "executable": "${workspaceRoot}/.pio/build/disco_l476vg/firmware.elf",
        "servertype": "qemu",
        "preLaunchTask": "PlatformIO: Build",
        "serverpath": "/abs/path/to/comp2300-disco-emulator/target/debug/discoserver",
        "postLaunchCommands": [
            "-break-insert main"
        ]
    }
    ```

4. Select the `Build & Debug (Emulator)` option in the debug selection and use it like normal.


### Project structure

- `src/main.rs`: The entry point of the `discoserver` executable. Wraps the `disco_emulator` library in a GDB remote protocol compatible server.
- `disco_emulator/src/lib.rs`: The entry point for the emulator itself.
- `tests/*`: Tests for the emulator.


### Tests

Run the tests with `cargo test --workspace`. Typically each integration test compiles a corresponding program using `arm-none-eabi-as` and `arm-none-eabi-ld`, so make sure these are on your PATH. It then steps through, checking registers and flags for correct values.
