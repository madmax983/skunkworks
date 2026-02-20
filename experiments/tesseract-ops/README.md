# Tesseract Ops ⚛️⬛🖥️

**A Hyper-Dimensional System Monitor**

Part of the "Spaces that Shouldn't Exist" series.

## Concept
This experiment visualizes your system's "Vital Signs" as the deformation of a 4-Dimensional Hypercube (Tesseract).
A healthy, idle system is a perfect Tesseract. As load increases, the dimensions stretch and warp.

## Lineage
- **Parent A**: `experiments/tesseract-time` (4D Visualization Engine)
- **Parent B**: `experiments/system-attractor` (System Metrics Monitoring)

## The Mapping
The 4 Dimensions ($x, y, z, w$) are mapped to system resources:
- **X-Axis (Width)**: CPU Usage. The higher the load, the wider the cube stretches.
- **Y-Axis (Height)**: Memory Usage. RAM usage stretches the cube vertically.
- **Z-Axis (Depth)**: Swap Usage.
- **W-Axis (Ana/Kata)**: System Load Average. This dimension "breathes" (oscillates) faster and deeper as the load average increases.

## Colors
- **Red Intensity**: CPU Usage
- **Blue Intensity**: Memory Usage
- **Green Intensity**: Inverse Load (Idle = Green)

## Controls
- **Arrow Keys**: Rotate the 3D Camera around the object.
- **W / S**: Zoom in/out.

## Implementation
- Uses `macroquad` for 3D rendering.
- Uses `sysinfo` for real-time system metrics.
- Custom `Vec4` math for 4D rotation and perspective projection.
