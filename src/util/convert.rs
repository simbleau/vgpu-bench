use crate::models::Measurable;

pub fn erase(y: impl Measurable + 'static) -> Box<dyn erased_serde::Serialize> {
    let x: Box<dyn erased_serde::Serialize> = { Box::new(y) };
    x
}

pub fn to_serializable<T>(vec: Vec<T>) -> Vec<Box<dyn erased_serde::Serialize>>
where
    T: serde::Serialize + 'static,
{
    vec.into_iter()
        .map(|x| -> Box<dyn erased_serde::Serialize> { Box::new(x) })
        .collect()
}
