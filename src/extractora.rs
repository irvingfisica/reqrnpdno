use std::error::Error;
use std::collections::BTreeMap;
use reqwest::blocking::Client;
use serde::{Deserialize};
use crate::parameters::Parametros;
use crate::urls;

pub fn totales(cliente: &Client, parametros: &Parametros) -> Result<BTreeMap<String,String>, Box<dyn Error>> {

    let url = urls::totales_url();
    let params = parametros.to_tuples();

    let response = cliente.post(url).form(&params).send()?;
    let mut datos: BTreeMap<String,String> = response.json()?;

    for (_key,value) in datos.iter_mut() {
        *value = value.replace(",","").replace(" %","").to_string();
    }

    Ok(datos)
}

pub fn por_estado(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::por_estado_url();
    let params = parametros.to_tuples();

    let response = cliente.post(url).form(&params).send()?;
    let datos: Data = response.json()?;

    Ok(datos)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Data {
    pub series: Vec<Serie>,
    pub x_axis_categories: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct Serie {
    pub name: String,
    pub data: Vec<u32>
}
