use reqrnpdno::{extractora,parameters::Parametros,utilidades};
use extractora::Iteradora;
use std::time::Instant;

fn main () {

    // Aquí se define la ruta en donde se guardarán los datos. 
    // En este caso es un directorio llamado "30-nov-2021" que aún no existe pero que crearemos en el directorio "salida". 
    // Para crearlo usamos la función de ayuda "crear_directorio()" para la cual el primer parámetro es la ruta objetivo y el segundo parámetro es el nombre del directorio a crear. 
    let ruta_salida = utilidades::crear_directorio("./salida/", "25-mar-2022_b_f_d").unwrap();

    // Aquí creamos la estructura de los parámetros necesarios para realizar la petición. 
    // Esta estructura es la que se necesita modificar si quieres aplicar algún filtro. 
    // Si no quieres filtrar no es necesario modificar la estructura.
    let mut parametros = Parametros::new();

    parametros.id_estatus_victima = "7".to_string();
    parametros.titulo = "PERSONAS DESAPARECIDAS Y NO LOCALIZADAS".to_string();
    parametros.fecha_inicio = "2018-01-20".to_string();
    parametros.fecha_fin = "2018-01-20".to_string();

    // Aquí creamos la estructura de variables a iterar. 
    // Esta estructura es la que se necesita modificar si se necesita iterar por otras variables. 
    // La creación activa variables básicas para una petición.
    let variables = Iteradora::new();

    // Para este caso no modificaremos la estructura con los parametros. 

    let start = Instant::now();

    // Por último, utilizamos la función de alto nivel "extraer_todo_iterando" para obtener nuestros datos.
    extractora::extraer_iterando(&parametros, &ruta_salida, &variables).unwrap();

    let duration = start.elapsed();
    println!("Tiempo empleado para realizar la petición: {:?}", duration);

}