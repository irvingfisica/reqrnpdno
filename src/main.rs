use reqrnpdno::parameters;
use reqrnpdno::extractora;
use reqrnpdno::utilidades;
use parameters::Parametros;

use std::error::Error;

fn main () {
    run().unwrap();
}

fn run() -> Result<(), Box<dyn Error>> {

    let ruta_salida = utilidades::crear_directorio("./datos_procesados/", "dc")?;

    // Obtener los diccionarios
    // extractora::get_diccionarios(&ruta_salida)?;

    let mut parametros = Parametros::new();
    parametros.id_estatus_victima = "7".to_string();
    parametros.titulo = "PERSONAS DESAPARECIDAS Y NO LOCALIZADAS".to_string();
    parametros.fecha_inicio = "2000-01-01".to_string();
    parametros.fecha_fin = "2021-08-20".to_string();

    extractora::extraer_nacional(&parametros, &ruta_salida)?;
    extractora::extraer_por_municipios(&parametros, &ruta_salida)?;

    Ok(())
}
