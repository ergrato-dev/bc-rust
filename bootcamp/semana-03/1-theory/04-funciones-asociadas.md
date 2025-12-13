# 🏭 Funciones Asociadas

![Método vs Función Asociada](../0-assets/03-metodo-vs-asociada.svg)

## ¿Qué son las Funciones Asociadas?

Las **funciones asociadas** son funciones dentro de `impl` que **NO** reciben `self`. No operan sobre una instancia, sino sobre el tipo en sí.

---

## Diferencia: Método vs Función Asociada

| Característica | Método | Función Asociada |
|----------------|--------|------------------|
| Primer parámetro | `self`, `&self`, `&mut self` | No tiene `self` |
| Llamada | `instancia.metodo()` | `Tipo::funcion()` |
| Acceso a datos | Sí, a través de self | No, no hay instancia |
| Uso común | Operar sobre datos | Constructores |

---

## El Constructor new()

![Patrón new()](../0-assets/05-patron-new.svg)

El patrón más común es usar `new()` como constructor:

```rust
struct Usuario {
    nombre: String,
    email: String,
    edad: u32,
}

impl Usuario {
    // Función asociada (constructor)
    fn new(nombre: String, email: String) -> Self {
        Self {
            nombre,
            email,
            edad: 0,
        }
    }
}

fn main() {
    // Llamada con Tipo::funcion()
    let usuario = Usuario::new(
        String::from("Ana"),
        String::from("ana@email.com")
    );
}
```

---

## Self vs NombreStruct

Dentro de `impl`, `Self` es un alias del tipo:

```rust
impl Usuario {
    // Estas dos formas son equivalentes:
    
    fn new_v1(nombre: String) -> Usuario {
        Usuario {
            nombre,
            email: String::new(),
            edad: 0,
        }
    }

    fn new_v2(nombre: String) -> Self {
        Self {
            nombre,
            email: String::new(),
            edad: 0,
        }
    }
}
```

✅ **Preferir `Self`**: Es más idiomático y facilita refactoring.

---

## Múltiples Constructores

```rust
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    // Constructor principal
    fn new(ancho: u32, alto: u32) -> Self {
        Self { ancho, alto }
    }

    // Constructor para cuadrado
    fn cuadrado(lado: u32) -> Self {
        Self {
            ancho: lado,
            alto: lado,
        }
    }

    // Constructor con valores por defecto
    fn default() -> Self {
        Self {
            ancho: 1,
            alto: 1,
        }
    }
}

fn main() {
    let rect = Rectangulo::new(10, 5);
    let cuadrado = Rectangulo::cuadrado(8);
    let defecto = Rectangulo::default();
}
```

---

## Funciones Asociadas No-Constructores

No todas las funciones asociadas crean instancias:

```rust
struct Circulo {
    radio: f64,
}

impl Circulo {
    const PI: f64 = 3.14159265358979;

    fn new(radio: f64) -> Self {
        Self { radio }
    }

    // Función asociada que calcula sin instancia
    fn area_con_radio(radio: f64) -> f64 {
        Self::PI * radio * radio
    }

    // Función asociada de utilidad
    fn es_radio_valido(radio: f64) -> bool {
        radio > 0.0
    }
}

fn main() {
    // Usar sin crear instancia
    if Circulo::es_radio_valido(5.0) {
        let area = Circulo::area_con_radio(5.0);
        println!("Área: {}", area);
    }

    // O crear instancia
    let circulo = Circulo::new(5.0);
}
```

---

## Ejemplo: String::from()

`String::from()` es una función asociada que ya conoces:

```rust
// String::from es una función asociada
let s = String::from("hola");

// Equivale a algo como:
impl String {
    fn from(s: &str) -> String {
        // ... crea un String desde &str
    }
}
```

---

## Patrón Builder

Para structs con muchos campos opcionales:

```rust
struct Configuracion {
    host: String,
    puerto: u16,
    timeout: u32,
    debug: bool,
}

impl Configuracion {
    fn new(host: String) -> Self {
        Self {
            host,
            puerto: 8080,      // valor por defecto
            timeout: 30,       // valor por defecto
            debug: false,      // valor por defecto
        }
    }

    fn con_puerto(mut self, puerto: u16) -> Self {
        self.puerto = puerto;
        self
    }

    fn con_timeout(mut self, timeout: u32) -> Self {
        self.timeout = timeout;
        self
    }

    fn con_debug(mut self) -> Self {
        self.debug = true;
        self
    }
}

fn main() {
    let config = Configuracion::new(String::from("localhost"))
        .con_puerto(3000)
        .con_debug();
}
```

---

## 📝 Resumen

| Concepto | Sintaxis | Uso |
|----------|----------|-----|
| Función asociada | `fn nombre() -> Self` | Constructores |
| Llamada | `Tipo::funcion()` | Sin instancia |
| `Self` | Alias del tipo | Dentro de impl |
| `new()` | Constructor principal | Patrón idiomático |

---

*Siguiente: [05-tipos-structs.md](./05-tipos-structs.md)*
