# 🎭 Definición de Enums

> **Enums** - Tipos que pueden ser una de varias variantes

---

## ¿Qué es un Enum?

Un **enum** (enumeración) es un tipo que puede tener uno de varios valores posibles llamados **variantes**.

```rust
enum Direccion {
    Norte,
    Sur,
    Este,
    Oeste,
}
```

Cada valor de tipo `Direccion` es **exactamente una** de estas cuatro variantes.

---

## Crear y Usar Enums

```rust
enum Semaforo {
    Rojo,
    Amarillo,
    Verde,
}

fn main() {
    let luz = Semaforo::Verde;
    
    // Acceder con el prefijo del tipo
    let otra_luz = Semaforo::Rojo;
}
```

Las variantes están **namespaced** bajo el nombre del enum.

---

## Enums vs Constantes

¿Por qué no usar constantes?

```rust
// ❌ Con constantes - sin seguridad de tipos
const ROJO: u8 = 0;
const AMARILLO: u8 = 1;
const VERDE: u8 = 2;

fn cambiar_luz(luz: u8) {
    // ¿Qué pasa si alguien pasa 5?
}

// ✅ Con enums - seguridad de tipos
enum Semaforo { Rojo, Amarillo, Verde }

fn cambiar_luz(luz: Semaforo) {
    // Solo puede recibir variantes válidas
}
```

---

## Enums en Match

Los enums brillan con `match`:

```rust
enum Dia {
    Lunes,
    Martes,
    Miercoles,
    Jueves,
    Viernes,
    Sabado,
    Domingo,
}

fn es_fin_de_semana(dia: Dia) -> bool {
    match dia {
        Dia::Sabado | Dia::Domingo => true,
        _ => false,
    }
}
```

---

## Derivar Traits Comunes

Los enums pueden derivar traits útiles:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Estado {
    Activo,
    Inactivo,
    Pendiente,
}

fn main() {
    let e1 = Estado::Activo;
    let e2 = e1;  // Copy
    
    println!("{:?}", e1);  // Debug
    
    if e1 == Estado::Activo {  // PartialEq
        println!("Está activo");
    }
}
```

---

## Métodos en Enums

Puedes implementar métodos con `impl`:

```rust
enum Moneda {
    Peso,
    Dolar,
    Euro,
}

impl Moneda {
    fn simbolo(&self) -> &str {
        match self {
            Moneda::Peso => "$",
            Moneda::Dolar => "USD",
            Moneda::Euro => "€",
        }
    }
    
    fn a_pesos(&self, cantidad: f64) -> f64 {
        match self {
            Moneda::Peso => cantidad,
            Moneda::Dolar => cantidad * 850.0,
            Moneda::Euro => cantidad * 920.0,
        }
    }
}
```

---

## Enums como Tipos de Estado

Patrón común para máquinas de estado:

```rust
enum EstadoConexion {
    Desconectado,
    Conectando,
    Conectado,
    Error,
}

struct Conexion {
    estado: EstadoConexion,
    intentos: u32,
}

impl Conexion {
    fn puede_enviar(&self) -> bool {
        matches!(self.estado, EstadoConexion::Conectado)
    }
}
```

---

## Resumen

| Concepto | Descripción |
|----------|-------------|
| Enum | Tipo con variantes finitas |
| Variante | Cada valor posible del enum |
| `::` | Acceso a variantes |
| `match` | Manejar cada variante |

---

## 🧪 Ejercicio Mental

¿Qué representa mejor un enum?

1. Una lista de usuarios
2. El estado de un botón (presionado/liberado)
3. Los colores de una paleta
4. Un número de teléfono

<details>
<summary>Ver respuesta</summary>

**2 y 3** son buenos candidatos para enums:
- Estado de botón: `Presionado`, `Liberado`
- Colores: `Rojo`, `Verde`, `Azul`

1 es mejor con `Vec<Usuario>`
4 es mejor con `String`

</details>

---

## 📚 Siguiente

[Enums con Datos →](02-enums-con-datos.md)
