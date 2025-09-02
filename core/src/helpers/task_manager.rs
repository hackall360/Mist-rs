use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use uuid::Uuid;

/// Manages asynchronous tasks spawned via `tokio`.
#[derive(Default, Clone)]
pub struct TaskManager {
    tasks: Arc<Mutex<HashMap<Uuid, JoinHandle<()>>>>,
}

impl TaskManager {
    /// Create a new [`TaskManager`].
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Spawn a new asynchronous task and return its identifier.
    pub async fn spawn<F>(&self, fut: F) -> Uuid
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let id = Uuid::new_v4();
        let handle = tokio::spawn(fut);
        self.tasks.lock().await.insert(id, handle);
        id
    }

    /// Cancel a running task if it exists.
    pub async fn cancel(&self, id: Uuid) {
        if let Some(handle) = self.tasks.lock().await.remove(&id) {
            handle.abort();
        }
    }

    /// Wait for a task to finish and remove it from management.
    pub async fn join(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(handle) = self.tasks.lock().await.remove(&id) {
            handle.await?;
        }
        Ok(())
    }
}
