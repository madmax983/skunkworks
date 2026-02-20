struct Uniforms {
    width: u32,
    height: u32,
    depth: u32,
    _pad1: u32,

    time: f32,
    seed: f32,
    temperature: f32,
    field: f32,

    lattice_type: u32, // 0=SC, 1=BCC, 2=FCC
    brush_radius: f32,
    brush_strength: f32,
    _pad2: u32,

    brush_pos: vec4<f32>, // using vec4 for alignment (xyz, w=ignored)
}

struct Cell {
    spin: f32,
    state: f32, // 0.0 = Void, 1.0 = Matter
}

@group(0) @binding(0) var<storage, read> cells_in: array<Cell>;
@group(0) @binding(1) var<storage, read_write> cells_out: array<Cell>;
@group(1) @binding(0) var<uniform> uniforms: Uniforms;

// PCG Hash for RNG
fn hash(v: u32) -> u32 {
    var state = v * 747796405u + 2891336453u;
    var word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

fn rand_f32(seed: u32) -> f32 {
    return f32(hash(seed)) / 4294967295.0;
}

fn get_index(x: u32, y: u32, z: u32) -> u32 {
    return x + y * uniforms.width + z * uniforms.width * uniforms.height;
}

fn get_spin(x: u32, y: u32, z: u32) -> f32 {
    // Wrap around (Periodic Boundary Conditions)
    let wx = x % uniforms.width;
    let wy = y % uniforms.height;
    let wz = z % uniforms.depth;
    let idx = get_index(wx, wy, wz);
    // If neighbor is void, it contributes 0 to the field
    if (cells_in[idx].state == 0.0) {
        return 0.0;
    }
    return cells_in[idx].spin;
}

@compute @workgroup_size(64)
fn compute_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    let total_cells = uniforms.width * uniforms.height * uniforms.depth;
    if (index >= total_cells) {
        return;
    }

    // Preserve state
    cells_out[index].state = cells_in[index].state;

    // If void, do nothing
    if (cells_in[index].state == 0.0) {
        cells_out[index].spin = 0.0;
        return;
    }

    // Decode Index to (x, y, z)
    let z = index / (uniforms.width * uniforms.height);
    let rem_z = index % (uniforms.width * uniforms.height);
    let y = rem_z / uniforms.width;
    let x = rem_z % uniforms.width;

    let my_spin = cells_in[index].spin;

    // Calculate Neighbor Sum
    var neighbor_sum: f32 = 0.0;

    // Lattice Neighbors
    if (uniforms.lattice_type == 0u) { // Simple Cubic (6)
        neighbor_sum += get_spin(x + 1u, y, z);
        neighbor_sum += get_spin(x - 1u, y, z);
        neighbor_sum += get_spin(x, y + 1u, z);
        neighbor_sum += get_spin(x, y - 1u, z);
        neighbor_sum += get_spin(x, y, z + 1u);
        neighbor_sum += get_spin(x, y, z - 1u);
    } else if (uniforms.lattice_type == 1u) { // BCC (8)
        // Diagonals
        neighbor_sum += get_spin(x + 1u, y + 1u, z + 1u);
        neighbor_sum += get_spin(x - 1u, y + 1u, z + 1u);
        neighbor_sum += get_spin(x + 1u, y - 1u, z + 1u);
        neighbor_sum += get_spin(x - 1u, y - 1u, z + 1u);
        neighbor_sum += get_spin(x + 1u, y + 1u, z - 1u);
        neighbor_sum += get_spin(x - 1u, y + 1u, z - 1u);
        neighbor_sum += get_spin(x + 1u, y - 1u, z - 1u);
        neighbor_sum += get_spin(x - 1u, y - 1u, z - 1u);
    } else if (uniforms.lattice_type == 2u) { // FCC (12)
        // Face diagonals
        neighbor_sum += get_spin(x + 1u, y + 1u, z);
        neighbor_sum += get_spin(x - 1u, y + 1u, z);
        neighbor_sum += get_spin(x + 1u, y - 1u, z);
        neighbor_sum += get_spin(x - 1u, y - 1u, z);

        neighbor_sum += get_spin(x + 1u, y, z + 1u);
        neighbor_sum += get_spin(x - 1u, y, z + 1u);
        neighbor_sum += get_spin(x + 1u, y, z - 1u);
        neighbor_sum += get_spin(x - 1u, y, z - 1u);

        neighbor_sum += get_spin(x, y + 1u, z + 1u);
        neighbor_sum += get_spin(x, y - 1u, z + 1u);
        neighbor_sum += get_spin(x, y + 1u, z - 1u);
        neighbor_sum += get_spin(x, y - 1u, z - 1u);
    }

    // Calculate effective field (H + Brush)
    var h = uniforms.field;

    // Brush
    let pos = vec3<f32>(f32(x), f32(y), f32(z));
    let brush_dist = distance(pos, uniforms.brush_pos.xyz);
    if (brush_dist < uniforms.brush_radius) {
        h += uniforms.brush_strength;
    }

    // Metropolis Update
    // dE = 2 * S_i * (J * sum(S_j) + H)
    // Assume J = 1.0 (Ferromagnetic)
    let J = 1.0;
    let dE = 2.0 * my_spin * (J * neighbor_sum + h);

    var new_spin = my_spin;

    // RNG
    let rng_seed = index ^ u32(uniforms.seed * 1000.0) ^ u32(uniforms.time * 60.0);
    let r = rand_f32(rng_seed);

    if (dE < 0.0) {
        new_spin = -my_spin; // Flip to lower energy
    } else {
        if (r < exp(-dE / uniforms.temperature)) {
            new_spin = -my_spin; // Thermal flip
        }
    }

    cells_out[index].spin = new_spin;
}


// --- VERTEX SHADER ---

struct CameraUniform {
    view_proj: mat4x4<f32>,
    position: vec4<f32>,
}
@group(0) @binding(0) var<uniform> camera: CameraUniform;
@group(1) @binding(0) var<storage, read> render_cells: array<Cell>;
@group(2) @binding(0) var<uniform> render_params: Uniforms; // Reusing Uniforms struct

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct InstanceInput {
    @builtin(instance_index) instance_idx: u32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) world_pos: vec3<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let index = instance.instance_idx;
    var out: VertexOutput;

    // Check if void
    if (render_cells[index].state == 0.0) {
        // Degenerate triangle outside clip space
        out.clip_position = vec4<f32>(0.0, 0.0, 0.0, 0.0);
        return out;
    }

    // Decode Index
    let w = render_params.width;
    let h = render_params.height;
    let z_idx = index / (w * h);
    let rem = index % (w * h);
    let y_idx = rem / w;
    let x_idx = rem % w;

    var pos = vec3<f32>(f32(x_idx), f32(y_idx), f32(z_idx));

    // Center the grid
    let grid_center = vec3<f32>(f32(w)*0.5, f32(h)*0.5, f32(render_params.depth)*0.5);
    pos = pos - grid_center;

    // Apply scaling
    let scale = 0.8;
    let world_pos = pos + model.position * scale;

    out.clip_position = camera.view_proj * vec4<f32>(world_pos, 1.0);
    out.normal = model.normal;
    out.world_pos = world_pos;

    let spin = render_cells[index].spin;
    if (spin > 0.0) {
        out.color = vec3<f32>(0.2, 0.4, 0.9); // Blue Up
    } else {
        out.color = vec3<f32>(0.9, 0.2, 0.2); // Red Down
    }

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple lighting
    let light_dir = normalize(vec3<f32>(1.0, 2.0, 3.0));
    let diffuse = max(dot(in.normal, light_dir), 0.2);

    // Specular
    let view_dir = normalize(camera.position.xyz - in.world_pos);
    let half_dir = normalize(light_dir + view_dir);
    let spec = pow(max(dot(in.normal, half_dir), 0.0), 32.0);

    let color = in.color * diffuse + vec3<f32>(0.5) * spec * 0.5;

    return vec4<f32>(color, 1.0);
}
