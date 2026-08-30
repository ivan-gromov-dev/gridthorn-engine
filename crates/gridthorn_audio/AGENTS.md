# Audio Crate Instructions

- Own decoded audio data, playback commands, voice identifiers, and future
  output-device adapters.
- Keep backend and operating-system audio types behind engine-owned contracts.
- Audio commands are presentation effects and must not become authoritative
  simulation state.
- Headless use of decoded clips and command queues must not require an output
  device.
