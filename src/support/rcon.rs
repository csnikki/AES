use std::fmt::Write;
use std::fs;

pub fn gen_rcon() {
    let mut buffer = String::new();
    let mut rcon = [0u8; 10];

    for i in 0..10 {
        match i > 0 {
            true => {
                let last = rcon[i - 1];
                if (last & 0x80) != 0 {
                    rcon[i] = (((last as u16) << 1) ^ 0x11B) as _;
                } else {
                    rcon[i] = last << 1;
                }
            }
            false => rcon[i] = 0x01,
        }
    }

    write!(&mut buffer, "{:?}", rcon).unwrap();
    fs::create_dir_all("tables").unwrap();
    fs::write("tables/rcon.tbl", buffer).unwrap();
}
