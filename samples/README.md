# Sample Directory

Place your audio files in the appropriate directories:

- `drums/` - Drum samples (kicks, snares, hi-hats, etc.)
- `bass/` - Bass samples and low-frequency sounds
- `lead/` - Lead synths and melodic elements
- `vocal/` - Vocal samples and voice sounds

## Supported Formats
- WAV (.wav)
- MP3 (.mp3)
- FLAC (.flac)
- OGG (.ogg)

## Sample-to-Pad Assignment

### Automatic Assignment (Default)
Files are loaded alphabetically to the first available pad.

### Naming Convention for Specific Pads
Include a number in your filename to assign to a specific pad:

**Examples:**
- `kick_01.wav` → Pad 0 (key '7')
- `snare-05.wav` → Pad 4 (key 'U') 
- `hihat_13.wav` → Pad 12 (key 'M')
- `01_kick.wav` → Pad 0 (key '7')
- `13.wav` → Pad 12 (key 'M')
- `16.wav` → Pad 15 (key '/')

**Supported Patterns:**
- `NUMBER_name.ext` (01_kick.wav)
- `name_NUMBER.ext` (kick_01.wav)
- `name-NUMBER.ext` (snare-05.wav)
- `name.NUMBER.ext` (hihat.12.wav)
- `padNUMBER.ext` (pad00.wav)
- `pNUMBER_name.ext` (p15_crash.wav)

### Pad Layout Reference
```
Pads 0-3:   7 8 9 0    (Files: 01-04.wav)
Pads 4-7:   U I O P    (Files: 05-08.wav)
Pads 8-11:  J K L ;    (Files: 09-12.wav)
Pads 12-15: M , . /    (Files: 13-16.wav)
```

**For M,./ keys specifically:**
- `13.wav` → M key (Pad 12)
- `14.wav` → , key (Pad 13)  
- `15.wav` → . key (Pad 14)
- `16.wav` → / key (Pad 15)

## Advanced Configuration (JSON)
For precise control, create a `config.json` file in the samples directory:

```bash
cargo run generate-config  # Creates config.example.json
cp config.example.json config.json  # Copy and edit as needed
```

**JSON Format:**
```json
{
  "mappings": [
    {
      "group": 0,
      "pad": 0, 
      "file": "drums/kick.wav",
      "name": "Kick"
    }
  ]
}
```

- `group`: 0=DRUMS, 1=BASS, 2=LEAD, 3=VOCAL
- `pad`: 0-15 (see pad layout above)
- `file`: Path relative to samples/ or absolute path
- `name`: Optional display name (overrides filename)

## Loading Priority
1. **JSON Config** - If `samples/config.json` exists, uses exact mappings
2. **Filename Convention** - Extracts pad numbers from filenames  
3. **Alphabetical** - Loads remaining files to first available pads

## Usage
1. Copy your audio files to subdirectories
2. Choose your preferred assignment method:
   - Use naming conventions (kick_01.wav)
   - Create config.json for exact control
   - Let files auto-load alphabetically
3. Restart the application

## Empty State
All pads start empty. Load your own samples to begin making music!