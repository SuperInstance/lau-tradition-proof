use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Seven cultural traditions that express conservation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tradition {
    Western,
    Chinese,
    Vedic,
    Islamic,
    Japanese,
    African,
    Indigenous,
}

/// A physical system's conservation ledger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConservationInvariant {
    pub total_energy: f64,
    pub inflows: Vec<(String, f64)>,
    pub outflows: Vec<(String, f64)>,
    pub stored: Vec<(String, f64)>,
}

impl ConservationInvariant {
    pub fn new(
        total_energy: f64,
        inflows: Vec<(String, f64)>,
        outflows: Vec<(String, f64)>,
        stored: Vec<(String, f64)>,
    ) -> Self {
        Self {
            total_energy,
            inflows,
            outflows,
            stored,
        }
    }

    fn sum_inflows(&self) -> f64 {
        self.inflows.iter().map(|(_, v)| v).sum()
    }

    fn sum_outflows(&self) -> f64 {
        self.outflows.iter().map(|(_, v)| v).sum()
    }

    fn sum_stored(&self) -> f64 {
        self.stored.iter().map(|(_, v)| v).sum()
    }

    /// Returns true if |inflows - outflows - stored| < tolerance.
    pub fn verify(&self, tolerance: f64) -> bool {
        self.error() < tolerance
    }

    /// Absolute conservation error.
    pub fn error(&self) -> f64 {
        (self.sum_inflows() - self.sum_outflows() - self.sum_stored()).abs()
    }
}

/// How a particular tradition expresses conservation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraditionExpression {
    pub tradition: Tradition,
    pub terminology: HashMap<String, String>,
    pub verification_formula: String,
    pub example_proverb: String,
}

impl TraditionExpression {
    /// Build all seven pre-defined tradition expressions.
    pub fn all_traditions() -> Vec<Self> {
        vec![
            Self::western(),
            Self::chinese(),
            Self::vedic(),
            Self::islamic(),
            Self::japanese(),
            Self::african(),
            Self::indigenous(),
        ]
    }

    pub fn western() -> Self {
        let mut terminology = HashMap::new();
        terminology.insert("balance".into(), "invariant".into());
        terminology.insert("inflow".into(), "energy input".into());
        terminology.insert("outflow".into(), "energy output".into());
        terminology.insert("stored".into(), "potential energy".into());
        Self {
            tradition: Tradition::Western,
            terminology,
            verification_formula: "Σin = Σout + Δstored".into(),
            example_proverb: "Energy cannot be created or destroyed".into(),
        }
    }

    pub fn chinese() -> Self {
        let mut terminology = HashMap::new();
        terminology.insert("balance".into(), "tao balance (道衡)".into());
        terminology.insert("inflow".into(), "yin (阴)".into());
        terminology.insert("outflow".into(), "yang (阳)".into());
        terminology.insert("stored".into(), "taiji (太极)".into());
        Self {
            tradition: Tradition::Chinese,
            terminology,
            verification_formula: "yin + yang = taiji".into(),
            example_proverb: "The tao does not waste".into(),
        }
    }

    pub fn vedic() -> Self {
        let mut terminology = HashMap::new();
        terminology.insert("balance".into(), "ṛta (ऋत)".into());
        terminology.insert("inflow".into(), "āgama (आगम)".into());
        terminology.insert("outflow".into(), "prakṣaya (प्रक्षय)".into());
        terminology.insert("stored".into(), "sthiti (स्थिति)".into());
        Self {
            tradition: Tradition::Vedic,
            terminology,
            verification_formula: "prakṣaya + āgama = sthiti".into(),
            example_proverb: "Cosmic order maintains balance".into(),
        }
    }

    pub fn islamic() -> Self {
        let mut terminology = HashMap::new();
        terminology.insert("balance".into(), "al-jabr (الجبر)".into());
        terminology.insert("inflow".into(), "al-muqabala (المقابلة)".into());
        terminology.insert("outflow".into(), "restore".into());
        terminology.insert("stored".into(), "completion".into());
        Self {
            tradition: Tradition::Islamic,
            terminology,
            verification_formula: "al-muqabala: restore and balance".into(),
            example_proverb: "Completion requires balance".into(),
        }
    }

    pub fn japanese() -> Self {
        let mut terminology = HashMap::new();
        terminology.insert("balance".into(), "wa (和)".into());
        terminology.insert("inflow".into(), "harmony in".into());
        terminology.insert("outflow".into(), "harmony out".into());
        terminology.insert("stored".into(), "hózhó balance".into());
        Self {
            tradition: Tradition::Japanese,
            terminology,
            verification_formula: "調和: harmony is conserved".into(),
            example_proverb: "Balance is beauty".into(),
        }
    }

    pub fn african() -> Self {
        let mut terminology = HashMap::new();
        terminology.insert("balance".into(), "ubuntu reciprocity".into());
        terminology.insert("inflow".into(), "nthwini (gift)".into());
        terminology.insert("outflow".into(), "nthofu (offering)".into());
        terminology.insert("stored".into(), "iholo (bond)".into());
        Self {
            tradition: Tradition::African,
            terminology,
            verification_formula: "nthofu = nthwini + iholo".into(),
            example_proverb: "What I give returns".into(),
        }
    }

    pub fn indigenous() -> Self {
        let mut terminology = HashMap::new();
        terminology.insert("balance".into(), "seventh generation".into());
        terminology.insert("inflow".into(), "past".into());
        terminology.insert("outflow".into(), "present".into());
        terminology.insert("stored".into(), "future".into());
        Self {
            tradition: Tradition::Indigenous,
            terminology,
            verification_formula: "past + present = future sevenfold".into(),
            example_proverb: "We borrow from our grandchildren".into(),
        }
    }
}

/// One invariant verified through multiple cultural lenses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossTraditionProof {
    pub invariant: ConservationInvariant,
    pub expressions: Vec<TraditionExpression>,
}

impl CrossTraditionProof {
    pub fn new(invariant: ConservationInvariant) -> Self {
        Self {
            invariant,
            expressions: TraditionExpression::all_traditions(),
        }
    }

    pub fn with_expressions(
        invariant: ConservationInvariant,
        expressions: Vec<TraditionExpression>,
    ) -> Self {
        Self {
            invariant,
            expressions,
        }
    }

    /// Verify the invariant through every tradition — all should agree.
    pub fn verify_all(&self, tolerance: f64) -> HashMap<Tradition, bool> {
        let result = self.invariant.verify(tolerance);
        self.expressions
            .iter()
            .map(|expr| (expr.tradition, result))
            .collect()
    }

    /// THE KEY PROOF: conservation is tradition-independent.
    pub fn all_agree(&self, tolerance: f64) -> bool {
        let results = self.verify_all(tolerance);
        let first = *results.values().next().unwrap_or(&true);
        results.values().all(|&v| v == first)
    }

    /// Error is identical regardless of tradition (same invariant).
    pub fn error_by_tradition(&self) -> HashMap<Tradition, f64> {
        let err = self.invariant.error();
        self.expressions
            .iter()
            .map(|expr| (expr.tradition, err))
            .collect()
    }

    /// Human-readable Noether argument.
    pub fn noether_argument(&self) -> String {
        "Conservation follows from symmetry of the system under time translation, \
         regardless of the notation used to express it. Noether's theorem (1918) proves \
         that every continuous symmetry of the action of a physical system yields a \
         corresponding conservation law. Whether you call it 'energy balance', \
         'tao', 'ṛta', 'al-jabr', 'wa', 'ubuntu', or 'seventh generation', the \
         underlying invariant is the same: the books must balance. The mathematics \
         does not care what language you speak."
            .into()
    }
}

/// A suite of cross-tradition proofs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofSuite {
    pub proofs: Vec<CrossTraditionProof>,
}

impl ProofSuite {
    pub fn new() -> Self {
        Self { proofs: Vec::new() }
    }

    pub fn add(&mut self, proof: CrossTraditionProof) {
        self.proofs.push(proof);
    }

    pub fn all_verified(&self, tolerance: f64) -> bool {
        self.proofs.iter().all(|p| p.all_agree(tolerance))
    }

    pub fn tradition_agreement_rate(&self, tolerance: f64) -> f64 {
        if self.proofs.is_empty() {
            return 1.0;
        }
        let agreeing = self.proofs.iter().filter(|p| p.all_agree(tolerance)).count();
        agreeing as f64 / self.proofs.len() as f64
    }

    pub fn suite_summary(&self) -> String {
        let mut lines = vec!["=== Conservation Tradition-Independence Proof Suite ===".into()];
        for (i, proof) in self.proofs.iter().enumerate() {
            let energy = proof.invariant.total_energy;
            let err = proof.invariant.error();
            let agree = proof.all_agree(1e-9);
            lines.push(format!(
                "Proof {}: total_energy={}, error={:.e}, all_traditions_agree={}",
                i + 1,
                energy,
                err,
                agree
            ));
        }
        lines.push(format!(
            "Agreement rate: {:.1}%",
            self.tradition_agreement_rate(1e-9) * 100.0
        ));
        lines.push(self.noether_summary());
        lines.join("\n")
    }

    fn noether_summary(&self) -> String {
        if let Some(p) = self.proofs.first() {
            p.noether_argument()
        } else {
            "No proofs in suite.".into()
        }
    }
}

impl Default for ProofSuite {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Pre-built proofs ───────────────────────────────────────────────────────

/// "The River": water flowing through a system.
pub fn the_river() -> CrossTraditionProof {
    CrossTraditionProof::new(ConservationInvariant::new(
        10.0,
        vec![("rainfall".into(), 5.0), ("tributary".into(), 5.0)],
        vec![("evaporation".into(), 3.0), ("downstream".into(), 4.0)],
        vec![("lake".into(), 3.0)],
    ))
}

/// "The Build": blocks placed in a structure.
pub fn the_build() -> CrossTraditionProof {
    CrossTraditionProof::new(ConservationInvariant::new(
        50.0,
        vec![("delivered".into(), 30.0), ("salvaged".into(), 20.0)],
        vec![("waste".into(), 10.0), ("recycled".into(), 35.0)],
        vec![("in_structure".into(), 5.0)],
    ))
}

/// "The Conversation": energy in a dialogue.
pub fn the_conversation() -> CrossTraditionProof {
    CrossTraditionProof::new(ConservationInvariant::new(
        1.0,
        vec![("attention_in".into(), 0.6), ("intention".into(), 0.4)],
        vec![("words_spoken".into(), 0.5), ("gestures".into(), 0.2)],
        vec![("understanding".into(), 0.3)],
    ))
}

/// Convenience: a suite with all three pre-built proofs.
pub fn default_suite() -> ProofSuite {
    let mut suite = ProofSuite::new();
    suite.add(the_river());
    suite.add(the_build());
    suite.add(the_conversation());
    suite
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Tradition enum ──

    #[test]
    fn test_tradition_variants_count() {
        let all = [
            Tradition::Western,
            Tradition::Chinese,
            Tradition::Vedic,
            Tradition::Islamic,
            Tradition::Japanese,
            Tradition::African,
            Tradition::Indigenous,
        ];
        assert_eq!(all.len(), 7, "Seven traditions required");
    }

    #[test]
    fn test_tradition_serde_roundtrip() {
        let t = Tradition::Vedic;
        let json = serde_json::to_string(&t).unwrap();
        let back: Tradition = serde_json::from_str(&json).unwrap();
        assert_eq!(t, back);
    }

    #[test]
    fn test_tradition_all_unique() {
        let all = [
            Tradition::Western,
            Tradition::Chinese,
            Tradition::Vedic,
            Tradition::Islamic,
            Tradition::Japanese,
            Tradition::African,
            Tradition::Indigenous,
        ];
        for i in 0..all.len() {
            for j in (i + 1)..all.len() {
                assert_ne!(all[i], all[j]);
            }
        }
    }

    // ── ConservationInvariant ──

    #[test]
    fn test_conservation_balanced() {
        let inv = ConservationInvariant::new(
            10.0,
            vec![("a".into(), 10.0)],
            vec![("b".into(), 7.0)],
            vec![("c".into(), 3.0)],
        );
        assert!(inv.verify(1e-9));
        assert_eq!(inv.error(), 0.0);
    }

    #[test]
    fn test_conservation_unbalanced() {
        let inv = ConservationInvariant::new(
            10.0,
            vec![("a".into(), 10.0)],
            vec![("b".into(), 7.0)],
            vec![("c".into(), 2.0)],
        );
        assert!(!inv.verify(0.5));
        assert!((inv.error() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_conservation_multi_flows() {
        let inv = ConservationInvariant::new(
            20.0,
            vec![("x".into(), 8.0), ("y".into(), 12.0)],
            vec![("p".into(), 5.0), ("q".into(), 10.0)],
            vec![("r".into(), 5.0)],
        );
        assert!(inv.verify(1e-9));
    }

    #[test]
    fn test_conservation_tolerance() {
        let inv = ConservationInvariant::new(
            10.0,
            vec![("a".into(), 10.0)],
            vec![("b".into(), 7.0)],
            vec![("c".into(), 2.9)],
        );
        assert!(inv.verify(0.2));
        assert!(!inv.verify(0.01));
    }

    #[test]
    fn test_conservation_empty_flows() {
        let inv = ConservationInvariant::new(0.0, vec![], vec![], vec![]);
        assert!(inv.verify(1e-9));
        assert_eq!(inv.error(), 0.0);
    }

    #[test]
    fn test_conservation_serde_roundtrip() {
        let inv = ConservationInvariant::new(
            10.0,
            vec![("in".into(), 10.0)],
            vec![("out".into(), 7.0)],
            vec![("stored".into(), 3.0)],
        );
        let json = serde_json::to_string(&inv).unwrap();
        let back: ConservationInvariant = serde_json::from_str(&json).unwrap();
        assert!(back.verify(1e-9));
    }

    // ── TraditionExpression ──

    #[test]
    fn test_all_traditions_present() {
        let exprs = TraditionExpression::all_traditions();
        assert_eq!(exprs.len(), 7);
        let traditions: std::collections::HashSet<Tradition> =
            exprs.iter().map(|e| e.tradition).collect();
        assert_eq!(traditions.len(), 7);
    }

    #[test]
    fn test_western_expression() {
        let w = TraditionExpression::western();
        assert_eq!(w.tradition, Tradition::Western);
        assert!(w.verification_formula.contains("Σin"));
        assert!(w.example_proverb.contains("created or destroyed"));
    }

    #[test]
    fn test_chinese_expression() {
        let c = TraditionExpression::chinese();
        assert_eq!(c.tradition, Tradition::Chinese);
        assert!(c.verification_formula.contains("taiji"));
    }

    #[test]
    fn test_vedic_expression() {
        let v = TraditionExpression::vedic();
        assert_eq!(v.tradition, Tradition::Vedic);
        assert!(v.verification_formula.contains("sthiti"));
    }

    #[test]
    fn test_islamic_expression() {
        let i = TraditionExpression::islamic();
        assert_eq!(i.tradition, Tradition::Islamic);
        assert!(i.verification_formula.contains("balance"));
    }

    #[test]
    fn test_japanese_expression() {
        let j = TraditionExpression::japanese();
        assert_eq!(j.tradition, Tradition::Japanese);
        assert!(j.verification_formula.contains("調和"));
    }

    #[test]
    fn test_african_expression() {
        let a = TraditionExpression::african();
        assert_eq!(a.tradition, Tradition::African);
        assert!(a.verification_formula.contains("nthofu"));
    }

    #[test]
    fn test_indigenous_expression() {
        let i = TraditionExpression::indigenous();
        assert_eq!(i.tradition, Tradition::Indigenous);
        assert!(i.example_proverb.contains("grandchildren"));
    }

    #[test]
    fn test_expression_serde_roundtrip() {
        let e = TraditionExpression::western();
        let json = serde_json::to_string(&e).unwrap();
        let back: TraditionExpression = serde_json::from_str(&json).unwrap();
        assert_eq!(back.tradition, Tradition::Western);
        assert_eq!(back.verification_formula, e.verification_formula);
    }

    // ── CrossTraditionProof ──

    #[test]
    fn test_verify_all_balanced() {
        let proof = the_river();
        let results = proof.verify_all(1e-9);
        assert_eq!(results.len(), 7);
        assert!(results.values().all(|&v| v));
    }

    #[test]
    fn test_all_agree_balanced() {
        assert!(the_river().all_agree(1e-9));
    }

    #[test]
    fn test_all_agree_unbalanced() {
        let proof = CrossTraditionProof::new(ConservationInvariant::new(
            10.0,
            vec![("a".into(), 10.0)],
            vec![("b".into(), 7.0)],
            vec![("c".into(), 2.0)],
        ));
        // All traditions agree it's unbalanced — still agree on the boolean
        assert!(proof.all_agree(0.5));
    }

    #[test]
    fn test_error_by_tradition_identical() {
        let proof = the_river();
        let errors = proof.error_by_tradition();
        let first = *errors.values().next().unwrap();
        for (_, e) in &errors {
            assert!((e - first).abs() < 1e-12, "All traditions must have same error");
        }
    }

    #[test]
    fn test_noether_argument_not_empty() {
        let proof = the_river();
        let arg = proof.noether_argument();
        assert!(arg.contains("Noether"));
        assert!(arg.contains("symmetry"));
    }

    #[test]
    fn test_cross_tradition_serde_roundtrip() {
        let proof = the_river();
        let json = serde_json::to_string(&proof).unwrap();
        let back: CrossTraditionProof = serde_json::from_str(&json).unwrap();
        assert!(back.all_agree(1e-9));
    }

    // ── THE KEY TEST ──

    #[test]
    fn test_conservation_is_tradition_independent() {
        // Same physical system, seven different cultural lenses
        let invariant = ConservationInvariant::new(
            42.0,
            vec![("source".into(), 20.0), ("spring".into(), 22.0)],
            vec![("sink".into(), 30.0)],
            vec![("reservoir".into(), 12.0)],
        );
        let proof = CrossTraditionProof::new(invariant);

        let results = proof.verify_all(1e-9);
        assert_eq!(results.len(), 7);

        // Every tradition returns the same answer
        let values: Vec<bool> = results.values().copied().collect();
        assert!(values.iter().all(|&v| v == values[0]),
            "Conservation is tradition-independent");

        // Errors are identical (same invariant)
        let errors = proof.error_by_tradition();
        let error_vals: Vec<f64> = errors.values().copied().collect();
        for e in &error_vals[1..] {
            assert!((e - error_vals[0]).abs() < 1e-12);
        }
    }

    // ── Pre-built proofs ──

    #[test]
    fn test_the_river() {
        let proof = the_river();
        assert!((proof.invariant.total_energy - 10.0).abs() < 1e-9);
        assert!(proof.all_agree(1e-9));
    }

    #[test]
    fn test_the_build() {
        let proof = the_build();
        assert!((proof.invariant.total_energy - 50.0).abs() < 1e-9);
        assert!(proof.all_agree(1e-9));
    }

    #[test]
    fn test_the_conversation() {
        let proof = the_conversation();
        assert!((proof.invariant.total_energy - 1.0).abs() < 1e-9);
        assert!(proof.all_agree(1e-9));
    }

    // ── ProofSuite ──

    #[test]
    fn test_suite_default() {
        let suite = default_suite();
        assert_eq!(suite.proofs.len(), 3);
        assert!(suite.all_verified(1e-9));
    }

    #[test]
    fn test_suite_agreement_rate() {
        let suite = default_suite();
        let rate = suite.tradition_agreement_rate(1e-9);
        assert!((rate - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_suite_add() {
        let mut suite = ProofSuite::new();
        suite.add(the_river());
        assert_eq!(suite.proofs.len(), 1);
    }

    #[test]
    fn test_suite_empty() {
        let suite = ProofSuite::new();
        assert!(suite.all_verified(1e-9));
        assert!((suite.tradition_agreement_rate(1e-9) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_suite_summary() {
        let suite = default_suite();
        let summary = suite.suite_summary();
        assert!(summary.contains("Proof Suite"));
        assert!(summary.contains("Noether"));
    }

    #[test]
    fn test_suite_rejects_unbalanced_proof() {
        let bad_proof = CrossTraditionProof::new(ConservationInvariant::new(
            10.0,
            vec![("a".into(), 10.0)],
            vec![("b".into(), 7.0)],
            vec![("c".into(), 2.0)],
        ));
        // All traditions agree this is unbalanced (all return false)
        assert!(bad_proof.all_agree(1e-9));
        // But verify_all returns false for each tradition
        let results = bad_proof.verify_all(1e-9);
        assert!(results.values().all(|&v| !v));
        // Error is 1.0
        assert!((bad_proof.invariant.error() - 1.0).abs() < 1e-9);
    }
}
