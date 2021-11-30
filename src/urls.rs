//! Módulo encargado de generar las URLs de consulta a la API.

const URL_BASE: &str = "https://versionpublicarnpdno.segob.gob.mx/";

/// Generador de la URL base
pub fn url_base() -> String {
    URL_BASE.to_string()
}

/// Permite concatenar la URL base con un endpoint para generar una URL compuesta
///
/// # Argumentos
///
/// * `url_base` - URL base.
/// * `cola` - endpoint deseado
fn mix_urls(url_base: &String, cola: &str) -> String {
    let mut cadena = url_base.clone();
    cadena.push_str(cola);
    cadena
}

/// Endpoint del catálogo de estados
pub fn estados_url() -> String {
    mix_urls(&url_base(), "Catalogo/Estados")
}

/// Endpoint del catálogo de municipios
pub fn municipios_url() -> String {
    mix_urls(&url_base(), "Catalogo/Municipios")
}

/// Endpoint del catálogo de colonias
pub fn colonias_url() -> String {
    mix_urls(&url_base(), "Catalogo/Colonias")
}

/// Endpoint del catálogo de nacionalidades
pub fn nacionalidades_url() -> String {
    mix_urls(&url_base(), "Catalogo/Nacionalidades")
}

/// Endpoint del catálogo de hipótesis
pub fn hipotesis_url() -> String {
    mix_urls(&url_base(), "Catalogo/Hipotesis")
}

/// Endpoint del catálogo de delitos
pub fn delitos_url() -> String {
    mix_urls(&url_base(), "Catalogo/Delito")
}

/// Endpoint del catálogo de medios de conocimiento
pub fn medios_url() -> String {
    mix_urls(&url_base(), "Catalogo/MediosConocimiento")
}

/// Endpoint del catálogo de circusntancias
pub fn circunstancias_url() -> String {
    mix_urls(&url_base(), "Catalogo/Circunstancias")
}

/// Endpoint del catálogo de tipos de discapacidad
pub fn discapacidades_url() -> String {
    mix_urls(&url_base(), "Catalogo/TiposDiscapacidad")
}

/// Endpoint del catálogo de etnias
pub fn etnias_url() -> String {
    mix_urls(&url_base(), "Catalogo/Etnias")
}

/// Endpoint del catálogo de lenguas
pub fn lenguas_url() -> String {
    mix_urls(&url_base(), "Catalogo/Lenguas")
}

/// Endpoint del catálogo de religiones
pub fn religiones_url() -> String {
    mix_urls(&url_base(), "Catalogo/Religiones")
}

/// Endpoint del catálogo de estatus migratoria
pub fn emigratorio_url() -> String {
    mix_urls(&url_base(), "Catalogo/EstatusMigratorio")
}

/// Endpoint de datos totales
pub fn totales_url() -> String {
    mix_urls(&url_base(), "Sociodemografico/Totales")
}

/// Endpoint de datos de gráfica de barras por sexo y por estado
pub fn por_estado_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/BarChartSexoEstados")
}

/// Endpoint de datos de gráfica de barras por sexo y por municipio
pub fn por_municipio_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/BarChartSexoMunicipio")
}

/// Endpoint de datos de gráfica de barras por sexo y por colonia
pub fn por_colonia_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/BarChartSexoColonia")
}

/// Endpoint de datos de gráfica de área por sexo y por año
pub fn por_anio_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/AreaChartSexoAnio")
}

/// Endpoint de datos de gráfica de barras por sexo y por meses
pub fn ultimo_anio_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/BarChartSexoAnioMeses")
}

/// Endpoint de datos de gráfica de barras por sexo y por rango de edad
pub fn rango_edad_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/AreaChartSexoRango")
}

/// Endpoint de datos de gráfica de barras por sexo y por nacionalidad
pub fn nacionalidad_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/BarChartSexoNacionalidad")
}

/// Endpoint de datos de gráfica de barras por fiscalía
pub fn fiscalias_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/BarChartFoliosIniciadosActualizadosFiscalias")
}

/// Endpoint de datos de gráfica de barras por comisiones
pub fn comisiones_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/BarChartFoliosIniciadosActualizadosComisiones")
}

/// Endpoint de datos de gráfica de barras por folios
pub fn portal_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/BarChartFoliosPortal")
}

/// Endpoint de datos detallados de tablas
pub fn tabla_detalle_url() -> String {
    mix_urls(&url_base(), "SocioDemografico/TablaDetalle")
}

