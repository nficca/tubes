use std::sync::mpsc;

pub enum Tube<T> {
    Inactive,
    Active(Vec<mpsc::Sender<T>>)
}
