# Asset Crate Instructions

- Own asset decoding, validation, and engine-owned asset data.
- Do not expose decoder implementation types through public APIs.
- Keep filesystem loading separate from renderer GPU resources.
- Preserve validated acyclic dependency edges and atomic reload publication;
  failed preparation must leave published data and the retry baseline intact.
- The game owns reload scheduling at presentation boundaries. Background scans
  and decoding must not block frame polling or mutate authoritative game state.
- Keep registration fixed for a reload worker's lifetime; broader loader and
  watching capabilities require an explicit scope and updated asset contract.
