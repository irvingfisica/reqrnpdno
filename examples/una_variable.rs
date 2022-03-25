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