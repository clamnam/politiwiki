pub fn role_augment(current_role:f32,positive: bool)->f32{
    let mut mult = 0.9;
    if positive == true{
        mult = 1.1
    }
    let updated_role = current_role *mult;
    return updated_role
}