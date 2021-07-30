use reqrnpdno::cliente;
use reqrnpdno::parameters;
use reqrnpdno::extractora;
use parameters::Parametros;

use std::error::Error;

fn main () {
    run().unwrap();
}

fn run() -> Result<(), Box<dyn Error>> {

    let cliente = cliente::cliente_nuevo()?;

    let mut  parametros = Parametros::new();
    // parametros.id_estado = "9".to_string();
    // parametros.id_municipio = "2".to_string();

    let salida = extractora::completa(&cliente, &parametros);
    println!("{:?}",salida);

    salida.exportar("./datos_procesados/test.json")?;

    Ok(())
}
