use std::error::Error;
use std::collections::BTreeMap;
use reqwest::blocking::Client;
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
