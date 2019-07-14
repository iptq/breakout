use nalgebra::Vector2;
use ordered_float::OrderedFloat;

use crate::level::Direction;

pub fn calculate_vector_direction(target: &Vector2<f32>) -> Direction {
    let compass = [
        (Direction::Up, Vector2::from([0.0, 1.0])),
        (Direction::Right, Vector2::from([1.0, 0.0])),
        (Direction::Down, Vector2::from([0.0, -1.0])),
        (Direction::Left, Vector2::from([-1.0, 0.0])),
    ];
    let normal = target.normalize();
    let (result, _) = compass
        .into_iter()
        .max_by_key(|(_, v)| OrderedFloat(normal.dot(v)))
        .unwrap();
    *result
}
