# Hyperbolic Spectrum

**"The Echo of Infinity"**

A visualization of audio spectra on the Poincaré Disk. Sound waves are mapped to hyperbolic space, where they expand outwards towards the boundary of infinity.

## Lineage

*   **Parent A**: `experiments/spectral-scribe` (Audio Encoding/Decoding)
*   **Parent B**: `experiments/hyperbolic-lexicon` (Poincaré Disk Visualization)

## Concept

Text input is converted into an audio signal using spectral synthesis (mapping characters to frequency bands). This audio is then analyzed in real-time, and its frequency spectrum is projected onto the Poincaré Disk.

*   **Radial Axis**: Time. The center is the "Source" (Now). As sound travels, it moves outwards towards the boundary (Past).
*   **Angular Axis**: Frequency. Low frequencies to High frequencies are mapped around the circle.
*   **Hyperbolic Geometry**: Due to the exponential expansion of space near the boundary, the "past" is compressed into the infinite edge of the disk.

## Controls

*   **Type**: Enter text to be transmitted.
*   **Enter**: Transmit (Generate Audio & Visualize).
*   **Arrow Keys**: Pan the view (Hyperbolic Translation).
