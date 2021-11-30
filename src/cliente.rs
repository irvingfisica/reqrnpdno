//! Módulo encargado de generar los clientes que realizan las peticiones a la API.
//! 
use std::error::Error;
use reqwest::blocking;
use reqwest::header::{HeaderMap, CONTENT_LENGTH};
use crate::urls::url_base;

/// Genera un cliente nuevo el cual se inicializa haciendo una petición a la URL base para generar un cookie de sesión
pub fn cliente_nuevo() -> Result<blocking::Client, Box<dyn Error>> {

    let mut def_head = HeaderMap::new();
    def_head.insert(CONTENT_LENGTH, "0".parse().unwrap());

    let client = match blocking::Client::builder()
                    .cookie_store(true)
                    .default_headers(def_head)
                    .build() {
                        Ok(cliente) => cliente,
                        Err(err) => return Err(From::from(format!("El cliente no se pudo construir - {}", err)))
                    };

    match client.get(url_base()).send() {
        Ok(_) => {},
        Err(err) => return Err(From::from(format!("El cliente no se pudo inicializar - {}", err)))
    };

    Ok(client)
}

/// Genera un cliente nuevo para pruebas, este cliente no está inicializado y por lo tanto no incluye datos de cookies por lo cual puede generar problemas al extraer datos. Este cliente es solo para pruebas, Si intentas extraer datos usa la función 'cliente_nuevo()'
pub fn cliente_test() -> Result<blocking::Client, Box<dyn Error>> {

    let mut def_head = HeaderMap::new();
    def_head.insert(CONTENT_LENGTH, "0".parse().unwrap());

    let client = match blocking::Client::builder()
                    .cookie_store(true)
                    .default_headers(def_head)
                    .build() {
                        Ok(cliente) => cliente,
                        Err(err) => return Err(From::from(format!("El cliente no se pudo construir - {}", err)))
                    };

    // let _ = client.get(url_base()).send()?;

    Ok(client)
}