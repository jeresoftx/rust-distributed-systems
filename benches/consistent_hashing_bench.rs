use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_distributed_systems::consistent_hashing::{
    ConsistentHashRing, HashSlot, Key, NodeId, RingNode,
};

const ROUNDS: usize = 10_000;
const KEYS: [Key; 8] = [
    Key(5),
    Key(20),
    Key(39),
    Key(40),
    Key(59),
    Key(79),
    Key(81),
    Key(99),
];

fn main() {
    let results = [
        benchmark_owner_lookup(),
        benchmark_node_insert(),
        benchmark_node_remove(),
        benchmark_movement_comparison(),
    ];

    println!("\nConsistent hashing benchmark educativo");
    println!("Modelo: anillo ordenado, sucesor, wrap-around y movimiento acotado");
    println!("| Operación | Ops | Total | ns/op |");
    println!("|-----------|-----|-------|-------|");

    for result in results {
        println!(
            "| {} | {} | {:?} | {} |",
            result.name,
            result.operations,
            result.elapsed,
            result.nanoseconds_per_operation()
        );
    }
}

struct BenchmarkResult {
    name: &'static str,
    operations: usize,
    elapsed: Duration,
}

impl BenchmarkResult {
    fn nanoseconds_per_operation(&self) -> u128 {
        self.elapsed.as_nanos().div_ceil(self.operations as u128)
    }
}

fn benchmark_owner_lookup() -> BenchmarkResult {
    let ring = sample_ring();
    let start = Instant::now();

    for _ in 0..ROUNDS {
        for key in KEYS {
            let owner = ring.owner(black_box(key));
            assert!(owner.is_some());
            black_box(owner);
        }
    }

    BenchmarkResult {
        name: "consulta de dueño",
        operations: ROUNDS * KEYS.len(),
        elapsed: start.elapsed(),
    }
}

fn benchmark_node_insert() -> BenchmarkResult {
    let base = ConsistentHashRing::from_nodes([
        RingNode::new(NodeId(1), HashSlot(10)),
        RingNode::new(NodeId(3), HashSlot(80)),
    ]);
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut ring = base.clone();
        ring.insert_node(RingNode::new(NodeId(2), HashSlot(40)));

        assert_eq!(ring.owner(Key(39)), Some(NodeId(2)));
        black_box(ring);
    }

    BenchmarkResult {
        name: "agregar nodo",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_node_remove() -> BenchmarkResult {
    let base = sample_ring();
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut ring = base.clone();
        let removed = ring.remove_node(NodeId(2));

        assert_eq!(removed, Some(RingNode::new(NodeId(2), HashSlot(40))));
        assert_eq!(ring.owner(Key(39)), Some(NodeId(3)));
        black_box(ring);
    }

    BenchmarkResult {
        name: "retirar nodo",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_movement_comparison() -> BenchmarkResult {
    let before = ConsistentHashRing::from_nodes([
        RingNode::new(NodeId(1), HashSlot(10)),
        RingNode::new(NodeId(3), HashSlot(80)),
    ]);
    let mut after = before.clone();
    after.insert_node(RingNode::new(NodeId(2), HashSlot(40)));

    let start = Instant::now();

    for _ in 0..ROUNDS {
        let movements = ConsistentHashRing::movements_between(&before, &after, &KEYS);

        assert_eq!(movements.len(), 3);
        black_box(movements);
    }

    BenchmarkResult {
        name: "comparar movimientos",
        operations: ROUNDS * KEYS.len(),
        elapsed: start.elapsed(),
    }
}

fn sample_ring() -> ConsistentHashRing {
    ConsistentHashRing::from_nodes([
        RingNode::new(NodeId(1), HashSlot(10)),
        RingNode::new(NodeId(2), HashSlot(40)),
        RingNode::new(NodeId(3), HashSlot(80)),
    ])
}
