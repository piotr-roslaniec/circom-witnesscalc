use crate::field::M;
use ark_bn254::Fr;
use ark_ff::{BigInt, BigInteger, One, PrimeField, Zero};
use rand::Rng;
use ruint::aliases::U256;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::error::Error;
use std::ops::{BitOr, BitXor, Deref};
use std::{
    collections::HashMap,
    ops::{BitAnd, Shl, Shr},
};

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
use ruint::uint;

fn ark_se<S, A: CanonicalSerialize>(a: &A, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut bytes = vec![];
    a.serialize_with_mode(&mut bytes, Compress::Yes)
        .map_err(serde::ser::Error::custom)?;
    s.serialize_bytes(&bytes)
}

fn ark_de<'de, D, A: CanonicalDeserialize>(data: D) -> Result<A, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let s: Vec<u8> = serde::de::Deserialize::deserialize(data)?;
    let a = A::deserialize_with_mode(s.as_slice(), Compress::Yes, Validate::Yes);
    a.map_err(serde::de::Error::custom)
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Operation {
    Mul,
    Div,
    Add,
    Sub,
    Pow,
    Idiv,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Leq,
    Geq,
    Land,
    Lor,
    Shl,
    Shr,
    Bor,
    Band,
    Bxor,
}

impl Operation {
    // TODO: rewrite to &U256 type
    pub fn eval(&self, a: U256, b: U256) -> U256 {
        use Operation::*;
        match self {
            Mul => a.mul_mod(b, M),
            Div => {
                if b == U256::ZERO {
                    // as we are simulating a circuit execution with signals
                    // values all equal to 0, just return 0 here in case of
                    // division by zero
                    U256::ZERO
                } else {
                    a.mul_mod(b.inv_mod(M).unwrap(), M)
                }
            }
            Add => a.add_mod(b, M),
            Sub => a.add_mod(M - b, M),
            Pow => a.pow_mod(b, M),
            Mod => a.div_rem(b).1,
            Eq => U256::from(a == b),
            Neq => U256::from(a != b),
            Lt => u_lt(&a, &b),
            Gt => u_gt(&a, &b),
            Leq => u_lte(&a, &b),
            Geq => u_gte(&a, &b),
            Land => U256::from(a != U256::ZERO && b != U256::ZERO),
            Lor => U256::from(a != U256::ZERO || b != U256::ZERO),
            Shl => compute_shl_uint(a, b),
            Shr => compute_shr_uint(a, b),
            // TODO test with conner case when it is possible to get the number
            //      bigger then modulus
            Bor => a.bitor(b),
            Band => a.bitand(b),
            // TODO test with conner case when it is possible to get the number
            //      bigger then modulus
            Bxor => a.bitxor(b),
            Idiv => a / b,
        }
    }

    pub fn eval_fr(&self, a: Fr, b: Fr) -> Fr {
        use Operation::*;
        match self {
            Mul => a * b,
            // We always should return something on the circuit execution.
            // So in case of division by 0 we would return 0. And the proof
            // should be invalid in the end.
            Div => {
                if b.is_zero() {
                    Fr::zero()
                } else {
                    a / b
                }
            }
            Add => a + b,
            Sub => a - b,
            Idiv => {
                if b.is_zero() {
                    Fr::zero()
                } else {
                    Fr::new((Into::<U256>::into(a) / Into::<U256>::into(b)).into())
                }
            }
            Mod => {
                if b.is_zero() {
                    Fr::zero()
                } else {
                    Fr::new((Into::<U256>::into(a) % Into::<U256>::into(b)).into())
                }
            }
            Eq => match a.cmp(&b) {
                Ordering::Equal => Fr::one(),
                _ => Fr::zero(),
            },
            Neq => match a.cmp(&b) {
                Ordering::Equal => Fr::zero(),
                _ => Fr::one(),
            },
            Lt => Fr::new(u_lt(&a.into(), &b.into()).into()),
            Gt => Fr::new(u_gt(&a.into(), &b.into()).into()),
            Leq => Fr::new(u_lte(&a.into(), &b.into()).into()),
            Geq => Fr::new(u_gte(&a.into(), &b.into()).into()),
            Land => {
                if a.is_zero() || b.is_zero() {
                    Fr::zero()
                } else {
                    Fr::one()
                }
            }
            Lor => {
                if a.is_zero() && b.is_zero() {
                    Fr::zero()
                } else {
                    Fr::one()
                }
            }
            Shl => shl(a, b),
            Shr => shr(a, b),
            Bor => bit_or(a, b),
            Band => bit_and(a, b),
            Bxor => bit_xor(a, b),
            // TODO implement other operators
            _ => unimplemented!("operator {:?} not implemented for Montgomery", self),
        }
    }
}

impl From<&Operation> for crate::proto::DuoOp {
    fn from(v: &Operation) -> Self {
        match v {
            Operation::Mul => crate::proto::DuoOp::Mul,
            Operation::Div => crate::proto::DuoOp::Div,
            Operation::Add => crate::proto::DuoOp::Add,
            Operation::Sub => crate::proto::DuoOp::Sub,
            Operation::Pow => crate::proto::DuoOp::Pow,
            Operation::Idiv => crate::proto::DuoOp::Idiv,
            Operation::Mod => crate::proto::DuoOp::Mod,
            Operation::Eq => crate::proto::DuoOp::Eq,
            Operation::Neq => crate::proto::DuoOp::Neq,
            Operation::Lt => crate::proto::DuoOp::Lt,
            Operation::Gt => crate::proto::DuoOp::Gt,
            Operation::Leq => crate::proto::DuoOp::Leq,
            Operation::Geq => crate::proto::DuoOp::Geq,
            Operation::Land => crate::proto::DuoOp::Land,
            Operation::Lor => crate::proto::DuoOp::Lor,
            Operation::Shl => crate::proto::DuoOp::Shl,
            Operation::Shr => crate::proto::DuoOp::Shr,
            Operation::Bor => crate::proto::DuoOp::Bor,
            Operation::Band => crate::proto::DuoOp::Band,
            Operation::Bxor => crate::proto::DuoOp::Bxor,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum UnoOperation {
    Neg,
    Id, // identity - just return self
}

impl UnoOperation {
    pub fn eval(&self, a: U256) -> U256 {
        match self {
            UnoOperation::Neg => {
                if a == U256::ZERO {
                    U256::ZERO
                } else {
                    M - a
                }
            }
            UnoOperation::Id => a,
        }
    }

    pub fn eval_fr(&self, a: Fr) -> Fr {
        match self {
            UnoOperation::Neg => {
                if a.is_zero() {
                    Fr::zero()
                } else {
                    let mut x = Fr::MODULUS;
                    x.sub_with_borrow(&a.into_bigint());
                    Fr::from_bigint(x).unwrap()
                }
            }
            _ => unimplemented!("uno operator {:?} not implemented for Montgomery", self),
        }
    }
}

impl From<&UnoOperation> for crate::proto::UnoOp {
    fn from(v: &UnoOperation) -> Self {
        match v {
            UnoOperation::Neg => crate::proto::UnoOp::Neg,
            UnoOperation::Id => crate::proto::UnoOp::Id,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TresOperation {
    TernCond,
}

impl TresOperation {
    pub fn eval(&self, a: U256, b: U256, c: U256) -> U256 {
        match self {
            TresOperation::TernCond => {
                if a == U256::ZERO {
                    c
                } else {
                    b
                }
            }
        }
    }

    pub fn eval_fr(&self, a: Fr, b: Fr, c: Fr) -> Fr {
        match self {
            TresOperation::TernCond => {
                if a.is_zero() {
                    c
                } else {
                    b
                }
            }
        }
    }
}

impl From<&TresOperation> for crate::proto::TresOp {
    fn from(v: &TresOperation) -> Self {
        match v {
            TresOperation::TernCond => crate::proto::TresOp::TernCond,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Node {
    Input(usize),
    Constant(U256),
    #[serde(serialize_with = "ark_se", deserialize_with = "ark_de")]
    MontConstant(Fr),
    UnoOp(UnoOperation, usize),
    Op(Operation, usize, usize),
    TresOp(TresOperation, usize, usize, usize),
}

// TODO remove pub from Vec<Node>
pub struct Nodes(pub Vec<Node>);

impl Nodes {
    pub fn new() -> Self {
        Nodes(Vec::new())
    }

    pub fn to_const(&self, idx: NodeIdx) -> Result<U256, NodeConstErr> {
        let me = self.0.get(idx.0).ok_or(NodeConstErr::EmptyNode(idx))?;
        match me {
            Node::Constant(v) => Ok(v.clone()),
            Node::UnoOp(op, a) => Ok(op.eval(self.to_const(NodeIdx(*a))?)),
            Node::Op(op, a, b) => {
                Ok(op.eval(self.to_const(NodeIdx(*a))?, self.to_const(NodeIdx(*b))?))
            }
            Node::TresOp(op, a, b, c) => Ok(op.eval(
                self.to_const(NodeIdx(*a))?,
                self.to_const(NodeIdx(*b))?,
                self.to_const(NodeIdx(*c))?,
            )),
            Node::Input(_) => Err(NodeConstErr::InputSignal),
            Node::MontConstant(_) => {
                panic!("MontConstant should not be used here")
            }
        }
    }

    pub fn push(&mut self, n: Node) -> NodeIdx {
        self.0.push(n);
        NodeIdx(self.0.len() - 1)
    }

    pub fn get(&self, idx: NodeIdx) -> Option<&Node> {
        self.0.get(idx.0)
    }
}

impl Deref for Nodes {
    type Target = Vec<Node>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct NodeIdx(pub usize);

impl From<usize> for NodeIdx {
    fn from(v: usize) -> Self {
        NodeIdx(v)
    }
}

#[derive(Debug)]
pub enum NodeConstErr {
    EmptyNode(NodeIdx),
    InputSignal,
}

impl std::fmt::Display for NodeConstErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeConstErr::EmptyNode(idx) => {
                write!(f, "empty node at index {}", idx.0)
            }
            NodeConstErr::InputSignal => {
                write!(f, "input signal is not a constant")
            }
        }
    }
}

impl Error for NodeConstErr {}

fn compute_shl_uint(a: U256, b: U256) -> U256 {
    debug_assert!(b.lt(&U256::from(256)));
    let ls_limb = b.as_limbs()[0];
    a.shl(ls_limb as usize)
}

fn compute_shr_uint(a: U256, b: U256) -> U256 {
    debug_assert!(b.lt(&U256::from(256)));
    let ls_limb = b.as_limbs()[0];
    a.shr(ls_limb as usize)
}

/// All references must be backwards.
fn assert_valid(nodes: &[Node]) {
    for (i, &node) in nodes.iter().enumerate() {
        if let Node::Op(_, a, b) = node {
            assert!(a < i);
            assert!(b < i);
        } else if let Node::UnoOp(_, a) = node {
            assert!(a < i);
        } else if let Node::TresOp(_, a, b, c) = node {
            assert!(a < i);
            assert!(b < i);
            assert!(c < i);
        }
    }
}

pub fn optimize(nodes: &mut Vec<Node>, outputs: &mut [usize]) {
    log::info!("Optimizing circuit");
    tree_shake(nodes, outputs);
    propagate(nodes);
    value_numbering(nodes, outputs);
    constants(nodes);
    tree_shake(nodes, outputs);
    montgomery_form(nodes);
    log::info!("Optimization done");
}

pub fn evaluate(nodes: &[Node], inputs: &[U256], outputs: &[usize]) -> Vec<U256> {
    // assert_valid(nodes);

    // Evaluate the graph.
    let mut values = Vec::with_capacity(nodes.len());
    for (_, &node) in nodes.iter().enumerate() {
        let value = match node {
            Node::Constant(c) => Fr::new(c.into()),
            Node::MontConstant(c) => c,
            Node::Input(i) => Fr::new(inputs[i].into()),
            Node::Op(op, a, b) => op.eval_fr(values[a], values[b]),
            Node::UnoOp(op, a) => op.eval_fr(values[a]),
            Node::TresOp(op, a, b, c) => op.eval_fr(values[a], values[b], values[c]),
        };
        values.push(value);
    }

    // Convert from Montgomery form and return the outputs.
    let mut out = vec![U256::ZERO; outputs.len()];
    for i in 0..outputs.len() {
        out[i] = U256::try_from(values[outputs[i]].into_bigint()).unwrap();
    }

    out
}

/// Constant propagation
pub fn propagate(nodes: &mut [Node]) {
    assert_valid(nodes);
    let mut constants = 0_usize;
    for i in 0..nodes.len() {
        if let Node::Op(op, a, b) = nodes[i] {
            if let (Node::Constant(va), Node::Constant(vb)) = (nodes[a], nodes[b]) {
                nodes[i] = Node::Constant(op.eval(va, vb));
                constants += 1;
            } else if a == b {
                // Not constant but equal
                use Operation::*;
                if let Some(c) = match op {
                    Eq | Leq | Geq => Some(true),
                    Neq | Lt | Gt => Some(false),
                    _ => None,
                } {
                    nodes[i] = Node::Constant(U256::from(c));
                    constants += 1;
                }
            }
        } else if let Node::UnoOp(op, a) = nodes[i] {
            if let Node::Constant(va) = nodes[a] {
                nodes[i] = Node::Constant(op.eval(va));
                constants += 1;
            }
        } else if let Node::TresOp(op, a, b, c) = nodes[i] {
            if let (Node::Constant(va), Node::Constant(vb), Node::Constant(vc)) =
                (nodes[a], nodes[b], nodes[c])
            {
                nodes[i] = Node::Constant(op.eval(va, vb, vc));
                constants += 1;
            }
        }
    }

    eprintln!("Propagated {constants} constants");
}

/// Remove unused nodes
pub fn tree_shake(nodes: &mut Vec<Node>, outputs: &mut [usize]) {
    assert_valid(nodes);

    // Mark all nodes that are used.
    let mut used = vec![false; nodes.len()];
    for &i in outputs.iter() {
        used[i] = true;
    }

    // Work backwards from end as all references are backwards.
    for i in (0..nodes.len()).rev() {
        if used[i] {
            if let Node::Op(_, a, b) = nodes[i] {
                used[a] = true;
                used[b] = true;
            }
            if let Node::UnoOp(_, a) = nodes[i] {
                used[a] = true;
            }
            if let Node::TresOp(_, a, b, c) = nodes[i] {
                used[a] = true;
                used[b] = true;
                used[c] = true;
            }
        }
    }

    // Remove unused nodes
    let n = nodes.len();
    let mut retain = used.iter();
    nodes.retain(|_| *retain.next().unwrap());
    let removed = n - nodes.len();

    // Renumber references.
    let mut renumber = vec![None; n];
    let mut index = 0;
    for (i, &used) in used.iter().enumerate() {
        if used {
            renumber[i] = Some(index);
            index += 1;
        }
    }
    assert_eq!(index, nodes.len());
    for (&used, renumber) in used.iter().zip(renumber.iter()) {
        assert_eq!(used, renumber.is_some());
    }

    // Renumber references.
    for node in nodes.iter_mut() {
        if let Node::Op(_, a, b) = node {
            *a = renumber[*a].unwrap();
            *b = renumber[*b].unwrap();
        }
        if let Node::UnoOp(_, a) = node {
            *a = renumber[*a].unwrap();
        }
        if let Node::TresOp(_, a, b, c) = node {
            *a = renumber[*a].unwrap();
            *b = renumber[*b].unwrap();
            *c = renumber[*c].unwrap();
        }
    }
    for output in outputs.iter_mut() {
        *output = renumber[*output].unwrap();
    }

    eprintln!("Removed {removed} unused nodes");
}

/// Randomly evaluate the graph
fn random_eval(nodes: &mut Vec<Node>) -> Vec<U256> {
    let mut rng = rand::thread_rng();
    let mut values = Vec::with_capacity(nodes.len());
    let mut inputs = HashMap::new();
    let mut prfs = HashMap::new();
    let mut prfs_uno = HashMap::new();
    let mut prfs_tres = HashMap::new();
    for node in nodes.iter() {
        use Operation::*;
        let value = match node {
            // Constants evaluate to themselves
            Node::Constant(c) => *c,

            Node::MontConstant(_) => unimplemented!("should not be used"),

            // Algebraic Ops are evaluated directly
            // Since the field is large, by Swartz-Zippel if
            // two values are the same then they are likely algebraically equal.
            Node::Op(op @ (Add | Sub | Mul), a, b) => op.eval(values[*a], values[*b]),

            // Input and non-algebraic ops are random functions
            // TODO: https://github.com/recmo/uint/issues/95 and use .gen_range(..M)
            Node::Input(i) => *inputs.entry(*i).or_insert_with(|| rng.gen::<U256>() % M),
            Node::Op(op, a, b) => *prfs
                .entry((*op, values[*a], values[*b]))
                .or_insert_with(|| rng.gen::<U256>() % M),
            Node::UnoOp(op, a) => *prfs_uno
                .entry((*op, values[*a]))
                .or_insert_with(|| rng.gen::<U256>() % M),
            Node::TresOp(op, a, b, c) => *prfs_tres
                .entry((*op, values[*a], values[*b], values[*c]))
                .or_insert_with(|| rng.gen::<U256>() % M),
        };
        values.push(value);
    }
    values
}

/// Value numbering
pub fn value_numbering(nodes: &mut Vec<Node>, outputs: &mut [usize]) {
    assert_valid(nodes);

    // Evaluate the graph in random field elements.
    let values = random_eval(nodes);

    // Find all nodes with the same value.
    let mut value_map = HashMap::new();
    for (i, &value) in values.iter().enumerate() {
        value_map.entry(value).or_insert_with(Vec::new).push(i);
    }

    // For nodes that are the same, pick the first index.
    let mut renumber = Vec::with_capacity(nodes.len());
    for value in values {
        renumber.push(value_map[&value][0]);
    }

    // Renumber references.
    for node in nodes.iter_mut() {
        if let Node::Op(_, a, b) = node {
            *a = renumber[*a];
            *b = renumber[*b];
        }
        if let Node::UnoOp(_, a) = node {
            *a = renumber[*a];
        }
        if let Node::TresOp(_, a, b, c) = node {
            *a = renumber[*a];
            *b = renumber[*b];
            *c = renumber[*c];
        }
    }
    for output in outputs.iter_mut() {
        *output = renumber[*output];
    }

    eprintln!("Global value numbering applied");
}

/// Probabilistic constant determination
pub fn constants(nodes: &mut Vec<Node>) {
    assert_valid(nodes);

    // Evaluate the graph in random field elements.
    let values_a = random_eval(nodes);
    let values_b = random_eval(nodes);

    // Find all nodes with the same value.
    let mut constants = 0;
    for i in 0..nodes.len() {
        if let Node::Constant(_) = nodes[i] {
            continue;
        }
        if values_a[i] == values_b[i] {
            nodes[i] = Node::Constant(values_a[i]);
            constants += 1;
        }
    }
    eprintln!("Found {} constants", constants);
}

/// Convert to Montgomery form
pub fn montgomery_form(nodes: &mut [Node]) {
    for node in nodes.iter_mut() {
        use Node::*;
        use Operation::*;
        match node {
            Constant(c) => *node = MontConstant(Fr::new((*c).into())),
            MontConstant(..) => (),
            Input(..) => (),
            Op(
                Mul | Div | Add | Sub | Idiv | Mod | Eq | Neq | Lt | Gt | Leq | Geq | Land | Lor
                | Shl | Shr | Bor | Band | Bxor,
                ..,
            ) => (),
            Op(op @ Pow, ..) => unimplemented!("Operators Montgomery form: {:?}", op),
            UnoOp(UnoOperation::Neg, ..) => (),
            UnoOp(op, ..) => unimplemented!("Uno Operators Montgomery form: {:?}", op),
            TresOp(TresOperation::TernCond, ..) => (),
        }
    }
    eprintln!("Converted to Montgomery form");
}

fn shl(a: Fr, b: Fr) -> Fr {
    if b.is_zero() {
        return a;
    }

    if b.cmp(&Fr::from(Fr::MODULUS_BIT_SIZE)).is_ge() {
        return Fr::zero();
    }

    let n = b.into_bigint().0[0] as u32;

    let mut a = a.into_bigint();
    a.muln(n);
    return Fr::from_bigint(a).unwrap();
}

fn shr(a: Fr, b: Fr) -> Fr {
    if b.is_zero() {
        return a;
    }

    match b.cmp(&Fr::from(254u64)) {
        Ordering::Equal => return Fr::zero(),
        Ordering::Greater => return Fr::zero(),
        _ => (),
    };

    let mut n = b.into_bigint().to_bytes_le()[0];
    let mut result = a.into_bigint();
    let c = result.as_mut();
    while n >= 64 {
        for i in 0..3 {
            c[i as usize] = c[(i + 1) as usize];
        }
        c[3] = 0;
        n -= 64;
    }

    if n == 0 {
        return Fr::from_bigint(result).unwrap();
    }

    let mask: u64 = (1 << n) - 1;
    let mut carrier: u64 = c[3] & mask;
    c[3] >>= n;
    for i in (0..3).rev() {
        let new_carrier = c[i] & mask;
        c[i] = (c[i] >> n) | (carrier << (64 - n));
        carrier = new_carrier;
    }
    Fr::from_bigint(result).unwrap()
}

fn bit_and(a: Fr, b: Fr) -> Fr {
    let a = a.into_bigint();
    let b = b.into_bigint();
    let mut c: [u64; 4] = [0; 4];
    for i in 0..4 {
        c[i] = a.0[i] & b.0[i];
    }
    let mut d: BigInt<4> = BigInt::new(c);
    if d > Fr::MODULUS {
        d.sub_with_borrow(&Fr::MODULUS);
    }

    Fr::from_bigint(d).unwrap()
}

fn bit_or(a: Fr, b: Fr) -> Fr {
    let a = a.into_bigint();
    let b = b.into_bigint();
    let mut c: [u64; 4] = [0; 4];
    for i in 0..4 {
        c[i] = a.0[i] | b.0[i];
    }
    let mut d: BigInt<4> = BigInt::new(c);
    if d > Fr::MODULUS {
        d.sub_with_borrow(&Fr::MODULUS);
    }

    Fr::from_bigint(d).unwrap()
}

fn bit_xor(a: Fr, b: Fr) -> Fr {
    let a = a.into_bigint();
    let b = b.into_bigint();
    let mut c: [u64; 4] = [0; 4];
    for i in 0..4 {
        c[i] = a.0[i] ^ b.0[i];
    }
    let mut d: BigInt<4> = BigInt::new(c);
    if d > Fr::MODULUS {
        d.sub_with_borrow(&Fr::MODULUS);
    }

    Fr::from_bigint(d).unwrap()
}

// M / 2
const halfM: U256 =
    uint!(10944121435919637611123202872628637544274182200208017171849102093287904247808_U256);

fn u_gte(a: &U256, b: &U256) -> U256 {
    let a_neg = &halfM < a;
    let b_neg = &halfM < b;

    match (a_neg, b_neg) {
        (false, false) => U256::from(a >= b),
        (true, false) => uint!(0_U256),
        (false, true) => uint!(1_U256),
        (true, true) => U256::from(a >= b),
    }
}

fn u_lte(a: &U256, b: &U256) -> U256 {
    let a_neg = &halfM < a;
    let b_neg = &halfM < b;

    match (a_neg, b_neg) {
        (false, false) => U256::from(a <= b),
        (true, false) => uint!(1_U256),
        (false, true) => uint!(0_U256),
        (true, true) => U256::from(a <= b),
    }
}

fn u_gt(a: &U256, b: &U256) -> U256 {
    let a_neg = &halfM < a;
    let b_neg = &halfM < b;

    match (a_neg, b_neg) {
        (false, false) => U256::from(a > b),
        (true, false) => uint!(0_U256),
        (false, true) => uint!(1_U256),
        (true, true) => U256::from(a > b),
    }
}

fn u_lt(a: &U256, b: &U256) -> U256 {
    let a_neg = &halfM < a;
    let b_neg = &halfM < b;

    match (a_neg, b_neg) {
        (false, false) => U256::from(a < b),
        (true, false) => uint!(1_U256),
        (false, true) => uint!(0_U256),
        (true, true) => U256::from(a < b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ruint::uint;
    use std::ops::Div;
    use std::str::FromStr;

    #[test]
    fn test_ok() {
        let a = Fr::from(4u64);
        let b = Fr::from(2u64);
        let c = shl(a, b);
        assert_eq!(c.cmp(&Fr::from(16u64)), Ordering::Equal)
    }

    #[test]
    fn test_div() {
        assert_eq!(
            Operation::Div.eval_fr(Fr::from(2u64), Fr::from(3u64)),
            Fr::from_str(
                "7296080957279758407415468581752425029516121466805344781232734728858602831873"
            )
            .unwrap()
        );

        assert_eq!(
            Operation::Div.eval_fr(Fr::from(6u64), Fr::from(2u64)),
            Fr::from_str("3").unwrap()
        );

        assert_eq!(
            Operation::Div.eval_fr(Fr::from(7u64), Fr::from(2u64)),
            Fr::from_str(
                "10944121435919637611123202872628637544274182200208017171849102093287904247812"
            )
            .unwrap()
        );
    }

    #[test]
    fn test_idiv() {
        assert_eq!(
            Operation::Idiv.eval_fr(Fr::from(2u64), Fr::from(3u64)),
            Fr::from_str("0").unwrap()
        );

        assert_eq!(
            Operation::Idiv.eval_fr(Fr::from(6u64), Fr::from(2u64)),
            Fr::from_str("3").unwrap()
        );

        assert_eq!(
            Operation::Idiv.eval_fr(Fr::from(7u64), Fr::from(2u64)),
            Fr::from_str("3").unwrap()
        );
    }

    #[test]
    fn test_fr_mod() {
        assert_eq!(
            Operation::Mod.eval_fr(Fr::from(7u64), Fr::from(2u64)),
            Fr::from_str("1").unwrap()
        );

        assert_eq!(
            Operation::Mod.eval_fr(Fr::from(7u64), Fr::from(9u64)),
            Fr::from_str("7").unwrap()
        );
    }

    #[test]
    fn test_greater_then_module() {
        // println!("{}", Fr::MODULUS);
        // let f = Fr::from_str("21888242871839275222246405745257275088548364400416034343698204186575808495619").unwrap();
        // println!("[2] {}", f);
        // let mut i = f.into_bigint();
        // println!("[3] {}", i);
        // let j = i.add_with_carry(&Fr::MODULUS);
        // println!("[4] {}", i);
        // println!("[5] {}", j);
        // if i.cmp(&Fr::MODULUS).is_ge() {
        //     i.sub_with_borrow(&Fr::MODULUS);
        // }
        // let f2 = Fr::from_bigint(i).unwrap();
        // println!("[6] {}", f2);
        // let a= Fr::from(4u64);
        // let b= Fr::from(2u64);
        // let c = shl(a, b);
        // assert_eq!(c.cmp(&Fr::from(16u64)), Ordering::Equal)
    }

    #[test]
    fn test_u_gte() {
        let result = u_gte(&uint!(10_U256), &uint!(3_U256));
        assert_eq!(result, uint!(1_U256));

        let result = u_gte(&uint!(3_U256), &uint!(3_U256));
        assert_eq!(result, uint!(1_U256));

        let result = u_gte(&uint!(2_U256), &uint!(3_U256));
        assert_eq!(result, uint!(0_U256));

        // -1 >= 3 => 0
        let result = u_gte(
            &uint!(
                21888242871839275222246405745257275088548364400416034343698204186575808495616_U256
            ),
            &uint!(3_U256),
        );
        assert_eq!(result, uint!(0_U256));

        // -1 >= -2 => 1
        let result = u_gte(
            &uint!(
                21888242871839275222246405745257275088548364400416034343698204186575808495616_U256
            ),
            &uint!(
                21888242871839275222246405745257275088548364400416034343698204186575808495615_U256
            ),
        );
        assert_eq!(result, uint!(1_U256));

        // -2 >= -1 => 0
        let result = u_gte(
            &uint!(
                21888242871839275222246405745257275088548364400416034343698204186575808495615_U256
            ),
            &uint!(
                21888242871839275222246405745257275088548364400416034343698204186575808495616_U256
            ),
        );
        assert_eq!(result, uint!(0_U256));

        // -2 == -2 => 1
        let result = u_gte(
            &uint!(
                21888242871839275222246405745257275088548364400416034343698204186575808495615_U256
            ),
            &uint!(
                21888242871839275222246405745257275088548364400416034343698204186575808495615_U256
            ),
        );
        assert_eq!(result, uint!(1_U256));
    }

    #[test]
    fn test_x() {
        let x = M.div(uint!(2_U256));

        println!("x: {:?}", x.as_limbs());
        println!("x: {}", M);
    }

    #[test]
    fn test_2() {
        let nodes: Vec<Node> = vec![];
        // let node = nodes[0];
        let node = nodes.get(0);
        println!("{:?}", node);
    }
}
