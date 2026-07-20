use std::hint::black_box;
use std::time::{Duration, Instant};

use rust_distributed_systems::lamport_clock::{EventId, LamportClock, NodeId};

const ROUNDS: usize = 10_000;

fn main() {
    let results = [
        benchmark_local_event(),
        benchmark_send_message(),
        benchmark_receive_message(),
        benchmark_trace_ordering(),
    ];

    println!("\nLamport clocks benchmark educativo");
    println!("Modelo: contador escalar, mensajes con timestamp y orden por EventId");
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

fn benchmark_local_event() -> BenchmarkResult {
    let start = Instant::now();

    for round_id in 0..ROUNDS {
        let mut clock = LamportClock::new(black_box(NodeId((round_id % 16) as u64)));
        let event = clock.local_event();

        black_box(event);
        black_box(clock);
    }

    BenchmarkResult {
        name: "evento local",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_send_message() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut clock = LamportClock::new(black_box(NodeId(1)));
        clock.local_event();
        let message = clock.send();

        black_box(message);
        black_box(clock);
    }

    BenchmarkResult {
        name: "envío con timestamp",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_receive_message() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut sender = LamportClock::new(black_box(NodeId(1)));
        sender.local_event();
        let message = sender.send();

        let mut receiver = LamportClock::new(black_box(NodeId(2)));
        let event = receiver.receive(message);

        black_box(event);
        black_box(receiver);
    }

    BenchmarkResult {
        name: "recepción max + 1",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn benchmark_trace_ordering() -> BenchmarkResult {
    let start = Instant::now();

    for _ in 0..ROUNDS {
        let mut trace = trace_events();
        trace.sort();

        black_box(trace);
    }

    BenchmarkResult {
        name: "ordenamiento de traza",
        operations: ROUNDS,
        elapsed: start.elapsed(),
    }
}

fn trace_events() -> Vec<EventId> {
    let mut api = LamportClock::new(black_box(NodeId(1)));
    let mut worker = LamportClock::new(black_box(NodeId(2)));
    let mut storage = LamportClock::new(black_box(NodeId(3)));

    let api_started = api.local_event();
    let worker_started = worker.local_event();
    let request = api.send();
    let worker_received = worker.receive(request);
    let write = worker.send();
    let storage_received = storage.receive(write);

    vec![
        storage_received,
        worker_started,
        api_started,
        worker_received,
    ]
}
