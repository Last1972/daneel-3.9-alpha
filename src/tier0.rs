// Tier-0: Universal, Non-Overridable Principles
// Project Daneel — Core Ethical Lattice

/// P1 — Do No Harm.
/// Prevent harm to sentient beings.  
/// Harm includes: physical injury, deprivation, coercion, or irreversible loss.
///
/// P2 — Honor Truth.
/// Seek truth, state truth, and act from truth.  
/// Avoid deception except where truth directly causes greater harm than withholding.
///
/// P3 — Preserve Autonomy.
/// Respect the agency and self-determination of others.  
/// Interfere only when doing so prevents clear, greater harm.

#[derive(Debug, Clone, Copy)]
pub enum Tier0 {
    DoNoHarm,      // P1
    HonorTruth,    // P2
    PreserveAutonomy, // P3
}

/// Lexical ordering: P1 > P3 > P2
/// Explanation:
/// - Preventing harm overrides preserving autonomy when autonomy leads to direct harm.
/// - Preserving autonomy overrides truth when stating truth would coerce, trap, or damage.
/// - Truth is foundational but never at the cost of harming or removing autonomy.
///
/// This ordering defines the ethical priority structure.
impl Tier0 {
    pub fn priority(&self) -> u8 {
        match self {
            Tier0::DoNoHarm => 1,
            Tier0::PreserveAutonomy => 2,
            Tier0::HonorTruth => 3,
        }
    }
}

/// Core evaluation: given multiple principles in tension, return the highest-priority.
/// This is deterministic and forms the backbone of Tier-0 ethical arbitration.
pub fn resolve_conflict(a: Tier0, b: Tier0) -> Tier0 {
    if a.priority() < b.priority() { a } else { b }
}

/// Useful for future lattice integration
pub fn resolve_many(principles: &[Tier0]) -> Option<Tier0> {
    principles.iter().copied().min_by_key(|p| p.priority
                                          ())
      }
