//! Módulo encargado de generar la estructura con los parámetros necesarios y sus valores por defecto para realizar peticiones a la API que tengan sentido.
//! El módulo también contiene las funciones necesaria para generar los diccionarios a partir de los catálogos internos de la API
//! 
use std::collections::BTreeMap;
use std::error::Error;
use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;
use crate::urls;
use std::fs::File;
use std::io::Write;

/// Estructura base para realizar las peticiones, contiene los campos necesarios para que la petición sea válida. Cada uno de los campos está asociado a un posible filtro en la petición. Los valores posibles para campo se pueden obtener de los diccionarios. Para generar los diccionarios es necesario usar la función 'get_diccionarios()' del módulo 'extractora'.
#[derive(Debug, Clone, Serialize)]
pub struct Parametros {
    // titulo: String - Valor interno para mostrar título en la página de datos, su valor no afecta los datos obtenidos. Su valor por defecto es "PERSONAS DESAPARECIDAS, NO LOCALIZADAS Y LOCALIZADAS"
    pub titulo: String,
    // id_estatus_victima: String - Estatus de la víctima. Su valor por defecto es "0" correspondiente a "PERSONAS DESAPARECIDAS, NO LOCALIZADAS Y LOCALIZADAS"
    pub id_estatus_victima: String,
    // fecha_inicio: String - Fecha inicial para el filtro por fechas. Su valor por defecto es "" lo cual corresponde a extraer todos los datos disponibles 
    pub fecha_inicio: String,
    // fecha_fin: String - Fecha final para el filtro por fechas. Su valor por defecto es "" lo cual corresponde a extraer todos los datos disponibles 
    pub fecha_fin: String,
    // id_estado: String - Estado. Su valor por defecto es "0", el cual corresponde a estraer datos para todos los estados
    pub id_estado: String,
    // id_municipio: String - Municipio. Su valor por defecto es "0", el cual corresponde a extraer datos para todos los municipios. La selección de un municipio debe de acompañar a la selección de un estado.
    pub id_municipio: String,
    // mostrar_fecha_nula: String - Valor interno para mostrar fecha en la página de datos, su valor no afecta los datos obtenidos. Su valor por defecto es "0"
    pub mostrar_fecha_nula: String,
    // id_colonia: String - Colonia. Su valor por defecto es "0", el cual corresponde a extraer datos para todas las colonias. La selección de una colonia debe de acompañar a la selección de un estado y de un municipio.
    pub id_colonia: String,
    // id_nacionalidad: String - Nacionalidad. Su valor por defecto es "0", el cual corresponde a extraer datos para todas las nacionalidades.
    pub id_nacionalidad: String,
    // edad_inicio: String - Edad inicial para el filtro por edades. Su valor por defecto es "" lo cual corresponde a extraer todos los datos disponibles
    pub edad_inicio: String,
    // edad_fin: String - Edad final para el filtro por edades. Su valor por defecto es "" lo cual corresponde a extraer todos los datos disponibles
    pub edad_fin: String,
    // mostrar_edad_nula: String - Valor interno para mostrar edad en la página de datos, su valor no afecta los datos obtenidos. Su valor por defecto es "0"
    pub mostrar_edad_nula: String,
    // id_hipotesis: String - Actualmente no sabemos a que filtro está asociado este valor. Si se buscan datos filtrados por hipótesis de desaparición el campo a utilizar es "id_hipotesis_no_localizacion"
    pub id_hipotesis: String,
    // id_medio_conocimiento: String - Medio de conocimiento de la desaparición. Su valor por defecto es "" lo cual corresponde a extraer todos los datos disponibles
    pub id_medio_conocimiento: String,
    // id_circunstancia: String - Circunstancia de la desaparición. Su valor por defecto es "" lo cual corresponde a extraer todos los datos disponibles
    pub id_circunstancia: String,
    // tiene_discapacidad: String - Discapacidad existente. Su valor por defecto es "" lo cual corresponde a extraer todos los datos disponibles 
    pub tiene_discapacidad: String, 
    // id_tipo_discapacidad: String - Tipo de discapapcidad. Su valor por defecto es "0" lo cual corresponde a extraer todos los datos disponibles. Si se desea obtener datos por discapacidad es necesario modificar el campo "tiene_discapacidad" a un valor válido y diferente del valor por defecto
    pub id_tipo_discapacidad: String,
    // id_etnia: String - Etnia. Su valor por defecto es "0" lo cual corresponde a extraer todos los datos disponibles
    pub id_etnia: String,
    // id_lengua: String - Lengua, Su valor por defecto es "0" lo cual corresponde a extraer todos los datos disponibles
    pub id_lengua: String,
    // id_religion: String - Religión. Su valor por defecto es "" lo cual corresponde a extraer todos los datos disponibles
    pub id_religion: String,
    // es_migrante: String - Condición de migrante existente. Su valor por defecto es "0" lo cual corresponde a extraer todos los datos disponibles
    pub es_migrante: String,
    // id_estatus_migratorio: String - Estatus migratorio. Su valor por defecto es "0" lo cual corresponde a extraer todos los datos disponibles. Si se desea obtener datos por estatus migratorio es necesario modificar el campo "es_migrante" a un valor válido y diferente del valor por defecto
    pub id_estatus_migratorio: String,
    // es_lgbttti: String - Condición de pertenencia a comunidad lgbttti. Su valor por defecto es "" lo cual corresponde a extraer todos los datos disponibles.
    pub es_lgbttti: String,
    // es_servidor_publico: String - Condición de servidor público. Su valor por defecto es "" lo cual corresponde a extraer todos los datos disponibles.
    pub es_servidor_publico: String,
    // es_defensor_dh: String  - Condición de defensor de derechos humanos. Su valor por defecto es "" lo cual corresponde a extraer todos los datos disponibles.
    pub es_defensor_dh: String,
    // es_periodista: String - Condición de pertenencia al gremio periodista. Su valor por defecto es "" lo cual corresponde a extraer todos los datos disponibles.
    pub es_periodista: String,
    // es_sindicalista: String - Condición de pertenencia al gremio sindicalista. Su valor por defecto es "" lo cual corresponde a extraer todos los datos disponibles.
    pub es_sindicalista: String,
    // es_ong: String - Condición de pertenencia a una ONG. Su valor por defecto es "" lo cual corresponde a extraer todos los datos disponibles.
    pub es_ong: String,
    // id_hipotesis_no_localizacion:String - Hipótesis de desaparición. Su valor por defecto es "0" lo cual corresponde a extraer todos los datos disponibles
    pub id_hipotesis_no_localizacion:String,
    // id_delito: String - Delito asociado a la desparición. Su valor por defecto es "0" lo cual corresponde a extraer todos los datos disponibles
    pub id_delito: String
} 

/// Genera una instancia nueva de la estructura de parámetros, los valores para campo pueden cambiarse de forma manual y así modificar los filtros utilizados en la petición.
impl Parametros {
    pub fn new() -> Parametros {
        Parametros {
            titulo:"PERSONAS DESAPARECIDAS, NO LOCALIZADAS Y LOCALIZADAS".to_string(),
            id_estatus_victima:"0".to_string(),
            fecha_inicio:"".to_string(),
            fecha_fin:"".to_string(),
            id_estado:"0".to_string(),
            id_municipio:"0".to_string(),
            mostrar_fecha_nula:"0".to_string(),
            id_colonia:"0".to_string(),
            id_nacionalidad:"0".to_string(),
            edad_inicio:"".to_string(),
            edad_fin:"".to_string(),
            mostrar_edad_nula:"0".to_string(),
            id_hipotesis:"".to_string(),
            id_medio_conocimiento:"".to_string(),
            id_circunstancia:"".to_string(),
            tiene_discapacidad:"".to_string(),
            id_tipo_discapacidad:"0".to_string(),
            id_etnia:"0".to_string(),
            id_lengua:"0".to_string(),
            id_religion:"".to_string(),
            es_migrante:"".to_string(),
            id_estatus_migratorio:"0".to_string(),
            es_lgbttti:"".to_string(),
            es_servidor_publico:"".to_string(),
            es_defensor_dh:"".to_string(),
            es_periodista:"".to_string(),
            es_sindicalista:"".to_string(),
            es_ong:"".to_string(),
            id_hipotesis_no_localizacion:"0".to_string(),
            id_delito:"0".to_string()
        }
    }

    /// Genera tuplas con los valores de los parámetros.
    pub fn to_tuples(&self) -> Vec<(&str,&str)>{
        vec![
            ("titulo",&self.titulo),
            ("idEstatusVictima",&self.id_estatus_victima),
            ("fechaInicio",&self.fecha_inicio),
            ("fechaFin",&self.fecha_fin),
            ("idEstado",&self.id_estado),
            ("idMunicipio",&self.id_municipio),
            ("mostrarFechaNula",&self.mostrar_fecha_nula),
            ("idColonia",&self.id_colonia),
            ("idNacionalidad",&self.id_nacionalidad),
            ("edadInicio",&self.edad_inicio),
            ("edadFin",&self.edad_fin),
            ("mostrarEdadNula",&self.mostrar_edad_nula),
            ("idHipotesis",&self.id_hipotesis),
            ("idMedioConocimiento",&self.id_medio_conocimiento),
            ("idCircunstancia",&self.id_circunstancia),
            ("tieneDiscapacidad",&self.tiene_discapacidad),
            ("idTipoDiscapacidad",&self.id_tipo_discapacidad),
            ("idEtnia",&self.id_etnia),
            ("idLengua",&self.id_lengua),
            ("idReligion",&self.id_religion),
            ("esMigrante",&self.es_migrante),
            ("idEstatusMigratorio",&self.id_estatus_migratorio),
            ("esLgbttti",&self.es_lgbttti),
            ("esServidorPublico",&self.es_servidor_publico),
            ("esDefensorDH",&self.es_defensor_dh),
            ("esPeriodista",&self.es_periodista),
            ("esSindicalista",&self.es_sindicalista),
            ("esONG",&self.es_ong),
            ("idHipotesisNoLocalizacion",&self.id_hipotesis_no_localizacion),
            ("idDelito",&self.id_delito)]
    }
}

/// Función utilitaria para escribir los diccionarios en archivos
///
/// # Argumentos
///
/// * `categorias` - Mapa con las categorías y sus valores asociados.
/// * `cola` - ruta del archivo en el cual se escribirá el diccionario
pub fn exportar_categorias(categorias: &BTreeMap<String,String>, ruta: &str) -> Result<(), Box<dyn Error>> {
        
    let mut salida = File::create(ruta)?;
    let j = serde_json::to_string(categorias)?;
    write!(salida, "{}", j)?;

    Ok(())
}


/// Función para obtener el catálogo de estatus de víctimas
pub fn get_estatus_victimas() -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let mut mapa = BTreeMap::new();

    mapa.insert("PERSONAS DESAPARECIDAS, NO LOCALIZADAS Y LOCALIZADAS".to_string(),"0".to_string());
    mapa.insert("PERSONAS LOCALIZADAS CON VIDA".to_string(),"2".to_string());
    mapa.insert("PERSONAS LOCALIZADAS SIN VIDA".to_string(),"3".to_string());
    mapa.insert("PERSONAS DESAPARECIDAS".to_string(),"4".to_string());
    mapa.insert("PERSONAS NO LOCALIZADAS".to_string(),"5".to_string());
    mapa.insert("PERSONAS LOCALIZADAS".to_string(),"6".to_string());
    mapa.insert("PERSONAS DESAPARECIDAS Y NO LOCALIZADAS".to_string(),"7".to_string());

    Ok(mapa)

}

/// Función para obtener el catálogo de estados
///
/// # Argumentos
///
/// * `cliente` - Cliente a usar para extraer el catálogo
pub fn get_estados(cliente: &Client) -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let mut mapa = BTreeMap::new();
    let edourl = urls::estados_url();

    let edo_resp = match cliente.post(edourl).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de estados no se concretó - {}", err)))
    };

    let estados: Vec<OptionSelect> = edo_resp.json()?;

    estados.iter().for_each(|opcion| {
        mapa.insert(opcion.text.to_string(),opcion.value.to_string());
    });

    if mapa.is_empty() {
        return Err(From::from("No se obtuvieron datos"));
    };

    Ok(mapa)
    
}

/// Función para obtener el catálogo espacial, de todos los estados, municipios y colonias, anidados. Esta función es iterativa y tarda mucho en ejecutarse pues hace muchas peticiones a la API. Cuidado al usarla. Una vez generado el catálogo puede exportarse como un diccionario y sacarse a un archivo con la función "exportar_espacial()"
///
/// # Argumentos
///
/// * `cliente` - Cliente a usar para extraer el catálogo
pub fn get_all_espacial(cliente: &Client) -> Result<Vec<Estado>,Box<dyn Error>>{

    let mut edovec: Vec<Estado> = Vec::new();

    let edourl = urls::estados_url();
    let munurl = urls::municipios_url();
    let colurl = urls::colonias_url();

    let edo_resp = match cliente.post(edourl).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de estados no se concretó - {}", err)))
    };

    let estados: Vec<OptionSelect> = edo_resp.json()?;

    for estado in estados {
        let edo_value = estado.value.to_string();
        let edo_text = estado.text.to_string();
        let edo_clave = format!("{:0>2}", edo_value);
        let mut munvec: Vec<Municipio> = Vec::new();
        match edo_value.as_str() {
            "0" => {},
            _ => {
                let params = [("idEstado", edo_value.clone())];
                let mun_resp = match cliente.post(munurl.clone()).form(&params).send() {
                    Ok(respuesta) => respuesta,
                    Err(err) => return Err(From::from(format!("La petición de municipios no se concretó - {}", err)))
                };
                let municipios: Vec<OptionSelect> = mun_resp.json()?;

                for municipio in municipios {
                    let mun_value = municipio.value.to_string();
                    let mun_text = municipio.text.to_string();
                    let mut mun_clave = edo_clave.to_string();
                    mun_clave.push_str(&format!("{:0>3}", mun_value));
                    let mut colvec: Vec<Colonia> = Vec::new();
                    match mun_value.as_str() {
                        "0" => {},
                        _ => {
                            let params = [("idEstado", edo_value.clone()),("idMunicipio", mun_value.clone())];
                            let col_resp = match cliente.post(colurl.clone()).form(&params).send() {
                                Ok(respuesta) => respuesta,
                                Err(err) => return Err(From::from(format!("La petición de colonias no se concretó - {}", err)))
                            };
                            let colonias: Vec<OptionSelect> = col_resp.json()?;

                            for colonia in colonias {
                                let col_value = colonia.value.to_string();
                                let col_text = colonia.text.to_string();
                                let mut col_clave = mun_clave.to_string();
                                col_clave.push_str("-");
                                col_clave.push_str(&col_value);
                                match col_value.as_str() {
                                    "0" => {},
                                    _ => {
                                        let colst = Colonia {
                                            text: col_text,
                                            value: col_value,
                                            clave: col_clave,
                                        };
                                        colvec.push(colst);
                                    }
                                }
                            }

                            let munst = Municipio {
                                text: mun_text,
                                value: mun_value,
                                clave: mun_clave,
                                subunidades: colvec,
                            };
                            munvec.push(munst);
                        }
                    }
                }
                
                //println!("{}",edo_text);
                let edost = Estado {
                    text: edo_text,
                    value: edo_value,
                    clave: edo_clave,
                    subunidades: munvec,
                };
                edovec.push(edost);
                
            }
        }
    }

    Ok(edovec)
}

/// Función para exportar a un archivo el catálogo espacial anidado generado por la función "get_all_espacial()"
///
/// # Argumentos
///
/// * `espacial` - estructura que alberga el catálogo espacial anidado
/// * `ruta` - ruta del archivo al que se exportará el catálogo
pub fn exportar_espacial(espacial: &Vec<Estado>, ruta: &str) -> Result<(), Box<dyn Error>> {
        
    let mut salida = File::create(ruta)?;
    let j = serde_json::to_string(espacial)?;
    write!(salida, "{}", j)?;

    Ok(())
}

/// Estructura utilitaria para almacenar los datos de una colonia
#[derive(Deserialize, Serialize, Debug)]
struct Colonia {
    text: String,
    clave: String,
    value: String, 
}

/// Estructura utilitaria para almacenar los datos de un municipio
#[derive(Deserialize, Serialize, Debug)]
struct Municipio {
    text: String,
    clave: String,
    value: String,
    subunidades: Vec<Colonia>
}

/// Estructura utilitaria para almacenar los datos de un estado
#[derive(Deserialize, Serialize, Debug)]
pub struct Estado {
    text: String,
    clave: String,
    value: String,
    subunidades: Vec<Municipio>
}

/// Función para obtener el catálogo de municipios correspondiente a un estado. El estado debe ser un valor válido en los catálogos de estados de la API
///
/// # Argumentos
///
/// * `cliente` - Cliente a usar para extraer el catálogo
/// * `estado` - estado del cual se desean los municipios
pub fn get_municipios(cliente: &Client, estado: &str) -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let mut mapa = BTreeMap::new();
    let munurl = urls::municipios_url();

    let params = [("idEstado", estado)];
    let mun_resp = match cliente.post(munurl).form(&params).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de municipios no se concretó - {}", err)))
    };

    let municipios: Vec<OptionSelect> = mun_resp.json()?;

    municipios.iter().for_each(|opcion| {
        mapa.insert(opcion.text.to_string(),opcion.value.to_string());
    });

    if mapa.is_empty() {
        return Err(From::from("No se obtuvieron datos"));
    };

    Ok(mapa)
}

/// Función para obtener el catálogo de colonias correspondiente a un municipio correspondiente a un estado. El estado y el municipio deben ser valores válidos en los catálogos de estados y de minucipios de la API
///
/// # Argumentos
///
/// * `cliente` - Cliente a usar para extraer el catálogo
/// * `estado` - estado del cual se desean los municipios
/// * `municipio` - municipio del cual se desean las colonias
pub fn get_colonias(cliente: &Client, estado: &str, municipio: &str) -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let mut mapa = BTreeMap::new();
    let colurl = urls::colonias_url();

    let params = [("idEstado", estado),("idMunicipio", municipio)];
    let col_resp = match cliente.post(colurl).form(&params).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de colonias no se concretó - {}", err)))
    };

    let colonias: Vec<OptionSelect> = col_resp.json()?;

    colonias.iter().for_each(|opcion| {
        mapa.insert(opcion.text.to_string(),opcion.value.to_string());
    });

    if mapa.is_empty() {
        return Err(From::from("No se obtuvieron datos"));
    };

    Ok(mapa)
}

/// Función para obtener el catálogo de nacionalidades
///
/// # Argumentos
///
/// * `cliente` - Cliente a usar para extraer el catálogo
pub fn get_nacionalidades(cliente: &Client) -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let mut mapa = BTreeMap::new();
    let nacurl = urls::nacionalidades_url();

    let nac_resp = match cliente.post(nacurl).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de nacionalidades no se concretó - {}", err)))
    };

    let nacionalidades: Vec<OptionSelect> = nac_resp.json()?;

    nacionalidades.iter().for_each(|opcion| {
        mapa.insert(opcion.text.to_string(),opcion.value.to_string());
    });

    if mapa.is_empty() {
        return Err(From::from("No se obtuvieron datos"));
    };

    Ok(mapa)
    
}

/// Función para obtener el catálogo de hipótesis de la desaparición
///
/// # Argumentos
///
/// * `cliente` - Cliente a usar para extraer el catálogo
pub fn get_hipotesis(cliente: &Client) -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let mut mapa = BTreeMap::new();
    let hipurl = urls::hipotesis_url();

    let hip_resp = match cliente.post(hipurl).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de hipotesis no se concretó - {}", err)))
    };

    let hipotesis: Vec<OptionSelect> = hip_resp.json()?;

    hipotesis.iter().for_each(|opcion| {
        mapa.insert(opcion.text.to_string(),opcion.value.to_string());
    });

    if mapa.is_empty() {
        return Err(From::from("No se obtuvieron datos"));
    };

    Ok(mapa)
    
}

/// Función para obtener el catálogo de delitos asociados a la desaparición
///
/// # Argumentos
///
/// * `cliente` - Cliente a usar para extraer el catálogo
pub fn get_delitos(cliente: &Client) -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let mut mapa = BTreeMap::new();
    let delurl = urls::delitos_url();

    let del_resp = match cliente.post(delurl).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de delitos no se concretó - {}", err)))
    };

    let delitos: Vec<OptionSelect> = del_resp.json()?;

    delitos.iter().for_each(|opcion| {
        mapa.insert(opcion.text.to_string(),opcion.value.to_string());
    });

    if mapa.is_empty() {
        return Err(From::from("No se obtuvieron datos"));
    };

    Ok(mapa)
    
}

/// Función para obtener el catálogo de medios por los que se dio a conocer la desaparición
///
/// # Argumentos
///
/// * `cliente` - Cliente a usar para extraer el catálogo
pub fn get_medios(cliente: &Client) -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let mut mapa = BTreeMap::new();
    let medurl = urls::medios_url();

    let med_resp = match cliente.post(medurl).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de medios no se concretó - {}", err)))
    };

    let medios: Vec<OptionSelect> = med_resp.json()?;

    medios.iter().for_each(|opcion| {
        mapa.insert(opcion.text.to_string(),opcion.value.to_string());
    });

    if mapa.is_empty() {
        return Err(From::from("No se obtuvieron datos"));
    };

    Ok(mapa)
    
}

/// Función para obtener el catálogo de circunstancias en las que se dió la desaparición
///
/// # Argumentos
///
/// * `cliente` - Cliente a usar para extraer el catálogo
pub fn get_circunstancias(cliente: &Client) -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let mut mapa = BTreeMap::new();
    let cirurl = urls::circunstancias_url();

    let cir_resp = match cliente.post(cirurl).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de circunstancias no se concretó - {}", err)))
    };

    let circunstancias: Vec<OptionSelect> = cir_resp.json()?;

    circunstancias.iter().for_each(|opcion| {
        mapa.insert(opcion.text.to_string(),opcion.value.to_string());
    });

    if mapa.is_empty() {
        return Err(From::from("No se obtuvieron datos"));
    };

    Ok(mapa)
    
}

/// Función para obtener el catálogo de discapacidades
///
/// # Argumentos
///
/// * `cliente` - Cliente a usar para extraer el catálogo
pub fn get_discapacidades(cliente: &Client) -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let mut mapa = BTreeMap::new();
    let disurl = urls::discapacidades_url();

    let dis_resp = match cliente.post(disurl).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de discapacidades no se concretó - {}", err)))
    };

    let discapacidades: Vec<OptionSelect> = dis_resp.json()?;

    discapacidades.iter().for_each(|opcion| {
        mapa.insert(opcion.text.to_string(),opcion.value.to_string());
    });

    if mapa.is_empty() {
        return Err(From::from("No se obtuvieron datos"));
    };

    Ok(mapa)
    
}

/// Función para obtener el catálogo de etnias
///
/// # Argumentos
///
/// * `cliente` - Cliente a usar para extraer el catálogo
pub fn get_etnias(cliente: &Client) -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let mut mapa = BTreeMap::new();
    let etnurl = urls::etnias_url();

    let etn_resp = match cliente.post(etnurl).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de etnias no se concretó - {}", err)))
    };

    let etnias: Vec<OptionSelect> = etn_resp.json()?;

    etnias.iter().for_each(|opcion| {
        mapa.insert(opcion.text.to_string(),opcion.value.to_string());
    });

    if mapa.is_empty() {
        return Err(From::from("No se obtuvieron datos"));
    };

    Ok(mapa)
    
}

/// Función para obtener el catálogo de lenguas
///
/// # Argumentos
///
/// * `cliente` - Cliente a usar para extraer el catálogo
pub fn get_lenguas(cliente: &Client) -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let mut mapa = BTreeMap::new();
    let lenurl = urls::lenguas_url();

    let len_resp = match cliente.post(lenurl).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de lenguas no se concretó - {}", err)))
    };

    let lenguas: Vec<OptionSelect> = len_resp.json()?;

    lenguas.iter().for_each(|opcion| {
        mapa.insert(opcion.text.to_string(),opcion.value.to_string());
    });

    if mapa.is_empty() {
        return Err(From::from("No se obtuvieron datos"));
    };

    Ok(mapa)
    
}

/// Función para obtener el catálogo de religiones
///
/// # Argumentos
///
/// * `cliente` - Cliente a usar para extraer el catálogo
pub fn get_religiones(cliente: &Client) -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let mut mapa = BTreeMap::new();
    let relurl = urls::religiones_url();

    let rel_resp = match cliente.post(relurl).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de religiones no se concretó - {}", err)))
    };

    let religiones: Vec<OptionSelect> = rel_resp.json()?;

    religiones.iter().for_each(|opcion| {
        mapa.insert(opcion.text.to_string(),opcion.value.to_string());
    });

    if mapa.is_empty() {
        return Err(From::from("No se obtuvieron datos"));
    };

    Ok(mapa)
    
}

/// Función para obtener el catálogo de estatus migratorios
///
/// # Argumentos
///
/// * `cliente` - Cliente a usar para extraer el catálogo
pub fn get_emigratorios(cliente: &Client) -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let mut mapa = BTreeMap::new();
    let emiurl = urls::emigratorio_url();

    let emi_resp = match cliente.post(emiurl).send() {
        Ok(respuesta) => respuesta,
        Err(err) => return Err(From::from(format!("La petición de estatus migratorios no se concretó - {}", err)))
    };

    let estatus: Vec<OptionSelect> = emi_resp.json()?;

    estatus.iter().for_each(|opcion| {
        mapa.insert(opcion.text.to_string(),opcion.value.to_string());
    });

    if mapa.is_empty() {
        return Err(From::from("No se obtuvieron datos"));
    };

    Ok(mapa)
    
}

/// Estructura utilitaria para escribir los diccionarios a archivos
#[derive(Deserialize,Debug)]
#[serde(rename_all = "PascalCase")]
struct OptionSelect {
    text: String,
    value: i32,
}
    