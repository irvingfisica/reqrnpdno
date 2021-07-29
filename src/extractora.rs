use std::error::Error;
use std::collections::BTreeMap;
use reqwest::blocking::Client;
use serde::{Deserialize};
use scraper::{Html,Selector};
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

pub fn por_edades_completo(cliente: &Client, parametros: &Parametros) -> Result<Data, Box<dyn Error>> {

    let url = urls::tabla_detalle_url();
    let mut params = parametros.to_tuples();

    params.push(("TipoDetalle","5"));

    let response = cliente.post(url).form(&params).send()?;
    let tabla: Tabla = response.json()?;
    
    let datos = parse_table(&tabla)?;

    Ok(datos)
}

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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Tabla {
    pub html: String,
}
