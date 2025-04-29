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
        let base_delta = e.powf(-x) / (e.powf(-x) + 1.0) / 1.0;
        let scaled_delta = (1.0 - x) * base_delta; 
        if positive {
            x + scaled_delta
        } else {
            x - scaled_delta
        }
    }
