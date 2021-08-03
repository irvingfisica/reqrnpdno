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

    //let espacial = parameters::get_all_espacial(&cliente)?;
    //parameters::exportar_espacial(&espacial, "./datos_procesados/espacial.json")?;

    // let estados = parameters::get_estados(&cliente)?;
    //parameters::exportar_categorias(&estados, "./datos_procesados/estados.json")?;

    // let nacionalidades = parameters::get_nacionalidades(&cliente)?;
    // parameters::exportar_categorias(&nacionalidades, "./datos_procesados/nacionalidades.json")?;

    // let hipotesis = parameters::get_hipotesis(&cliente)?;
    // parameters::exportar_categorias(&hipotesis, "./datos_procesados/hipotesis.json")?;

    // let delitos = parameters::get_delitos(&cliente)?;
    // parameters::exportar_categorias(&delitos, "./datos_procesados/delitos.json")?;

    // let medios = parameters::get_medios(&cliente)?;
    // parameters::exportar_categorias(&medios, "./datos_procesados/medios.json")?;

    // let circunstancias = parameters::get_circunstancias(&cliente)?;
    // parameters::exportar_categorias(&circunstancias, "./datos_procesados/circunstancias.json")?;

    // let discapacidades = parameters::get_discapacidades(&cliente)?;
    // parameters::exportar_categorias(&discapacidades, "./datos_procesados/discapapcidades.json")?;

    // let etnias = parameters::get_etnias(&cliente)?;
    // parameters::exportar_categorias(&etnias, "./datos_procesados/etnias.json")?;

    // let lenguas = parameters::get_lenguas(&cliente)?;
    // parameters::exportar_categorias(&lenguas, "./datos_procesados/lenguas.json")?;

    // let religiones = parameters::get_religiones(&cliente)?;
    // parameters::exportar_categorias(&religiones, "./datos_procesados/religiones.json")?;

    // let emigratorios = parameters::get_emigratorios(&cliente)?;
    // parameters::exportar_categorias(&emigratorios, "./datos_procesados/emigratorios.json")?;
    
    // println!("{:?}",emigratorios);

    // let mut parametros = Parametros::new();

    // for (llave,dato) in estados {

    //     match dato.as_str() {
    //         "0" => {},
    //         _ => {
    //             parametros.id_estado = dato.to_string();

    //             let municipios = parameters::get_municipios(&cliente, &dato)?;

    //             for (mllave, mdato) in municipios {
    //                 println!("{},{}: {},{}",mdato,dato,llave,mllave);
    //                 cliente = cliente::cliente_nuevo()?;
    //                 parametros.id_municipio = mdato.to_string();
    //                 let salida = extractora::completa(&cliente, &parametros);

    //                 let mut ruta = "./datos_procesados/municipios/".to_string();
    //                 ruta.push_str(&dato);
    //                 ruta.push_str("_");
    //                 ruta.push_str(&mdato);
    //                 ruta.push_str(".json");

    //                 salida.exportar(&ruta)?;
    //             }
    //         }
    //     }
    // };

    // let categoria = "sindicalista";
    // parametros.es_sindicalista = "true".to_string();

    // let salida = extractora::completa(&cliente, &parametros);
    // let mut ruta = "./datos_procesados/por_categoria/".to_string();
    // ruta.push_str(categoria);
    // ruta.push_str(".json");
    // salida.exportar(&ruta)?;

    // salida.exportar(",/datos_procesados/test.json")?;

    Ok(())
}
