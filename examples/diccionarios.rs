use reqrnpdno::extractora;

fn main () {

    // Definimos la ruta donde se escribirán los diccionarios, en este caso la ruta es un directorio.
    let ruta_salida = "./".to_string();

    // Obtener los diccionarios
    extractora::get_diccionarios(&ruta_salida).unwrap();

}