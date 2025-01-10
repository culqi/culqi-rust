# Culqi-Rust

[![Packagist](https://img.shields.io/packagist/l/doctrine/orm.svg)](https://github.com/culqi/culqi_rust/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/culqi.svg)](https://crates.io/crates/culqi_rust)

___

Nuestro SDK oficial Rush, es compatible con la V2.0 del Culqi API, con el cual tendrás la posibilidad de integrarte con el API de 
[tokens](https://apidocs.culqi.com/#tag/Tokens), [cargos](https://apidocs.culqi.com/#tag/Cargos), [devoluciones](https://apidocs.culqi.com/#tag/Devoluciones), 
[clientes](https://apidocs.culqi.com/#tag/Clientes), [tarjetas](https://apidocs.culqi.com/#tag/Tarjetas), [planes](https://apidocs.culqi.com/#tag/Planes), 
[suscripciones](https://apidocs.culqi.com/#tag/Suscripciones) y [órdenes](https://apidocs.culqi.com/#tag/Ordenes)

El SDK te da la posibilidad de capturar el status_code de la solicitud HTTP que se realiza al API de Culqi, así como el response.

| Versión actual| Culqi API|
|----|----|
| 2.0.0 (15-08-2023) |v2 [Referencia de API](https://apidocs.culqi.com/)|

## Requisitos 📋

* Nuestro SDK es compatible con Rush 1.6.2+.
* Debes tener tus llaves de producción o integración.
* Si aun no te afilias a Culqi, lo puedes hacer desde [aquí](https://afiliate.culqi.com/).
* Para encriptar el payload debes generar un id y llave RSA ingresando a tu CulqiPanel > Desarrollo > RSA Keys.


## Instalación

Agregar la siguiente dependencia en tu arhivo `Cargo.toml` (Ultima version): 

```toml
[dependencies]
LibCulqi = "1.0.2"
```

O ejecuta el sisguiente comando `Cargo.toml`:

```bash
cargo install LibCulqi
```


## Integracion 🚀
Para integrar Culqi Rust SDK con tu proyecto, sigue estos pasos: 

- Asegúrate de tener Rust y Cargo instalados en tu equipo.
- Configura tus **[credenciales](#configuracion-de-credenciales)**
- Crea tu instancia **[Client](#agrega-headers-personalizados)**   con tus credenciales
- Configura o agrega tus **[headers](#configuracion-de-credenciales)** de ser necesario
- Utiliza o llama al Modulo deseado 
    - **[Crear Token](#crear-token)**
    - **[Crear Charge](#crear-cargo-charge)**
    - **[Crear Refund](#crear-devolución-refund)**
    - **[Crear Customer](#crear-cliente-customer)**
    - **[Crear Card](#crear-tarjeta-card)**  
    - **[Crear Plan](#crear-plan)**
    - **[Crear Subscription](#crear-suscripcion-subscription)**
    - **[Crear Order](#crear-orden-order)**

## Configuracion de credenciales
- Archivo **/tests/config/credentials.rs** configura tus llaves.
- Para enviar peticiones al API de Culqi debes configurar tu llave pública (pk), llave privada (sk).
- Para enviar peticiones encriptadas y seguras debes configurar tus llaves `rsa_id` y `rsa_public_key` **(Como generar llaves RSA)**

```rust
const RSA_ID : &str = "Ingresa tu rsa_id";
const RSA_KEY : &str = "Ingresa tu rsa_public_key";
const PUBLIC_KEY : &str = "Ingresa tu llave pública";
const SECRET_KEY : &str = "Ingresa tu llave privada";
```

## Generar llaves RSA
Para encriptar el payload necesitas las llaves `rsa_id` y `rsa_public_key`, para esto debes ingresa a tu panel y hacer click en la sección **“Desarrollo / RSA Keys”** de la barra de navegación a la mano izquierda.

![alt tag](http://i.imgur.com/NhE6mS9.png)

Ahora declara en variables `rsa_id` y `rsa_public_key` y trabaja de manera segura.

## Crea tu instancia Client
¿Como funciona  y en que nos ayuda esta instacia?

Esta instancia nos ayuda utilizar tus credenciales para integrarnos con las APIS de Culqi, hay diferentes maneras de crearlas.

- Si deseas generar solicitudes sin RSA
```rust
let client = Client::config(&SECRET_KEY, &PUBLIC_KEY, None,);
```

- Si deseas generar solicitudes con RSA
```rust
let client = Client::config(&SECRET_KEY, &PUBLIC_KEY, Some(&RSA_KEY),);
```

## Agrega Headers personalizados
Hemos agregado una nueva funcionalidad que permitira que realizes diferentes actividades agregando encabezados, te muestro algunos ejemplos:

- Si deseas realizar peticiones con RSA
Agrega en tu header esta seccion para encriptar el payload -> **Es requerido si defines Some(&RSA_KEY) en lugar de  None**

**Servicios como Crear Token lo utilizan porque es requerido, los servicos como Crar Cargo, Crar Plan, Crar Subscription, entre otros es opcional**
```rust
let headers = json!({
    "x-culqi-rsa-id": &RSA_ID 
})
```

- Si deseas crear un cargo recurrente

**Servicio Crear Cargo lo utiliza es opcional**
```rust
let headers = json!({
    "X-Charge-Channels": "recurrent"
})
```
## Crear Token
Luego de realizar o leer los pasos de Integracion, te muestro un ejemplo de **Crear Token**, este servicio utiliza el encriptamiento RSA en el payload de forma obligatorio, para cuidar los datos de tu tarjeta.

**Ejemplo**

```rust 
const PUBLIC_KEY : &str = "Ingresa tu llave pública";
const SECRET_KEY : &str = "Ingresa tu llave privada";

let request_create_token = json!({
    "card_number": "4111111111111111",
    "cvv": "123",
    "expiration_month": "09",
    "expiration_year": "2025",
    "email": "brando.carquin@culqi.com",
    "metadata": {
        "comment": "Tarjeta de Prueba",
        "document_number": "12345678",
    },
})
let client = Client::config(&SECRET_KEY, &PUBLIC_KEY, None,);

return Token::create(client, &request_create_token, None).await;
```

## Crear Cargo (Charge)
Crear un cargo significa **cobrar una venta** a una tarjeta. Para esto previamente deberías generar el  `token` y enviarlo en parámetro **source_id**.

Los cargos pueden ser creados vía [API de devolución](https://apidocs.culqi.com/#tag/Cargos/operation/crear-cargo).

**Ejemplo**

```rust 
const PUBLIC_KEY : &str = "Ingresa tu llave pública";
const SECRET_KEY : &str = "Ingresa tu llave privada";

let request_create_token = json!({
    "amount": 10000,
    "currency_code": "PEN",
    "email": "brando.carquin@culqi.com",
    "source_id": "tkn_test_0CjjdWhFpEAZlxlz", // Crear Token -> Id
    "capture": true,
    "antifraud_details": {
        "address": "Avenida Lima 1234",
        "address_city": "Lima",
        "country_code": "PE",
        "first_name": "culqi",
        "last_name": "core",
        "phone_number": "999777666"
    },
    "metadata": {
        "documentNumber": "77723083"
    }
})

let client = Client::config(&SECRET_KEY, &PUBLIC_KEY, None,);

return Charge::create(client, &request_create_token, None).await;
```

## Crear Devolución (Refund)
Solicita la devolución de las compras de tus clientes (parcial o total) de forma gratuita a través del API y CulqiPanel. 

Las devoluciones pueden ser creados vía [API de devolución](https://apidocs.culqi.com/#tag/Devoluciones/operation/crear-devolucion).

**Ejemplo**

```rust 
const PUBLIC_KEY : &str = "Ingresa tu llave pública";
const SECRET_KEY : &str = "Ingresa tu llave privada";

let request_create_token = json!({
    "amount": 2000,
    "charge_id": "chr_test_3xWxRF1Zswgp6C7N", // Crear Cargo -> Id
    "reason": "fraudulento"
})

let client = Client::config(&SECRET_KEY, &PUBLIC_KEY, None,);

return Refund::create(client, &request_create_token, None).await;
```

## Crear Cliente (Customer)
El **cliente** es un servicio que te permite guardar la información de tus clientes. Es un paso necesario para generar una [tarjeta](/es/documentacion/pagos-online/recurrencia/one-click/tarjetas).

Los clientes pueden ser creados vía [API de cliente](https://apidocs.culqi.com/#tag/Clientes/operation/crear-cliente).

**Ejemplo**

```rust 
const PUBLIC_KEY : &str = "Ingresa tu llave pública";
const SECRET_KEY : &str = "Ingresa tu llave privada";

let request_create_token = json!({
    "first_name": "Richard",
    "last_name": "Hendricks",
    "email": "richard@piedpiper.com",
    "address": "San Francisco Bay Area",
    "address_city": "Palo Alto",
    "country_code": "US",
    "phone_number": "6505434800"
})

let client = Client::config(&SECRET_KEY, &PUBLIC_KEY, None,);

return Customer::create(client, &request_create_token, None).await;
```

## Crear Tarjeta (Card)
La **tarjeta** es un servicio que te permite guardar la información de las tarjetas de crédito o débito de tus clientes para luego realizarles cargos one click o recurrentes (cargos posteriores sin que tus clientes vuelvan a ingresar los datos de su tarjeta).

Las tarjetas pueden ser creadas vía [API de tarjeta](https://apidocs.culqi.com/#tag/Tarjetas/operation/crear-tarjeta).

**Ejemplo**

```rust 
const PUBLIC_KEY : &str = "Ingresa tu llave pública";
const SECRET_KEY : &str = "Ingresa tu llave privada";

let request_create_token = json!({
    "customer_id": "cus_test_Lz6Yfsm7QqCPIECW", // Crear Customer -> Id
    "token_id": "tkn_test_vEcZSCOVz5PGDPdQ", // Crear Token -> Id
    "validate": true,
    "metadata": {
        "marca_tarjeta": "VISA"
    }
})

let client = Client::config(&SECRET_KEY, &PUBLIC_KEY, None,);

return Card::create(client, &request_create_token, None).await;
```
## Crear Plan
El plan es un servicio que te permite definir con qué frecuencia deseas realizar cobros a tus clientes.

Un plan define el comportamiento de las suscripciones. Los planes pueden ser creados vía el [API de Plan](https://apidocs.culqi.com/#/planes#create) o desde el **CulqiPanel**.


**Ejemplo**

```rust 
const PUBLIC_KEY : &str = "Ingresa tu llave pública";
const SECRET_KEY : &str = "Ingresa tu llave privada";

let request_create_token = json!({
    "name": "Plan de Prueba.",
    "short_name": "plan-de-prueba-001",
    "description": "Descripción Plan de Prueba",
    "amount": 5,
    "currency": "PEN",
    "interval_unit_time": 1,
    "interval_count": 1,
    "initial_cycles": {
        "count": 0,
        "has_initial_charge": false,
        "amount": 0,
        "interval_unit_time": 1
    },
    "metadata": {
        "DNI": 123456782
    }
})

let client = Client::config(&SECRET_KEY, &PUBLIC_KEY, None,);

return Plan::create(client, &request_create_token, None).await;
```

## Crear Suscripcion (Subscription)
El plan es un servicio que te permite definir con qué frecuencia deseas realizar cobros a tus clientes.

Un plan define el comportamiento de las suscripciones. Los planes pueden ser creados vía el [API de Plan](https://apidocs.culqi.com/#/planes#create) o desde el **CulqiPanel**.


**Ejemplo**

```rust 
const PUBLIC_KEY : &str = "Ingresa tu llave pública";
const SECRET_KEY : &str = "Ingresa tu llave privada";

let request_create_token = json!({
    "card_id": "crd_test_RzjTyGUwZioJLpZt", // Crear Card -> Id
    "plan_id": "pln_test_XXXXXXXXXXXXXXXX", // Crear Plan -> Id
    "tyc": true,
    "metadata": {
        "DNI": 123456782
    }
})

let client = Client::config(&SECRET_KEY, &PUBLIC_KEY, None,);

return Subscription::create(client, &request_create_token, None).await;
```

## Crear Orden (Order)
Es un servicio que te permite generar una orden de pago para una compra potencial.
La orden contiene la información necesaria para la venta y es usado por el sistema de **PagoEfectivo** para realizar los pagos diferidos.

Las órdenes pueden ser creadas vía [API de orden](https://apidocs.culqi.com/#tag/Ordenes/operation/crear-orden).

**Ejemplo**

```rust 
const PUBLIC_KEY : &str = "Ingresa tu llave pública";
const SECRET_KEY : &str = "Ingresa tu llave privada";

let request_create_token = json!({
    "amount": 60000,
    "currency_code": "PEN",
    "description": " Venta de polo",
    "order_number": "#id-9999",
    "expiration_date": "1476132639",
    "client_details": {
        "first_name": "Richard",
        "last_name": "Hendricks",
        "email": "richard@piedpiper.com",
        "phone_number": "999999987"
    },
    "confirm": true,
    "metadata": {
        "dni": "71702999"
    }
})

let client = Client::config(&SECRET_KEY, &PUBLIC_KEY, None,);

return Order::create(client, &request_create_token, None).await;
```

## Crear Servicio Encriptado
Ejemplo de un servicio de Culqi encriptado, este servicio envia un payload encriptado con sus llaves RSA generadas.

**Ejemplo**

```rust 
const RSA_ID : &str = "Ingresa tu rsa_id";
const RSA_KEY : &str = "Ingresa tu rsa_public_key";
const PUBLIC_KEY : &str = "Ingresa tu llave pública";
const SECRET_KEY : &str = "Ingresa tu llave privada";

let request_create_token = json!({
    "card_number": "4111111111111111",
    "cvv": "123",
    "expiration_month": "09",
    "expiration_year": "2025",
    "email": "brando.carquin@culqi.com",
    "metadata": {
        "comment": "Tarjeta de Prueba",
        "document_number": "12345678",
    },
})
let client = Client::config(&SECRET_KEY, &PUBLIC_KEY, Some(&RSA_KEY),);

let headers = json!({
    "x-culqi-rsa-id": &RSA_ID 
})

return Token::create(client, &request_create_token, Some(headers)).await;
```
## Modulos
Conoce los metodos de integracion tenemos por servicio
- Token
```rust 
// Crear Token
return Token::create(&util::create_client(), &create_token_request(), None,).await;
// Crear Token Encriptado
return Token::create(&util::create_client_encrypt(), &create_token_request(), Some(header_rsa::get_header_encrypt(),),).await;
// Obtener Token por Id
return Token::get(&util::create_client(), &token_id, None,).await;
// Listar Token
return Token::all(&util::create_client(), &request_token_all(), None,).await;
// Update Token
return Token::patch(&util::create_client(), &token_id, &update_token_request(), None,).await;
// Crear Token Yape
return Token::yape(&util::create_client(), &token_id, &create_token_yape_request(), None,).await;
```
- Charge
```rust 
// Crear Charge
return Charge::create(&util::create_client(), &create_charge_request().await, None,).await;
// Crear Charge Recurrente
return Charge::create(&util::create_client(),&create_charge_request().await,Some(get_header_charge_recurrent(),),).await;
// Crear Charge Encriptado
return Charge::create(&util::create_client_encrypt(), &create_charge_request(), Some(header_rsa::get_header_encrypt(),),).await;
// Obtener Charge por Id
return Charge::get(&util::create_client(), &charge_id, None,).await;
// Listar Charge
return Charge::all(&util::create_client(), &request_charge_all(), None,).await;
// Update Charge
return Charge::patch(&util::create_client(), &charge_id, &update_charge_request(), None,).await;
// Capturar Charge
return Charge::capture(&util::create_client(), &charge_id, None,).await;
```
- Card
```rust 
// Crear Card
return Card::create(&util::create_client(), &create_card_request().await, None,).await;
// Crear Card Encriptado
return Card::create(&util::create_client_encrypt(), &create_card_request(), Some(header_rsa::get_header_encrypt(),),).await;
// Obtener Card por Id
return Card::get(&util::create_client(), &card_id, None,).await;
// Listar Card
return Card::all(&util::create_client(), &request_card_all(), None,).await;
// Update Card
return Card::patch(&util::create_client(), &card_id, &update_card_request(), None,).await;
// Delete Card
return Card::delete(&util::create_client(), &charge_id, None,).await;
```
- Customer
```rust 
// Crear Customer
return Customer::create(&util::create_client(), &create_customer_request().await, None,).await;
// Crear Customer Encriptado
return Customer::create(&util::create_client_encrypt(), &create_customer_request(), Some(header_rsa::get_header_encrypt(),),).await;
// Obtener Customer por Id
return Customer::get(&util::create_client(), &customer_id, None,).await;
// Listar Customer
return Customer::all(&util::create_client(), &request_customer_all(), None,).await;
// Update Customer
return Customer::patch(&util::create_client(), &customer_id, &update_customer_request(), None,).await;
// Delete Customer
return Customer::delete(&util::create_client(), &customer_id, None,).await;
```
- Refund
```rust 
// Crear Refund
return Refund::create(&util::create_client(), &create_refund_request().await, None,).await;
// Crear Refund Encriptado
return Refund::create(&util::create_client_encrypt(), &create_refund_request(), Some(header_rsa::get_header_encrypt(),),).await;
// Obtener Refund por Id
return Refund::get(&util::create_client(), &refund_id, None,).await;
// Listar Refund
return Refund::all(&util::create_client(), &request_card_all(), None,).await;
// Update Refund
return Refund::patch(&util::create_client(), &refund_id, &update_refund_request(), None,).await;
```
- Plan
```rust 
// Crear Plan
return Plan::create(&util::create_client(), &create_plan_request().await, None,).await;
// Crear Plan Encriptado
return Plan::create(&util::create_client_encrypt(), &create_plan_request(), Some(header_rsa::get_header_encrypt(),),).await;
// Obtener Plan por Id
return Plan::get(&util::create_client(), &plan_id, None,).await;
// Listar Plan
return Plan::all(&util::create_client(), &request_plan_all(), None,).await;
// Update Plan
return Plan::patch(&util::create_client(), &plan_id, &request_plan_update(), None,).await;
// Delete Plan
return Plan::delete(&util::create_client(), &plan_id, None,).await;
```
- Order
```rust 
// Crear Order
return Order::create(&util::create_client(), &create_order_request().await, None,).await;
// Crear Order Encriptado
return Order::create(&util::create_client_encrypt(), &create_order_request(), Some(header_rsa::get_header_encrypt(),),).await;
// Obtener Order por Id
return Order::get(&util::create_client(), &order_id, None,).await;
// Listar Order
return Order::all(&util::create_client(), &request_order_all(), None,).await;
// Update Order
return Order::patch(&util::create_client(), &order_id, &update_order_request(), None,).await;
// Delete Order
return Order::delete(&util::create_client(), &order_id, None,).await;
// Confirm Order
return Order::confirm(&util::create_client(), &order_id, None,).await;
// Confirm Type Order
return Order::type_confirm(&util::create_client(), &order_type_confirm_request(&order_id,), None,).await;
```
- Subscription
```rust 
// Crear Subscription
return Subscription::create(&util::create_client(), &create_subscription_request().await, None,).await;
// Crear Subscription Encriptado
return Subscription::create(&util::create_client_encrypt(), &create_subscription_request(), Some(header_rsa::get_header_encrypt(),),).await;
// Obtener Subscription por Id
return Subscription::get(&util::create_client(), &subscription_id, None,).await;
// Listar Subscription
return Subscription::all(&util::create_client(), &request_plan_all(), None,).await;
// Update Subscription
return Subscription::patch(&util::create_client(), &subscription_id, &update_subscription_request(), None,).await;
// Delete Subscription
return Subscription::delete(&util::create_client(), &subscription_id, None,).await;
```

## Ejecuta los TEST
 - Comando para ejecutar todos los test
```bash
cargo test
```
 - Comando para ejecutar una prueba especificada
```bash
cargo test tests::test_plan_update
```
## Formatea el proyecto 
```bash
cargo +nightly fmt
```

## Documentación

- [Referencia de Documentación](https://docs.culqi.com/)
- [Referencia de API](https://apidocs.culqi.com/)
- [Demo Checkout V4 + Culqi 3DS](https://github.com/culqi/culqi-rust-demo-checkoutv4-culqi3ds/)
- [Wiki](https://github.com/culqi/culqi-rust/wiki)

## Changelog

Todos los cambios en las versiones de esta biblioteca están listados en
[CHANGELOG](CHANGELOG).

## Autor
Team Culqi

## Licencia
El código fuente de culqi-rust está distribuido bajo MIT License, revisar el archivo LICENSE.