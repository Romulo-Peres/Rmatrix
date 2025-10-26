use clap::{error, Parser};

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Customizable matrix for terminal interfaces")]
pub struct Args {
    #[arg(long="edge-color", short='e', num_args=3, default_values_t=[255, 255, 255])]
    pub edge_color: Vec<u8>,

    #[arg(long="body-color", short='b', num_args=3, default_values_t=[0, 255, 0])]
    pub body_color: Vec<u8>,

    #[arg(long = "render-speed", short = 'r', default_value_t = 30)]
    pub render_speed: u64,

    #[arg(long = "string-interval", short = 'n', default_value_t = 20)]
    pub string_interval: u64,

    #[arg(long = "rainbow-mode", short = 'a', default_value_t = false)]
    pub raindow_mode: bool,

    #[arg(long = "minimum-stream-delay", short = 'm', default_value_t = 20)]
    pub minimum_stream_delay: u16,

    #[arg(long = "maximum-stream-delay", short = 'M', default_value_t = 90)]
    pub maximum_stream_delay: u16
}

impl Args {
    pub fn validate(&self) {
        if self.minimum_stream_delay >= self.maximum_stream_delay {
            clap::error::Error::<error::DefaultFormatter>::raw(
                clap::error::ErrorKind::InvalidValue,
                "--maximum-stream-delay (-M) must be greater than --minimum-stream-delay (-m)\n",
            )
            .exit();
        }
    }
}

macro_rules! vec_to_array {
    ($vector:expr) => {
        [$vector[0], $vector[1], $vector[2]]
    };
}

pub(crate) use vec_to_array;
