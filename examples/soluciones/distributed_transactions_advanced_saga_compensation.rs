use rust_distributed_systems::distributed_transactions::{
    Saga, SagaOutcome, SagaStep, SagaStepId, TransactionId,
};

fn main() {
    let mut saga = Saga::from_steps([
        SagaStep::new(SagaStepId("reservar-asiento"), true),
        SagaStep::new(SagaStepId("cobrar-tarjeta"), true),
        SagaStep::new(SagaStepId("emitir-boleto"), false),
    ]);
    let transaction = TransactionId(30);

    let outcome = saga.run(transaction);

    assert_eq!(
        outcome,
        SagaOutcome::Compensated {
            transaction,
            failed_step: SagaStepId("emitir-boleto"),
            applied: vec![SagaStepId("reservar-asiento"), SagaStepId("cobrar-tarjeta"),],
            compensated: vec![SagaStepId("cobrar-tarjeta"), SagaStepId("reservar-asiento"),],
        }
    );

    println!(
        "La saga {:?} compensó {:?}.",
        transaction,
        saga.compensated_steps(transaction)
    );
}
