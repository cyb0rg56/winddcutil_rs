# winddcutil_rs

A Rust implementation of the [ddcutil](https://github.com/rockowitz/ddcutil) Linux program for Windows. Uses the VESA Monitor Control Command Set (MCCS) over the Display Data Channel Command Interface Standard (DDC-CI) to query and change monitor settings, such as brightness, color levels, input switching and power modes.

## Features

- Detect connected monitors
- Query monitor capabilities
- Get VCP feature values
- Set VCP feature values

## Installation

### From Source

```
git clone https://github.com/cyb0rg56/winddcutil_rs.git
cd winddcutil_rs
cargo build --release
```

The executable will be located at `target/release/winddcutil_rs.exe`.

## Usage

```
winddcutil_rs [COMMAND]
```

### Commands

- `detect`: Lists all connected monitors with DDC/CI support
- `capabilities <display>`: Shows the capabilities of the specified monitor
- `getvcp <display> <feature_code>`: Gets the current value of a VCP feature
- `setvcp <display> <feature_code> <new_value>`: Sets a VCP feature to a specific value

### Examples

```
# List all monitors
winddcutil_rs detect

# Get monitor capabilities
winddcutil_rs capabilities 1

# Get brightness (feature code 0x10)
winddcutil_rs getvcp 1 0x10

# Set brightness to 75
winddcutil_rs setvcp 1 0x10 75

# Toggle input source between HDMI1 (0x11) and HDMI2 (0x12)
winddcutil_rs setvcp 1 0x60 0x11
```

## Common VCP Feature Codes

- `0x10`: Brightness
- `0x12`: Contrast
- `0x60`: Input Source
- `0xD6`: Power Mode

## Troubleshooting

### No Monitors Found

If you receive a "No monitors with DDC/CI support found" error:

1. Ensure your monitor supports DDC/CI
2. Enable DDC/CI in your monitor's OSD menu
3. Try using a different connection type (HDMI, DisplayPort, etc.)
4. Some monitors require specific drivers to be installed

### Invalid Feature Code

If you receive an "Invalid feature code" error:

1. Ensure the feature code is valid (between 0x00 and 0xFF)
2. Check if your monitor supports the specific feature code
3. Use `capabilities` command to check supported features
