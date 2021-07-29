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

pub fn por_municipio(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::por_municipio_url();
    let params = parametros.to_tuples();

    let response = cliente.post(url).form(&params).send()?;
    let datos: Data = response.json()?;

    Ok(datos)
}

pub fn por_colonia(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::por_colonia_url();
    let params = parametros.to_tuples();

    let response = cliente.post(url).form(&params).send()?;
    let datos: Data = response.json()?;

    Ok(datos)
}

pub fn por_anio(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::por_anio_url();
    let params = parametros.to_tuples();

    let response = cliente.post(url).form(&params).send()?;
    let datos: Data = response.json()?;

    Ok(datos)
}

pub fn por_mes(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::ultimo_anio_url();
    let params = parametros.to_tuples();

    let response = cliente.post(url).form(&params).send()?;
    let datos: Data = response.json()?;

    Ok(datos)
}

pub fn por_rango_edad(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::rango_edad_url();
    let params = parametros.to_tuples();

    let response = cliente.post(url).form(&params).send()?;
    let datos: Data = response.json()?;

    Ok(datos)
}

pub fn por_nacionalidad(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::nacionalidad_url();
    let params = parametros.to_tuples();

    let response = cliente.post(url).form(&params).send()?;
    let datos: Data = response.json()?;

    Ok(datos)
}

pub fn fiscalias(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::fiscalias_url();
    let params = parametros.to_tuples();

    let response = cliente.post(url).form(&params).send()?;
    let datos: Data = response.json()?;

    Ok(datos)
}

pub fn comisiones(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::comisiones_url();
    let params = parametros.to_tuples();

    let response = cliente.post(url).form(&params).send()?;
    let datos: Data = response.json()?;

    Ok(datos)
}

pub fn portal(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::portal_url();
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

#[derive(Deserialize, Debug)]
pub struct Serie {
    pub name: String,
    pub data: Vec<u32>
}
