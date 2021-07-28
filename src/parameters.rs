use std::collections::BTreeMap;
use std::error::Error;
use serde::{Deserialize};
use reqwest::blocking::Client;
use crate::urls;

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