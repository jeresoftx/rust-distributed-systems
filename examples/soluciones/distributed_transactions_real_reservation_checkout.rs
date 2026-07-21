use rust_distributed_systems::distributed_transactions::{
    Saga, SagaOutcome, SagaStep, SagaStepId, TransactionId,
};

fn main() {
    let checkout_id = TransactionId(74);
    let mut checkout = Saga::from_steps([
        SagaStep::new(SagaStepId("retener-inventario"), true),
        SagaStep::new(SagaStepId("autorizar-pago"), true),
        SagaStep::new(SagaStepId("emitir-reserva"), false),
    ]);

    let first_attempt = checkout.run(checkout_id);
    let retry = checkout.run(checkout_id);

    assert_eq!(first_attempt, retry);
    assert!(matches!(
        first_attempt,
        SagaOutcome::Compensated {
            failed_step: SagaStepId("emitir-reserva"),
            ..
        }
    ));
    assert_eq!(
        checkout.compensated_steps(checkout_id),
        [
            SagaStepId("autorizar-pago"),
            SagaStepId("retener-inventario"),
        ]
    );

    println!(
        "Checkout {:?} quedó compensado sin duplicar efectos en el reintento.",
        checkout_id
    );
}
