# Culqi-Rust

[![Packagist](https://img.shields.io/packagist/l/doctrine/orm.svg)](https://github.com/culqi/culqi_rust/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/culqi.svg)](https://crates.io/crates/culqi_rust)


Nuestra Biblioteca Rust oficial, es compatible con la v2.0 del Culqi API, con el cual tendrás la posibilidad de realizar cobros con tarjetas de débito y crédito, Yape, PagoEfectivo, billeteras móviles y Cuotéalo con solo unos simples pasos de configuración.

Nuestra biblioteca te da la posibilidad de capturar el `status_code` de la solicitud HTTP que se realiza al API de Culqi, así como el `response` que contiene el cuerpo de la respuesta obtenida.

| Versión actual| Culqi API|
|----|----|
| 1.0.0 (15-08-2023) |v2 [Referencia de API](https://apidocs.culqi.com/)|


## Requisitos

- Rust 1.6.2+
* Afiliate [aquí](https://afiliate.culqi.com/).
* Si vas a realizar pruebas obtén tus llaves desde [aquí](https://integ-panel.culqi.com/#/registro), si vas a realizar transacciones reales obtén tus llaves desde [aquí](https://panel.culqi.com/#/registro).

> Recuerda que para obtener tus llaves debes ingresar a tu CulqiPanel > Desarrollo > ***API Keys***.

![alt tag](http://i.imgur.com/NhE6mS9.png)

> Recuerda que las credenciales son enviadas al correo que registraste en el proceso de afiliación.

* Para encriptar el payload debes generar un id y llave RSA  ingresando a CulqiPanel > Desarrollo  > RSA Keys.

## Instalación

Agregar la siguiente dependencia en tu arhivo `Cargo.toml`:

```toml
[dependencies]
culqi = "0.0.1"
```

En caso de usar linux ejecutar los siguiente comandos:

```bash
sudo apt install libssl-dev
sudo apt install pkg-config
```

## Configuración

Para empezar a enviar peticiones al API de Culqi debes configurar tu llave pública (pk), llave privada (sk).
Para habilitar encriptación de payload debes configurar tu rsa_id y rsa_public_key.

En el archivo /src/lib.rs podemo configurar nuestras llaves.

```rust
const pkey : &'static str = "Ingresa tu llave pública";
const skey : &'static str = "Ingresa tu llave privada";

```

## Encriptar payload

Para encriptar el payload necesitas crear un id RSA y llave RSA, para esto debes ingresa a tu panel y hacer click en la sección “Desarrollo / RSA Keys” de la barra de navegación a la mano izquierda.

Luego declara en variables el id RSA y llave RSA en tu backend, y envialo en las funciones de la librería.

Ejemplo

```rust 

const rsaid : &'static str = "Ingresa tu RSA id";
const CULQI_RSA_KEY: &'static str = "Ingresa tu RSA public key";

 let body = "{\"card_number\":\"4111111111111111\",\"cvv\":\"123\",\"expiration_month\":\"09\",\"expiration_year\":\"2025\",\"email\":\"alexis.pumayalla@culqi.com\",\"metadata\":{\"coment\":\"Tarjeta de prueba alexis\"}}";


        match createEncrypt(body, "tokens") {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
            }
            Err(err) => println!("Error: {:?}", err),
        }
```

## Servicios

### Crear un token

Antes de crear un Cargo o Card es necesario crear un `token` de tarjeta. 
Lo recomendable es generar los 'tokens' con [Culqi Checkout v4](https://docs.culqi.com/es/documentacion/checkout/v4/culqi-checkout/) o [Culqi JS v4](https://docs.culqi.com/es/documentacion/culqi-js/v4/culqi-js/) **debido a que es muy importante que los datos de tarjeta sean enviados desde el dispositivo de tus clientes directamente a los servidores de Culqi**, para no poner en riesgo los datos sensibles de la tarjeta de crédito/débito.

> Recuerda que cuando interactúas directamente con el [API Token](https://apidocs.culqi.com/#tag/Tokens/operation/crear-token) necesitas cumplir la normativa de PCI DSS 3.2. Por ello, te pedimos que llenes el [formulario SAQ-D](https://listings.pcisecuritystandards.org/documents/SAQ_D_v3_Merchant.pdf) y lo envíes al buzón de riesgos Culqi.

```rust
statusCode, res, err := culqi.CreateToken(jsonData)
```

### Crear un cargo

Crear un cargo significa cobrar una venta a una tarjeta. Para esto previamente deberías generar el  `token` y enviarlo en parámetro **source_id**.

Los cargos pueden ser creados vía [API de devolución](https://apidocs.culqi.com/#tag/Cargos/operation/crear-cargo).

```rust
 match create(body, "charges") {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
            }
            Err(err) => println!("Error: {:?}", err),
        }
```

### Crear devolución

Solicita la devolución de las compras de tus clientes (parcial o total) de forma gratuita a través del API y CulqiPanel. 

Las devoluciones pueden ser creados vía [API de devolución](https://apidocs.culqi.com/#tag/Devoluciones/operation/crear-devolucion).

```rust
 match create(body, "charges") {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
            }
            Err(err) => println!("Error: {:?}", err),
        }
```

### Crear un Cliente (customer)

El **cliente** es un servicio que te permite guardar la información de tus clientes. Es un paso necesario para generar una [tarjeta](/es/documentacion/pagos-online/recurrencia/one-click/tarjetas).

Los clientes pueden ser creados vía [API de cliente](https://apidocs.culqi.com/#tag/Clientes/operation/crear-cliente).

```rust
 match create(body, "charges") {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
            }
            Err(err) => println!("Error: {:?}", err),
        }
```

### Crear una tarjeta (card)

La **tarjeta** es un servicio que te permite guardar la información de las tarjetas de crédito o débito de tus clientes para luego realizarles cargos one click o recurrentes (cargos posteriores sin que tus clientes vuelvan a ingresar los datos de su tarjeta).

Las tarjetas pueden ser creadas vía [API de tarjeta](https://apidocs.culqi.com/#tag/Tarjetas/operation/crear-tarjeta).

```rust
 match create(body, "charges") {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
            }
            Err(err) => println!("Error: {:?}", err),
        }
```


### Crear un plan

El plan es un servicio que te permite definir con qué frecuencia deseas realizar cobros a tus clientes.

Un plan define el comportamiento de las suscripciones. Los planes pueden ser creados vía el [API de Plan](https://apidocs.culqi.com/#/planes#create) o desde el **CulqiPanel**.

```rust
 match create(body, "charges") {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
            }
            Err(err) => println!("Error: {:?}", err),
        }
```


### Crear una suscripción (suscription)  

La suscripción es un servicio que asocia la tarjeta de un cliente con un plan establecido por el comercio.

Las suscripciones pueden ser creadas vía [API de suscripción](https://apidocs.culqi.com/#tag/Suscripciones/operation/crear-suscripcion).

```rust
 match create(body, "charges") {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
            }
            Err(err) => println!("Error: {:?}", err),
        }
```


### Crear una orden

Es un servicio que te permite generar una orden de pago para una compra potencial.
La orden contiene la información necesaria para la venta y es usado por el sistema de **PagoEfectivo** para realizar los pagos diferidos.

Las órdenes pueden ser creadas vía [API de orden](https://apidocs.culqi.com/#tag/Ordenes/operation/crear-orden).

```rust
 match create(body, "charges") {
            Ok((response_text, status_code)) => {
                println!("Status Code: {}", status_code);
                println!("Response Text: {}", response_text);
            }
            Err(err) => println!("Error: {:?}", err),
        }

```


## Pruebas

```bash
$ cargo test
$ cargo test tests::test_token_encrypt
```

---

## Documentación

- [Referencia de Documentación](https://docs.culqi.com/)
- [Referencia de API](https://apidocs.culqi.com/)
- [Demo Checkout V4 + Culqi 3DS](https://github.com/culqi/culqi-go-demo-checkoutv4-culqi3ds)
- [Wiki](https://github.com/culqi/culqi-go/wiki)

## Changelog

Todos los cambios en las versiones de esta biblioteca están listados en
[CHANGELOG](CHANGELOG).

## Autor
Team Culqi

## Licencia
El código fuente de culqi-python está distribuido bajo MIT License, revisar el archivo LICENSE.