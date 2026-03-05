# chron-hologram ⏳🔮

**Lineage:** `experiments/chrontext` × `experiments/hologram-text`

## Concept

"Spectral History". This experiment crosses the git blame chronological age parsing of `chrontext` with the FFT-based optical interference rendering of `hologram-text`.

It computes the age of every line of code in a file and uses that scalar as a density field. This density field is then passed through a Fast Fourier Transform to simulate a holographic interference pattern in 2D space. Old code generates low-frequency waves, while new code introduces high-frequency turbulence.

## Novel Trait

**Codebase Holography.** By transforming a file's history into an interference pattern, you are viewing the *spectral signature* of the codebase's age. It provides an emergent way to "see" technical debt or active feature development not as simple colored lines, but as rippling, colliding waveforms.

## Lineage Details
- From `chrontext`: Git repository parsing, `BlameOptions`, age score normalization based on commit time.
- From `hologram-text`: The Fast Fourier Transform (`rustfft`) rendering pipeline turning scalar arrays into 2D visual interference patterns.
- Novel Emergence: The translation of time (age) directly into physical wavelength (frequency).
