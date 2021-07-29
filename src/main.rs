use reqrnpdno::cliente;
use reqrnpdno::parameters;

use std::error::Error;

fn main () {
    run().unwrap();
}

fn run() -> Result<(), Box<dyn Error>> {

    let cliente = cliente::cliente_nuevo()?;

    let estados = parameters::get_estados(&cliente)?;
    println!("Estados: {:?}",estados);

    let municipios = parameters::get_municipios(&cliente, "1")?;
    println!("Municipios: {:?}",municipios);

    let colonias = parameters::get_colonias(&cliente, "1", "1")?;
    println!("Colonias: {:?}",colonias);

    let nacionalidades = parameters::get_nacionalidades(&cliente)?;
    println!("Nacionalidades: {:?}",nacionalidades);

    let hipotesis = parameters::get_hipotesis(&cliente)?;
    println!("Hipotesis: {:?}",hipotesis);

    let delitos = parameters::get_delitos(&cliente)?;
    println!("Delitos: {:?}",delitos);

    let medios = parameters::get_medios(&cliente)?;
    println!("Medios de conocimiento: {:?}",medios);

    let circunstancias = parameters::get_circunstancias(&cliente)?;
    println!("Circunstancias: {:?}",circunstancias);

    let discapacidades = parameters::get_discapacidades(&cliente)?;
    println!("Discapacidades: {:?}",discapacidades);

    let etnias = parameters::get_etnias(&cliente)?;
    println!("Etnias: {:?}",etnias);

    let lenguas = parameters::get_lenguas(&cliente)?;
    println!("Lenguas: {:?}",lenguas);

    let religiones = parameters::get_religiones(&cliente)?;
    println!("Religiones: {:?}",religiones);

    let emigratorios = parameters::get_emigratorios(&cliente)?;
    println!("Estatus migratorios: {:?}",emigratorios);

    Ok(())
}
