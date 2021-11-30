# Crate para consumir los datos públicos de desaparecidos en México

El crate contiene funciones para poder consumir los datos de la [Versión Pública RNPDNO](https://versionpublicarnpdno.segob.gob.mx/Dashboard/Index).

Los datos de desaparecidos en México son públcios, sin embargo la única manera de consumirlos es a través de un sitio web en el cual se pueden introducir filtros y obtener los valores en la base de datos que satisfacen los filtros. Además la web no tiene un acceso a los datos, solo realiza algunas gráficas a partir de los datos obtenidos. Esto hace que el obtener información sea un proceso repetitivo y complejo, además el sitio está protegido por un Captcha que hace más lento el proceso de realizar peticiones a la base de datos. 

Con este crate es posible consumir de forma sistemática los datos de la base de datos sin pasar por el sitio web y el captcha.

### Operaciones básicas

Este crate tiene codificado en funciones de alto nivel 4 acciones:
- Generación de diccionarios a través de la función "get_diccionarios()"
- Petición general de datos codificando filtros en la estructura Parametros a través de la función "extraer()"
- Petición iterada de datos sobre una variable de interes a través de las funciones de alto nivel "extraer_por_..."
- Petición que extrae todos los datos en su nivel de desagregación más completo, iterando sobre todas las variables posibles en los filtros y sus valores. a través de la función "extraer_todo_iterando()"

### Secciones en los datos obtenidos

El módulo 'extractora' proporciona funciones de alto nivel para realizar peticiones generales a la base de datos las cuales pueden ser filtradas a partir de una estructura general con los parámetros de la API.

Todas las peticiones que se realizan con las funciones de alto nivel proporcionadas arrojan datos con diferentes niveles de agregación con una sola ejecución.

Los niveles de agregación en cada archivo de salida se corresponden con los niveles de agregación presentados en las gráficas del RNPDNO. En cada sección a excepción de la sección de totales los datos se presentan desagregados por sexo. Actualmente se generan los siguientes:
- Sección de totales: Contiene los datos para los diferentes estatus de desaparición en porcentajes y en números absolutos.
- Sección espacial: Contiene los datos desagregados por unidad espacial, las unidades espaciales de agregación dependen del nivel seleccionado en el filtro. Así, si el filtro indica una búsqueda nacional la desagregación corresponde a estado, si el filtro indica una búsqueda a nivel estatal la desagregación es municipal y si el filtro indica una búsqueda municipal la desagregación será a nivel colonia.
- Sección anual: Contiene los datos desagregados por año de la desaparición.
- Sección Mensual, último año: Contiene los datos desagregados por mes de desaparición para los últimos doce meses.
- Sección por edad: Contiene los datos desagregados por edad de la persona desaparecida
- Sección por nacionalidad: Contiene los datos desagregados por nacionalidad de la persona desaparecida
- Sección parámetros: Contiene un resumen de los valores utilizados para el filtro con el que se realizó la búsqueda.

### Procedimiento básico para hacer una petición

Generar diccionarios debería de hacerse solo una vez y guardarlos. El repositorio de este crate contiene los diccionarios por si no quieres ejecutar la función.

Para el resto de las acciones el procedimiento es el mismo:
1) Determinar una ruta para escribir los archivos
2) Crear una estructura con los parámetros a utilizar en las peticiones
3) Modificar los parámetros para crear el filtro deseado
4) Realizar la petición

## Ejemplos de peticiones

Para ejecutar los siguientes ejemplos es necesario instalar Rust en tu máquina siguiendo estas [instrucciones](https://www.rust-lang.org/tools/install). 

Después necesitas descargar este repositorio. Si abres el archivo main.rs en el directorio src podrás ver el primer ejemplo. Para poder ejecutarlo necesitas en tu terminal entrar a la carpeta del proyecto, compilar y ejecutar el archivo a través de la instrucción "cargo run --release".

Si quieres probar ejemplos más complejos tendrás que cambiar el contenido del archivo main.rs por el del nuevo ejemplo o por el de tu código

### Ejemplo de petición simple

El siguiente ejemplo muestra como realizar una petición simple. 

~~~rust
use reqrnpdno::{extractora,parameters::Parametros};

fn main () {

    // Aquí se define la ruta en donde se guardarán los datos. En este caso es un archivo de tipo JSON llamado datos.json
    let rutam = "./datos.json".to_string();

    // Aquí creamos la estructura de los parámetros necesarios para realizar la petición. Esta estructura es la que se necesita modificar si quieres aplicar algún filtro. Si no quieres filtrar no es necesario modificar la estructura. 
    let mut parametros = Parametros::new();

    // Aquí se modifican los valores de los parámetros para aplicar algunos filtros, en este caso se coloca el valor "7" en el campo "id_estatus_victima" el cual corresponde a "PERSONAS DESAPARECIDAS Y NO LOCALIZADAS". También cambiamos el parámetro "titulo" y colocamos el título. Además usamos los parámetros "fecha_inicio" y "fecha_fin" para limitar la búsqueda a un rango de fechas.
    parametros.id_estatus_victima = "7".to_string();
    parametros.titulo = "PERSONAS DESAPARECIDAS Y NO LOCALIZADAS".to_string();
    parametros.fecha_inicio = "2000-01-01".to_string();
    parametros.fecha_fin = "2021-08-20".to_string();

    // Por último, utilizamos la función de alto nivel "extraer" para obtener nuestros datos.
    extractora::extraer(&parametros, &rutam).unwrap();

}
~~~