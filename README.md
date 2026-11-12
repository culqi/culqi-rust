# Culqi para Rust

Cliente Rust para la API v2 de Culqi. El paquete se llama `culqi` y su
versión declarada en este repositorio es `0.2.1`.

## Requisitos

- Rust y Cargo compatibles con esta versión del proyecto.
- Llaves de prueba de Culqi para realizar llamadas de integración.

Este código usa dependencias y atributos antiguos de Rust; podría requerir un
toolchain nightly compatible.

## Instalación

Agrega la dependencia a `Cargo.toml`:

```toml
[dependencies]
culqi = "0.2.1"
```

## Configuración de credenciales

Configura tus llaves de prueba en el entorno local. No las incluyas en el
código ni las subas al repositorio:

```bash
export CULQI_PUBLIC_KEY=INSERTE_SU_LLAVE_PUBLICA
export CULQI_SECRET_KEY=INSERTE_SU_LLAVE_PRIVADA
```

Los placeholders no son credenciales válidas. Las llaves reales de prueba se
obtienen desde CulqiPanel. La llave pública se usa para crear tokens; la llave
privada, para operaciones que requieren autenticación privada.

Si trabajas con una rama que implementa cifrado RSA, usa también
`INSERTE_SU_LLAVE_RSA` y `INSERTE_SU_RSA_ID` como placeholders; sustitúyelos
solo localmente por tus propios datos.

## Uso

El cliente expone operaciones como `Token::create`, `Charge::create` y
`Customer::create`. Por ejemplo, esta función construye y envía una solicitud
para crear un token:

```rust
extern crate culqi;

fn create_token(
    public_key: &str,
    card_number: &str,
    cvv: &str,
    email: &str,
) -> String {
    let client = culqi::Client::new(public_key);
    let token = culqi::Token::new(card_number, cvv, 12, 2030, email, None);
    culqi::Token::create(&client, &token)
}
```

Esta llamada accede a la API de Culqi. Usa datos de prueba y no ejecutes
integraciones con credenciales reales durante el desarrollo.

## Documentación

- [Documentación de Culqi](https://docs.culqi.com/)
- [Referencia de la API](https://apidocs.culqi.com/)
- [Documentación del paquete](https://docs.rs/culqi/0.2.1/culqi/)

## Pruebas

Ejecuta `cargo test` con un toolchain compatible. Las llamadas de integración
requieren credenciales de prueba configuradas localmente.

## Licencia

MIT. Consulta [LICENSE](LICENSE).
