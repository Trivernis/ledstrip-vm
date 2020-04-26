pub const RCS: u8 = 0x01;
pub const RCR: u8 = 0x02;
pub const RCG: u8 = 0x03;
pub const RCB: u8 = 0x04;
pub const RGD: u8 = 0x05;
pub const RGP: u8 = 0x06;
pub const RGI: u8 = 0x07;
pub const RGO: u8 = 0x08;
pub const RGL: u8 = 0x09;

/// Maps a register name to the bytecode value
pub fn get_register_by_name(name: &str) -> Option<u8> {
    let item = [
        ("rcs", RCS),
        ("rcr", RCR),
        ("rcg", RCG),
        ("rcb", RCB),
        ("rgd", RGD),
        ("rgp", RGP),
        ("rgi", RGI),
        ("rgo", RGO),
        ("rgl", RGL),
    ]
    .iter()
    .find(|(reg, _)| *reg == name);

    if let Some(item) = item {
        Some((*item).1)
    } else {
        println!("Unknown register: {}", name);
        None
    }
}
