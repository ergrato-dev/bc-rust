use wasmtime::*;

/// Módulo WASM mínimo en formato WAT (WebAssembly Text Format).
///
/// Define una función `suma` que recibe dos i32 y retorna su suma.
const WASM_SUMA: &str = r#"
(module
  (func (export "suma") (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add
  )
)
"#;

fn main() -> anyhow::Result<()> {
    // Crear el motor y el almacén de estado
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    // Compilar el módulo WAT directamente
    let module = Module::new(&engine, WASM_SUMA)?;

    // Instanciar sin imports
    let instance = Instance::new(&mut store, &module, &[])?;

    // Obtener la función exportada
    let suma = instance
        .get_typed_func::<(i32, i32), i32>(&mut store, "suma")?;

    // Llamar a la función WASM
    let resultado = suma.call(&mut store, (10, 32))?;
    println!("suma(10, 32) = {resultado}");
    assert_eq!(resultado, 42, "El resultado debe ser 42");

    let resultado2 = suma.call(&mut store, (100, -58))?;
    println!("suma(100, -58) = {resultado2}");
    assert_eq!(resultado2, 42, "El resultado debe ser 42");

    println!("✓ Wasmtime ejecutó el módulo WASM correctamente");
    Ok(())
}
