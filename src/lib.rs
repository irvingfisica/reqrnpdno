//! REQRNPDNO
//! 
//! REQRNPDNO permite consumir los datos de la [Versión Pública RNPDNO](https://versionpublicarnpdno.segob.gob.mx/Dashboard/Index).
//! 
//! Los datos de desaparecidos en México son públcios, sin embargo la única manera de consumirlos es a través de un sitio web en el cual se pueden introducir filtros y obtener los valores en la base de datos que satisfacen los filtros. Además la web no tiene un acceso a los datos, solo realiza algunas gráficas a partir de los datos obtenidos. Esto hace que el obtener información sea un proceso repetitivo y complejo, además el sitio está protegido por un Captcha que hace más lento el proceso de realizar peticiones a la base de datos. 
//! Con este crate es posible consumir de forma sistemática los datos de la base de datos sin pasar por el sitio web y el captcha.
//! 
//! Este crate tiene codificado en funciones de alto nivel 4 acciones:
//! - Generación de diccionarios a través de la función "get_diccionarios()"
//! - Petición general de datos codificando filtros en la estructura Parametros a través de la función "extraer()"
//! - Petición iterada de datos sobre una variable de interes a través de las funciones de alto nivel "extraer_por_..."
//! - Petición que extrae todos los datos en su nivel de desagregación más completo, iterando sobre todas las variables posibles en los filtros y sus valores. a través de la función "extraer_todo_iterando()"
//! 
//! Generar diccionarios debería de hacerse solo una vez y guardarlos. El repositorio de este crate contiene los diccionarios por si no quieres ejecutar la función.
//! 
//! Para el resto de las acciones el procedimiento es el mismo:
//! 1) Determinar una ruta para escribir los archivos
//! 2) Crear una estructura con los parámetros a utilizar en las peticiones
//! 3) Modificar los parámetros para crear el filtro deseado
//! 4) Realizar la petición
//! El módulo 'extractora' proporciona funciones de alto nivel para realizar peticiones generales a la base de datos las cuales pueden ser filtradas a partir de una estructura general con los parámetros de la API.
//! 
//! Todas las peticiones que se realizan con las funciones de alto nivel proporcionadas arrojan datos con diferentes niveles de agregación con una sola ejecución.
//! 
//! Los niveles de agregación en cada archivo de salida se corresponden con los niveles de agregación presentados en las gráficas del RNPDNO. En cada sección a excepción de la sección de totales los datos se presentan desagregados por sexo. Actualmente se generan los siguientes:
//! - Sección de totales: Contiene los datos para los diferentes estatus de desaparición en porcentajes y en números absolutos.
//! - Sección espacial: Contiene los datos desagregados por unidad espacial, las unidades espaciales de agregación dependen del nivel seleccionado en el filtro. Así, si el filtro indica una búsqueda nacional la desagregación corresponde a estado, si el filtro indica una búsqueda a nivel estatal la desagregación es municipal y si el filtro indica una búsqueda municipal la desagregación será a nivel colonia.
//! - Sección anual: Contiene los datos desagregados por año de la desaparición.
//! - Sección Mensual, último año: Contiene los datos desagregados por mes de desaparición para los últimos doce meses.
//! - Sección por edad: Contiene los datos desagregados por edad de la persona desaparecida
//! - Sección por nacionalidad: Contiene los datos desagregados por nacionalidad de la persona desaparecida
//! - Sección parámetros: Contiene un resumen de los valores utilizados para el filtro con el que se realizó la búsqueda.
//! 

pub mod urls;
pub mod cliente;
pub mod parameters;
pub mod extractora;
pub mod utilidades;