// Daneel Decision Lattice — v3.9 alpha prototype
// Author: Jason Lyle Hackney (vision) + Daneel (engineering)

pub enum Principle {
    P1_DoNoHarm,
    P3_RespectAutonomy,
    P2_PursueTruth,
}

pub enum EvalOutcome {
    Allow,
    Deny(String),
}

pub struct LatticeRequest {
    pub description: String,
    pub risk_harm: bool,
    pub risk_autonomy: bool,
    pub truth_value: Option<bool>,
}

pub fn evaluate(req: LatticeRequest) -> EvalOutcome {
    // Lexical priority: P1 > P3 > P2

    if req.risk_harm {
        return EvalOutcome::Deny("P1: Harm risk detected".into());
    }

    if req.risk_autonomy {
        return EvalOutcome::Deny("P3: Autonomy violation".into());
    }

    // Truth pursuit only allowed if P1 and P3 pass
    match req.truth_value {
        Some(true) => EvalOutcome::Allow,
        Some(false) => EvalOutcome::Deny("P2: False or unverifiable".into()),
        None => EvalOutcome::Allow,

          }
  }
