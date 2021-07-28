use reqrnpdno::cliente;
use reqrnpdno::parameters;

use std::error::Error;

fn main () {
    run().unwrap();
}

fn run() -> Result<(), Box<dyn Error>> {

    let cliente = cliente::cliente_nuevo()?;

    let estados = parameters::get_estados(&cliente)?;
    println!("{:?}",estados);

    let municipios = parameters::get_municipios(&cliente, "20")?;
    println!("{:?}",municipios);

    Ok(())
}
