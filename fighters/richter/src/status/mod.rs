use super::*;
use globals::*;

mod attacks3;
mod special_n;
mod special_s;

pub fn install(agent: &mut Agent) {
    attacks3::install(agent);
    special_n::install(agent);
    special_s::install(agent);
}