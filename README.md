# Rust Fluid
2d *fluid simulation* made with rust and ggez


# Run
```bash
git clone git@github.com:gufoe/rust-fluid.git
cd rust-fluid
cargo run --release
```

# Info
The simulator is implemented using a particle swarm.
Each particle can see all the particles in a certain radius (`Agent.view_range`)
and changes its position according to the neighbors distance and velocity.

I'm using `rayon` to distribute the workload across multiple threads and `ggez`
for the interface and input handling.

The code is optimized enough to be able to use up to 100.000 particles in
real-time (although depending on your hardware it may lag).

# Customize
You can adjust the window size, brush size and particle count in `main.rs`,
while the behavior of the agents can be controlled using the variables in the
`Agent::new` method, in `agent.rs`.
