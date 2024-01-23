use flutter_rust_bridge::frb;

pub mod counter;

#[derive(Clone)]
#[frb(opaque)]
pub struct State<T>
where
    T: PartialEq + Clone,
{
    pub data: T,
}
