use reqrnpdno::cliente;
use reqrnpdno::parameters;
use reqrnpdno::extractora;
use parameters::Parametros;

use std::error::Error;

fn main () {
    run().unwrap();
}

fn run() -> Result<(), Box<dyn Error>> {

    let mut cliente = cliente::cliente_nuevo()?;

    let estados = parameters::get_estados(&cliente)?;
    println!("Estados: {:?}",estados);

    let municipios = parameters::get_municipios(&cliente, "9")?;
    println!("Municipios: {:?}",municipios);

    let mut  parametros = Parametros::new();
    parametros.id_estado = "9".to_string();
    parametros.id_municipio = "2".to_string();

    let totales = match extractora::totales(&cliente, &parametros) {
        Ok(datos) => datos,
        Err(_) => {
            println!("Es necesario un nuevo cliente");
            cliente = cliente::cliente_nuevo()?;
            extractora::totales(&cliente, &parametros).unwrap()
        }
    };
    println!("Totales: {:?}",totales);

    let colonias = extractora::por_colonias_completo(&cliente, &parametros)?;
    println!("Colonias: {:?}",colonias.to_map());

    Ok(())
}
