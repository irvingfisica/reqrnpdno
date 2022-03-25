//! Módulo encargado de extraer los datos de la API. Las funciones de alto nivel para extraer datos se encuentran aquí.
//! 
use std::error::Error;
use std::collections::BTreeMap;
use reqwest::blocking::Client;
use serde::{Deserialize,Serialize};
use scraper::{Html,Selector};
use crate::parameters::Parametros;
use crate::urls;
use crate::cliente;
use crate::parameters;
use std::fs::File;
use std::fs;
use std::io::Write;

/// Función de alto nivel para obtener los diccionarios correspondientes a todos los catálogos. 
///
/// # Argumentos
///
/// * `ruta` - Ruta en donde se pretenden guardar los diccionarios, si la carpeta "diccionarios" no existe en la ruta será creada
pub fn get_diccionarios(ruta: &str) -> Result<(), Box<dyn Error>> {

    let mut ruta_dir = ruta.to_string();
    ruta_dir.push_str("/diccionarios");
    match fs::read_dir(&ruta_dir) {
        Ok(_) => {},
        _ => {
            fs::create_dir(&ruta_dir)?;
        }
    };

    let mut ruta_evictimas = ruta_dir.to_string();
    ruta_evictimas.push_str("/estatus_victimas.json");
    let evictimas = parameters::get_estatus_victimas()?;
    parameters::exportar_categorias(&evictimas, &ruta_evictimas)?;

    let cliente = cliente::cliente_nuevo()?;

    let mut ruta_espacial = ruta_dir.to_string();
    ruta_espacial.push_str("/espacial.json");
    let espacial = parameters::get_all_espacial(&cliente)?;
    parameters::exportar_espacial(&espacial, &ruta_espacial)?;

    let mut ruta_estados = ruta_dir.to_string();
    ruta_estados.push_str("/estados.json");
    let estados = parameters::get_estados(&cliente)?;
    parameters::exportar_categorias(&estados, &ruta_estados)?;

    let mut ruta_nacionalidades = ruta_dir.to_string();
    ruta_nacionalidades.push_str("/nacionalidades.json");
    let nacionalidades = parameters::get_nacionalidades(&cliente)?;
    parameters::exportar_categorias(&nacionalidades, &ruta_nacionalidades)?;

    let mut ruta_hipotesis = ruta_dir.to_string();
    ruta_hipotesis.push_str("/hipotesis.json");
    let hipotesis = parameters::get_hipotesis(&cliente)?;
    parameters::exportar_categorias(&hipotesis, &ruta_hipotesis)?;

    let mut ruta_delitos = ruta_dir.to_string();
    ruta_delitos.push_str("/delitos.json");
    let delitos = parameters::get_delitos(&cliente)?;
    parameters::exportar_categorias(&delitos, &ruta_delitos)?;

    let mut ruta_medios = ruta_dir.to_string();
    ruta_medios.push_str("/medios.json");
    let medios = parameters::get_medios(&cliente)?;
    parameters::exportar_categorias(&medios, &ruta_medios)?;

    let mut ruta_circunstancias = ruta_dir.to_string();
    ruta_circunstancias.push_str("/circunstancias.json");
    let circunstancias = parameters::get_circunstancias(&cliente)?;
    parameters::exportar_categorias(&circunstancias, &ruta_circunstancias)?;

    let mut ruta_discapapcidades = ruta_dir.to_string();
    ruta_discapapcidades.push_str("/discapacidades.json");
    let discapacidades = parameters::get_discapacidades(&cliente)?;
    parameters::exportar_categorias(&discapacidades, &ruta_discapapcidades)?;

    let mut ruta_etnias = ruta_dir.to_string();
    ruta_etnias.push_str("/etnias.json");
    let etnias = parameters::get_etnias(&cliente)?;
    parameters::exportar_categorias(&etnias, &ruta_etnias)?;

    let mut ruta_lenguas = ruta_dir.to_string();
    ruta_lenguas.push_str("/lenguas.json");
    let lenguas = parameters::get_lenguas(&cliente)?;
    parameters::exportar_categorias(&lenguas, &ruta_lenguas)?;

    let mut ruta_religiones = ruta_dir.to_string();
    ruta_religiones.push_str("/religiones.json");
    let religiones = parameters::get_religiones(&cliente)?;
    parameters::exportar_categorias(&religiones, &ruta_religiones)?;

    let mut ruta_emigratorios = ruta_dir.to_string();
    ruta_emigratorios.push_str("/estatus_migratorios.json");
    let emigratorios = parameters::get_emigratorios(&cliente)?;
    parameters::exportar_categorias(&emigratorios, &ruta_emigratorios)?;

    Ok(())
}

/// Función de alto nivel para realizar una petición de datos. Es posible filtrar esta petición con otras variables modificando la estructura de parametros que alimenta a la función previamente. Esta función es la función principal con la cual se realizan peticiones simples. El módulo contiene otras funciones de alto nivel con las cuales es posible realizar iteraciones sobre los valores de una de las variables que conforman una búsqueda.
///
/// # Argumentos
///
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
/// * `ruta` - Ruta en donde se pretenden guardar los datos la cual incluya el nombre del archivo, la función creará un archivo JSON de salida llamado nacional.json
pub fn extraer(parametros: &Parametros, ruta: &str) -> Result<(), Box<dyn Error>> {

    let params = parametros.clone();
    let cliente = cliente::cliente_nuevo()?;

    let salida = completa(&cliente, &params);

    let rutam = ruta.to_string();

    salida.exportar(&rutam)?;

    Ok(())
}


/// Función de alto nivel para realizar una petición de datos que itera por sobre todos los valores de las variables que se pueden filtrar. Es posible filtrar esta petición con otras variables modificando la estructura de parametros que alimenta a la función previamente. Mucho cuidado con esta función, realiza muchas peticiones pues su objetivo es obtener los datos en los niveles menores de desagregación. Su ejecución tardará muchísimo.
///
/// # Argumentos
///
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
/// * `ruta` - Ruta en donde se pretenden guardar los datos la cual incluya el nombre del archivo, la función creará un archivo JSON de salida llamado nacional.json
pub fn extraer_todo_iterando(parametros: &Parametros, ruta: &str) -> Result<(), Box<dyn Error>> {

    let mut rutam = ruta.to_string();
    rutam.push_str("/");
    rutam.push_str("general.json");

    extraer(&parametros, &rutam)?;
    println!("Los datos generales han sido obtenidos");
    let evcta = extraer_por_estatus_victimas(&parametros, &ruta)?;
    println!("Los datos por estatus de victima han sido obtenidos. Se obtuvieron {} archivos", evcta);
    let edcta = extraer_por_estados(&parametros, &ruta)?;
    println!("Los datos por estado han sido obtenidos. Se obtuvieron {} archivos", edcta);
    let mucta = extraer_por_municipios(&parametros, &ruta)?;
    println!("Los datos por municipio han sido obtenidos. Se obtuvieron {} archivos", mucta);
    let hicta = extraer_por_hipotesis(&parametros, &ruta)?;
    println!("Los datos por hipótesis de desaparición han sido obtenidos. Se obtuvieron {} archivos", hicta);
    let mecta = extraer_por_medios(&parametros, &ruta)?;
    println!("Los datos por medio de conocimiento de la desaparición han sido obtenidos. Se obtuvieron {} archivos", mecta);
    let decta = extraer_por_delitos(&parametros, &ruta)?;
    println!("Los datos por delitos han sido obtenidos. Se obtuvieron {} archivos", decta);
    let cicta = extraer_por_circunstancias(&parametros, &ruta)?;
    println!("Los datos por circunstancias han sido obtenidos. Se obtuvieron {} archivos", cicta);
    let dicta = extraer_por_discapacidades(&parametros, &ruta)?;
    println!("Los datos por discapapcidad han sido obtenidos. Se obtuvieron {} archivos", dicta);
    let etcta = extraer_por_etnias(&parametros, &ruta)?;
    println!("Los datos por etnias han sido obtenidos. Se obtuvieron {} archivos", etcta);
    let lecta = extraer_por_lenguas(&parametros, &ruta)?;
    println!("Los datos por lengua han sido obtenidos. Se obtuvieron {} archivos", lecta);
    let recta = extraer_por_religiones(&parametros, &ruta)?;
    println!("Los datos por religión han sido obtenidos. Se obtuvieron {} archivos", recta);
    let emcta = extraer_por_estatus_migratorio(&parametros, &ruta)?;
    println!("Los datos por estatus migratorio han sido obtenidos. Se obtuvieron {} archivos", emcta);
    extraer_por_categoria(&parametros, &ruta)?;
    println!("Los datos por categoría han sido obtenidos");

    Ok(())
}

/// Función de alto nivel para realizar una petición de datos que itera por sobre todos los valores de las variables que se pueden filtrar sin explorar el nivel de colonia. Es posible filtrar esta petición con otras variables modificando la estructura de parametros que alimenta a la función previamente. Mucho cuidado con esta función, realiza muchas peticiones pues su objetivo es obtener los datos en los niveles menores de desagregación. Su ejecución tardará muchísimo.
///
/// # Argumentos
///
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
/// * `ruta` - Ruta en donde se pretenden guardar los datos la cual incluya el nombre del archivo, la función creará un archivo JSON de salida llamado nacional.json
pub fn extraer_todo_iterando_municipal(parametros: &Parametros, ruta: &str) -> Result<(), Box<dyn Error>> {

    let mut rutam = ruta.to_string();
    rutam.push_str("/");
    rutam.push_str("general.json");

    extraer(&parametros, &rutam)?;
    println!("Los datos generales han sido obtenidos");
    let evcta = extraer_por_estatus_victimas(&parametros, &ruta)?;
    println!("Los datos por estatus de victima han sido obtenidos. Se obtuvieron {} archivos", evcta);
    let edcta = extraer_por_estados(&parametros, &ruta)?;
    println!("Los datos por estado han sido obtenidos. Se obtuvieron {} archivos", edcta);
    let hicta = extraer_por_hipotesis(&parametros, &ruta)?;
    println!("Los datos por hipótesis de desaparición han sido obtenidos. Se obtuvieron {} archivos", hicta);
    let mecta = extraer_por_medios(&parametros, &ruta)?;
    println!("Los datos por medio de conocimiento de la desaparición han sido obtenidos. Se obtuvieron {} archivos", mecta);
    let decta = extraer_por_delitos(&parametros, &ruta)?;
    println!("Los datos por delitos han sido obtenidos. Se obtuvieron {} archivos", decta);
    let cicta = extraer_por_circunstancias(&parametros, &ruta)?;
    println!("Los datos por circunstancias han sido obtenidos. Se obtuvieron {} archivos", cicta);
    let dicta = extraer_por_discapacidades(&parametros, &ruta)?;
    println!("Los datos por discapapcidad han sido obtenidos. Se obtuvieron {} archivos", dicta);
    let etcta = extraer_por_etnias(&parametros, &ruta)?;
    println!("Los datos por etnias han sido obtenidos. Se obtuvieron {} archivos", etcta);
    let lecta = extraer_por_lenguas(&parametros, &ruta)?;
    println!("Los datos por lengua han sido obtenidos. Se obtuvieron {} archivos", lecta);
    let recta = extraer_por_religiones(&parametros, &ruta)?;
    println!("Los datos por religión han sido obtenidos. Se obtuvieron {} archivos", recta);
    let emcta = extraer_por_estatus_migratorio(&parametros, &ruta)?;
    println!("Los datos por estatus migratorio han sido obtenidos. Se obtuvieron {} archivos", emcta);
    extraer_por_categoria(&parametros, &ruta)?;
    println!("Los datos por categoría han sido obtenidos");

    Ok(())
}

pub struct Iteradora {
    pub estatus_victima: bool,
    pub estados: bool,
    pub municipios: bool,
    pub hipotesis: bool,
    pub medios: bool,
    pub delitos: bool,
    pub circunstancias: bool,
    pub discapacidades: bool,
    pub etnias: bool,
    pub lenguas: bool,
    pub religiones: bool,
    pub emigratorio: bool,
    pub categorias: bool,
}

impl Iteradora {
    pub fn new() -> Self {
        Iteradora {
            estatus_victima: true,
            estados: true,
            municipios: false,
            hipotesis: true,
            medios: false,
            delitos: false,
            circunstancias: true,
            discapacidades: false,
            etnias: false,
            lenguas: false,
            religiones: false,
            emigratorio: false,
            categorias: true,
        }
    }
}

pub fn extraer_iterando(parametros: &Parametros, ruta: &str, variables: &Iteradora) -> Result<(), Box<dyn Error>> {

    let mut rutam = ruta.to_string();
    rutam.push_str("/");
    rutam.push_str("general.json");

    extraer(&parametros, &rutam)?;
    println!("Los datos generales han sido obtenidos");
    if variables.estatus_victima {
        let evcta = extraer_por_estatus_victimas(&parametros, &ruta)?;
        println!("Los datos por estatus de victima han sido obtenidos. Se obtuvieron {} archivos", evcta);
    }
    if variables.estados {
        let edcta = extraer_por_estados(&parametros, &ruta)?;
        println!("Los datos por estado han sido obtenidos. Se obtuvieron {} archivos", edcta);
    }
    if variables.municipios {
        let mucta = extraer_por_municipios(&parametros, &ruta)?;
        println!("Los datos por municipio han sido obtenidos. Se obtuvieron {} archivos", mucta);
    }
    if variables.hipotesis {
        let hicta = extraer_por_hipotesis(&parametros, &ruta)?;
        println!("Los datos por hipótesis de desaparición han sido obtenidos. Se obtuvieron {} archivos", hicta);
    }
    if variables.medios {
        let mecta = extraer_por_medios(&parametros, &ruta)?;
        println!("Los datos por medio de conocimiento de la desaparición han sido obtenidos. Se obtuvieron {} archivos", mecta);
    }
    if variables.delitos {
        let decta = extraer_por_delitos(&parametros, &ruta)?;
        println!("Los datos por delitos han sido obtenidos. Se obtuvieron {} archivos", decta);
    }
    if variables.circunstancias {
        let cicta = extraer_por_circunstancias(&parametros, &ruta)?;
        println!("Los datos por circunstancias han sido obtenidos. Se obtuvieron {} archivos", cicta);
    }
    if variables.discapacidades {
        let dicta = extraer_por_discapacidades(&parametros, &ruta)?;
        println!("Los datos por discapapcidad han sido obtenidos. Se obtuvieron {} archivos", dicta);
    }
    if variables.etnias {
        let etcta = extraer_por_etnias(&parametros, &ruta)?;
        println!("Los datos por etnias han sido obtenidos. Se obtuvieron {} archivos", etcta);
    }
    if variables.lenguas {
        let lecta = extraer_por_lenguas(&parametros, &ruta)?;
        println!("Los datos por lengua han sido obtenidos. Se obtuvieron {} archivos", lecta);
    }
    if variables.religiones {
        let recta = extraer_por_religiones(&parametros, &ruta)?;
        println!("Los datos por religión han sido obtenidos. Se obtuvieron {} archivos", recta);
    }
    if variables.emigratorio {
        let emcta = extraer_por_estatus_migratorio(&parametros, &ruta)?;
        println!("Los datos por estatus migratorio han sido obtenidos. Se obtuvieron {} archivos", emcta);
    }
    if variables.categorias {
        extraer_por_categoria(&parametros, &ruta)?;
        println!("Los datos por categoría han sido obtenidos");
    }

    Ok(())
}

/// Función de alto nivel para obtener los datos de desaparecidos iterando sobre los posibles valores de Estatus de víctimas. La función generará un archivo por cada uno de los valores posibles. Es posible filtrar esta petición con otras variables modificando la estructura de parametros que alimenta a la función previamente.  
///
/// # Argumentos
///
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
/// * `ruta` - Ruta en donde se pretenden guardar los datos, si la carpeta "estatus_victimas" no existe en la ruta será creada
pub fn extraer_por_estatus_victimas(parametros: &Parametros, ruta: &str) -> Result<i32, Box<dyn Error>> {

    let mut ruta_dir = ruta.to_string();
    ruta_dir.push_str("/estatus_victimas");
    match fs::read_dir(&ruta_dir) {
        Ok(_) => {},
        _ => {
            fs::create_dir(&ruta_dir)?;
        }
    };

    let mut params = parametros.clone();

    let estatus = parameters::get_estatus_victimas()?;

    let mut cuenta = 0;
    for (titulo,dato) in estatus {

        let cliente = cliente::cliente_nuevo()?;
        params.id_estatus_victima = dato.to_string();
        params.titulo = titulo.to_string();
        let salida = completa(&cliente, &params);

        let mut rutam = ruta_dir.to_string();
        rutam.push_str("/");
        rutam.push_str(&dato);
        rutam.push_str(".json");

        salida.exportar(&rutam)?;

        cuenta = cuenta + 1;

    };

    Ok(cuenta)
}

/// Función de alto nivel para obtener los datos de desaparecidos iterando sobre los posibles valores de Estados. La función generará un archivo por cada uno de los valores posibles. Es posible filtrar esta petición con otras variables modificando la estructura de parametros que alimenta a la función previamente. En el campo espacial cada archivo tendrá los datos desagregados a nivel municipio.  
///
/// # Argumentos
///
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
/// * `ruta` - Ruta en donde se pretenden guardar los datos, si la carpeta "estados" no existe en la ruta será creada
pub fn extraer_por_estados(parametros: &Parametros, ruta: &str) -> Result<i32, Box<dyn Error>> {

    let mut ruta_dir = ruta.to_string();
    ruta_dir.push_str("/estados");
    match fs::read_dir(&ruta_dir) {
        Ok(_) => {},
        _ => {
            fs::create_dir(&ruta_dir)?;
        }
    };

    let mut params = parametros.clone();
    let mut cliente = cliente::cliente_nuevo()?;

    let estados = parameters::get_estados(&cliente)?;

    let mut cuenta = 0;
    for (_,dato) in estados {

        cliente = cliente::cliente_nuevo()?;
        params.id_estado = dato.to_string();
        let salida = completa(&cliente, &params);

        let mut rutam = ruta_dir.to_string();
        rutam.push_str("/");
        rutam.push_str(&dato);
        rutam.push_str(".json");

        salida.exportar(&rutam)?;

        cuenta = cuenta + 1;

    };

    Ok(cuenta)
}

/// Función de alto nivel para obtener los datos de desaparecidos iterando sobre los posibles valores de Municipios. La función generará un archivo por cada uno de los valores posibles. Es posible filtrar esta petición con otras variables modificando la estructura de parametros que alimenta a la función previamente. Cuidado con esta función pues es una doble iteración, sobre los estados y sobre los municipios. Realiza muchas peticiones y tarda mucho en ejecutar. En el campo espacial cada archivo tendrá los datos desagregados a nivel colonia.  
///
/// # Argumentos
///
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
/// * `ruta` - Ruta en donde se pretenden guardar los datos, si la carpeta "municipios" no existe en la ruta será creada
pub fn extraer_por_municipios(parametros: &Parametros, ruta: &str) -> Result<i32, Box<dyn Error>> {

    let mut ruta_dir = ruta.to_string();
    ruta_dir.push_str("/municipios");
    match fs::read_dir(&ruta_dir) {
        Ok(_) => {},
        _ => {
            fs::create_dir(&ruta_dir)?;
        }
    };

    let mut params = parametros.clone();
    let mut cliente = cliente::cliente_nuevo()?;

    let estados = parameters::get_estados(&cliente)?;

    let mut cuenta = 0;

    for (_,dato) in estados {

        match dato.as_str() {
            "0" => {},
            _ => {
                params.id_estado = dato.to_string();

                let municipios = parameters::get_municipios(&cliente, &dato)?;

                for (_, mdato) in municipios {
                    cliente = cliente::cliente_nuevo()?;
                    params.id_municipio = mdato.to_string();
                    let salida = completa(&cliente, &params);

                    let mut rutam = ruta_dir.to_string();
                    rutam.push_str("/");
                    rutam.push_str(&dato);
                    rutam.push_str("_");
                    rutam.push_str(&mdato);
                    rutam.push_str(".json");

                    salida.exportar(&rutam)?;

                    cuenta = cuenta + 1;
                }
            }
        }
    };

    Ok(cuenta)
}

/// Función de alto nivel para obtener los datos de desaparecidos iterando sobre los posibles valores de Hipótesis de desaparición. La función generará un archivo por cada uno de los valores posibles. Es posible filtrar esta petición con otras variables modificando la estructura de parametros que alimenta a la función previamente.  
///
/// # Argumentos
///
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
/// * `ruta` - Ruta en donde se pretenden guardar los datos, si la carpeta "hipotesis" no existe en la ruta será creada
pub fn extraer_por_hipotesis(parametros: &Parametros, ruta: &str) -> Result<i32, Box<dyn Error>> {

    let mut ruta_dir = ruta.to_string();
    ruta_dir.push_str("/hipotesis");
    match fs::read_dir(&ruta_dir) {
        Ok(_) => {},
        _ => {
            fs::create_dir(&ruta_dir)?;
        }
    };

    let mut params = parametros.clone();
    let mut cliente = cliente::cliente_nuevo()?;

    let hipotesis = parameters::get_hipotesis(&cliente)?;

    let mut cuenta = 0;

    for (_,dato) in hipotesis {

        cliente = cliente::cliente_nuevo()?;
        params.id_hipotesis_no_localizacion = dato.to_string();
        let salida = completa(&cliente, &params);

        let mut rutam = ruta_dir.to_string();
        rutam.push_str("/");
        rutam.push_str(&dato);
        rutam.push_str(".json");

        salida.exportar(&rutam)?;

        cuenta = cuenta + 1;

    };

    Ok(cuenta)
}

/// Función de alto nivel para obtener los datos de desaparecidos iterando sobre los posibles valores de delitos. La función generará un archivo por cada uno de los valores posibles. Es posible filtrar esta petición con otras variables modificando la estructura de parametros que alimenta a la función previamente.  
///
/// # Argumentos
///
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
/// * `ruta` - Ruta en donde se pretenden guardar los datos, si la carpeta "delitos" no existe en la ruta será creada
pub fn extraer_por_delitos(parametros: &Parametros, ruta: &str) -> Result<i32, Box<dyn Error>> {

    let mut ruta_dir = ruta.to_string();
    ruta_dir.push_str("/delitos");
    match fs::read_dir(&ruta_dir) {
        Ok(_) => {},
        _ => {
            fs::create_dir(&ruta_dir)?;
        }
    };

    let mut params = parametros.clone();
    let mut cliente = cliente::cliente_nuevo()?;

    let delitos = parameters::get_delitos(&cliente)?;

    let mut cuenta = 0;

    for (_,dato) in delitos {

        cliente = cliente::cliente_nuevo()?;
        params.id_delito = dato.to_string();
        let salida = completa(&cliente, &params);

        let mut rutam = ruta_dir.to_string();
        rutam.push_str("/");
        rutam.push_str(&dato);
        rutam.push_str(".json");

        salida.exportar(&rutam)?;

        cuenta = cuenta + 1;

    };

    Ok(cuenta)
}

/// Función de alto nivel para obtener los datos de desaparecidos iterando sobre los posibles valores de medios por los cuales se dio a conocer la desaparición. La función generará un archivo por cada uno de los valores posibles. Es posible filtrar esta petición con otras variables modificando la estructura de parametros que alimenta a la función previamente.  
///
/// # Argumentos
///
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
/// * `ruta` - Ruta en donde se pretenden guardar los datos, si la carpeta "medios" no existe en la ruta será creada
pub fn extraer_por_medios(parametros: &Parametros, ruta: &str) -> Result<i32, Box<dyn Error>> {

    let mut ruta_dir = ruta.to_string();
    ruta_dir.push_str("/medios");
    match fs::read_dir(&ruta_dir) {
        Ok(_) => {},
        _ => {
            fs::create_dir(&ruta_dir)?;
        }
    };

    let mut params = parametros.clone();
    let mut cliente = cliente::cliente_nuevo()?;

    let medios = parameters::get_medios(&cliente)?;

    let mut cuenta = 0;

    for (_,dato) in medios {

        cliente = cliente::cliente_nuevo()?;
        params.id_medio_conocimiento = dato.to_string();
        let salida = completa(&cliente, &params);

        let mut rutam = ruta_dir.to_string();
        rutam.push_str("/");
        rutam.push_str(&dato);
        rutam.push_str(".json");

        salida.exportar(&rutam)?;

        cuenta = cuenta + 1;

    };

    Ok(cuenta)
}

/// Función de alto nivel para obtener los datos de desaparecidos iterando sobre los posibles valores de circunstancias asociadas a la desaparición. La función generará un archivo por cada uno de los valores posibles. Es posible filtrar esta petición con otras variables modificando la estructura de parametros que alimenta a la función previamente.  
///
/// # Argumentos
///
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
/// * `ruta` - Ruta en donde se pretenden guardar los datos, si la carpeta "circunstancias" no existe en la ruta será creada
pub fn extraer_por_circunstancias(parametros: &Parametros, ruta: &str) -> Result<i32, Box<dyn Error>> {

    let mut ruta_dir = ruta.to_string();
    ruta_dir.push_str("/circunstancias");
    match fs::read_dir(&ruta_dir) {
        Ok(_) => {},
        _ => {
            fs::create_dir(&ruta_dir)?;
        }
    };

    let mut params = parametros.clone();
    let mut cliente = cliente::cliente_nuevo()?;

    let circunstancias = parameters::get_circunstancias(&cliente)?;

    let mut cuenta = 0;

    for (_,dato) in circunstancias {

        cliente = cliente::cliente_nuevo()?;
        params.id_circunstancia = dato.to_string();
        let salida = completa(&cliente, &params);

        let mut rutam = ruta_dir.to_string();
        rutam.push_str("/");
        rutam.push_str(&dato);
        rutam.push_str(".json");

        salida.exportar(&rutam)?;

        cuenta = cuenta + 1;

    };

    Ok(cuenta)
}

/// Función de alto nivel para obtener los datos de desaparecidos iterando sobre los posibles valores de discapapcidad. La función generará un archivo por cada uno de los valores posibles. Es posible filtrar esta petición con otras variables modificando la estructura de parametros que alimenta a la función previamente.  
///
/// # Argumentos
///
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
/// * `ruta` - Ruta en donde se pretenden guardar los datos, si la carpeta "discapacidades" no existe en la ruta será creada
pub fn extraer_por_discapacidades(parametros: &Parametros, ruta: &str) -> Result<i32, Box<dyn Error>> {

    let mut ruta_dir = ruta.to_string();
    ruta_dir.push_str("/discapacidades");
    match fs::read_dir(&ruta_dir) {
        Ok(_) => {},
        _ => {
            fs::create_dir(&ruta_dir)?;
        }
    };

    let mut params = parametros.clone();
    let mut cliente = cliente::cliente_nuevo()?;

    let discapacidades = parameters::get_discapacidades(&cliente)?;

    let mut cuenta  = 0;

    for (_,dato) in discapacidades {

        cliente = cliente::cliente_nuevo()?;
        params.tiene_discapacidad = "true".to_string();
        params.id_tipo_discapacidad = dato.to_string();
        let salida = completa(&cliente, &params);

        let mut rutam = ruta_dir.to_string();
        rutam.push_str("/");
        rutam.push_str(&dato);
        rutam.push_str(".json");

        salida.exportar(&rutam)?;

        cuenta = cuenta + 1;

    };

    Ok(cuenta)
}

/// Función de alto nivel para obtener los datos de desaparecidos iterando sobre los posibles valores de Etnia. La función generará un archivo por cada uno de los valores posibles. Es posible filtrar esta petición con otras variables modificando la estructura de parametros que alimenta a la función previamente.  
///
/// # Argumentos
///
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
/// * `ruta` - Ruta en donde se pretenden guardar los datos, si la carpeta "etnias" no existe en la ruta será creada
pub fn extraer_por_etnias(parametros: &Parametros, ruta: &str) -> Result<i32, Box<dyn Error>> {

    let mut ruta_dir = ruta.to_string();
    ruta_dir.push_str("/etnias");
    match fs::read_dir(&ruta_dir) {
        Ok(_) => {},
        _ => {
            fs::create_dir(&ruta_dir)?;
        }
    };

    let mut params = parametros.clone();
    let mut cliente = cliente::cliente_nuevo()?;

    let etnias = parameters::get_etnias(&cliente)?;

    let mut cuenta = 0;

    for (_,dato) in etnias {

        cliente = cliente::cliente_nuevo()?;
        params.id_etnia = dato.to_string();
        let salida = completa(&cliente, &params);

        let mut rutam = ruta_dir.to_string();
        rutam.push_str("/");
        rutam.push_str(&dato);
        rutam.push_str(".json");

        salida.exportar(&rutam)?;

        cuenta = cuenta + 1;

    };

    Ok(cuenta)
}

/// Función de alto nivel para obtener los datos de desaparecidos iterando sobre los posibles valores de lenguas. La función generará un archivo por cada uno de los valores posibles. Es posible filtrar esta petición con otras variables modificando la estructura de parametros que alimenta a la función previamente.  
///
/// # Argumentos
///
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
/// * `ruta` - Ruta en donde se pretenden guardar los datos, si la carpeta "lenguas" no existe en la ruta será creada
pub fn extraer_por_lenguas(parametros: &Parametros, ruta: &str) -> Result<i32, Box<dyn Error>> {

    let mut ruta_dir = ruta.to_string();
    ruta_dir.push_str("/lenguas");
    match fs::read_dir(&ruta_dir) {
        Ok(_) => {},
        _ => {
            fs::create_dir(&ruta_dir)?;
        }
    };

    let mut params = parametros.clone();
    let mut cliente = cliente::cliente_nuevo()?;

    let lenguas = parameters::get_lenguas(&cliente)?;

    let mut cuenta = 0;

    for (_,dato) in lenguas {

        cliente = cliente::cliente_nuevo()?;
        params.id_lengua = dato.to_string();
        let salida = completa(&cliente, &params);

        let mut rutam = ruta_dir.to_string();
        rutam.push_str("/");
        rutam.push_str(&dato);
        rutam.push_str(".json");

        salida.exportar(&rutam)?;

        cuenta = cuenta + 1;

    };

    Ok(cuenta)
}

/// Función de alto nivel para obtener los datos de desaparecidos iterando sobre los posibles valores de religiones. La función generará un archivo por cada uno de los valores posibles. Es posible filtrar esta petición con otras variables modificando la estructura de parametros que alimenta a la función previamente.  
///
/// # Argumentos
///
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
/// * `ruta` - Ruta en donde se pretenden guardar los datos, si la carpeta "religiones" no existe en la ruta será creada
pub fn extraer_por_religiones(parametros: &Parametros, ruta: &str) -> Result<i32, Box<dyn Error>> {

    let mut ruta_dir = ruta.to_string();
    ruta_dir.push_str("/religiones");
    match fs::read_dir(&ruta_dir) {
        Ok(_) => {},
        _ => {
            fs::create_dir(&ruta_dir)?;
        }
    };

    let mut params = parametros.clone();
    let mut cliente = cliente::cliente_nuevo()?;

    let religiones = parameters::get_religiones(&cliente)?;

    let mut cuenta = 0;

    for (_,dato) in religiones {

        cliente = cliente::cliente_nuevo()?;
        params.id_religion = dato.to_string();
        let salida = completa(&cliente, &params);

        let mut rutam = ruta_dir.to_string();
        rutam.push_str("/");
        rutam.push_str(&dato);
        rutam.push_str(".json");

        salida.exportar(&rutam)?;

        cuenta = cuenta + 1;

    };

    Ok(cuenta)
}

/// Función de alto nivel para obtener los datos de desaparecidos iterando sobre los posibles valores de Estatus migratorio. La función generará un archivo por cada uno de los valores posibles. Es posible filtrar esta petición con otras variables modificando la estructura de parametros que alimenta a la función previamente.  
///
/// # Argumentos
///
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
/// * `ruta` - Ruta en donde se pretenden guardar los datos, si la carpeta "estatus_migratorio" no existe en la ruta será creada
pub fn extraer_por_estatus_migratorio(parametros: &Parametros, ruta: &str) -> Result<i32, Box<dyn Error>> {

    let mut ruta_dir = ruta.to_string();
    ruta_dir.push_str("/estatus_migratorio");
    match fs::read_dir(&ruta_dir) {
        Ok(_) => {},
        _ => {
            fs::create_dir(&ruta_dir)?;
        }
    };

    let mut params = parametros.clone();
    let mut cliente = cliente::cliente_nuevo()?;

    let emigratorios = parameters::get_emigratorios(&cliente)?;

    let mut cuenta = 0;

    for (_,dato) in emigratorios {

        cliente = cliente::cliente_nuevo()?;
        params.es_migrante = "true".to_string();
        params.id_estatus_migratorio = dato.to_string();
        let salida = completa(&cliente, &params);

        let mut rutam = ruta_dir.to_string();
        rutam.push_str("/");
        rutam.push_str(&dato);
        rutam.push_str(".json");

        salida.exportar(&rutam)?;

        cuenta = cuenta + 1;

    };

    Ok(cuenta)
}

/// Función de alto nivel para obtener los datos de desaparecidos iterando sobre categorías de pertenencia del desaparecido. La función generará un archivo por cada uno de los valores posibles. Es posible filtrar esta petición con otras variables modificando la estructura de parametros que alimenta a la función previamente.  
///
/// # Argumentos
///
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
/// * `ruta` - Ruta en donde se pretenden guardar los datos, si la carpeta "por_categoria" no existe en la ruta será creada
pub fn extraer_por_categoria(parametros: &Parametros, ruta: &str) -> Result<(), Box<dyn Error>> {

    let mut ruta_dir = ruta.to_string();
    ruta_dir.push_str("/por_categoria");
    match fs::read_dir(&ruta_dir) {
        Ok(_) => {},
        _ => {
            fs::create_dir(&ruta_dir)?;
        }
    };

    let mut params = parametros.clone();
    let mut cliente;

    let categoria = "sindicalista";
    params.es_sindicalista = "true".to_string();
    cliente = cliente::cliente_nuevo()?;
    let salida = completa(&cliente, &params);
    let mut rutam = ruta_dir.to_string();
    rutam.push_str("/");
    rutam.push_str(categoria);
    rutam.push_str(".json");
    salida.exportar(&rutam)?;
    params.es_sindicalista = "".to_string();

    let categoria = "servidor_publico";
    params.es_servidor_publico = "true".to_string();
    cliente = cliente::cliente_nuevo()?;
    let salida = completa(&cliente, &params);
    let mut rutam = ruta_dir.to_string();
    rutam.push_str("/");
    rutam.push_str(categoria);
    rutam.push_str(".json");
    salida.exportar(&rutam)?;
    params.es_servidor_publico = "".to_string();

    let categoria = "periodista";
    params.es_periodista = "true".to_string();
    cliente = cliente::cliente_nuevo()?;
    let salida = completa(&cliente, &params);
    let mut rutam = ruta_dir.to_string();
    rutam.push_str("/");
    rutam.push_str(categoria);
    rutam.push_str(".json");
    salida.exportar(&rutam)?;
    params.es_periodista = "".to_string();

    let categoria = "ong";
    params.es_ong = "true".to_string();
    cliente = cliente::cliente_nuevo()?;
    let salida = completa(&cliente, &params);
    let mut rutam = ruta_dir.to_string();
    rutam.push_str("/");
    rutam.push_str(categoria);
    rutam.push_str(".json");
    salida.exportar(&rutam)?;
    params.es_ong = "".to_string();

    let categoria = "migrante";
    params.es_migrante = "true".to_string();
    cliente = cliente::cliente_nuevo()?;
    let salida = completa(&cliente, &params);
    let mut rutam = ruta_dir.to_string();
    rutam.push_str("/");
    rutam.push_str(categoria);
    rutam.push_str(".json");
    salida.exportar(&rutam)?;
    params.es_migrante = "".to_string();

    let categoria = "lgbttti";
    params.es_lgbttti = "true".to_string();
    cliente = cliente::cliente_nuevo()?;
    let salida = completa(&cliente, &params);
    let mut rutam = ruta_dir.to_string();
    rutam.push_str("/");
    rutam.push_str(categoria);
    rutam.push_str(".json");
    salida.exportar(&rutam)?;
    params.es_lgbttti = "".to_string();

    let categoria = "defensor_dh";
    params.es_defensor_dh = "true".to_string();
    cliente = cliente::cliente_nuevo()?;
    let salida = completa(&cliente, &params);
    let mut rutam = ruta_dir.to_string();
    rutam.push_str("/");
    rutam.push_str(categoria);
    rutam.push_str(".json");
    salida.exportar(&rutam)?;
    params.es_defensor_dh = "".to_string();

    Ok(())
}

/// Función para realizar una petición al endpoint de datos totales filtrada conforme los valores de la estructura parametros. Esta función no es de alto nivel y solamente debería de ser usada si se quiere estructurar una petición particular de manera excepcional. La manera usual de pedir datos es utilizar las funciones de alto nivel proporcionadas en este mismo módulo. Si la petición particular se va a realizar de forma recurrente el mejor camino para hacerlo sería crear una función de alto nivel similar a las que se proporcionan en este módulo.
///
/// # Argumentos
///
/// * `cliente` - Cliente válido con el cual se realizarán las peticiones.
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
pub fn totales(cliente: &Client, parametros: &Parametros) -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let url = urls::totales_url();
    let params = parametros.to_tuples();

    let response = match cliente.post(url).form(&params).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de datos totales no se concretó - {}", err)))
    };
    let mut datos: BTreeMap<String,String> = response.json()?;

    for (_key,value) in datos.iter_mut() {
        *value = value.replace(",","").replace(" %","").to_string();
    }

    Ok(datos)
}

/// Función para realizar una petición al endpoint de datos por estado filtrada conforme los valores de la estructura parametros. Esta función no es de alto nivel y solamente debería de ser usada si se quiere estructurar una petición particular de manera excepcional. La manera usual de pedir datos es utilizar las funciones de alto nivel proporcionadas en este mismo módulo. Si la petición particular se va a realizar de forma recurrente el mejor camino para hacerlo sería crear una función de alto nivel similar a las que se proporcionan en este módulo.
///
/// # Argumentos
///
/// * `cliente` - Cliente válido con el cual se realizarán las peticiones.
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
pub fn por_estado(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::por_estado_url();
    let params = parametros.to_tuples();

    let response = match cliente.post(url).form(&params).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición datos por estado no se concretó - {}", err)))
    };
    let datos: Data = response.json()?;

    Ok(datos)
}

/// Función para realizar una petición al endpoint de datos por municipio filtrada conforme los valores de la estructura parametros. Esta función no es de alto nivel y solamente debería de ser usada si se quiere estructurar una petición particular de manera excepcional. La manera usual de pedir datos es utilizar las funciones de alto nivel proporcionadas en este mismo módulo. Si la petición particular se va a realizar de forma recurrente el mejor camino para hacerlo sería crear una función de alto nivel similar a las que se proporcionan en este módulo.
///
/// # Argumentos
///
/// * `cliente` - Cliente válido con el cual se realizarán las peticiones.
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
pub fn por_municipio(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::por_municipio_url();
    let params = parametros.to_tuples();

    let response = match cliente.post(url).form(&params).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de datos por municipio no se concretó - {}", err)))
    };
    let datos: Data = response.json()?;

    Ok(datos)
}

/// Función para realizar una petición al endpoint de datos por colonia filtrada conforme los valores de la estructura parametros. Esta función no es de alto nivel y solamente debería de ser usada si se quiere estructurar una petición particular de manera excepcional. La manera usual de pedir datos es utilizar las funciones de alto nivel proporcionadas en este mismo módulo. Si la petición particular se va a realizar de forma recurrente el mejor camino para hacerlo sería crear una función de alto nivel similar a las que se proporcionan en este módulo.
///
/// # Argumentos
///
/// * `cliente` - Cliente válido con el cual se realizarán las peticiones.
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
pub fn por_colonia(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::por_colonia_url();
    let params = parametros.to_tuples();

    let response = match cliente.post(url).form(&params).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de datos por colonia no se concretó - {}", err)))
    };
    let datos: Data = response.json()?;

    Ok(datos)
}

/// Función para realizar una petición al endpoint de datos por año filtrada conforme los valores de la estructura parametros. Esta función no es de alto nivel y solamente debería de ser usada si se quiere estructurar una petición particular de manera excepcional. La manera usual de pedir datos es utilizar las funciones de alto nivel proporcionadas en este mismo módulo. Si la petición particular se va a realizar de forma recurrente el mejor camino para hacerlo sería crear una función de alto nivel similar a las que se proporcionan en este módulo.
///
/// # Argumentos
///
/// * `cliente` - Cliente válido con el cual se realizarán las peticiones.
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
pub fn por_anio(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::por_anio_url();
    let params = parametros.to_tuples();

    let response = match cliente.post(url).form(&params).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de datos por año no se concretó - {}", err)))
    };
    let datos: Data = response.json()?;

    Ok(datos)
}

/// Función para realizar una petición al endpoint de datos por mes filtrada conforme los valores de la estructura parametros. Esta función no es de alto nivel y solamente debería de ser usada si se quiere estructurar una petición particular de manera excepcional. La manera usual de pedir datos es utilizar las funciones de alto nivel proporcionadas en este mismo módulo. Si la petición particular se va a realizar de forma recurrente el mejor camino para hacerlo sería crear una función de alto nivel similar a las que se proporcionan en este módulo.
///
/// # Argumentos
///
/// * `cliente` - Cliente válido con el cual se realizarán las peticiones.
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
pub fn por_mes(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::ultimo_anio_url();
    let params = parametros.to_tuples();

    let response = match cliente.post(url).form(&params).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de datos por mes no se concretó - {}", err)))
    };
    let datos: Data = response.json()?;

    Ok(datos)
}

/// Función para realizar una petición al endpoint de datos por edad filtrada conforme los valores de la estructura parametros. Esta función no es de alto nivel y solamente debería de ser usada si se quiere estructurar una petición particular de manera excepcional. La manera usual de pedir datos es utilizar las funciones de alto nivel proporcionadas en este mismo módulo. Si la petición particular se va a realizar de forma recurrente el mejor camino para hacerlo sería crear una función de alto nivel similar a las que se proporcionan en este módulo.
///
/// # Argumentos
///
/// * `cliente` - Cliente válido con el cual se realizarán las peticiones.
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
pub fn por_rango_edad(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::rango_edad_url();
    let params = parametros.to_tuples();

    let response = match cliente.post(url).form(&params).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de datos por rango de edad no se concretó - {}", err)))
    };
    let datos: Data = response.json()?;

    Ok(datos)
}

/// Función para realizar una petición al endpoint de datos por nacionalidad filtrada conforme los valores de la estructura parametros. Esta función no es de alto nivel y solamente debería de ser usada si se quiere estructurar una petición particular de manera excepcional. La manera usual de pedir datos es utilizar las funciones de alto nivel proporcionadas en este mismo módulo. Si la petición particular se va a realizar de forma recurrente el mejor camino para hacerlo sería crear una función de alto nivel similar a las que se proporcionan en este módulo.
///
/// # Argumentos
///
/// * `cliente` - Cliente válido con el cual se realizarán las peticiones.
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
pub fn por_nacionalidad(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::nacionalidad_url();
    let params = parametros.to_tuples();

    let response = match cliente.post(url).form(&params).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de datos por nacionalidad no se concretó - {}", err)))
    };
    let datos: Data = response.json()?;

    Ok(datos)
}

/// Función para realizar una petición al endpoint de datos por fiscalias filtrada conforme los valores de la estructura parametros. Esta función no es de alto nivel y solamente debería de ser usada si se quiere estructurar una petición particular de manera excepcional. La manera usual de pedir datos es utilizar las funciones de alto nivel proporcionadas en este mismo módulo. Si la petición particular se va a realizar de forma recurrente el mejor camino para hacerlo sería crear una función de alto nivel similar a las que se proporcionan en este módulo.
///
/// # Argumentos
///
/// * `cliente` - Cliente válido con el cual se realizarán las peticiones.
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
pub fn fiscalias(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::fiscalias_url();
    let params = parametros.to_tuples();

    let response = match cliente.post(url).form(&params).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de datos por fiscalias no se concretó - {}", err)))
    };
    let datos: Data = response.json()?;

    Ok(datos)
}

/// Función para realizar una petición al endpoint de datos por comisiones filtrada conforme los valores de la estructura parametros. Esta función no es de alto nivel y solamente debería de ser usada si se quiere estructurar una petición particular de manera excepcional. La manera usual de pedir datos es utilizar las funciones de alto nivel proporcionadas en este mismo módulo. Si la petición particular se va a realizar de forma recurrente el mejor camino para hacerlo sería crear una función de alto nivel similar a las que se proporcionan en este módulo.
///
/// # Argumentos
///
/// * `cliente` - Cliente válido con el cual se realizarán las peticiones.
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
pub fn comisiones(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::comisiones_url();
    let params = parametros.to_tuples();

    let response = match cliente.post(url).form(&params).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de datos por comisiones no se concretó - {}", err)))
    };
    let datos: Data = response.json()?;

    Ok(datos)
}

/// Función para realizar una petición al endpoint de datos por portal filtrada conforme los valores de la estructura parametros. Esta función no es de alto nivel y solamente debería de ser usada si se quiere estructurar una petición particular de manera excepcional. La manera usual de pedir datos es utilizar las funciones de alto nivel proporcionadas en este mismo módulo. Si la petición particular se va a realizar de forma recurrente el mejor camino para hacerlo sería crear una función de alto nivel similar a las que se proporcionan en este módulo.
///
/// # Argumentos
///
/// * `cliente` - Cliente válido con el cual se realizarán las peticiones.
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
pub fn portal(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::portal_url();
    let params = parametros.to_tuples();

    let response = match cliente.post(url).form(&params).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de datos por portal no se concretó - {}", err)))
    };
    let datos: Data = response.json()?;

    Ok(datos)
}

/// Función para realizar una petición al endpoint de datos por edades filtrada conforme los valores de la estructura parametros. Los datos se extraen de las tablas generadas en el sitio del RNPDNO. Esta función no es de alto nivel y solamente debería de ser usada si se quiere estructurar una petición particular de manera excepcional. La manera usual de pedir datos es utilizar las funciones de alto nivel proporcionadas en este mismo módulo. Si la petición particular se va a realizar de forma recurrente el mejor camino para hacerlo sería crear una función de alto nivel similar a las que se proporcionan en este módulo.
///
/// # Argumentos
///
/// * `cliente` - Cliente válido con el cual se realizarán las peticiones.
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
pub fn por_edades_completo(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::tabla_detalle_url();
    let mut params = parametros.to_tuples();

    params.push(("TipoDetalle","5"));

    let response = match cliente.post(url).form(&params).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de datos de tabla de edad no se concretó - {}", err)))
    };
    let tabla: Tabla = response.json()?;
    
    let datos = parse_table(&tabla)?;

    Ok(datos)
}

/// Función para realizar una petición al endpoint de datos por nacionalidades filtrada conforme los valores de la estructura parametros. Los datos se extraen de las tablas generadas en el sitio del RNPDNO. Esta función no es de alto nivel y solamente debería de ser usada si se quiere estructurar una petición particular de manera excepcional. La manera usual de pedir datos es utilizar las funciones de alto nivel proporcionadas en este mismo módulo. Si la petición particular se va a realizar de forma recurrente el mejor camino para hacerlo sería crear una función de alto nivel similar a las que se proporcionan en este módulo.
///
/// # Argumentos
///
/// * `cliente` - Cliente válido con el cual se realizarán las peticiones.
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
pub fn por_nacionalidades_completo(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::tabla_detalle_url();
    let mut params = parametros.to_tuples();

    params.push(("TipoDetalle","2"));

    let response = match cliente.post(url).form(&params).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de datos de tabla de nacionalidad no se concretó - {}", err)))
    };
    let tabla: Tabla = response.json()?;
    
    let datos = parse_table(&tabla)?;

    Ok(datos)
}

/// Función para realizar una petición al endpoint de datos por municipios filtrada conforme los valores de la estructura parametros. Los datos se extraen de las tablas generadas en el sitio del RNPDNO. Esta función no es de alto nivel y solamente debería de ser usada si se quiere estructurar una petición particular de manera excepcional. La manera usual de pedir datos es utilizar las funciones de alto nivel proporcionadas en este mismo módulo. Si la petición particular se va a realizar de forma recurrente el mejor camino para hacerlo sería crear una función de alto nivel similar a las que se proporcionan en este módulo.
///
/// # Argumentos
///
/// * `cliente` - Cliente válido con el cual se realizarán las peticiones.
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
pub fn por_municipios_completo(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::tabla_detalle_url();
    let mut params = parametros.to_tuples();

    params.push(("TipoDetalle","3"));

    let response = match cliente.post(url).form(&params).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de datos de tabla de municipios no se concretó - {}", err)))
    };
    let tabla: Tabla = response.json()?;
    
    let datos = parse_table(&tabla)?;

    Ok(datos)
}

/// Función para realizar una petición al endpoint de datos por colonias filtrada conforme los valores de la estructura parametros. Los datos se extraen de las tablas generadas en el sitio del RNPDNO. Esta función no es de alto nivel y solamente debería de ser usada si se quiere estructurar una petición particular de manera excepcional. La manera usual de pedir datos es utilizar las funciones de alto nivel proporcionadas en este mismo módulo. Si la petición particular se va a realizar de forma recurrente el mejor camino para hacerlo sería crear una función de alto nivel similar a las que se proporcionan en este módulo.
///
/// # Argumentos
///
/// * `cliente` - Cliente válido con el cual se realizarán las peticiones.
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
pub fn por_colonias_completo(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::tabla_detalle_url();
    let mut params = parametros.to_tuples();

    params.push(("TipoDetalle","4"));

    let response = match cliente.post(url).form(&params).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de datos de tabla de colonias no se concretó - {}", err)))
    };
    let tabla: Tabla = response.json()?;
    
    let datos = parse_table(&tabla)?;

    Ok(datos)
}

/// Función para realizar una petición a todos los endpoints disponibles. La solicitud es filtrada conforme los valores de la estructura parametros. Algunos de los datos se extraen de las tablas generadas en el sitio del RNPDNO. Esta función no es de alto nivel y solamente debería de ser usada si se quiere estructurar una petición particular de manera excepcional. La manera usual de pedir datos es utilizar las funciones de alto nivel proporcionadas en este mismo módulo. Si la petición particular se va a realizar de forma recurrente el mejor camino para hacerlo sería crear una función de alto nivel similar a las que se proporcionan en este módulo.
///
/// # Argumentos
///
/// * `cliente` - Cliente válido con el cual se realizarán las peticiones.
/// * `parametros` - Estructura del tipo Parametros que contenga los valores del filtro deseado para las otras variables
pub fn completa(cliente: &Client, parametros: &Parametros) -> General {

    let mut salida = General::new(parametros);

    match totales(&cliente, &parametros) {
        Ok(datos) => {salida.totales = datos},
        Err(err) => {println!("No se pudieron obtener los totales - {}",err)}
    }

    match (parametros.id_estado.as_str(),parametros.id_municipio.as_str(),parametros.id_colonia.as_str()) {
        ("0","0","0") => {
            match por_estado(&cliente, &parametros) {
                Ok(datos) => {salida.espacial = datos.to_map()},
                Err(err) => {println!("No se pudo obtener la información espacial - {}",err)}
            };
        },
        (_,"0","0") => {
            match por_municipios_completo(&cliente, &parametros) {
                Ok(datos) => {salida.espacial = datos.to_map()},
                Err(err) => {println!("No se pudo obtener la información espacial - {}",err)}
            };
        },
        (_,_,"0") => {
            match por_colonias_completo(&cliente, &parametros) {
                Ok(datos) => {salida.espacial = datos.to_map()},
                Err(err) => {println!("No se pudo obtener la información espacial - {}", err)}
            };
        },
        (_,_,_) => {}
    };

    match por_anio(&cliente, &parametros) {
        Ok(datos) => {salida.anual = datos.to_map()},
        Err(err) => {println!("No se pudo obtener la información anual - {}", err)}
    };

    match por_mes(&cliente, &parametros) {
        Ok(datos) => {salida.mensual_ultimo_anio = datos.to_map()},
        Err(err) => {println!("No se pudo obtener la información mensual - {}", err)}
    };

    match por_edades_completo(&cliente, &parametros) {
        Ok(datos) => {salida.por_edad = datos.to_map()},
        Err(err) => {println!("No se pudo obtener la información por edades - {}", err)}
    };

    match por_nacionalidades_completo(&cliente, &parametros) {
        Ok(datos) => {salida.por_nacionalidad = datos.to_map()},
        Err(err) => {println!("No se pudo obtener la información por nacionalidades - {}", err)}
    };

    salida
}

/// Función utilitaria para poder parsear las tablas del sitio del RNPDNO las cuales contienen los datos completos para algunas variables que no caben desagregadas en las gráficas.
fn parse_table(tabla: &Tabla) -> Result<Data, Box<dyn Error>> {

    let mut cabeza: BTreeMap<usize,String> = BTreeMap::new();
    let mut valores: BTreeMap<usize,Vec<String>> = BTreeMap::new();

    let html = Html::parse_document(&tabla.html);

    let sel_head = Selector::parse("thead").unwrap();
    let sel_body = Selector::parse("tbody").unwrap();
    let sel_tr = Selector::parse("tr").unwrap();
    let sel_td = Selector::parse("td").unwrap();
    let sel_th = Selector::parse("th").unwrap();

    html.select(&sel_head).next().unwrap()
        .select(&sel_tr).next().unwrap()
        .select(&sel_th).enumerate().for_each(|(index,element)|{
            let catego = element.text().next().unwrap().to_string();
            cabeza.insert(index,catego);
        });

    html.select(&sel_body).next().unwrap()
        .select(&sel_tr).for_each(|fila|{
            fila.select(&sel_td).enumerate().for_each(|(index,element)| {
                let valor = element.text().next().unwrap().to_string();
                let vector = valores.entry(index).or_insert(Vec::new());
                vector.push(valor);
            });
        });


    let mut x_axis_categories = Vec::new();
    let mut series = Vec::new(); 
    for (llave,vector) in valores.iter() {
        match llave {
            0 => {
                x_axis_categories = vector.clone();
            },
            _ => {
                let serie = Serie {
                    data: vector.iter().map(|val| val.parse::<u32>().unwrap()).collect(),
                    name: cabeza.get(&llave).unwrap().clone()
                };
                series.push(serie);
            }
        }
    }

    let data = Data {x_axis_categories,series};

    Ok(data)
}

/// Estructura utilitaria que contiene los datos parseados de una gráfica de la API
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub series: Vec<Serie>,
    pub x_axis_categories: Vec<String>
}

impl Data {
    pub fn to_map(&self) -> BTreeMap<String,BTreeMap<String,u32>> {
        let mut mapa = BTreeMap::new();

        for serie in self.series.iter() {
            let llave = serie.name.clone();
            let mut minimapa: BTreeMap<String, u32> = BTreeMap::new();
            for (catego,valor) in self.x_axis_categories.iter().zip(serie.data.iter()) {
                minimapa.insert(catego.clone(),valor.clone());
            };
            mapa.insert(llave,minimapa);
        };

        mapa
    }
}

/// Estructura utilitaria que contiene los datos parseados de una de las series de una gráfica de la API
#[derive(Deserialize, Debug)]
pub struct Serie {
    pub name: String,
    pub data: Vec<u32>
}

/// Estructura utilitaria que contiene los datos parseados de una tabla de la API
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Tabla {
    pub html: String,
}

/// Estructura utilitaria que contendrá los datos obtenidos de la API
#[derive(Debug,Serialize)]
pub struct General {
    pub totales: BTreeMap<String,String>,
    pub espacial: BTreeMap<String,BTreeMap<String,u32>>,
    pub anual: BTreeMap<String,BTreeMap<String,u32>>,
    pub mensual_ultimo_anio: BTreeMap<String,BTreeMap<String,u32>>,
    pub por_edad: BTreeMap<String,BTreeMap<String,u32>>,
    pub por_nacionalidad: BTreeMap<String,BTreeMap<String,u32>>,
    pub parametros: Parametros,
}

impl General {
    pub fn new(parametros: &Parametros) -> Self {
        General {
            totales: BTreeMap::new(),
            espacial: BTreeMap::new(),
            anual: BTreeMap::new(),
            mensual_ultimo_anio: BTreeMap::new(),
            por_edad: BTreeMap::new(),
            por_nacionalidad: BTreeMap::new(),
            parametros: parametros.clone(),
        }
    }

    pub fn exportar(&self, ruta: &str) -> Result<(), Box<dyn Error>> {
        
        let mut salida = File::create(ruta)?;
        let j = serde_json::to_string_pretty(&self)?;
        write!(salida, "{}", j)?;

        Ok(())
    }
}
