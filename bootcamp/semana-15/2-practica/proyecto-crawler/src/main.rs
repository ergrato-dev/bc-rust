//! Proyecto Final: Web Crawler Asíncrono
//!
//! Un crawler que explora páginas web de forma concurrente.

use scraper::{Html, Selector};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};
use thiserror::Error;
use tokio::sync::{mpsc, Mutex, Semaphore};
use tokio::time::{interval, timeout};
use url::Url;

// =============================================================================
// TIPOS Y ERRORES
// =============================================================================

/// Errores del crawler.
#[derive(Error, Debug)]
pub enum CrawlerError {
    #[error("Error de red: {0}")]
    Network(#[from] reqwest::Error),

    #[error("URL inválida: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Timeout")]
    Timeout,

    #[error("Profundidad máxima alcanzada")]
    MaxDepth,
}

/// Configuración del crawler.
#[derive(Clone, Debug)]
pub struct CrawlerConfig {
    /// Profundidad máxima de crawling.
    pub max_depth: u32,
    /// Número máximo de requests concurrentes.
    pub max_concurrent: usize,
    /// Timeout por request en milisegundos.
    pub timeout_ms: u64,
    /// Delay mínimo entre requests (rate limiting).
    pub rate_limit_ms: u64,
}

impl Default for CrawlerConfig {
    fn default() -> Self {
        CrawlerConfig {
            max_depth: 2,
            max_concurrent: 5,
            timeout_ms: 5000,
            rate_limit_ms: 100,
        }
    }
}

/// Resultado de crawlear una página individual.
#[derive(Debug, Clone)]
pub struct PageResult {
    pub url: String,
    pub status: u16,
    pub links: Vec<String>,
    pub depth: u32,
}

/// Resultado final del crawling.
#[derive(Debug)]
pub struct CrawlResult {
    pub pages_visited: usize,
    pub links_found: usize,
    pub errors: usize,
    pub duration: Duration,
    pub pages: Vec<PageResult>,
}

/// Tarea de crawling pendiente.
#[derive(Debug)]
struct CrawlTask {
    url: String,
    depth: u32,
}

// =============================================================================
// CRAWLER
// =============================================================================

/// Web crawler asíncrono.
pub struct Crawler {
    config: CrawlerConfig,
    client: reqwest::Client,
    visited: Arc<Mutex<HashSet<String>>>,
    semaphore: Arc<Semaphore>,
}

impl Crawler {
    /// Crea un nuevo crawler con la configuración dada.
    pub fn new(config: CrawlerConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(config.timeout_ms))
            .user_agent("RustBootcampCrawler/1.0")
            .build()
            .expect("Error creando cliente HTTP");

        let semaphore = Arc::new(Semaphore::new(config.max_concurrent));

        Crawler {
            config,
            client,
            visited: Arc::new(Mutex::new(HashSet::new())),
            semaphore,
        }
    }

    /// Inicia el crawling desde una URL semilla.
    pub async fn crawl(&self, seed_url: &str) -> CrawlResult {
        let start = Instant::now();

        // TODO: Implementar el crawling
        //
        // Algoritmo sugerido:
        // 1. Crear channel para tareas pendientes
        // 2. Crear channel para resultados
        // 3. Agregar URL semilla como primera tarea
        // 4. Loop mientras haya tareas:
        //    a. Tomar tarea del channel
        //    b. Verificar si ya fue visitada
        //    c. Verificar profundidad
        //    d. Spawn task para procesar la URL
        //    e. Recolectar resultado
        //    f. Agregar nuevos enlaces como tareas
        // 5. Recolectar estadísticas

        // Placeholder - reemplazar con implementación real
        let pages = self.crawl_internal(seed_url).await;

        let links_found: usize = pages.iter().map(|p| p.links.len()).sum();

        CrawlResult {
            pages_visited: pages.len(),
            links_found,
            errors: 0, // TODO: Contar errores reales
            duration: start.elapsed(),
            pages,
        }
    }

    /// Implementación interna del crawling.
    async fn crawl_internal(&self, seed_url: &str) -> Vec<PageResult> {
        let mut results = Vec::new();
        let (task_tx, mut task_rx) = mpsc::channel::<CrawlTask>(1000);
        let (result_tx, mut result_rx) = mpsc::channel::<PageResult>(1000);

        // Enviar tarea inicial
        task_tx
            .send(CrawlTask {
                url: seed_url.to_string(),
                depth: 0,
            })
            .await
            .ok();

        let mut pending_tasks = 1;
        let mut rate_limiter = interval(Duration::from_millis(self.config.rate_limit_ms));

        while pending_tasks > 0 {
            tokio::select! {
                // Recibir nueva tarea
                Some(task) = task_rx.recv() => {
                    // Verificar si ya visitamos esta URL
                    {
                        let mut visited = self.visited.lock().await;
                        if visited.contains(&task.url) {
                            pending_tasks -= 1;
                            continue;
                        }
                        visited.insert(task.url.clone());
                    }

                    // Verificar profundidad
                    if task.depth > self.config.max_depth {
                        pending_tasks -= 1;
                        continue;
                    }

                    // Rate limiting
                    rate_limiter.tick().await;

                    // Procesar URL
                    let client = self.client.clone();
                    let semaphore = self.semaphore.clone();
                    let result_tx = result_tx.clone();
                    let task_tx = task_tx.clone();
                    let max_depth = self.config.max_depth;
                    let timeout_ms = self.config.timeout_ms;

                    tokio::spawn(async move {
                        let _permit = semaphore.acquire().await.unwrap();

                        match Self::fetch_page(&client, &task.url, timeout_ms).await {
                            Ok(page_result) => {
                                let result = PageResult {
                                    url: task.url.clone(),
                                    status: page_result.0,
                                    links: page_result.1.clone(),
                                    depth: task.depth,
                                };

                                // Enviar resultado
                                result_tx.send(result).await.ok();

                                // Agregar nuevas tareas si no excedemos profundidad
                                if task.depth < max_depth {
                                    for link in page_result.1 {
                                        task_tx.send(CrawlTask {
                                            url: link,
                                            depth: task.depth + 1,
                                        }).await.ok();
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Error procesando {}: {}", task.url, e);
                            }
                        }
                    });
                }

                // Recibir resultado
                Some(result) = result_rx.recv() => {
                    pending_tasks += result.links.len();
                    pending_tasks -= 1;
                    results.push(result);
                }

                else => break,
            }

            // Timeout global para evitar loop infinito
            if results.len() > 100 {
                break;
            }
        }

        results
    }

    /// Descarga una página y extrae sus enlaces.
    async fn fetch_page(
        client: &reqwest::Client,
        url: &str,
        timeout_ms: u64,
    ) -> Result<(u16, Vec<String>), CrawlerError> {
        let fetch_future = async {
            let response = client.get(url).send().await?;
            let status = response.status().as_u16();
            let body = response.text().await?;

            let base_url = Url::parse(url)?;
            let links = Self::extract_links(&body, &base_url);

            Ok::<(u16, Vec<String>), CrawlerError>((status, links))
        };

        match timeout(Duration::from_millis(timeout_ms), fetch_future).await {
            Ok(result) => result,
            Err(_) => Err(CrawlerError::Timeout),
        }
    }

    /// Extrae enlaces de un documento HTML.
    fn extract_links(html: &str, base_url: &Url) -> Vec<String> {
        let document = Html::parse_document(html);
        let selector = Selector::parse("a[href]").unwrap();

        document
            .select(&selector)
            .filter_map(|el| el.value().attr("href"))
            .filter_map(|href| {
                // Resolver URLs relativas
                base_url.join(href).ok()
            })
            .filter(|url| {
                // Solo HTTP/HTTPS
                url.scheme() == "http" || url.scheme() == "https"
            })
            .filter(|url| {
                // Opcional: Solo mismo dominio
                url.host_str() == base_url.host_str()
            })
            .map(|url| url.to_string())
            .collect()
    }
}

// =============================================================================
// MAIN
// =============================================================================

#[tokio::main]
async fn main() {
    println!("=== Web Crawler Asíncrono ===\n");

    let config = CrawlerConfig {
        max_depth: 1,
        max_concurrent: 3,
        timeout_ms: 10000,
        rate_limit_ms: 500,
    };

    println!("Configuración:");
    println!("  Profundidad máxima: {}", config.max_depth);
    println!("  Concurrencia máxima: {}", config.max_concurrent);
    println!("  Timeout: {}ms", config.timeout_ms);
    println!("  Rate limit: {}ms\n", config.rate_limit_ms);

    let crawler = Crawler::new(config);

    // URL de prueba (example.com es seguro para testing)
    let seed_url = "https://example.com";
    println!("Iniciando crawl desde: {}\n", seed_url);

    let result = crawler.crawl(seed_url).await;

    println!("=== Resultados ===");
    println!("Páginas visitadas: {}", result.pages_visited);
    println!("Enlaces encontrados: {}", result.links_found);
    println!("Errores: {}", result.errors);
    println!("Tiempo total: {:?}", result.duration);

    if !result.pages.is_empty() {
        println!("\nPáginas procesadas:");
        for page in &result.pages {
            println!(
                "  [{}] {} (depth={}, links={})",
                page.status,
                page.url,
                page.depth,
                page.links.len()
            );
        }
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_links_absolutos() {
        let html = r#"
            <html>
                <body>
                    <a href="https://example.com/page1">Link 1</a>
                    <a href="https://example.com/page2">Link 2</a>
                </body>
            </html>
        "#;

        let base = Url::parse("https://example.com").unwrap();
        let links = Crawler::extract_links(html, &base);

        assert_eq!(links.len(), 2);
        assert!(links.contains(&"https://example.com/page1".to_string()));
        assert!(links.contains(&"https://example.com/page2".to_string()));
    }

    #[test]
    fn test_extract_links_relativos() {
        let html = r#"
            <html>
                <body>
                    <a href="/page1">Link 1</a>
                    <a href="page2">Link 2</a>
                </body>
            </html>
        "#;

        let base = Url::parse("https://example.com/dir/").unwrap();
        let links = Crawler::extract_links(html, &base);

        assert!(links.contains(&"https://example.com/page1".to_string()));
        assert!(links.contains(&"https://example.com/dir/page2".to_string()));
    }

    #[test]
    fn test_extract_links_filtra_otros_dominios() {
        let html = r#"
            <html>
                <body>
                    <a href="https://example.com/internal">Internal</a>
                    <a href="https://other.com/external">External</a>
                </body>
            </html>
        "#;

        let base = Url::parse("https://example.com").unwrap();
        let links = Crawler::extract_links(html, &base);

        // Solo debe incluir enlaces del mismo dominio
        assert_eq!(links.len(), 1);
        assert!(links.contains(&"https://example.com/internal".to_string()));
    }

    #[test]
    fn test_config_default() {
        let config = CrawlerConfig::default();

        assert_eq!(config.max_depth, 2);
        assert_eq!(config.max_concurrent, 5);
        assert_eq!(config.timeout_ms, 5000);
        assert_eq!(config.rate_limit_ms, 100);
    }

    #[tokio::test]
    async fn test_crawler_creation() {
        let config = CrawlerConfig::default();
        let crawler = Crawler::new(config);

        // Verificar que se creó correctamente
        assert!(crawler.visited.lock().await.is_empty());
    }

    // Test con mock server (requiere wiremock)
    // #[tokio::test]
    // async fn test_crawler_with_mock() {
    //     use wiremock::{MockServer, Mock, ResponseTemplate};
    //     use wiremock::matchers::path;
    //
    //     let mock_server = MockServer::start().await;
    //
    //     Mock::given(path("/"))
    //         .respond_with(ResponseTemplate::new(200)
    //             .set_body_string("<html><a href=\"/page1\">Link</a></html>"))
    //         .mount(&mock_server)
    //         .await;
    //
    //     let config = CrawlerConfig {
    //         max_depth: 1,
    //         ..Default::default()
    //     };
    //
    //     let crawler = Crawler::new(config);
    //     let result = crawler.crawl(&mock_server.uri()).await;
    //
    //     assert!(result.pages_visited >= 1);
    // }
}
