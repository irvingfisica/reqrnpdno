# Crate para consumir los datos públicos de desaparecidos en México

[![CratesIo](https://img.shields.io/crates/v/reqrnpdno.svg)](https://crates.io/crates/reqrnpdno) [![Documentacion](https://docs.rs/reqrnpdno/badge.svg)](https://docs.rs/reqrnpdno/)

La documentación de todas las funciones la encuentras [aquí](https://docs.rs/reqrnpdno/). Si tienes alguna duda de como usar el crate después de haber leído esta introducción y los ejemplos no dudes en echarme un mensaje por twitter a [@moaimx](https://twitter.com/moaimx)

El crate contiene funciones para poder consumir los datos de la [Versión Pública RNPDNO](https://versionpublicarnpdno.segob.gob.mx/Dashboard/Index).

Los datos de desaparecidos en México son públicos, sin embargo la única manera de consumirlos es a través de un sitio web en el cual se pueden introducir filtros y obtener los valores en la base de datos que satisfacen los filtros. Además la web no tiene un acceso a los datos, solo realiza algunas gráficas a partir de los datos obtenidos. Esto hace que el obtener información sea un proceso repetitivo y complejo, además el sitio está protegido por un Captcha que hace más lento el proceso de realizar peticiones a la base de datos. 

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

Una vez que instalaste Rust hay dos maneras de usar el crate.

La primera es descargando este repositorio. Una vez que lo descargues tienes que entrar al directorio donde se encuentran todos los archivos y en un editor de código como [VSCode](https://code.visualstudio.com/) abrir el archivo main.rs en el directorio src podrás. Al abrirlo notarás que tiene el código del primer ejemplo. Para poder ejecutarlo necesitas en tu terminal entrar a la carpeta del proyecto, compilar y ejecutar el archivo a través de la instrucción "cargo run --release". Al teclear esta instrucción y dar enter se ejecutará el ejemplo. Despues de que ejecute deberás ver el archivo con los datos descargados desde la plataforma del RNPDNO.

Si quieres probar ejemplos más complejos tendrás que cambiar el contenido del archivo main.rs por el del nuevo ejemplo o por el de tu código propio.

La segunda manera es utilizando este crate como crate desde los repositorios de Rust, de esta manera no es necesario descargar nada de este repositorio. Para ello tendras que crear un nuevo proyecto usando cargo. En tu terminal, en la carpeta donde quieras crearlo, ejecuta la instrucción "cargo new nombre_del_proyecto" donde sustituyes nombre_del_proyecto por el nombre que quieras. Luego tendrás que entrar a la carpeta que acaba de crear cargo. Lo puedes hacer con "cd nombre_del_proyecto".

Una vez que estes dentro de la carpeta de tu proyecto necesitas abrir el archivo Cargo.toml y en la sección de "dependencies" agregar la linea:

reqrnpdno = "0.1"

basta con que la pongas debajo. Tu archivo debe verse similar a esto:

~~~toml
[package]
name = "nombre_del_proyecto"
version = "0.1.0"
edition = "2018"

[dependencies]
reqrnpdno = "0.1"
~~~

Ahora, en el archivo main.src debes sustituir el contenido por el contenido del primer ejemplo. Para poder ejecutarlo necesitas en tu terminal entrar a la carpeta del proyecto, compilar y ejecutar el archivo a través de la instrucción "cargo run --release". Al teclear esta instrucción y dar enter se ejecutará el ejemplo. Despues de que ejecute deberás ver el archivo con los datos descargados desde la plataforma del RNPDNO.

Si quieres probar ejemplos más complejos tendrás que cambiar el contenido del archivo main.rs por el del nuevo ejemplo o por el de tu código propio.

### Ejemplo de petición simple

El siguiente ejemplo muestra como realizar una petición simple. 

~~~rust
use reqrnpdno::{extractora,parameters::Parametros};

fn main () {

    // Aquí se define la ruta en donde se guardarán los datos. En este caso es un archivo de tipo JSON llamado datos.json
    let rutam = "./datos.json".to_string();

    // Aquí creamos la estructura de los parámetros necesarios para realizar la petición. 
    // Esta estructura es la que se necesita modificar si quieres aplicar algún filtro. 
    // Si no quieres filtrar no es necesario modificar la estructura. 
    let mut parametros = Parametros::new();

    // Aquí se modifican los valores de los parámetros para aplicar algunos filtros. 
    // En este caso se coloca el valor "7" en el campo "id_estatus_victima" el cual corresponde a "PERSONAS DESAPARECIDAS Y NO LOCALIZADAS". 
    // También cambiamos el parámetro "titulo" y colocamos el título. 
    // Además usamos los parámetros "fecha_inicio" y "fecha_fin" para limitar la búsqueda a un rango de fechas.
    parametros.id_estatus_victima = "7".to_string();
    parametros.titulo = "PERSONAS DESAPARECIDAS Y NO LOCALIZADAS".to_string();
    parametros.fecha_inicio = "2000-01-01".to_string();
    parametros.fecha_fin = "2021-08-20".to_string();

    // Por último, utilizamos la función de alto nivel "extraer" para obtener nuestros datos.
    extractora::extraer(&parametros, &rutam).unwrap();

}
~~~

### Ejemplo de creación de diccionarios

El siguiente ejemplo muestra como crear los diccionarios. Para ejecutar este ejemplo necesitas modificar el archivo main.rs en la carpeta src y colocar lo siguiente:

~~~rust
use reqrnpdno::extractora;

fn main () {

    // Definimos la ruta donde se escribirán los diccionarios, en este caso la ruta es un directorio.
    let ruta_salida = "./".to_string();

    // Obtener los diccionarios
    extractora::get_diccionarios(&ruta_salida).unwrap();

}
~~~

### Ejemplo de petición iterando una variable

A veces queremos los datos desagregados por alguna de las variables con las cuales es posible filtrar en la plataforma del RNPDNO. Es decir, quisieramos iterar sobre todos los valores posibles para una variable. El crate contiene funciones de alto nivel para poder realizar estas peticiones iteradas sin tener que escribir un ciclo. En este ejemplo queremos iterar sobre la variable circunstancias por lo cual usaremos la función "extraer_por_circunstancias()". La salida de todas estas funciones que iteran es un archivo por cada uno de los valores de la variable, por lo cual la ruta necesita ser un directorio. Para ejecutar este ejemplo necesitas modificar el archivo main.rs en la carpeta src y colocar lo siguiente: 

~~~rust
use reqrnpdno::{extractora,parameters::Parametros};

fn main () {

    // Aquí se define la ruta en donde se guardarán los datos. En este caso es el directorio salida. 
    // Puedes cambiar el nombre pero tiene que ser un directorio que exista.
    let ruta_salida = "./salida/".to_string();

    // Aquí creamos la estructura de los parámetros necesarios para realizar la petición. 
    // Esta estructura es la que se necesita modificar si quieres aplicar algún filtro. 
    // Si no quieres filtrar no es necesario modificar la estructura.
    let mut parametros = Parametros::new();

    // Aquí se modifican los valores de los parámetros para aplicar algunos filtros. 
    // En este caso se coloca el valor "7" en el campo "id_estatus_victima" el cual corresponde a "PERSONAS DESAPARECIDAS Y NO LOCALIZADAS". 
    // También cambiamos el parámetro "titulo" y colocamos el título.
    parametros.id_estatus_victima = "7".to_string();
    parametros.titulo = "PERSONAS DESAPARECIDAS Y NO LOCALIZADAS".to_string();

    // Por último, utilizamos la función de alto nivel "extraer_por_circunstancias" para obtener nuestros datos.
    // La salida será una serie de archivos JSON, uno por cada opción en la variable a iterar, en este caso es la variable circunstancias
    extractora::extraer_por_circunstancias(&parametros, &ruta_salida).unwrap();
}
~~~

### Ejemplo de petición iterando sobre todas las variables

Por último mostraremos el uso de la función de alto nivel que nos permite obtener la mayor cantidad de datos, sin embargo antes de que la ejecutes es importante que sepas que esta función hace un número grande de peticiones por lo cual tardará un tiempo largo en ejecutarse por completo. NO USES ESTA FUNCIÓN SOLAMENTE PARA PROBAR, USALA SOLAMENTE SI QUIERES TODOS LOS DATOS QUE GENERA. La función itera sobre todos los valores posibles de las variables con las cuales es posible filtrar en la plataforma del RNPDNO. La salida de esta función es una estructura de directorios en donde se encuentran los datos, por lo cual la ruta necesita ser un directorio. En este ejemplo también se muestra el uso de una función auxiliar que nos permite crear el directorio en donde se descargarán todos los datos. Para ejecutar este ejemplo necesitas modificar el archivo main.rs en la carpeta src y colocar lo siguiente: 

~~~rust
use reqrnpdno::{extractora,parameters::Parametros,utilidades};

fn main () {

    // Aquí se define la ruta en donde se guardarán los datos. 
    // En este caso es un directorio llamado "30-nov-2021" que aún no existe pero que crearemos en el directorio "salida". 
    // Para crearlo usamos la función de ayuda "crear_directorio()" para la cual el primer parámetro es la ruta objetivo y el segundo parámetro es el nombre del directorio a crear. 
    let ruta_salida = utilidades::crear_directorio("./salida/", "30-nov-2021").unwrap();

    // Aquí creamos la estructura de los parámetros necesarios para realizar la petición. 
    // Esta estructura es la que se necesita modificar si quieres aplicar algún filtro. 
    // Si no quieres filtrar no es necesario modificar la estructura.
    let parametros = Parametros::new();

    // Para este caso no modificaremos la estructura con los parametros. 

    // Por último, utilizamos la función de alto nivel "extraer_todo_iterando" para obtener nuestros datos.
    extractora::extraer_todo_iterando(&parametros, &ruta_salida).unwrap();

}
~~~