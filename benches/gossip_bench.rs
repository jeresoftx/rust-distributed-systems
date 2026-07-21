use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_distributed_systems::gossip::{Fanout, GossipCluster, GossipFact, GossipNodeId};

const ROUNDS: usize = 10_000;

fn main() {
    let results = [
        benchmark_single_round_spread(),
        benchmark_unavailable_skip(),
        benchmark_eventual_convergence(),
        benchmark_recovered_catch_up(),
    ];

    println!("\nProtocolo gossip benchmark educativo");
    println!("Modelo: propagación epidémica, fanout acotado y fallas temporales");
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

fn benchmark_single_round_spread() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut cluster = sample_cluster();
        let fact = GossipFact(1);
        cluster.insert_fact(GossipNodeId(1), fact);

        let report = cluster.run_round(Fanout(3));

        assert_eq!(report.messages_sent, 3);
        assert_eq!(cluster.coverage(fact), 4);
        black_box(cluster);
    }

    BenchmarkResult {
        name: "propagación de una ronda",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_unavailable_skip() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut cluster = sample_cluster();
        let fact = GossipFact(2);
        cluster.insert_fact(GossipNodeId(1), fact);
        cluster.set_available(GossipNodeId(4), false);

        let report = cluster.run_round(Fanout(3));

        assert_eq!(report.messages_sent, 2);
        assert_eq!(cluster.coverage(fact), 3);
        assert!(!cluster.knows(GossipNodeId(4), fact));
        black_box(report);
    }

    BenchmarkResult {
        name: "omitir nodo no disponible",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_eventual_convergence() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut cluster = sample_cluster();
        let fact = GossipFact(3);
        cluster.insert_fact(GossipNodeId(1), fact);

        for _ in 0..4 {
            cluster.run_round(Fanout(1));
        }

        assert_eq!(cluster.coverage(fact), 4);
        assert!(cluster.available_nodes_converged());
        black_box(cluster);
    }

    BenchmarkResult {
        name: "convergencia eventual",
        operations: ROUNDS * 4,
        elapsed: start.elapsed(),
    }
}

fn benchmark_recovered_catch_up() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut cluster = sample_cluster();
        let fact = GossipFact(4);
        cluster.insert_fact(GossipNodeId(1), fact);
        cluster.set_available(GossipNodeId(4), false);
        cluster.run_round(Fanout(3));

        cluster.set_available(GossipNodeId(4), true);
        cluster.run_round(Fanout(3));

        assert_eq!(cluster.coverage(fact), 4);
        assert!(cluster.available_nodes_converged());
        black_box(cluster);
    }

    BenchmarkResult {
        name: "recuperación posterior",
        operations: ROUNDS * 2,
        elapsed: start.elapsed(),
    }
}

fn sample_cluster() -> GossipCluster {
    GossipCluster::from_nodes([
        GossipNodeId(1),
        GossipNodeId(2),
        GossipNodeId(3),
        GossipNodeId(4),
    ])
}
