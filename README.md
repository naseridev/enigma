# Enigma Machine

**Languages:** English | [Deutsch](README.de.md)

A historically accurate implementation of the WWII Enigma machine cipher system written in Rust. This simulator faithfully reproduces the mechanical operation of the original Enigma machine, including rotor stepping mechanics, plugboard configurations, and reflector functionality.

## Overview

The Enigma machine was a cipher device used extensively by Nazi Germany during World War II for encrypting and decrypting secret messages. This implementation provides a complete simulation of the machine's cryptographic mechanisms, making it suitable for educational purposes, historical research, and cryptographic analysis.

### Key Features

- **Authentic Rotor Mechanics**: Three-rotor system with historically accurate stepping behavior including double-stepping
- **Configurable Plugboard**: Support for bidirectional character swapping through TOML configuration
- **Random Reflector Generation**: Automatic generation of valid reflector wirings
- **Serialized Rotor States**: Binary serialization of daily key configurations
- **Command-Line Interface**: Full CLI with generation and encryption capabilities
- **Error Handling**: Comprehensive error management for invalid inputs and configurations

## Technical Architecture

### Rotor System

The implementation uses three rotors with the following characteristics:

- **Rotor 1 (Right)**: Fast rotor with notch at position 16
- **Rotor 2 (Middle)**: Medium rotor with notch at position 4
- **Rotor 3 (Left)**: Slow rotor with notch at position 21

Each rotor implements both forward and backward encoding paths, simulating the electrical signal path through the physical machine.

### Plugboard Configuration

The plugboard allows for up to 13 bidirectional character pairs to be configured via TOML files. This feature significantly increases the keyspace and was a crucial security enhancement in later Enigma variants.

### Alphabet Support

This implementation supports a 53-character alphabet including:
- Lowercase letters (a-z)
- Uppercase letters (A-Z)
- Space character

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo package manager

### Dependencies

```toml
[dependencies]
clap = "4.0"
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
bincode = "1.3"
toml = "0.8"
```

### Building

```bash
git clone https://github.com/naseridev/enigma.git
cd enigma
cargo build --release
```

## Usage

### Initial Setup

Before using the Enigma machine, you must first generate the necessary configuration files:

#### 1. Generate Rotor Configuration (Daily Key)

Generate a new daily key file with randomized rotor wirings:

```bash
# Generate with default filename
./enigma --generate

# Generate with custom filename
./enigma --generate --rotor-file my_daily_key.enigma

# Generate with specific output path
./enigma -g -r /path/to/keys/enigma_20241125.key
```

#### 2. Generate Plugboard Configuration

Create a plugboard configuration template file:

```bash
# Generate with default filename
./enigma --generate-plugboard

# Generate with custom filename
./enigma --generate-plugboard --plugboard-file my_plugboard.toml

# Generate with specific output path
./enigma -p -b /path/to/configs/station_plugboard.toml
```

After generation, edit the plugboard file to add your desired character pairs:

```toml
pairs = [
    "ab",  # Maps 'a' to 'b' and 'b' to 'a'
    "CD",  # Maps 'C' to 'D' and 'D' to 'C'
    "ef",  # Maps 'e' to 'f' and 'f' to 'e'
    "XY",  # Maps 'X' to 'Y' and 'Y' to 'X'
    "z ",  # Maps 'z' to space and space to 'z'
]
```

### Basic Operations

#### Simple Encryption/Decryption

Basic encryption using default settings (rotor positions "aaa"):

```bash
./enigma "HELLO WORLD"
# Output: encoded message

./enigma "hello world"
# Output: encoded message (case preserved)

./enigma "The quick brown fox jumps over the lazy dog"
# Output: encoded message with spaces preserved
```

#### Using Custom Rotor Positions

Set initial rotor positions for encryption:

```bash
# Set rotor positions to A, B, C
./enigma --start-positions "ABC" "SECRET MESSAGE"

# Set rotor positions to X, Y, Z
./enigma -s "XYZ" "TOP SECRET"

# Use lowercase positions
./enigma --start-positions "xyz" "classified intel"

# Mix case positions
./enigma -s "AbC" "mixed case message"
```

### Advanced Configuration

#### Using Custom Configuration Files

Specify custom rotor and plugboard files:

```bash
# Use custom rotor file only
./enigma --rotor-file custom_rotors.enigma "MESSAGE"

# Use custom plugboard file only
./enigma --plugboard-file custom_plugboard.toml "MESSAGE"

# Use both custom files
./enigma --rotor-file daily_key_nov25.enigma --plugboard-file station_alpha.toml "MESSAGE"

# Use abbreviated flags
./enigma -r my_key.enigma -b my_plugboard.toml -s "DEF" "ENCRYPTED TEXT"
```

#### Complete Command Examples

Full encryption with all parameters specified:

```bash
# Morning transmission setup
./enigma \
  --rotor-file keys/morning_key.enigma \
  --plugboard-file configs/station_1_plugboard.toml \
  --start-positions "MRN" \
  "Weather report clear skies expected"

# Evening transmission setup
./enigma \
  -r keys/evening_key.enigma \
  -b configs/station_2_plugboard.toml \
  -s "EVE" \
  "Mission accomplished return to base"

# Emergency transmission (minimal plugboard)
./enigma \
  --rotor-file emergency_key.enigma \
  --start-positions "SOS" \
  "Emergency evacuation required immediately"
```

### Decryption Process

The Enigma machine is symmetric - decryption uses the same process as encryption:

```bash
# Original message
./enigma -s "ABC" "ATTACK AT DAWN"
# Output example: "FKPQM ZU XWVN"

# Decrypt (same settings)
./enigma -s "ABC" "FKPQM ZU XWVN"
# Output: "ATTACK AT DAWN"
```

### Batch Operations

#### Multiple Messages with Same Settings

```bash
# Encrypt multiple messages maintaining rotor state
./enigma -s "KEY" "FIRST MESSAGE" > encrypted1.txt
./enigma -s "KEY" "SECOND MESSAGE" > encrypted2.txt
./enigma -s "KEY" "THIRD MESSAGE" > encrypted3.txt
```

#### Different Daily Keys

```bash
# Monday's messages
./enigma -r keys/monday.enigma -s "MON" "Morning briefing complete"

# Tuesday's messages
./enigma -r keys/tuesday.enigma -s "TUE" "Afternoon patrol scheduled"

# Wednesday's messages
./enigma -r keys/wednesday.enigma -s "WED" "Evening report submitted"
```

### Troubleshooting Common Issues

#### File Not Found Errors

```bash
# Check if rotor file exists
ls -la daily_key.enigma

# Generate if missing
./enigma --generate

# Use absolute path if needed
./enigma --rotor-file /full/path/to/daily_key.enigma "MESSAGE"
```

#### Invalid Character Errors

```bash
# Valid characters only
./enigma "VALID MESSAGE with spaces"

# Invalid characters will cause errors
./enigma "Invalid: 123 !@#"  # Numbers and symbols not supported
```

#### Invalid Rotor Position Errors

```bash
# Valid positions (must be 3 characters from alphabet + space)
./enigma -s "ABC" "message"  # Valid
./enigma -s "xyz" "message"  # Valid
./enigma -s "A z" "message"  # Valid (includes space)

# Invalid positions
./enigma -s "AB" "message"   # Error: too short
./enigma -s "ABCD" "message" # Error: too long
./enigma -s "AB1" "message"  # Error: invalid character
```

### Command-Line Arguments Reference

#### Required Arguments
- `<message>`: Message to encrypt/decrypt (required unless using generation flags)

#### Optional Arguments
- `-g, --generate`: Generate new rotor configuration file
- `-p, --generate-plugboard`: Generate plugboard configuration template
- `-r, --rotor-file <FILE>`: Path to rotor configuration file (default: `./daily_key.enigma`)
- `-b, --plugboard-file <FILE>`: Path to plugboard configuration file (default: `./plugboard.toml`)
- `-s, --start-positions <POSITIONS>`: Initial rotor positions as exactly 3 characters (default: `"aaa"`)

#### Help and Version
```bash
./enigma --help     # Display help information
./enigma --version  # Display version information
```

### Performance Considerations

For optimal performance with large messages:

```bash
# Pipe large files
cat large_message.txt | xargs ./enigma -s "KEY"

# Process multiple files
for file in messages/*.txt; do
    ./enigma -s "$(date +%j)" "$(cat "$file")" > "encrypted/$(basename "$file")"
done
```

## Configuration Files

### Rotor Configuration

Rotor configurations are stored as binary files using bincode serialization. The structure contains three randomized substitution alphabets ensuring no fixed points exist (a security requirement).

### Plugboard Configuration

Plugboard settings use TOML format:

```toml
# Enigma Plugboard Configuration
pairs = [
    "ab",  # a <-> b
    "CD",  # C <-> D
    "X ",  # X <-> space
]
```

## Security Considerations

### Historical Context

While this implementation is historically accurate, the Enigma cipher was broken during WWII through various cryptanalytic techniques. Modern security applications should not rely on Enigma-based encryption.

### Cryptographic Weaknesses

- **No Self-Encryption**: Characters cannot encrypt to themselves
- **Predictable Rotor Movement**: Regular stepping patterns can be exploited
- **Limited Keyspace**: Despite seeming complexity, the effective keyspace is smaller than modern standards
- **Frequency Analysis Vulnerability**: Long messages remain susceptible to statistical attacks

## Educational Applications

This simulator serves multiple educational purposes:

### Cryptography Education

- Demonstrates historical symmetric encryption techniques
- Illustrates the importance of key management and distribution
- Shows evolution from mechanical to electronic cryptographic systems

### Computer Science Pedagogy

- Example of complex state machine implementation
- Demonstrates serialization and configuration management
- Illustrates command-line application design patterns

### Historical Research

- Provides accurate simulation for studying WWII communications
- Enables analysis of historical intercepts and decryptions
- Supports understanding of codebreaking methodologies

## Implementation Details

### Rotor Stepping Algorithm

The stepping mechanism implements the "double stepping" anomaly present in actual Enigma machines, where the middle rotor steps on consecutive key presses when aligned with its notch position.

### Reflector Generation

The reflector is randomly generated ensuring each character maps to exactly one other character, with no character mapping to itself.

### Error Handling

Comprehensive error handling covers:
- Invalid rotor positions
- Malformed messages containing unsupported characters
- File I/O errors
- Serialization/deserialization failures
- Invalid plugboard configurations

## Performance Characteristics

- **Memory Usage**: Minimal heap allocation during operation
- **Processing Speed**: Linear time complexity relative to message length
- **File I/O**: Efficient binary serialization for rotor states
- **Configuration Loading**: TOML parsing with validation

## Contributing

Contributions are welcome for:

- Additional rotor configurations based on historical variants
- Enhanced error messages and user experience improvements
- Performance optimizations
- Additional alphabet support
- Historical accuracy improvements