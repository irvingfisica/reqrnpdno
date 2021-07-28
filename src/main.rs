use reqrnpdno::urls;
use reqrnpdno::cliente;

use std::error::Error;

fn main () {
    run().unwrap();
}

fn run() -> Result<(), Box<dyn Error>> {

    let edourl = urls::estados_url();
    let munurl = urls::municipios_url();

    let cliente = cliente::cliente_nuevo()?;

    let edo_resp = cliente.post(edourl).send()?;

    println!("Respuesta: {:?}", edo_resp.text()?);

    let params = [("idEstado", "33")];
    let mun_resp = cliente.post(munurl).form(&params).send()?;

    println!("Respuesta: {:?}", mun_resp.text()?);
    
    Ok(())
}
