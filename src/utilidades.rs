//! Módulo con funciones utilitarias.
//! 
use std::fs;
use std::error::Error;

/// Función para crear un directorio en la ruta establecida
///
/// # Argumentos
///
/// * `ruta` - Ruta en donde se pretende crear el directorio
/// * `nombre` - Nombre del directorio a crear
pub fn crear_directorio(ruta: &str, nombre: &str) -> Result<String, Box<dyn Error>> {

    let mut ruta_salida = ruta.to_string();

    ruta_salida.push_str(nombre);
    match fs::read_dir(&ruta_salida) {
        Ok(_) => {},
        _ => {
            match fs::create_dir(&ruta_salida) {
                Ok(_) => {},
                Err(err) => return Err(From::from(format!("El directorio no se pudo crear - {}", err)))
            }
        }
    };

    Ok(ruta_salida.to_string())

}