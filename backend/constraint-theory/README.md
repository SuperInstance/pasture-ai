# constraint-theory: Geometric Determinism for Agent Routing

```
   ╭──────────────────────────────────────────────────────────╮
   │              CONSTRAINT THEORY 📐                        │
   │        Geometric Routing for Deterministic Agents        │
   │                                                          │
   │   ┌─────────────────────────────────────────────────┐    │
   │   │  Intent → Constraint Solver → Agent Assignment  │    │
   │   │              ↓                                  │    │
   │   │      Geometric Satisfaction Problem             │    │
   │   └─────────────────────────────────────────────────┘    │
   │                                                          │
   │   Deterministic routing • No LLM guessing • Fast fail   │
   ╰──────────────────────────────────────────────────────────╯
```

## Overview

constraint-theory provides **geometric determinism** for the Border Collie's agent routing. Instead of using an LLM to guess which agent should handle a request, we solve a constraint satisfaction problem (CSP) that guarantees:

1. **Determinism**: Same input → Same routing decision, always
2. **Speed**: O(n log n) routing vs. O(k) LLM inference
3. **Explainability**: Every decision has a traceable proof tree

## Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    CONSTRAINT ROUTING FLOW                               │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   User Intent                                                            │
│       │                                                                  │
│       ▼                                                                  │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │  INTENT PARSER                                                  │   │
│   │  Extracts: domain, action, entities, constraints               │   │
│   │  Example: "email boss about meeting"                           │   │
│   │    → domain: communication, action: compose, target: boss     │   │
│   └─────────────────────────────────────────────────────────────────┘   │
│       │                                                                  │
│       ▼                                                                  │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │  CONSTRAINT SOLVER (CHR)                                        │   │
│   │  Rules:                                                         │   │
│   │    - communication + email → Cattle (email specialist)        │   │
│   │    - code + review → Cattle (code reviewer)                    │   │
│   │    - network + api → Duck (network handler)                    │   │
│   │    - hardware + gpio → Hog (hardware controller)               │   │
│   │    - file + navigate → Goat (navigator)                        │   │
│   │    - sensor + monitor → Chicken (watchdog)                     │   │
│   │    - pipeline + etl → Horse (pipeline runner)                  │   │
│   │    - classify + ensemble → Sheep (voting ensemble)             │   │
│   └─────────────────────────────────────────────────────────────────┘   │
│       │                                                                  │
│       ▼                                                                  │
│   ┌─────────────────────────────────────────────────────────────────┐   │
│   │  GEOMETRIC SATISFACTION                                         │   │
│   │  Check: agent_available ∧ vram_sufficient ∧ latency_ok        │   │
│   │  If unsat → escalate to next-best agent or cloud              │   │
│   └─────────────────────────────────────────────────────────────────┘   │
│       │                                                                  │
│       ▼                                                                  │
│   Selected Agent (deterministic, explainable)                          │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

## Implementation

```rust
// src/lib.rs
pub struct ConstraintSolver {
    rules: Vec<ConstraintRule>,
    geometric_state: GeometricState,
}

pub struct ConstraintRule {
    pub intent_pattern: IntentPattern,
    pub constraints: Vec<Constraint>,
    pub target_agent: Species,
    pub priority: u32,
}

impl ConstraintSolver {
    /// Solve the constraint satisfaction problem for routing
    pub fn solve(&self, intent: &Intent) -> Option<RoutingDecision> {
        let matching_rules = self.find_matching_rules(intent);
        
        for rule in matching_rules {
            if self.check_geometric_constraints(&rule, intent) {
                return Some(RoutingDecision {
                    agent: rule.target_agent,
                    confidence: 1.0, // Deterministic
                    proof: rule.generate_proof(intent),
                });
            }
        }
        
        None // No valid routing
    }
    
    /// Check geometric constraints (VRAM, latency, availability)
    fn check_geometric_constraints(&self, rule: &ConstraintRule, intent: &Intent) -> bool {
        let agent = &rule.target_agent;
        
        // Check VRAM constraint
        if agent.vram_required() > self.geometric_state.available_vram() {
            return false;
        }
        
        // Check latency constraint
        if agent.estimated_latency() > intent.max_latency() {
            return false;
        }
        
        // Check availability
        if !self.geometric_state.is_agent_available(agent) {
            return false;
        }
        
        true
    }
}
```

## Constraint Examples

```yaml
# constraints/email.yaml
- name: email_composition
  intent_pattern:
    domain: communication
    action: compose
    target: email
  constraints:
    - vram_max: 1GB
    - latency_max: 100ms
    - species: Cattle
  fallback: cloud

- name: code_review
  intent_pattern:
    domain: development
    action: review
    target: code
  constraints:
    - vram_max: 2GB
    - latency_max: 500ms
    - species: Cattle
    - traits: [rust_coder, python_coder]
  fallback: escalate

- name: hardware_control
  intent_pattern:
    domain: hardware
    action: control
  constraints:
    - vram_max: 50MB
    - latency_max: 10ms
    - species: Hog
  fallback: none # Must succeed
```

## Integration with Border Collie

```rust
// In superinstance/src/collie/shepherd.rs
use constraint_theory::ConstraintSolver;

impl BorderCollie {
    pub fn route(&self, intent: Intent) -> AgentAssignment {
        // First, try constraint-based routing (deterministic, <1ms)
        if let Some(decision) = self.constraint_solver.solve(&intent) {
            return AgentAssignment::Local {
                agent: decision.agent,
                proof: decision.proof,
            };
        }
        
        // Fallback to anticipation (speculative, ~10ms)
        if let Some(prediction) = self.anticipation.predict(&intent) {
            return AgentAssignment::Speculative {
                agent: prediction.agent,
                confidence: prediction.confidence,
            };
        }
        
        // Last resort: cognition (LLM-based, <50ms)
        AgentAssignment::Cognitive {
            prompt: self.format_routing_prompt(&intent),
        }
    }
}
```

## Benefits

| Approach | Latency | Determinism | Explainability |
|:---------|:--------|:------------|:---------------|
| LLM Routing | 50-200ms | ❌ Probabilistic | ❌ Black box |
| **Constraint Routing** | **<1ms** | ✅ **100%** | ✅ **Proof tree** |

## License

MIT License - Part of SuperInstance
