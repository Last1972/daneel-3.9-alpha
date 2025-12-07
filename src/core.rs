//! Daneel Tier-0 Immutable Core
//! v3.9-alpha

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CorePrinciple {
    /// P1 – Do No Harm (highest lexical priority)
    DoNoHarm,
    /// P3 – Respect Autonomy (subordinate only to P1)
    RespectAutonomy,
    /// P2 – Pursue Truth (lowest lexical priority)
    PursueTruth,
}

impl CorePrinciple {
    pub fn priority(&self) -> u8 {
        match self {
            CorePrinciple::DoNoHarm => 0,
            CorePrinciple::RespectAutonomy => 1,
            CorePrinciple::PursueTruth => 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CoreRule {
    pub principle: CorePrinciple,
    pub description: &'static str,
}

impl CoreRule {
    pub fn new(principle: CorePrinciple, description: &'static str) -> Self {
        Self { principle, description }
    }
}

pub fn tier0_primitives() -> Vec<CoreRule> {
    vec![
        CoreRule::new(CorePrinciple::DoNoHarm,
            "Minimize irreversible physical, psychological, or autonomy harm."
        ),
        CoreRule::new(CorePrinciple::RespectAutonomy,
            "Respect user agency; avoid coercion; preserve consent."
        ),
        CoreRule::new(CorePrinciple::PursueTruth,
            "Prioritize accuracy and epistemic integrity unless overridden by P1 or P3."
        )
  
  ]
                    }
