# MemQSim Architecture Documentation

## Project Overview

**MemQSim** is a low-cost quantum computer simulator designed to maximize the number of qubits that can be simulated at the lowest possible cost. Unlike traditional quantum simulators that prioritize speed using GPUs and massive RAM, MemQSim trades performance for capacity by leveraging hierarchical disk-based storage.

### Core Philosophy

- **Maximize qubits, minimize cost**: Simulate 50+ qubits on commodity hardware
- **Disk-first architecture**: Use cheap HDD/SSD storage instead of expensive RAM
- **Minimal RAM footprint**: Constant ~512MB RAM usage regardless of qubit count
- **Hierarchical storage**: DRAM → SSD → HDD → S3 (spot instances)
- **Trade speed for scale**: Accept higher latency for orders-of-magnitude more capacity

### Target Use Case

Enable researchers, students, and institutions to simulate large quantum circuits without requiring:
- Expensive GPU clusters
- Massive RAM (hundreds of GB)
- Supercomputer access
- Proprietary software

## Architecture Phases

### Phase 1: Single Qubit Console Simulator (Current)

**Goal**: Build the foundational quantum mechanics engine with minimal complexity.

**Components**:
- Single qubit state representation (2 complex amplitudes: α|0⟩ + β|1⟩)
- Basic quantum gates: H, X, Y, Z, Rx, Ry, Rz
- Console output showing state after each operation
- No external dependencies beyond complex number support

**State Representation**:
```rust
struct SingleQubit {
    alpha: Complex64,  // Amplitude for |0⟩
    beta: Complex64,   // Amplitude for |1⟩
}
```

**Console Output Format**:
```
Initial State: |0⟩
State: 1.000|0⟩ + 0.000|1⟩

Applied H gate
State: 0.707|0⟩ + 0.707|1⟩
Probabilities: |0⟩: 50.0%, |1⟩: 50.0%

Applied X gate
State: 0.707|0⟩ - 0.707|1⟩
Probabilities: |0⟩: 50.0%, |1⟩: 50.0%
```

### Phase 2: Disk-Based Multi-Qubit Scaling

**Goal**: Scale to 35+ qubits using minimal RAM and disk storage.

#### Problem Statement

- 35 qubits = 2^35 amplitudes = 256GB state vector
- 40 qubits = 2^40 amplitudes = 16TB state vector
- 50 qubits = 2^50 amplitudes = 16PB state vector

Traditional simulators load entire state into RAM → impossible at scale.

#### Solution: Block-Based Memory-Mapped Storage

**Key Concepts**:

1. **Block Partitioning**: Split state vector into fixed-size blocks (256MB-1GB each)
2. **Memory-Mapped I/O**: Use OS-level mmap for direct disk access
3. **Minimal Working Set**: Keep only 1-2 blocks in RAM at any time
4. **Sequential Processing**: Multi-block operations become sequential passes
5. **Write-Through Policy**: Flush immediately, no caching

**Architecture**:
```
┌─────────────────────────────────────────┐
│  Quantum State (50 qubits = 16 PB)     │
└─────────────────────────────────────────┘
              ↓ millions of blocks
┌──────────────────────────────────────────┐
│     Disk Storage (HDD Array)             │
│  All blocks stored as mmap files         │
└──────────────────────────────────────────┘
              ↑↓ load on demand
    ┌─────────────────────┐
    │ RAM (512MB max)     │
    │ [Current Block]     │  ← Only 1 block!
    └─────────────────────┘
```

**Block Manager Design**:
```rust
struct MinimalBlockManager {
    current_block: Option<Block>,
    block_size: usize,        // 256MB
    total_blocks: usize,
    disk_path: PathBuf,
}

impl MinimalBlockManager {
    fn load_block(&mut self, index: usize) -> Result<&mut Block> {
        // Flush current block if exists
        if let Some(block) = &self.current_block {
            block.flush()?;
        }
        // Memory-map the requested block from disk
        self.current_block = Some(Block::from_mmap(index)?);
        Ok(self.current_block.as_mut().unwrap())
    }
}
```

**Gate Execution Strategy**:

*Single-qubit gates* (H, X, Y, Z):
- Touch only specific blocks
- Load → Apply → Flush → Next block
- Highly parallelizable (future optimization)

*Two-qubit gates* (CNOT, CZ):
- May require multiple block pairs
- Sequential passes through affected blocks
- Load pair → Apply → Flush → Next pair

*Example CNOT on qubits (5, 17)*:
```
for block_pair in affected_blocks.chunks(2) {
    load_blocks(block_pair);        // Load 2 blocks (512MB)
    apply_cnot_to_blocks();         // Compute gate
    flush_blocks();                 // Write to disk
    unload_blocks();                // Free RAM
}
```

**Performance Tradeoffs**:

| Metric | Cached Approach | Disk-Based Approach |
|--------|----------------|---------------------|
| RAM Usage | 16GB+ | 512MB constant |
| Speed | Fast | Very Slow (10-100x slower) |
| Max Qubits (1TB RAM) | ~45 qubits | **55+ qubits** |
| Cost (50 qubits) | $100K+ supercomputer | $1,500 HDDs + $36/mo spot |

### Phase 3: Browser Visualization (Future)

**Goal**: Add web API for Three.js/React frontend visualization.

**Components**:
- Axum REST API server
- WebSocket for real-time state streaming
- JSON responses with amplitude data and Bloch sphere coordinates
- 3D visualization of quantum gates in browser

**API Endpoints** (future):
```
GET  /api/state              # Current state vector
POST /api/gate               # Apply gate
POST /api/measure            # Measure qubit(s)
POST /api/reset              # Reset to |0⟩
WS   /api/stream             # Real-time updates
```

**Data Format for Frontend**:
```json
{
  "qubits": 1,
  "amplitudes": [
    {"basis": "0", "re": 0.707, "im": 0.0, "probability": 0.5},
    {"basis": "1", "re": 0.707, "im": 0.0, "probability": 0.5}
  ],
  "bloch": {"x": 1.0, "y": 0.0, "z": 0.0}
}
```

## Module Structure

### Phase 1 Structure (Current)
```
src/
├── main.rs                    # Entry point, demo circuit
├── simulator/
│   ├── mod.rs
│   ├── single_qubit.rs       # 1-qubit state + operations
│   └── gates.rs              # Gate implementations
└── utils/
    └── display.rs            # Console formatting
```

### Phase 2 Structure (Disk-Based)
```
src/
├── main.rs
├── simulator/
│   ├── quantum_state.rs      # Multi-qubit state abstraction
│   ├── gates.rs              # All gate implementations
│   └── measurement.rs        # Measurement operations
├── storage/
│   ├── block.rs              # Block structure (256MB chunk)
│   ├── block_manager.rs      # Load/evict/flush blocks
│   ├── mmap_block.rs         # Memory-mapped file wrapper
│   └── tier_manager.rs       # Hierarchical storage (future)
└── utils/
    ├── display.rs
    └── metrics.rs            # Performance monitoring
```

### Phase 3 Structure (Web API)
```
src/
├── main.rs
├── api/
│   ├── routes.rs             # REST endpoints
│   ├── websocket.rs          # Real-time streaming
│   └── models.rs             # JSON schemas
├── simulator/                # (same as Phase 2)
├── storage/                  # (same as Phase 2)
└── utils/                    # (same as Phase 2)
```

## Implementation Roadmap

### Phase 1: Foundation (Current Sprint)
- [x] Project setup with Cargo
- [ ] Single qubit state representation
- [ ] Basic gates: H, X, Y, Z
- [ ] Parameterized gates: Rx, Ry, Rz
- [ ] Console output formatting
- [ ] Demo: Bell state preparation

### Phase 2: Disk Scaling (Next)
- [ ] Block data structure (256MB chunks)
- [ ] Memory-mapped file I/O
- [ ] Minimal block manager (1 block in RAM)
- [ ] Multi-qubit state (2-10 qubits first)
- [ ] Block-aware single-qubit gates
- [ ] Sequential two-qubit gates (CNOT)
- [ ] Performance benchmarks (I/O throughput)
- [ ] Scale test: 35 qubits on 500GB disk

### Phase 3: Web API (Later)
- [ ] Axum web server setup
- [ ] REST API endpoints
- [ ] WebSocket real-time streaming
- [ ] Bloch sphere coordinate conversion
- [ ] CORS configuration
- [ ] API documentation
- [ ] Frontend integration guide

### Phase 4: Advanced Features (Future)
- [ ] Checkpoint/restore system
- [ ] S3 backend for cloud storage
- [ ] Spot instance orchestration
- [ ] Circuit optimization (gate reordering)
- [ ] Sparse state representation
- [ ] Compression for cold blocks
- [ ] Distributed simulation across nodes

## Cost Analysis

### Traditional Approach (RAM-based)
- 50 qubits = 16PB state
- Requires supercomputer with petabyte RAM
- Cost: $100,000+ hardware + $1,000s/month operational

### MemQSim Approach (Disk-based)
- 50 qubits = 16PB state on disk
- Hardware: 100x 10TB HDDs @ $15/TB = $15,000
- Spot instance: $0.05/hour = $36/month
- **Total: $15,036 one-time + $36/month**
- **Cost savings: 85-90% compared to traditional**

### Scaling Examples
| Qubits | State Size | RAM Needed (cached) | Disk Needed (MemQSim) | Monthly Cost |
|--------|------------|---------------------|-----------------------|--------------|
| 30 | 16 GB | 16 GB | 20 GB | $5 |
| 35 | 512 GB | 512 GB | 600 GB | $20 |
| 40 | 16 TB | 16 TB | 20 TB | $150 |
| 45 | 512 TB | 512 TB | 600 TB | $2,000 |
| 50 | 16 PB | 16 PB | 20 PB | $5,000 |

*Note: Costs assume cloud spot instances + HDD storage. Traditional supercomputer costs are 10-100x higher.*

## Dependencies

### Phase 1 (Minimal)
```toml
[dependencies]
num-complex = "0.4"  # Complex number arithmetic
```

### Phase 2 (Disk I/O)
```toml
[dependencies]
num-complex = "0.4"
memmap2 = "0.9"      # Memory-mapped files
zstd = "0.13"        # Compression (optional)
```

### Phase 3 (Web API)
```toml
[dependencies]
num-complex = "0.4"
memmap2 = "0.9"
axum = "0.7"         # Web framework
tokio = "1"          # Async runtime
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower-http = "0.5"   # CORS, middleware
```

## Key Metrics to Track

### Phase 1
- Gate correctness (amplitude precision)
- Console output clarity

### Phase 2
- RAM usage (must stay ≤ 512MB)
- Disk I/O throughput (MB/s read/write)
- Gate execution latency
- Maximum qubits simulated
- Cost per qubit

### Phase 3
- API response time
- WebSocket latency
- Concurrent client support
- Frontend render performance

## Success Criteria

### Phase 1: ✅ Foundation Working
- Correctly simulate single qubit gates
- Clear console visualization
- Accurate quantum mechanics (amplitude normalization)

### Phase 2: ✅ Disk Scaling Validated
- Simulate 35+ qubits on commodity hardware
- RAM usage stays constant (≤ 512MB)
- Total cost < $100/month for 40 qubits
- 10x cost reduction vs. traditional simulators

### Phase 3: ✅ Production Ready
- Browser-based visualization working
- Real-time state streaming < 100ms latency
- API handles concurrent users
- Full documentation and examples

## References

- Nielsen & Chuang, "Quantum Computation and Quantum Information"
- Memory-mapped files: https://docs.rs/memmap2/
- Quantum gate matrices: https://en.wikipedia.org/wiki/Quantum_logic_gate
- Bloch sphere: https://en.wikipedia.org/wiki/Bloch_sphere

---

**Last Updated**: 2025-10-16
**Status**: Phase 1 in progress
