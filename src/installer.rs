use crate::*;

#[smashline::installer]
pub fn installer() {
    install();
}

pub fn install() {
    //buddy::install();
    //pickel::install();
    common::install();
    wario::install();
}
