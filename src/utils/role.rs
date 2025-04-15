pub async fn role_augment(current_role:f32,polarity: bool){
    let mult = -0.1;
    if polarity = 1{ mult = 0.1}
    current_role += current_role *mult;
    return current_role
}