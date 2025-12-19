# 🎨 **AURORA RUNNER - ASSETS CREATION GUIDE**

Complete guide for creating custom sprites, audio, and visual assets to replace the placeholder graphics with professional artwork.

---

## 📋 **CURRENT STATE**

The game is **100% playable** with programmatic graphics (colored shapes). This guide helps you add custom artwork for a more polished look.

**What's Already Working:**
- ✅ Player (white rectangle with cyan glow)
- ✅ Obstacles (geometric shapes)
- ✅ Particles (colored circles)
- ✅ UI (text and rectangles)
- ✅ Effects (aurora waves, gradients)

---

## 🎨 **SPRITE REQUIREMENTS**

### File Formats Supported
- **PNG** (recommended - transparency)
- **WebP** (smaller file size, animated support)
- **GIF** (animated, older format)
- **JPEG** (no transparency, backgrounds only)

### Technical Specifications
- **Color Mode**: RGBA (32-bit)
- **Resolution**: Match game scale (low-res pixel art)
- **File Size**: <100 KB per sprite
- **Naming**: lowercase, no spaces (use-dashes)

---

## 🦌 **PLAYER SPRITE**

### Requirements
```
Dimensions: 16x16 pixels (base)
Frames: 4-6 (for animation)
Format: PNG or animated WebP
Style: Silhouette or pixel art
Colors: White/cyan theme
```

### Sprite Sheet Layout (Optional)
```
┌────┬────┬────┬────┐
│ 1  │ 2  │ 3  │ 4  │  Frame numbers
└────┴────┴────┴────┘
16px × 16px each frame
```

### Animation States

**1. Idle/Running (player.png)**
- Simple reindeer silhouette
- Glowing cyan eyes
- Small antlers
- Can be static or 2-frame idle

**2. Jump (player-jump.png)** *(Optional)*
- Legs tucked
- Upward pose
- Stretched vertically

**3. Fall (player-fall.png)** *(Optional)*
- Legs extended
- Downward pose
- Rotation applied in code

### Design Tips
- **Silhouette First**: Should be recognizable as black shape
- **Minimal Detail**: Low-res = less is more
- **Consistent Style**: Match game's pixel aesthetic
- **Readable**: Clear at 16×16 even when moving

### Color Palette
```
Body:       #ffffff (white)
Eyes:       #00ffff (cyan glow)
Antlers:    #ffffff or #cccccc
Outline:    Optional 1px black
```

### Example Pixel Art
```
    ██
  ██  ██      Antlers
  ██████      Head
  ○  ○        Eyes (cyan)
  ██████      Body
 ██    ██     Legs
```

### File Naming
```
sprites/player.png              # Main sprite
sprites/player-idle.png         # Alternative
sprites/player-jump.png         # Jump state
sprites/player-fall.png         # Fall state
```

---

## 💎 **OBSTACLE SPRITES**

### 1. Ice Crystal

```
Dimensions: 12x20 pixels
Shape: Diamond/crystal
Colors: Blues and whites
Transparency: Yes
```

**Visual Design:**
```
      ██
    ██████
  ██████████
██████████████
  ██████████
    ██████
      ██
```

**Color Palette:**
- Outer: #88ffffff (ice blue)
- Inner: #00ffff44 (cyan glow)
- Highlights: #ffffff (sparkle)

**File:** `sprites/crystal.png`

### 2. Floating Rock

```
Dimensions: 16x16 pixels
Shape: Rounded asteroid
Colors: Grays and purples
Transparency: Yes
```

**Visual Design:**
```
  ████████
████████████
████████████
████████████
  ████████
```

**Color Palette:**
- Base: #666666 (dark gray)
- Highlights: #999999 (light gray)
- Shadow: #333333 (darker)

**File:** `sprites/rock.png`

### 3. Dark Zone (Cloud)

```
Dimensions: 24x30 pixels
Shape: Wavy cloud
Colors: Purples and blacks
Transparency: Yes (semi-transparent)
```

**Animation:** Optional 2-3 frame wave

**Color Palette:**
- Base: #440044aa (purple, 66% opacity)
- Shadow: #220022dd (darker purple)

**File:** `sprites/dark-zone.png`

---

## ⭐ **COLLECTIBLE SPRITES**

### Star

```
Dimensions: 8x8 pixels
Shape: 5-point star
Animation: 2-4 frame sparkle
Format: Animated WebP or sprite sheet
```

**Visual Design:**
```
    ██
  ██████
██████████
  ██████
    ██
```

**Color Palette:**
- Main: #ffff00ff (gold)
- Highlight: #ffffff (white sparkle)
- Glow: #ffff0088 (soft yellow)

**Animation Frames:**
1. Normal size
2. Slightly larger
3. Back to normal
4. Smaller

**File:** `sprites/star.png` or `sprites/star.webp`

---

## ✨ **PARTICLE SPRITES** (Optional)

Replace circular particles with custom shapes:

### Sparkle Particle
```
Size: 4x4 pixels
Shape: Plus sign or diamond
Colors: Variable (set in code)
```

### Trail Particle
```
Size: 2x2 pixels
Shape: Small dot or streak
Colors: Cyan gradient
```

**Files:**
- `sprites/particle-sparkle.png`
- `sprites/particle-trail.png`

---

## 🎵 **AUDIO ASSETS**

### Sound Effects

#### 1. Jump Sound
```
Type: SFX
Duration: 0.1-0.3 seconds
Style: Soft whoosh, airy
Format: WAV or OGG
Volume: -10 dB (quiet)
```

**Creation Tips:**
- Use synthesizer with filter sweep
- Add slight reverb
- Pitch: Middle to high
- No harsh transients

**File:** `audio/jump.wav`

#### 2. Star Collection
```
Type: SFX
Duration: 0.2-0.5 seconds
Style: Magical chime
Format: WAV or OGG
Volume: -8 dB
```

**Creation Tips:**
- Bell or glockenspiel sound
- Major chord (C, E, G)
- Slight reverb
- Pleasant, not annoying

**File:** `audio/star-collect.wav`

#### 3. Obstacle Pass
```
Type: SFX
Duration: 0.1 seconds
Style: Subtle confirmation
Format: WAV or OGG
Volume: -15 dB (very quiet)
```

**Creation Tips:**
- Short click or tick
- High frequency
- Very subtle
- Background feedback

**File:** `audio/pass.wav`

#### 4. Death/Collision
```
Type: SFX
Duration: 0.3-0.5 seconds
Style: Soft impact
Format: WAV or OGG
Volume: -12 dB
```

**Creation Tips:**
- NOT harsh or loud
- Muffled impact
- Descending pitch
- Disappointing but not punishing

**File:** `audio/death.wav`

#### 5. Combo Milestone
```
Type: SFX
Duration: 0.5-1.0 seconds
Style: Ascending musical notes
Format: WAV or OGG
Volume: -10 dB
```

**Creation Tips:**
- 3-note ascending scale
- Major key (happy)
- Synthesizer or xylophone
- Celebratory but brief

**File:** `audio/combo.wav`

### Background Music

#### Title Screen Music
```
Type: Music
Duration: 60 seconds (looping)
Tempo: 80-100 BPM
Style: Ambient, atmospheric
Format: MP3 or OGG
```

**Instrumentation:**
- Synth pads (atmospheric)
- Bells or chimes (Christmas feel)
- Soft strings (ambient)
- Minimal percussion

**Mood:** Calm, inviting, magical

**File:** `audio/title-theme.mp3`

#### Gameplay Music
```
Type: Music
Duration: 2-3 minutes (looping)
Tempo: 100-120 BPM
Style: Flowing, meditative
Format: MP3 or OGG
```

**Instrumentation:**
- Piano (main melody)
- Synth arpeggio (movement)
- Strings (warmth)
- Soft kick/snare (rhythm)

**Mood:** Focused, uplifting, flow-inducing

**File:** `audio/gameplay-theme.mp3`

### Audio Technical Specs
```
Sample Rate: 44.1 kHz or 48 kHz
Bit Depth: 16-bit
Channels: Mono (SFX) or Stereo (Music)
Compression: MP3 (192 kbps) or OGG Vorbis (128 kbps)
```

---

## 🖼️ **UI ASSETS** (Optional)

### Title Logo
```
Dimensions: 256x64 pixels
Format: PNG with transparency
Style: Pixel art text
Colors: Aurora gradient
```

**File:** `sprites/logo.png`

### Button Sprites
```
Dimensions: 64x16 pixels
States: Normal, Hover, Pressed
Format: PNG
```

**Files:**
- `sprites/button-start.png`
- `sprites/button-retry.png`

### Icons
```
Dimensions: 16x16 pixels
Style: Pixel art
Format: PNG
```

**Files:**
- `sprites/icon-star.png` (UI display)
- `sprites/icon-combo.png` (multiplier)
- `sprites/icon-altitude.png` (meter)

---

## 🎨 **ASSET CREATION TOOLS**

### Free Tools

**Pixel Art**
- [Aseprite](https://www.aseprite.org/) - $20, best tool
- [Piskel](https://www.piskelapp.com/) - Free, web-based
- [LibreSprite](https://libresprite.github.io/) - Free Aseprite fork
- [GIMP](https://www.gimp.org/) - Free, full-featured

**Audio**
- [Audacity](https://www.audacityteam.org/) - Free audio editor
- [LMMS](https://lmms.io/) - Free music production
- [Bosca Ceoil](https://boscaceoil.net/) - Simple music maker
- [BeepBox](https://www.beepbox.co/) - Web-based chiptune

**Sound Effects**
- [Chiptone](https://sfbgames.itch.io/chiptone) - 8-bit SFX generator
- [BFXR](https://www.bfxr.net/) - Classic SFX generator
- [Audacity Generate Menu](https://www.audacityteam.org/) - Built-in tools

### Paid Tools (Professional)

**Pixel Art**
- [Aseprite](https://www.aseprite.org/) - $20
- [PyxelEdit](https://pyxeledit.com/) - $9

**Audio**
- [FL Studio](https://www.image-line.com/) - $99+
- [Ableton Live](https://www.ableton.com/) - $99+

---

## 📦 **ASSET PACKS** (Pre-Made)

### Free Asset Sources

**Sprites**
- [OpenGameArt.org](https://opengameart.org/) - CC0 assets
- [Itch.io Free Assets](https://itch.io/game-assets/free) - Various licenses
- [Kenney.nl](https://www.kenney.nl/) - Huge free library
- [Game-Icons.net](https://game-icons.net/) - Icon library

**Audio**
- [Freesound.org](https://freesound.org/) - Community SFX
- [OpenGameArt.org](https://opengameart.org/) - Music + SFX
- [Incompetech.com](https://incompetech.com/) - Kevin MacLeod music
- [Purple Planet Music](https://www.purple-planet.com/) - Free game music

### Paid Asset Sources

**Sprites**
- [Itch.io Marketplace](https://itch.io/game-assets) - $5-50
- [GameDev Market](https://www.gamedevmarket.net/) - Various prices
- [Unity Asset Store](https://assetstore.unity.com/) - Can use externally

**Audio**
- [AudioJungle](https://audiojungle.net/) - $1-30 per track
- [Epidemic Sound](https://www.epidemicsound.com/) - $15/month subscription
- [Artlist](https://artlist.io/) - $16.60/month subscription

---

## 🔧 **INTEGRATION GUIDE**

### Adding Sprites to Game

**1. Place Files**
```bash
cp your-sprite.png sprites/player.png
cp crystal.png sprites/crystal.png
```

**2. Update Code**

Replace this:
```rust
// Old: Programmatic drawing
rect!(
    x = self.player.x,
    y = self.player.y,
    w = 16,
    h = 16,
    color = COLOR_PLAYER
);
```

With this:
```rust
// New: Sprite drawing
sprite!(
    "player",
    x = self.player.x as i32,
    y = self.player.y as i32,
    w = 16,
    h = 16
);
```

**3. Test**
```bash
turbo run -w aurora-runner
```

Sprites auto-reload when changed!

### Adding Audio to Game

**1. Place Files**
```bash
cp jump.wav audio/
cp star-collect.wav audio/
```

**2. Update Code**

```rust
// Play sound effect
if gp.a.just_pressed() {
    sfx!("jump");
}

// Play music (looping)
music!("gameplay-theme", looping = true);
```

**3. Volume Control** (if needed)
```rust
// Set volume (0.0 to 1.0)
sfx!("jump", volume = 0.5);
music!("gameplay-theme", volume = 0.3, looping = true);
```

---

## 🎯 **PRIORITY ORDER**

What to create first:

### Phase 1: Essential
1. ✅ **Player sprite** - Most visible
2. ✅ **Crystal obstacle** - Most common
3. ✅ **Star collectible** - Important feedback

### Phase 2: Polish
4. **Jump sound** - Most frequent SFX
5. **Star collect sound** - Satisfying feedback
6. **Floating rock obstacle** - Variety

### Phase 3: Professional
7. **Background music** - Atmosphere
8. **Death sound** - Emotional impact
9. **UI logo** - Brand identity
10. **All remaining obstacles and effects**

---

## 📏 **ASSET DIMENSIONS REFERENCE**

Quick reference for all sprite sizes:

```
Player:        16×16 px
Crystal:       12×20 px
Rock:          16×16 px
Dark Zone:     24×30 px
Star:          8×8 px
Particles:     2-4 px
UI Icons:      16×16 px
Logo:          256×64 px
Buttons:       64×16 px
```

---

## 🎨 **STYLE GUIDE**

### Visual Consistency

**Do:**
- ✅ Keep pixel art crisp (no anti-aliasing)
- ✅ Use consistent outline thickness
- ✅ Match color palette
- ✅ Maintain same level of detail

**Don't:**
- ❌ Mix pixel art with HD graphics
- ❌ Use too many colors
- ❌ Add excessive detail
- ❌ Ignore transparency

### Audio Consistency

**Do:**
- ✅ Match volume levels
- ✅ Keep similar audio quality
- ✅ Use consistent style (8-bit, orchestral, etc.)
- ✅ Test in-game before finalizing

**Don't:**
- ❌ Mix loud and quiet sounds
- ❌ Use jarring sound effects
- ❌ Add voice acting (unless very good)
- ❌ Forget to loop music properly

---

## 💡 **PRO TIPS**

### Pixel Art
1. **Start Big, Scale Down**: Draw at 2x or 4x size
2. **Limited Palette**: Use 4-8 colors per sprite
3. **Dithering**: Add texture without extra colors
4. **Animation**: Odd number of frames (3, 5) looks smoother

### Audio
1. **Normalize Volume**: -3 dB peak maximum
2. **Fade In/Out**: Prevent audio pops
3. **Mono for SFX**: Saves space, sounds fine
4. **Loop Points**: Test seamless music loops
5. **Compression**: MP3/OGG for music, WAV for short SFX

### Workflow
1. **Test Often**: Add one asset at a time
2. **Version Control**: Keep original files
3. **Backup**: Save PSD/Aseprite source files
4. **Get Feedback**: Show to others early

---

## 📦 **EXAMPLE ASSET PACK**

Want to skip creation? Download these:

**Free Packs Matching This Style:**
- "Kenney Pixel Platformer" - Character sprites
- "Ice Crystal Pack" - Obstacle sprites  
- "Chiptone SFX Pack" - Sound effects
- "Ambient Synth Pack" - Background music

**Paid Packs ($5-20):**
- "Pixel Art Runner Kit" on Itch.io
- "Christmas Game SFX Bundle" on AudioJungle

---

## ✅ **ASSET CHECKLIST**

Use this to track progress:

**Sprites:**
- [ ] Player (main)
- [ ] Crystal obstacle
- [ ] Rock obstacle
- [ ] Dark zone
- [ ] Star collectible
- [ ] Particle effects (optional)
- [ ] UI logo (optional)

**Audio:**
- [ ] Jump SFX
- [ ] Star collect SFX
- [ ] Death SFX
- [ ] Combo milestone SFX
- [ ] Title music
- [ ] Gameplay music

**Testing:**
- [ ] All assets load correctly
- [ ] No visual glitches
- [ ] Audio plays at correct times
- [ ] File sizes optimized
- [ ] Game exports successfully

---

## 🚀 **READY TO CREATE!**

With this guide, you can:
1. Create custom pixel art sprites
2. Add professional sound effects
3. Integrate assets into the game
4. Export and deploy the polished version

**The game works perfectly now. Assets are the cherry on top!** 🍒

---

**Need help?** Check the Turbo docs: https://docs.turbo.computer/

**Good luck creating! 🎨✨**