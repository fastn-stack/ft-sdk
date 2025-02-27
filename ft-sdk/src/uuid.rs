// Reference - https://github.com/tmtmtoo/uuid-v4-wasm/blob/develop/src/uuid.rs
// License - MIT (https://github.com/tmtmtoo/uuid-v4-wasm/blob/develop/LICENSE)

enum UuidElements {
    Random09AF,
    Random89AB,
    Hyphen,
    Version,
}

const UUID_V4_FORMAT: [UuidElements; 36] = [
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Hyphen,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Hyphen,
    UuidElements::Version,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Hyphen,
    UuidElements::Random89AB,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Hyphen,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
    UuidElements::Random09AF,
];

const ERROR_MAKE_CHAR: &str = "Error in making char";

fn make_bytes(value: f64) -> [u8; 16] {
    let bytes = value.to_bits();

    let b1: u8 = ((bytes >> 56) & 0xff) as u8;
    let b2: u8 = ((bytes >> 48) & 0xff) as u8;
    let b3: u8 = ((bytes >> 40) & 0xff) as u8;
    let b4: u8 = ((bytes >> 36) & 0xff) as u8;
    let b5: u8 = ((bytes >> 24) & 0xff) as u8;
    let b6: u8 = ((bytes >> 16) & 0xff) as u8;
    let b7: u8 = ((bytes >> 8) & 0xff) as u8;
    let b8: u8 = (bytes & 0xff) as u8;

    [
        b8, b7, b6, b5, b4, b3, b2, b1, b1, b2, b3, b4, b5, b6, b7, b8,
    ]
}

/// generate UUID with XorShift algorithm
pub fn uuid() -> String {
    use rand::{Rng, SeedableRng};

    let seed = ft_sdk::env::random();

    let bytes = make_bytes(seed);
    let mut rng = rand::XorShiftRng::from_seed(bytes);

    // prevent duplication
    rng.gen_range(0., 1.);

    UUID_V4_FORMAT
        .into_iter()
        .map(|n| match n {
            UuidElements::Random09AF => {
                let random = rng.gen_range(0., 1.);
                char::from_digit((random * 16.) as u32, 16).expect(ERROR_MAKE_CHAR)
            }
            UuidElements::Random89AB => {
                let random = rng.gen_range(0., 1.);
                char::from_digit((random * 4.) as u32 + 8, 16).expect(ERROR_MAKE_CHAR)
            }
            UuidElements::Version => '4',
            UuidElements::Hyphen => '-',
        })
        .collect()
}

/// Generate UUID without dashes, this is useful for URLs, as one can click Alt-Backspace and
/// delete the whole UUID. In the of regular UUID, it will delete only one part of it, and one
/// has to press backspace multiple times to delete the whole UUID, which is annoying.
pub fn uuid_without_dashes() -> String {
    uuid().replace("-", "")
}
