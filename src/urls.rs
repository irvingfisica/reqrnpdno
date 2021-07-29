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

pub fn nacionalidades_url() -> String {
    mix_urls(&url_base(), "Catalogo/Nacionalidades")
}

pub fn hipotesis_url() -> String {
    mix_urls(&url_base(), "Catalogo/Hipotesis")
}

pub fn delitos_url() -> String {
    mix_urls(&url_base(), "Catalogo/Delito")
}

pub fn medios_url() -> String {
    mix_urls(&url_base(), "Catalogo/MediosConocimiento")
}

pub fn circunstancias_url() -> String {
    mix_urls(&url_base(), "Catalogo/Circunstancias")
}

pub fn discapacidades_url() -> String {
    mix_urls(&url_base(), "Catalogo/TiposDiscapacidad")
}

pub fn etnias_url() -> String {
    mix_urls(&url_base(), "Catalogo/Etnias")
}

pub fn lenguas_url() -> String {
    mix_urls(&url_base(), "Catalogo/Lenguas")
}

pub fn religiones_url() -> String {
    mix_urls(&url_base(), "Catalogo/Religiones")
}

pub fn emigratorio_url() -> String {
    mix_urls(&url_base(), "Catalogo/EstatusMigratorio")
}

pub fn totales_url() -> String {
    mix_urls(&url_base(), "Sociodemografico/Totales")
}

pub fn por_estado_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/BarChartSexoEstados")
}

pub fn por_municipio_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/BarChartSexoMunicipio")
}

pub fn por_colonia_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/BarChartSexoColonia")
}

pub fn por_anio_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/AreaChartSexoAnio")
}

pub fn ultimo_anio_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/BarChartSexoAnioMeses")
}

pub fn rango_edad_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/AreaChartSexoRango")
}

pub fn nacionalidad_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/BarChartSexoNacionalidad")
}

pub fn fiscalias_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/BarChartFoliosIniciadosActualizadosFiscalias")
}

pub fn comisiones_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/BarChartFoliosIniciadosActualizadosComisiones")
}

pub fn portal_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/BarChartFoliosPortal")
}

