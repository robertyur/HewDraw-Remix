use super::*;

mod acmd;
//mod opff;

pub fn install() {
    let agent = &mut Agent::new("packun_firebreath");
    acmd::install(agent);
    //opff::install(agent);
    agent.install();
}