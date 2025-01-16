use rand::Rng;

pub fn generate_random_color() -> [u8;3]{
    let mut rgb : [u8;3] = [0, 0, 0];

    rgb[0] = rand::thread_rng().gen_range(1..255);
    rgb[1] = rand::thread_rng().gen_range(1..255);
    rgb[2] = rand::thread_rng().gen_range(1..255);

    return rgb;
}
