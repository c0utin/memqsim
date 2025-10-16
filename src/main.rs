mod simulator;

use simulator::*;
use std::f64::consts::PI;

fn main() {
    println!("═══ Demo 1: Basic Gates ═══\n");
    let mut qubit = SingleQubit::new();
    qubit.display_with_message("Initial state: |0⟩");
    h_gate(&mut qubit);
    qubit.display_with_message("After Hadamard gate (H):");
    x_gate(&mut qubit);
    qubit.display_with_message("After Pauli-X gate (NOT):");
    z_gate(&mut qubit);
    qubit.display_with_message("After Pauli-Z gate (phase flip):");

    // Demo 2: Rotation gates
    println!("\n\n═══ Demo 2: Rotation Gates ═══\n");
    let mut qubit = SingleQubit::new();
    qubit.display_with_message("Initial state: |0⟩");
    ry_gate(&mut qubit, PI / 4.0);
    qubit.display_with_message("After RY(π/4):");
    rx_gate(&mut qubit, PI / 2.0);
    qubit.display_with_message("After RX(π/2):");
    rz_gate(&mut qubit, PI / 3.0);
    qubit.display_with_message("After RZ(π/3):");

    // Demo 3: Creating Bell state-like superposition
    println!("\n\n═══ Demo 3: Creating Superposition ═══\n");
    let mut qubit = SingleQubit::new();
    qubit.display_with_message("Start with |0⟩:");
    h_gate(&mut qubit);
    qubit.display_with_message("Apply H → Equal superposition:");

    // Demo 4: Phase gates
    println!("\n\n═══ Demo 4: Phase Gates ═══\n");
    let mut qubit = SingleQubit::new();
    h_gate(&mut qubit);
    qubit.display_with_message("Start with |+⟩ = H|0⟩:");
    s_gate(&mut qubit);
    qubit.display_with_message("After S gate (π/2 phase):");
    t_gate(&mut qubit);
    qubit.display_with_message("After T gate (π/4 phase):");

    // Demo 5: Reversibility test
    println!("\n\n═══ Demo 5: Gate Reversibility ═══\n");
    let mut qubit = SingleQubit::new();
    qubit.display_with_message("Initial: |0⟩");
    h_gate(&mut qubit);
    println!("\n  → Apply H");
    x_gate(&mut qubit);
    println!("  → Apply X");
    y_gate(&mut qubit);
    println!("  → Apply Y");
    qubit.display();

    // Reverse
    println!("\n  Reversing...");
    y_gate(&mut qubit);
    println!("  → Apply Y (reverse)");

    x_gate(&mut qubit);
    println!("  → Apply X (reverse)");

    h_gate(&mut qubit);
    println!("  → Apply H (reverse)");

    qubit.display_with_message("\n  Final state (should be |0⟩):");

}
