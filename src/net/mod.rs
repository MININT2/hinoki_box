mod wol;
mod upnp;

use rand::{
    self,
    Rng,
};

pub fn get_random_port() -> u16 {
    const LOWERBOUNDS: u16 = 32768;
    const UPPERBOUNDS: u16 = 65535;
    return rand::thread_rng().gen_range(LOWERBOUNDS..UPPERBOUNDS)
}