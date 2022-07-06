#[path = "src/support/mod.rs"]
mod support;

fn main() {
    support::gen_rcon();
    support::gen_sbox(false);
    support::gen_sbox(true);
}
