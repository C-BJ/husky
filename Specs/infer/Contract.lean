inductive Qualifier where
  | EvalRef
  | Copyable

inductive Contract where
  | EvalRef
  | Copyable
  | Any

def does_contract_contains_qualifer (contract: Contract)(qualifier: Qualifier) :Bool :=
  match contract with
  | Contract.EvalRef =>
    match qualifier with
    | Qualifier.EvalRef => true
    | _ => false
  | Contract.Copyable =>
    match qualifier with
    | Qualifier.Copyable => true
    | _ => false
  | Contract.Any => true

example: ∀ qualifier : Qualifier, does_contract_contains_qualifer Contract.Any qualifier = true :=
  fun qualifier: Qualifier => rfl