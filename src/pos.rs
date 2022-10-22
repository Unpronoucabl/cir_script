use serde::{Deserialize, Serialize};
mod shape;

#[derive(Clone, Debug)]
#[derive(Serialize,Deserialize)]
pub struct Data{
    ang: Option<f64>,
    dist: f64,
    x: f64,
    y: f64,
}

#[derive(Clone, Debug)]
#[derive(Serialize,Deserialize)]
pub struct Poi{
    id: String,
    data: Data,
    pub shapes: Vec<shape::Primitive>,
    pub slaves: Vec<Poi>,
}

impl Poi {

    pub fn new(id:String, ang:f64, dist:f64, master: &mut Poi) -> (){
        let slave = Poi
        {
            id: id,
            data: Data{
                ang:Some(ang),
                dist:dist,
                x:master.data.x + dist*ang.sin(),
                y:master.data.y - dist*ang.cos(),
            },
            shapes:Vec::new(),
            slaves:Vec::new(),
        };
        master.slaves.push(slave)
    }

    pub fn origin() -> Poi{
        Poi{
            id: String::from("origin"),
            data: Data{
                ang:None,
                dist:0.0,
                x:0.0,
                y:0.0,
            },
            shapes:Vec::new(),
            slaves:Vec::new(),
        }
    }
}