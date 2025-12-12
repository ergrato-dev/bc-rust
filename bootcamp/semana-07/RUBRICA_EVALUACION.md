# 📋 Rúbrica de Evaluación - Semana 07

## Módulos y Crates

### Competencias a Evaluar

| Competencia | Peso | Descripción |
|-------------|------|-------------|
| Módulos | 25% | Declaración y uso de módulos |
| Visibilidad | 25% | Control de acceso con pub |
| Estructura | 25% | Organización en archivos |
| Crates | 15% | Uso de dependencias externas |
| Proyecto | 10% | Biblioteca modular |

---

## 1. Módulos (25%)

### Nivel Avanzado (90-100%)
- Crea módulos anidados correctamente
- Entiende la jerarquía de módulos
- Usa re-exports efectivamente

### Nivel Intermedio (70-89%)
- Crea módulos básicos sin errores
- Entiende la diferencia entre inline y archivos
- Usa `mod` y `use` correctamente

### Nivel Básico (50-69%)
- Crea módulos simples con ayuda
- Confusión ocasional con la sintaxis

### Nivel Insuficiente (<50%)
- No puede crear módulos básicos
- Errores frecuentes de compilación

---

## 2. Visibilidad (25%)

### Nivel Avanzado (90-100%)
- Usa `pub`, `pub(crate)`, `pub(super)` apropiadamente
- Diseña APIs con encapsulación correcta
- Entiende el principio de mínima exposición

### Nivel Intermedio (70-89%)
- Usa `pub` y privado correctamente
- Entiende por qué la visibilidad importa

### Nivel Básico (50-69%)
- Pone `pub` en todo para que compile
- No entiende las implicaciones

### Nivel Insuficiente (<50%)
- No puede resolver errores de visibilidad

---

## 3. Estructura de Archivos (25%)

### Nivel Avanzado (90-100%)
- Organiza proyectos con mod.rs o módulos nombrados
- Estructura profesional y mantenible
- Separación clara de responsabilidades

### Nivel Intermedio (70-89%)
- Separa módulos en archivos correctamente
- Entiende la correspondencia archivo/módulo

### Nivel Básico (50-69%)
- Puede separar módulos con guía
- Errores ocasionales en imports

### Nivel Insuficiente (<50%)
- No puede estructurar proyectos multi-archivo

---

## 4. Crates Externos (15%)

### Nivel Avanzado (90-100%)
- Agrega y usa dependencias sin problemas
- Entiende versiones semánticas
- Lee documentación de crates

### Nivel Intermedio (70-89%)
- Usa `cargo add` y dependencias básicas
- Puede seguir ejemplos de documentación

### Nivel Básico (50-69%)
- Necesita ayuda para agregar dependencias
- Copia ejemplos sin entenderlos completamente

### Nivel Insuficiente (<50%)
- No puede usar crates externos

---

## 5. Proyecto: Biblioteca de Geometría (10%)

### Nivel Avanzado (90-100%)
- Biblioteca bien estructurada y documentada
- Tests para cada módulo
- API limpia y usable

### Nivel Intermedio (70-89%)
- Funcionalidad completa
- Estructura razonable

### Nivel Básico (50-69%)
- Funcionalidad parcial
- Estructura mejorable

### Nivel Insuficiente (<50%)
- Proyecto incompleto o no funcional

---

## Criterios Generales

### Código
- [ ] Compila sin warnings
- [ ] Estructura de módulos clara
- [ ] Visibilidad apropiada (no todo es pub)
- [ ] Imports organizados

### Documentación
- [ ] Módulos públicos documentados
- [ ] README del proyecto completo

### Tests
- [ ] Tests unitarios por módulo
- [ ] Tests pasan

---

## Distribución de Notas

| Tipo | Peso | Actividades |
|------|------|-------------|
| Conocimiento | 30% | Preguntas sobre módulos y visibilidad |
| Desempeño | 40% | Prácticas en clase |
| Producto | 30% | Proyecto biblioteca |
