# Volume Control by Process

This is a Rust program that allows you to control the volume of a specific application based on its Process ID (PID) on Linux. It utilizes tools such as `xdotool` and `pactl` to identify the PID of the focused application and adjust the corresponding volume.

# Prerequisites
##### Make sure you have the following tools installed on your system:

- `Rust`: You can install it via [Rust](https://rustup.rs/).
- `xdotool`: It's a tool that allows simulating key presses and mouse movements.
- `pulseaudio-utils`: This package contains the pactl tool that is used to control PulseAudio.

## Notes
- This program was developed and tested on Linux systems, specifically on Ubuntu. It may not work correctly on other operating systems.
- Ensure that the applications you wish to control are using PulseAudio as the audio server.
- This program assumes that you have adequate permissions to execute shell commands and access PulseAudio information.

# Contributing
Contributions are welcome! Feel free to open issues if you encounter bugs or have suggestions for improvements.

