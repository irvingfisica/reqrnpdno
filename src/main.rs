use reqrnpdno::{extractora,parameters::Parametros};
//use std::error::Error;

fn main () {

    let rutam = "./datos.json".to_string();

    let mut parametros = Parametros::new();

    parametros.id_estatus_victima = "7".to_string();
    parametros.titulo = "PERSONAS DESAPARECIDAS Y NO LOCALIZADAS".to_string();
    parametros.fecha_inicio = "2000-01-01".to_string();
    parametros.fecha_fin = "2021-08-20".to_string();

    extractora::extraer(&parametros, &rutam).unwrap();

    //run().unwrap();
}

// fn run() -> Result<(), Box<dyn Error>> {

//     //let ruta_salida = utilidades::crear_directorio("./datos_procesados/", "dc2")?;

//     let rutam = "./datos.json".to_string();

//     // Obtener los diccionarios
//     // extractora::get_diccionarios(&ruta_salida)?;

//     let mut parametros = Parametros::new();

//     parametros.id_estatus_victima = "7".to_string();
//     parametros.titulo = "PERSONAS DESAPARECIDAS Y NO LOCALIZADAS".to_string();
//     parametros.fecha_inicio = "2000-01-01".to_string();
//     parametros.fecha_fin = "2021-08-20".to_string();

//     // let mut rutam = ruta_salida.to_string();
//     // rutam.push_str("/");
//     // rutam.push_str("general.json");

//     extractora::extraer(&parametros, &rutam)?;
    
//     //extractora::extraer_por_municipios(&parametros, &ruta_salida)?;

//     Ok(())
// }
