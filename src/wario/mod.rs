//mod agent;
mod acmd;
mod status;
//mod frame;

pub fn install() {
    acmd::install();
    //agent::install();
    //frame::install();
    status::install();
}
