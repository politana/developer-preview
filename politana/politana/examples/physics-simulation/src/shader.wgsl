struct Particle {
    pos: vec2<f32>,
    vel: vec2<f32>,
}

struct Attractor {
    pos: vec2<f32>,
    strength: f32,
    _padding: f32, // Match the 16-byte alignment from Rust
}

@group(0) @binding(0) var<storage, read_write> particles: array<Particle>;
@group(0) @binding(1) var<storage, read> attractors: array<Attractor>;

// --- COMPUTE STAGE ---

const DT: f32 = 0.004;      // Time step
const FRICTION: f32 = 0.9999; // Damping factor to keep system stable
const EPS: f32 = 0.05;      // Softening factor for gravity

@compute @workgroup_size(1, 1, 1)
fn step_physics(@builtin(global_invocation_id) id: vec3<u32>) {
    let idx = id.y * 1000u + id.x;

    var p = particles[idx];
    var total_force = vec2<f32>(0.0, 0.0);

    // Calculate attraction from each attractor
    for (var i = 0u; i < arrayLength(&attractors); i++) {
        let a = attractors[i];
        let diff = a.pos - p.pos;
        let dist_sq = dot(diff, diff);

        // Newton's Law: F = G * (m1 * m2) / r^2
        let force_mag = a.strength / (dist_sq + EPS);
        total_force += normalize(diff) * force_mag;
    }

    // Update Velocity (Euler Integration)
    p.vel = (p.vel + total_force * DT) * FRICTION;

    // Update Position
    p.pos += p.vel * DT;

    // Optional: Boundary Check (Wrap around)
    if (p.pos.x > 1.0) { p.pos.x = -1.0; }
    if (p.pos.x < -1.0) { p.pos.x = 1.0; }
    if (p.pos.y > 1.0) { p.pos.y = -1.0; }
    if (p.pos.y < -1.0) { p.pos.y = 1.0; }

    particles[idx] = p;
}

// --- RENDER STAGE ---

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(
    @location(0) pos: vec2<f32>,
    @location(1) vel: vec2<f32>
) -> VertexOutput {
    var out: VertexOutput;

    // Pass position directly to clip space
    out.clip_pos = vec4<f32>(pos, 0.0, 1.0);

    // Colorize based on speed (magnitude of velocity)
    let speed = length(vel);
    let r = 0.2 + speed * 0.5;
    let g = 0.4 + speed * 0.2;
    let b = 0.8;

    out.color = vec4<f32>(vec3<f32>(r, g, b) * 0.5 + 0.5, 1.0);
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
