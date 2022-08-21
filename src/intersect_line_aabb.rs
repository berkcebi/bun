use bevy::prelude::Vec2;

pub fn is_intersecting(
    line_start: Vec2,
    line_end: Vec2,
    aabb_position: Vec2,
    aabb_size: Vec2,
) -> bool {
    let aabb_min_position = aabb_position - aabb_size / 2.0;
    let aabb_max_position = aabb_position + aabb_size / 2.0;

    let is_inside = |position: Vec2| {
        position.x >= aabb_min_position.x
            && position.y >= aabb_min_position.y
            && position.x <= aabb_max_position.x
            && position.y <= aabb_max_position.y
    };

    if is_inside(line_start) || is_inside(line_end) {
        return true;
    }

    let unit_vector = Vec2::splat(1.0) / (line_end - line_start).normalize();
    let aabb_min_vector = (aabb_min_position - line_start) * unit_vector;
    let aabb_max_vector = (aabb_max_position - line_start) * unit_vector;

    let t_min =
        (aabb_min_vector.x.min(aabb_max_vector.x)).max(aabb_min_vector.y.min(aabb_max_vector.y));
    let t_max =
        (aabb_min_vector.x.max(aabb_max_vector.x)).min(aabb_min_vector.y.max(aabb_max_vector.y));

    if t_max < 0.0 || t_min > t_max {
        return false;
    }

    let t = if t_min < 0.0 { t_max } else { t_min };

    t > 0.0 && t.powi(2) < (line_start - line_end).length_squared()
}

#[test]
fn test_is_intersecting() {
    assert!(is_intersecting(
        Vec2::new(1.0, 0.0),
        Vec2::new(4.0, 4.0),
        Vec2::new(2.0, 2.0),
        Vec2::new(2.0, 2.0),
    ));

    assert!(is_intersecting(
        Vec2::new(3.0, 0.0),
        Vec2::new(0.0, 4.0),
        Vec2::new(2.0, 2.0),
        Vec2::new(2.0, 2.0),
    ));

    assert!(is_intersecting(
        Vec2::new(3.0, 4.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(2.0, 2.0),
        Vec2::new(2.0, 2.0),
    ));

    assert!(is_intersecting(
        Vec2::new(1.0, 4.0),
        Vec2::new(3.0, 0.0),
        Vec2::new(2.0, 2.0),
        Vec2::new(2.0, 2.0),
    ));
}

#[test]
fn test_is_not_intersecting() {
    assert!(!is_intersecting(
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 4.0),
        Vec2::new(2.0, 2.0),
        Vec2::new(2.0, 2.0),
    ));

    assert!(!is_intersecting(
        Vec2::new(4.0, 0.0),
        Vec2::new(3.0, 4.0),
        Vec2::new(2.0, 2.0),
        Vec2::new(2.0, 2.0),
    ));

    assert!(!is_intersecting(
        Vec2::new(4.0, 4.0),
        Vec2::new(0.0, 4.0),
        Vec2::new(2.0, 2.0),
        Vec2::new(2.0, 2.0),
    ));

    assert!(!is_intersecting(
        Vec2::new(4.0, 4.0),
        Vec2::new(0.0, 4.0),
        Vec2::new(2.0, 2.0),
        Vec2::new(2.0, 2.0),
    ));
}
