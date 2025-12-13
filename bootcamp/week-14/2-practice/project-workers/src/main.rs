//! # Proyecto Final: Thread Pool Completo
//!
//! Implementa un Thread Pool robusto con las siguientes características:
//! - Pool de N workers configurables
//! - Cola de jobs con channel
//! - Shutdown graceful
//! - Estadísticas de ejecución
//! - Jobs con resultados (futures simulados)

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

fn main() {
    println!("=== Proyecto Final: Thread Pool ===\n");

    demo_basic();
    demo_with_results();
    demo_statistics();
}

// ============================================================================
// DEMOS
// ============================================================================

fn demo_basic() {
    println!("--- Demo: Uso Básico ---");

    let pool = ThreadPool::new(4);

    for i in 0..8 {
        pool.execute(move || {
            println!("  Job {} ejecutándose en {:?}", i, thread::current().id());
            thread::sleep(Duration::from_millis(100));
        });
    }

    pool.shutdown();
    println!("  Pool cerrado\n");
}

fn demo_with_results() {
    println!("--- Demo: Jobs con Resultados ---");

    let pool = ThreadPool::new(4);
    let mut receivers = vec![];

    // Enviar jobs que retornan resultados
    for i in 0..5 {
        let rx = pool.execute_with_result(move || {
            thread::sleep(Duration::from_millis(50));
            i * i
        });
        receivers.push(rx);
    }

    // Recolectar resultados
    let results: Vec<i32> = receivers
        .into_iter()
        .map(|rx| rx.recv().unwrap())
        .collect();

    println!("  Cuadrados: {:?}", results);
    pool.shutdown();
    println!();
}

fn demo_statistics() {
    println!("--- Demo: Estadísticas ---");

    let pool = ThreadPool::with_stats(4);

    for i in 0..20 {
        pool.execute(move || {
            thread::sleep(Duration::from_millis(10 * (i % 5 + 1)));
        });
    }

    // Esperar un poco
    thread::sleep(Duration::from_millis(500));

    let stats = pool.stats();
    println!("  Jobs completados: {}", stats.jobs_completed);
    println!("  Jobs pendientes: {}", stats.jobs_pending);
    println!("  Tiempo total: {:?}", stats.total_time);

    pool.shutdown();
    println!("  Pool cerrado\n");
}

// ============================================================================
// IMPLEMENTACIÓN - COMPLETA LOS TODOs
// ============================================================================

/// Mensaje que se envía a los workers
enum Message {
    /// Un job para ejecutar
    Job(Job),
    /// Señal para terminar
    Terminate,
}

/// Un job es una función que se ejecuta una vez
type Job = Box<dyn FnOnce() + Send + 'static>;

/// Estadísticas del pool
#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    pub jobs_completed: usize,
    pub jobs_pending: usize,
    pub total_time: Duration,
}

/// Thread Pool con workers
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Message>>,
    stats: Option<Arc<Mutex<PoolStats>>>,
    start_time: Instant,
}

impl ThreadPool {
    /// Crea un nuevo ThreadPool con `size` workers
    ///
    /// # Panics
    /// Panics si size es 0
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "El pool debe tener al menos 1 worker");

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), None));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
            stats: None,
            start_time: Instant::now(),
        }
    }

    /// Crea un ThreadPool con tracking de estadísticas
    pub fn with_stats(size: usize) -> Self {
        assert!(size > 0, "El pool debe tener al menos 1 worker");

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let stats = Arc::new(Mutex::new(PoolStats::default()));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), Some(Arc::clone(&stats))));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
            stats: Some(stats),
            start_time: Instant::now(),
        }
    }

    /// Ejecuta un job en el pool
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        if let Some(ref sender) = self.sender {
            // Incrementar pending si hay stats
            if let Some(ref stats) = self.stats {
                let mut s = stats.lock().unwrap();
                s.jobs_pending += 1;
            }

            sender.send(Message::Job(job)).expect("Error enviando job");
        }
    }

    /// Ejecuta un job y retorna un receiver para obtener el resultado
    pub fn execute_with_result<F, T>(&self, f: F) -> Receiver<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = mpsc::channel();

        self.execute(move || {
            let result = f();
            let _ = tx.send(result);
        });

        rx
    }

    /// Obtiene las estadísticas actuales
    pub fn stats(&self) -> PoolStats {
        if let Some(ref stats) = self.stats {
            let mut s = stats.lock().unwrap().clone();
            s.total_time = self.start_time.elapsed();
            s
        } else {
            PoolStats {
                total_time: self.start_time.elapsed(),
                ..Default::default()
            }
        }
    }

    /// Cierra el pool esperando a que terminen todos los jobs
    pub fn shutdown(mut self) {
        // Enviar señal de terminación a todos los workers
        if let Some(sender) = self.sender.take() {
            for _ in &self.workers {
                sender.send(Message::Terminate).expect("Error enviando Terminate");
            }
        }

        // Esperar a que terminen todos los workers
        for worker in self.workers.drain(..) {
            if let Some(thread) = worker.thread {
                thread.join().expect("Error en join de worker");
            }
        }
    }

    /// Retorna el número de workers
    pub fn worker_count(&self) -> usize {
        self.workers.len()
    }
}

/// Un worker que ejecuta jobs
struct Worker {
    #[allow(dead_code)]
    id: usize,
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<Receiver<Message>>>,
        stats: Option<Arc<Mutex<PoolStats>>>,
    ) -> Self {
        let thread = thread::spawn(move || loop {
            // Obtener mensaje (lock solo durante recv)
            let message = {
                let rx = receiver.lock().expect("Lock poisoned");
                rx.recv()
            };

            match message {
                Ok(Message::Job(job)) => {
                    // Ejecutar el job
                    job();

                    // Actualizar estadísticas
                    if let Some(ref stats) = stats {
                        let mut s = stats.lock().unwrap();
                        s.jobs_completed += 1;
                        if s.jobs_pending > 0 {
                            s.jobs_pending -= 1;
                        }
                    }
                }
                Ok(Message::Terminate) | Err(_) => {
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// ============================================================================
// ADDITIONAL EXERCISES
// ============================================================================

/// # Exercise 1: Job Timeout
///
/// Implementa una versión de execute que cancele jobs que tarden demasiado.
/// Hint: Usar un thread separado con sleep + flag atómico
#[allow(dead_code)]
fn exercise_timeout() {
    // TODO: Implementar execute_with_timeout
}

/// # Exercise 2: Job Priority
///
/// Implementa una cola con prioridades (alta, media, baja).
/// Los jobs de alta prioridad se ejecutan primero.
#[allow(dead_code)]
fn exercise_priority() {
    // TODO: Implementar PriorityThreadPool
}

/// # Exercise 3: Dynamic Pool
///
/// Implementa un pool que ajuste el número de workers según la carga.
/// - Si hay muchos jobs pendientes, agregar workers
/// - Si los workers están idle, reducir
#[allow(dead_code)]
fn exercise_dynamic() {
    // TODO: Implementar DynamicThreadPool
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_pool_creation() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.worker_count(), 4);
        pool.shutdown();
    }

    #[test]
    #[should_panic(expected = "al menos 1 worker")]
    fn test_pool_zero_workers() {
        ThreadPool::new(0);
    }

    #[test]
    fn test_execute_jobs() {
        let pool = ThreadPool::new(2);
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..10 {
            let c = Arc::clone(&counter);
            pool.execute(move || {
                c.fetch_add(1, Ordering::SeqCst);
            });
        }

        pool.shutdown();
        assert_eq!(counter.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_execute_with_result() {
        let pool = ThreadPool::new(2);

        let rx = pool.execute_with_result(|| 42);
        assert_eq!(rx.recv().unwrap(), 42);

        pool.shutdown();
    }

    #[test]
    fn test_multiple_results() {
        let pool = ThreadPool::new(4);
        let mut receivers = vec![];

        for i in 0..10 {
            let rx = pool.execute_with_result(move || i * 2);
            receivers.push((i, rx));
        }

        for (i, rx) in receivers {
            assert_eq!(rx.recv().unwrap(), i * 2);
        }

        pool.shutdown();
    }

    #[test]
    fn test_stats() {
        let pool = ThreadPool::with_stats(2);

        for _ in 0..5 {
            pool.execute(|| {
                thread::sleep(Duration::from_millis(10));
            });
        }

        thread::sleep(Duration::from_millis(200));
        pool.shutdown();

        let stats = pool.stats();
        assert_eq!(stats.jobs_completed, 5);
    }

    #[test]
    fn test_concurrent_access() {
        let pool = ThreadPool::new(4);
        let data = Arc::new(Mutex::new(Vec::new()));

        for i in 0..100 {
            let d = Arc::clone(&data);
            pool.execute(move || {
                let mut v = d.lock().unwrap();
                v.push(i);
            });
        }

        pool.shutdown();
        assert_eq!(data.lock().unwrap().len(), 100);
    }

    #[test]
    fn test_heavy_workload() {
        let pool = ThreadPool::new(8);
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..1000 {
            let c = Arc::clone(&counter);
            pool.execute(move || {
                c.fetch_add(1, Ordering::SeqCst);
            });
        }

        pool.shutdown();
        assert_eq!(counter.load(Ordering::SeqCst), 1000);
    }
}
