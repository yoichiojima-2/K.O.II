# K.O.II Terminal

A terminal-based drum machine and sequencer inspired by Teenage Engineering's K.O. II, built with Rust.

![K.O.II Terminal Demo](docs/images/ko-ii-demo.gif)

## Features

- **16-Pad Drum Machine**: 4x4 grid layout with keyboard controls
- **4 Sound Groups**: Drums, Bass, Lead, and Vocal - each with 16 sample slots
- **Real-time Sequencing**: Record and playback patterns with visual feedback
- **Pattern Management**: Up to 99 patterns per group
- **Mixer Controls**: Individual volume and mute controls for each group plus master
- **Built-in Sample Library**: 48 high-quality audio samples included
- **Visual Feedback**: Flashing pads on playback, step sequencer visualization
- **Flexible Tempo**: Adjustable from 60-300 BPM

## Controls

### Pad Controls
```
[7] [8] [9] [0]     (Row 1: Pads 1-4)
[U] [I] [O] [P]     (Row 2: Pads 5-8)
[J] [K] [L] [;]     (Row 3: Pads 9-12)
[M] [,] [.] [/]     (Row 4: Pads 13-16)
```

### Transport & Sequencing
- **SPACE**: Play/Stop playback
- **R**: Toggle recording mode
- **C**: Clear current pattern
- **TAB**: Switch between sound groups (Drums/Bass/Lead/Vocal)
- **←/→**: Navigate through patterns
- **↑/↓**: Adjust tempo (±5 BPM)

### Mixer Controls
- **= / -**: Master volume up/down
- **M**: Toggle master mute
- **1/!**: Drums volume up/down
- **2/@**: Bass volume up/down
- **3/#**: Lead volume up/down
- **4/$**: Vocal volume up/down
- **F1-F4**: Toggle mute for groups 1-4

### General
- **ESC**: Quit application

## Installation

### Prerequisites
- Rust 1.70 or higher
- Audio system support (ALSA on Linux, CoreAudio on macOS, WASAPI on Windows)

### Build from Source
```bash
# Clone the repository
git clone https://github.com/yourusername/ko-ii-terminal.git
cd ko-ii-terminal

# Build and run
cargo run --release
```

## Usage

### Basic Workflow
1. Launch the application with `cargo run`
2. Select a sound group with TAB (Drums, Bass, Lead, or Vocal)
3. Press pad keys (7-0, U-P, J-;, M-/) to trigger sounds
4. Press R to enable recording, then SPACE to start playback
5. Play pads in time to record your pattern
6. Use the mixer controls to adjust volumes and create your mix

### Loading Custom Samples
Create a `samples.json` configuration file to load your own samples:

```bash
cargo run generate-config
```

This creates example configuration files you can modify with your own sample paths.

## Architecture

Built with:
- **Rust**: For performance and reliability
- **Ratatui**: Terminal UI framework
- **Rodio**: Audio playback engine
- **Crossterm**: Cross-platform terminal manipulation

## License

MIT License - See LICENSE file for details

## Acknowledgments

Inspired by Teenage Engineering's K.O. II Sampler