use super::*;

mod acmd;
mod status;
mod opff;

pub fn install() {
    let agent = &mut Agent::new("murabito_clayrocket");
    acmd::install(agent);
    status::install(agent);
    opff::install(agent);
    agent.install();
}