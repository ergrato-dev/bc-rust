use pyo3::prelude::*;

/// Suma dos enteros — función expuesta a Python.
#[pyfunction]
fn suma(a: i64, b: i64) -> i64 {
    a + b
}

/// Cuenta las palabras en una cadena — función expuesta a Python.
#[pyfunction]
fn contar_palabras(texto: &str) -> usize {
    texto.split_whitespace().count()
}

/// Clase `Contador` expuesta a Python.
#[pyclass]
struct Contador {
    valor: i64,
}

#[pymethods]
impl Contador {
    #[new]
    fn new(inicial: i64) -> Self {
        Contador { valor: inicial }
    }

    fn incrementar(&mut self) {
        self.valor += 1;
    }

    fn valor(&self) -> i64 {
        self.valor
    }
}

/// Módulo Python `practice_03_pyo3_basico`
#[pymodule]
fn practice_03_pyo3_basico(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(suma, m)?)?;
    m.add_function(wrap_pyfunction!(contar_palabras, m)?)?;
    m.add_class::<Contador>()?;
    Ok(())
}
