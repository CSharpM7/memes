/*
Chargeable Throws

Lets players charge throws in a similar manner to Smash Attacks, players will need to hold the GRAB button to start the charge. Power gets increased for the first 60 seconds (or whatever the param is set to), and there is a cap to the total length it can be held. Opponents can mash out during the charge time. This can be implemented for every normal throw EXCEPT those that use the Kirby throw status

How to make your own charge throw:
- In your character, add entries for "throw_f_hold","throw_b_hold",etc (don't forget to make the cancel frame 0. Looping is optional). 
- Make the throw anim, make the acmd for the hold anims, make a proper config, you know the drill. Same stuff you'd do for making smash attacks.
- In the acmd for the throw, call WorkModule::on_flag(agent.module_accessor, fighter::instance::flag::THROW_SMASH_HOLD) on the frame you want to check for a charge
- That's pretty much it...

Important files:
- common/status/throw.rs: the magic
- wario/acmd/throwb.rs: how to implement acmd for the throw and the charge
*/

pub mod vars;
mod wario;
mod common;

pub fn install() {
    wario::install();
    common::install();
}
