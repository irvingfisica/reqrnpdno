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

    // let estados = parameters::get_estados(&cliente)?;
    // parameters::exportar_categorias(&estados, "./datos_procesados/estados.json")?;

    // let nacionalidades = parameters::get_nacionalidades(&cliente)?;
    // parameters::exportar_categorias(&nacionalidades, "./datos_procesados/nacionalidades.json")?;

    // let hipotesis = parameters::get_hipotesis(&cliente)?;
    // parameters::exportar_categorias(&hipotesis, "./datos_procesados/hipotesis.json")?;

    // let delitos = parameters::get_delitos(&cliente)?;
    // parameters::exportar_categorias(&delitos, "./datos_procesados/estados.json")?;

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

    let mut  parametros = Parametros::new();
    // parametros.id_estado = "9".to_string();
    // parametros.id_municipio = "2".to_string();
    parametros.id_circunstancia = "13".to_string();

    let salida = extractora::completa(&cliente, &parametros);
    println!("{:?}",salida);

    salida.exportar("./datos_procesados/circ_13.json")?;

    Ok(())
}
