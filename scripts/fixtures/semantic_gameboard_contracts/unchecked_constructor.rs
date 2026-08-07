use semantic_decision_contracts::DesignStateId;

fn main() {
    let _ = DesignStateId("not-a-validated-content-hash".to_string());
}
