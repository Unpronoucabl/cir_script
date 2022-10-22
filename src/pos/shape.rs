use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[derive(Serialize,Deserialize)]
pub enum PrimType {
    FullCir,
    CirArc,
    Line,
    Dot,
}

#[derive(Clone, Debug)]
#[derive(Serialize,Deserialize)]
pub struct Primitive{
    id: String,
    prim_type: PrimType,
    inner_bound: bool,
    radius: f64,
    width: f64,
}