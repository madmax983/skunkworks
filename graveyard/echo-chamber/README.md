# Echo Chamber ⚛️🔊

**Genesis: The Acoustician**

A 2D Acoustic Simulation using Finite Difference Time Domain (FDTD) to visualize and hear the shape of a room.

## Concept
Simulate sound as a pressure wave propagating through a 2D grid. The user can draw walls with rigid boundaries (Neumann conditions) and listen to the impulse response of the room in real-time.

## Tech Stack
- **Physics**: 2D Wave Equation (FDTD).
- **Audio**: `cpal` for real-time audio synthesis (44.1kHz).
- **Visuals**: `ratatui` for heat-map visualization of the pressure field.

## Controls
- **Arrow Keys**: Move Cursor.
- **Space**: Pluck (inject impulse at cursor).
- **w**: Toggle Wall at cursor.
- **l**: Move Listener (Microphone) to cursor.
- **1**: Clear Room.
- **2**: Box Preset.
- **3**: Chamber Preset.
- **q**: Quit.

## The Physics
The simulation solves the 2D Wave Equation:
$$ \frac{\partial^2 u}{\partial t^2} = c^2 \nabla^2 u $$

Using the FDTD method:
$$ u^{n+1}_{i,j} = 2u^n_{i,j} - u^{n-1}_{i,j} + C^2 (u^n_{i+1,j} + u^n_{i-1,j} + u^n_{i,j+1} + u^n_{i,j-1} - 4u^n_{i,j}) $$

Where $C = c \Delta t / \Delta x$ is the Courant number.

Rigid walls are modeled using Neumann boundary conditions ($\partial u / \partial n = 0$), meaning sound reflects without phase inversion, preserving the energy in the "room" mode.
