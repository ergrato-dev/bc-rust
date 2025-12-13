# Proyecto Final: Web Crawler Asíncrono

## 🎯 Objetivo

Implementar un web crawler asíncrono que explore páginas web de forma concurrente, respetando límites de rate limiting y profundidad.

## 📋 Requisitos

### Funcionalidades Principales

1. **Crawler Básico**
   - Descargar páginas web de forma asíncrona
   - Extraer enlaces de las páginas
   - Seguir enlaces hasta una profundidad máxima

2. **Control de Concurrencia**
   - Limitar número de requests concurrentes
   - Implementar rate limiting (requests por segundo)
   - Evitar visitar URLs duplicadas

3. **Manejo de Errores**
   - Timeouts por request
   - Reintentos en caso de error temporal
   - Logging de errores

4. **Estadísticas**
   - Contar páginas visitadas
   - Medir tiempo total
   - Reportar errores encontrados

## 🏗️ Estructura Sugerida

```rust
/// Configuración del crawler.
pub struct CrawlerConfig {
    pub max_depth: u32,
    pub max_concurrent: usize,
    pub timeout_ms: u64,
    pub rate_limit_ms: u64,
}

/// Resultado de crawlear una página.
pub struct PageResult {
    pub url: String,
    pub status: u16,
    pub links: Vec<String>,
    pub depth: u32,
}

/// El crawler principal.
pub struct Crawler {
    config: CrawlerConfig,
    client: reqwest::Client,
    visited: Arc<Mutex<HashSet<String>>>,
}

impl Crawler {
    /// Inicia el crawl desde una URL semilla.
    pub async fn crawl(&self, seed_url: &str) -> CrawlResult {
        todo!()
    }
}
```

## 📊 Ejemplo de Uso

```rust
#[tokio::main]
async fn main() {
    let config = CrawlerConfig {
        max_depth: 2,
        max_concurrent: 5,
        timeout_ms: 5000,
        rate_limit_ms: 100,
    };
    
    let crawler = Crawler::new(config);
    let result = crawler.crawl("https://example.com").await;
    
    println!("Páginas visitadas: {}", result.pages_visited);
    println!("Enlaces encontrados: {}", result.links_found);
    println!("Errores: {}", result.errors);
    println!("Tiempo: {:?}", result.duration);
}
```

## ✅ Criterios de Evaluación

| Criterio | Puntos |
|----------|--------|
| Crawling básico funciona | 20 |
| Extracción de enlaces correcta | 15 |
| Control de profundidad | 15 |
| Limita concurrencia | 15 |
| Rate limiting | 10 |
| Manejo de errores | 10 |
| No visita duplicados | 10 |
| Estadísticas correctas | 5 |

## 💡 Pistas de Implementación

### 1. Cliente HTTP

```rust
let client = reqwest::Client::builder()
    .timeout(Duration::from_millis(timeout_ms))
    .build()?;
```

### 2. Extracción de Enlaces

```rust
use scraper::{Html, Selector};

fn extraer_enlaces(html: &str, base_url: &Url) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a[href]").unwrap();
    
    document.select(&selector)
        .filter_map(|el| el.value().attr("href"))
        .filter_map(|href| base_url.join(href).ok())
        .map(|url| url.to_string())
        .collect()
}
```

### 3. Control de Concurrencia

```rust
use tokio::sync::Semaphore;

let semaphore = Arc::new(Semaphore::new(max_concurrent));

let permit = semaphore.acquire().await?;
// ... hacer request ...
drop(permit);
```

### 4. Rate Limiting

```rust
use tokio::time::{interval, Duration};

let mut rate_limiter = interval(Duration::from_millis(rate_limit_ms));

rate_limiter.tick().await; // Espera antes de cada request
```

### 5. URLs Visitadas

```rust
use std::collections::HashSet;
use tokio::sync::Mutex;

let visited = Arc::new(Mutex::new(HashSet::new()));

// En el loop de crawl:
{
    let mut set = visited.lock().await;
    if set.contains(&url) {
        continue;
    }
    set.insert(url.clone());
}
```

## 🧪 Testing

Para testing, considera:

1. **Mock Server**: Usar `wiremock` para simular respuestas
2. **URLs locales**: Crear servidor local con páginas de prueba
3. **Límites conocidos**: Probar con depth=1, max_concurrent=1

```rust
#[tokio::test]
async fn test_crawler_depth() {
    // Configurar mock server
    // Verificar que no excede profundidad
}
```

## 🚀 Extensiones Opcionales

- [ ] Filtrar URLs por dominio (solo mismo dominio)
- [ ] Respetar robots.txt
- [ ] Guardar páginas a disco
- [ ] Extraer texto además de enlaces
- [ ] Interfaz de línea de comandos
- [ ] Exportar resultados a JSON
