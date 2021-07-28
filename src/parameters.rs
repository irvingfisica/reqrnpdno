use std::collections::BTreeMap;
use std::error::Error;
use serde::{Deserialize};
use reqwest::blocking::Client;
use crate::urls;

pub struct Parametros {
    pub titulo: String,
    pub id_estatus_victima: String,
    pub fecha_inicio: String,
    pub fecha_fin: String,
    pub id_estado: String,
    pub id_municipio: String,
    pub mostrar_fecha_nula: String,
    pub id_colonia: String,
    pub id_nacionalidad: String,
    pub edad_inicio: String,
    pub edad_fin: String,
    pub mostrar_edad_nula: String,
    pub id_hipotesis: String,
    pub id_medio_conocimiento: String,
    pub id_circunstancia: String,
    pub tiene_discapacidad: String, 
    pub id_tipo_discapacidad: String,
    pub id_etnia: String,
    pub id_lengua: String,
    pub id_religion: String,
    pub es_migrante: String,
    pub id_estatus_migratorio: String,
    pub es_lgbttti: String,
    pub es_servidor_publico: String,
    pub es_defensor_dh: String,
    pub es_periodista: String,
    pub es_sindicalista: String,
    pub es_ong: String,
    pub id_hipotesis_no_localizacion:String,
    pub id_delito: String
} 

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

pub fn get_estados(cliente: &Client) -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let mut mapa = BTreeMap::new();
    let edourl = urls::estados_url();

    let edo_resp = cliente.post(edourl).send()?;

    let estados: Vec<OptionSelect> = edo_resp.json()?;

    estados.iter().for_each(|opcion| {
        mapa.insert(opcion.text.to_string(),opcion.value.to_string());
    });

    if mapa.is_empty() {
        return Err(From::from("No se obtuvieron datos"));
    };

    Ok(mapa)
    
}

pub fn get_municipios(cliente: &Client, estado: &str) -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let mut mapa = BTreeMap::new();
    let munurl = urls::municipios_url();

    let params = [("idEstado", estado)];
    let mun_resp = cliente.post(munurl).form(&params).send()?;

    let municipios: Vec<OptionSelect> = mun_resp.json()?;

    municipios.iter().for_each(|opcion| {
        mapa.insert(opcion.text.to_string(),opcion.value.to_string());
    });

    if mapa.is_empty() {
        return Err(From::from("No se obtuvieron datos"));
    };

    Ok(mapa)
}

pub fn get_colonias(cliente: &Client, estado: &str, municipio: &str) -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let mut mapa = BTreeMap::new();
    let colurl = urls::colonias_url();

    let params = [("idEstado", estado),("idMunicipio", municipio)];
    let col_resp = cliente.post(colurl).form(&params).send()?;

    let colonias: Vec<OptionSelect> = col_resp.json()?;

    colonias.iter().for_each(|opcion| {
        mapa.insert(opcion.text.to_string(),opcion.value.to_string());
    });

    if mapa.is_empty() {
        return Err(From::from("No se obtuvieron datos"));
    };

    Ok(mapa)
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct OptionSelect {
    text: String,
    value: i32,
}
    