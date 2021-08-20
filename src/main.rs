use reqrnpdno::parameters;
use reqrnpdno::extractora;
use reqrnpdno::utilidades;
use parameters::Parametros;

use std::error::Error;

fn main () {
    run().unwrap();
}

fn run() -> Result<(), Box<dyn Error>> {

    let ruta_salida = utilidades::crear_directorio("./datos_procesados/", "test")?;

    // Obtener los diccionarios, el único parámetro necesario es la ruta al directorio donde se guardarán
    // extractora::get_diccionarios(&ruta_salida)?;

    let parametros = Parametros::new();

    extractora::extraer_nacional(&parametros, &ruta_salida)?;

    Ok(())
}
