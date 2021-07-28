const URL_BASE: &str = "https://versionpublicarnpdno.segob.gob.mx/";

pub fn url_base() -> String {
    URL_BASE.to_string()
}

fn mix_urls(url_base: &String, cola: &str) -> String {
    let mut cadena = url_base.clone();
    cadena.push_str(cola);
    cadena
}

pub fn estados_url() -> String {
    mix_urls(&url_base(), "Catalogo/Estados")
}

pub fn municipios_url() -> String {
    mix_urls(&url_base(), "Catalogo/Municipios")
}

pub fn colonias_url() -> String {
    mix_urls(&url_base(), "Catalogo/Colonias")
}