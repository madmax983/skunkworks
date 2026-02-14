# Bifurcation Landscape 🦋

**A 3D Flight over the Edge of Chaos**

This experiment visualizes the [Bifurcation Diagram](https://en.wikipedia.org/wiki/Bifurcation_diagram) of the Logistic Map as a scrolling 3D terrain. It renders the emergence of chaos from order using terminal characters.

## The Math

The terrain is generated using the **Logistic Map** equation, a classic example of how complex, chaotic behavior can arise from very simple non-linear dynamical equations:

$$x_{n+1} = r x_n (1 - x_n)$$

*   **$x_n$**: A number between 0 and 1, representing the population ratio at year $n$.
*   **$r$**: The **growth rate** parameter.

As you fly forward, $r$ increases.
*   **$r < 3.0$**: The population stabilizes to a single value (flat terrain).
*   **$3.0 < r < 3.57$**: The population oscillates between 2, 4, 8... values (period doubling).
*   **$r > 3.57$**: Chaos ensues. The population fluctuates unpredictably (rugged terrain).

## How It Works

*   **Generation**: For each row (slice of $r$), we iterate the logistic map equation hundreds of times. The resulting values of $x$ are bucketed into columns to form a histogram, which becomes the "height" of the terrain at that point.
*   **Rendering**: The 3D effect is achieved using the **Painter's Algorithm**. We draw the furthest rows (high Z) first, and then draw closer rows (low Z) on top of them. This ensures that "closer" mountains correctly obscure "distant" ones.

## Controls

| Key | Action |
| :--- | :--- |
| **Space** | Pause/Resume flight |
| **Up** | Increase flight speed |
| **Down** | Decrease flight speed |
| **Q / Esc** | Quit |

## Running

```bash
cargo run -p bifurcation-landscape
```
