const ACCELERATION_GRAVITY_EARTH: f32 = 9.80664; // m s^-2

pub fn force_from_gravity(mass: f32 /* kg */) -> f32 {
    mass * ACCELERATION_GRAVITY_EARTH
}


#[cfg(test)]
mod tests {
    // #[test]
    // fn it_works() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }
}
