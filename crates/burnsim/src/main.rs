#![allow(dead_code)]
use gas::gas::*;
use gas::gasmixture::*;
use macros::make_answer;
mod turf;

make_answer!();

fn main() {

    let sleeping_agent = Gas::new()
        .name("Sleeping Agent".into())
        .heat(40.0);
    let rad_particles = Gas::new()
        .name("Rad Particles".into())
        .heat(20.0);

    let mut sleep_mix = GasMixture::new().gas(&sleeping_agent);

    sleep_mix = sleep_mix.add_trace_gas_by_name("Test".into());
    sleep_mix = sleep_mix.add_trace_gas_by_name("Test".into());

    println!("{}", answer());

    println!("{:?}\n", sleeping_agent);
    println!("{:?}\n", rad_particles);
    println!("{:#?}\n", sleep_mix);
}
