pub fn role_augment(current_role: f32, positive: bool) -> f32 {
    if current_role < 0.7 {
        if positive {
            return current_role * 1.1;
        }
        return current_role * 0.9;
    }

    augment(current_role, positive)
}

fn augment(x: f32, positive: bool) -> f32 {
    let e = std::f32::consts::E;
    let delta = e.powf(-x) / (e.powf(-x) + 1.0) / 14.0;
    if positive {
        x + delta
    } else {
        x - delta
    }
}
