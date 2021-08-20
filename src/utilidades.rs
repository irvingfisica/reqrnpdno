use std::fs;
use std::error::Error;

pub fn crear_directorio(ruta: &str, nombre: &str) -> Result<String, Box<dyn Error>> {

    let mut ruta_salida = ruta.to_string();

    ruta_salida.push_str(nombre);
    match fs::read_dir(&ruta_salida) {
        Ok(_) => {},
        _ => {
            fs::create_dir(&ruta_salida)?;
        }
    };

    Ok(ruta_salida.to_string())

}