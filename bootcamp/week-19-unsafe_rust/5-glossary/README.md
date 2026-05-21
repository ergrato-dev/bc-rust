# Glosario — Semana 19: `unsafe` Rust

| Término | Definición |
|---------|-----------|
| **Unsafe Rust** | Subconjunto de Rust que permite operaciones que el compilador no puede verificar automáticamente |
| **Raw pointer** | Puntero sin garantías de validez, lifetime ni aliasing (`*const T`, `*mut T`) |
| **UB (Undefined Behavior)** | Comportamiento que el estándar de C/Rust no define; puede causar cualquier resultado |
| **Invariante de seguridad** | Condición que debe cumplirse para que el código `unsafe` sea correcto |
| **`SAFETY:` comment** | Comentario obligatorio que justifica por qué un bloque `unsafe` es correcto |
| **`Send`** | Trait que indica que un tipo puede transferirse entre threads |
| **`Sync`** | Trait que indica que un tipo puede compartirse entre threads via referencias |
| **`NonNull<T>`** | Raw pointer garantizado non-null; reemplaza `*mut T` en API internas |
| **`transmute`** | Reinterpreta los bits de un tipo como otro; la función más peligrosa de Rust |
| **`mem::forget`** | Toma ownership sin llamar a `Drop`; produce memory leak intencional |
| **`MaybeUninit<T>`** | Wrapper para valores potencialmente no inicializados; evitar transmute para init diferida |
| **Layout** | Tamaño y alineación de un tipo en memoria (`std::alloc::Layout`) |
| **Global Allocator** | El allocator de memoria del proceso (`alloc::alloc`, `alloc::dealloc`) |
| **Dangling pointer** | Puntero a memoria ya liberada o nunca válida |
| **Double-free** | Liberar la misma memoria dos veces; UB con consecuencias de seguridad |
| **Aliasing mutable** | Tener dos `&mut T` al mismo dato al mismo tiempo; UB en Rust |
| **Miri** | Intérprete de Rust que detecta UB y accesos de memoria incorrectos |
