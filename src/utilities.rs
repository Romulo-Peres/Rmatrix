use rand::Rng;

pub fn generate_stream_characters(terminal_columns: u16) -> String {
    let available_characters = "'/a0b!1c@2d#3e$%4f&3g*(6h)7i|_8j-+9k=[]\\1l,/2m3n;.4o5p~`6q7r8s9\"^t0u1v2w3x4y5z6";
    let mut characters = String::new();
    let mut rng = rand::thread_rng();
    let mut slice_index;
    let mut i = 0;

    while i <= terminal_columns + 5 {
        slice_index = rng.gen_range(1..=available_characters.len());
        characters.push_str(&available_characters[slice_index - 1..slice_index]);

        i += 1;
    }

    return characters;
}

pub fn shade_stream_colors(trail_color: &mut [u8; 3], head_color: &mut [u8; 3]) {
    for color in trail_color {
        *color = *color / 2;
    }

    for color in head_color {
        *color = *color / 2;
    }
}

pub fn generate_random_color() -> [u8;3]{
    let mut rgb : [u8;3] = [0, 0, 0];

    rgb[0] = rand::thread_rng().gen_range(1..255);
    rgb[1] = rand::thread_rng().gen_range(1..255);
    rgb[2] = rand::thread_rng().gen_range(1..255);

    return rgb;
}

pub fn stream_position(columns: u16) -> u16 {
    let mut rng = rand::thread_rng();

    return rng.gen_range(0..columns);
}

pub fn stream_length() -> u16 {
    let mut rng = rand::thread_rng();

    return rng.gen_range(5..=18);
}

pub fn fifty_percent() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..2) == 0
}