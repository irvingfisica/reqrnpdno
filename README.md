# Crate para consumir los datos públicos de desaparecidos en México

El crate contiene funciones para poder consumir los datos de la [Versión Pública RNPDNO](https://versionpublicarnpdno.segob.gob.mx/Dashboard/Index).

Los datos de desaparecidos en México son públcios, sin embargo la única manera de consumirlos es a través de un sitio web en el cual se pueden introducir filtros y obtener los valores en la base de datos que satisfacen los filtros. Además la web no tiene un acceso a los datos, solo realiza algunas gráficas a partir de los datos obtenidos. Esto hace que el obtener información sea un proceso repetitivo y complejo, además el sitio está protegido por un Captcha que hace más lento el proceso de realizar peticiones a la base de datos. 

Con este crate es posible consumir de forma sistemática los datos de la base de datos sin pasar por el sitio web y el captcha.