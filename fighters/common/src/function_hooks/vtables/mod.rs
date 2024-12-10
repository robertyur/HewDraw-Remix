use super::*;

mod donkey;
mod gamewatch;
mod ganon;
mod lucario;
mod pickel;
mod ptrainer;
mod littlemac;
mod gekkouga;
mod reflet;
mod rockman;
mod krool;
mod brave;

mod murabito_clayrocket;
mod ryu_shinkuhadoken;

mod weapon;

pub fn install() {
    donkey::install();
    gamewatch::install();
    ganon::install();
    lucario::install();
    pickel::install();
    ptrainer::install();
    littlemac::install();
    gekkouga::install();
    reflet::install();
    rockman::install();
    krool::install();
    brave::install();

    murabito_clayrocket::install();
    ryu_shinkuhadoken::install();

    weapon::install();
}