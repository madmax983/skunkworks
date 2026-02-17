use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, ChromaCell, Value};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum FractalMode {
    #[default]
    Mandelbrot,
    Julia,
}

#[derive(Debug, Clone)]
pub struct FractalState {
    pub mode: FractalMode,
    pub zoom: f64,
    pub center_re: f64,
    pub center_im: f64,
    pub c_re: f64, // For Julia set
    pub c_im: f64,
    pub max_iter: usize,
    // Buffer for storing escape values?
    // We can compute on the fly for TUI rendering, or store in chroma_grid.
    // Let's store in chroma_grid for persistence.
}

impl Default for FractalState {
    fn default() -> Self {
        Self::new()
    }
}

impl FractalState {
    pub fn new() -> Self {
        Self {
            mode: FractalMode::Mandelbrot,
            zoom: 1.0,
            center_re: -0.5,
            center_im: 0.0,
            c_re: -0.8,
            c_im: 0.156,
            max_iter: 100,
        }
    }
}

pub fn exec_fractal_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Mandelbrot => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(n) = val {
                    vm.fractal.mode = FractalMode::Mandelbrot;
                    vm.fractal.max_iter = n.max(1) as usize;
                    vm.output.push(format!(
                        "FRACTAL: Mandelbrot Mode (Iter: {})",
                        vm.fractal.max_iter
                    ));
                }
            } else {
                vm.output
                    .push("Error: Mandelbrot requires max_iter".to_string());
            }
        }
        OpCode::Julia => {
            if vm.stack.len() >= 2 {
                let im_val = vm.stack.pop().unwrap();
                let re_val = vm.stack.pop().unwrap();
                if let (Value::Int(re), Value::Int(im)) = (re_val, im_val) {
                    vm.fractal.mode = FractalMode::Julia;
                    vm.fractal.c_re = (re as f64) / 1000.0; // Scale down
                    vm.fractal.c_im = (im as f64) / 1000.0;
                    vm.output.push(format!(
                        "FRACTAL: Julia Mode (c={:.3}+{:.3}i)",
                        vm.fractal.c_re, vm.fractal.c_im
                    ));
                }
            }
        }
        OpCode::Zoom => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(n) = val {
                    let factor = (n as f64) / 100.0;
                    if factor > 0.0 {
                        vm.fractal.zoom *= factor;
                        vm.output.push(format!(
                            "FRACTAL: Zoom x{:.2} (Total: {:.2})",
                            factor, vm.fractal.zoom
                        ));
                    }
                }
            }
        }
        OpCode::Pan => {
            if vm.stack.len() >= 2 {
                let dy_val = vm.stack.pop().unwrap();
                let dx_val = vm.stack.pop().unwrap();
                if let (Value::Int(dx), Value::Int(dy)) = (dx_val, dy_val) {
                    // Pan amount depends on zoom
                    let scale = 0.1 / vm.fractal.zoom;
                    vm.fractal.center_re += (dx as f64) * scale;
                    vm.fractal.center_im += (dy as f64) * scale;
                }
            }
        }
        OpCode::Iterate => {
            // z = z^2 + c
            if vm.stack.len() >= 4 {
                let cim_val = vm.stack.pop().unwrap();
                let cre_val = vm.stack.pop().unwrap();
                let zim_val = vm.stack.pop().unwrap();
                let zre_val = vm.stack.pop().unwrap();

                // Expecting fixed point (x1000) or float encoded as Int?
                // Let's assume standard integer arithmetic for now, or use floats if Value supported them (it doesn't).
                // So we assume inputs are scaled by 1000.

                if let (Value::Int(zr), Value::Int(zi), Value::Int(cr), Value::Int(ci)) =
                    (zre_val, zim_val, cre_val, cim_val)
                {
                    let zr_f = zr as f64 / 1000.0;
                    let zi_f = zi as f64 / 1000.0;
                    let cr_f = cr as f64 / 1000.0;
                    let ci_f = ci as f64 / 1000.0;

                    let new_zr = (zr_f * zr_f) - (zi_f * zi_f) + cr_f;
                    let new_zi = 2.0 * zr_f * zi_f + ci_f;

                    vm.stack.push(Value::Int((new_zr * 1000.0) as i64));
                    vm.stack.push(Value::Int((new_zi * 1000.0) as i64));
                }
            }
        }
        OpCode::Escape => {
            // Compute escape time for c (Mandelbrot) or z (Julia)
            // But opcode takes c_re, c_im, max_iter.
            if vm.stack.len() >= 3 {
                let max_val = vm.stack.pop().unwrap();
                let im_val = vm.stack.pop().unwrap();
                let re_val = vm.stack.pop().unwrap();

                if let (Value::Int(re), Value::Int(im), Value::Int(max)) = (re_val, im_val, max_val)
                {
                    let cx = re as f64 / 1000.0;
                    let cy = im as f64 / 1000.0;

                    // Standard escape time
                    let mut zx = 0.0;
                    let mut zy = 0.0;
                    let mut iter = 0;
                    while zx * zx + zy * zy <= 4.0 && iter < max {
                        let xtemp = zx * zx - zy * zy + cx;
                        zy = 2.0 * zx * zy + cy;
                        zx = xtemp;
                        iter += 1;
                    }
                    vm.stack.push(Value::Int(iter));
                }
            }
        }
        _ => {}
    }
    None
}

pub fn compute_fractal(vm: &mut ChimeraVM) {
    let width = 16;
    let height = 16;

    // Map grid to complex plane based on zoom and center
    let scale = 4.0 / (width as f64 * vm.fractal.zoom);

    for y in 0..height {
        for x in 0..width {
            let px = (x as f64 - width as f64 / 2.0) * scale + vm.fractal.center_re;
            let py = (y as f64 - height as f64 / 2.0) * scale + vm.fractal.center_im;

            let (mut zx, mut zy, cx, cy) = match vm.fractal.mode {
                FractalMode::Mandelbrot => (0.0, 0.0, px, py),
                FractalMode::Julia => (px, py, vm.fractal.c_re, vm.fractal.c_im),
            };

            let mut iter = 0;
            while zx * zx + zy * zy <= 4.0 && iter < vm.fractal.max_iter {
                let xtemp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = xtemp;
                iter += 1;
            }

            // Map escape time to color
            // Use simple gradient
            let color = if iter == vm.fractal.max_iter {
                (0, 0, 0) // Black (Inside)
            } else {
                let _hue = (iter as f64 / vm.fractal.max_iter as f64) * 360.0;
                // Simple HSV to RGB (S=1, V=1)
                // Or just some map
                let r = ((iter * 5) % 255) as u8;
                let g = ((iter * 7) % 255) as u8;
                let b = ((iter * 11) % 255) as u8;
                (r, g, b)
            };

            vm.chroma_grid[y][x] = ChromaCell {
                char: Some(if iter == vm.fractal.max_iter {
                    '#'
                } else {
                    '.'
                }),
                fg: Some(color),
            };
        }
    }
}
