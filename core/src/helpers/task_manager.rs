use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;
use uuid::Uuid;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

/// Callback invoked with task progress updates.
pub type ProgressHook = Arc<dyn Fn(Uuid, f32) + Send + Sync>;
/// Callback invoked when a task reports an error.
pub type ErrorHook = Arc<dyn Fn(Uuid, String) + Send + Sync>;
/// Callback invoked when a task is cancelled.
pub type CancelHook = Arc<dyn Fn(Uuid) + Send + Sync>;

struct ManagedTask {
    handle: JoinHandle<Result<(), BoxError>>,
}

/// Manages asynchronous tasks spawned via `tokio`.
#[derive(Clone, Default)]
pub struct TaskManager {
    tasks: Arc<Mutex<HashMap<Uuid, ManagedTask>>>,
    on_progress: Option<ProgressHook>,
    on_error: Option<ErrorHook>,
    on_cancel: Option<CancelHook>,
}

impl TaskManager {
    /// Create a new [`TaskManager`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a callback for progress updates.
    pub fn on_progress<F>(&mut self, f: F)
    where
        F: Fn(Uuid, f32) + Send + Sync + 'static,
    {
        self.on_progress = Some(Arc::new(f));
    }

    /// Set a callback for task errors.
    pub fn on_error<F>(&mut self, f: F)
    where
        F: Fn(Uuid, String) + Send + Sync + 'static,
    {
        self.on_error = Some(Arc::new(f));
    }

    /// Set a callback for task cancellation.
    pub fn on_cancel<F>(&mut self, f: F)
    where
        F: Fn(Uuid) + Send + Sync + 'static,
    {
        self.on_cancel = Some(Arc::new(f));
    }

    /// Spawn a new asynchronous task and return its identifier.
    ///
    /// The provided closure receives a [`mpsc::UnboundedSender`] which can
    /// be used to report progress as a float between 0.0 and 1.0.
    pub async fn spawn<F, Fut>(&self, f: F) -> Uuid
    where
        F: FnOnce(mpsc::UnboundedSender<f32>) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = Result<(), BoxError>> + Send + 'static,
    {
        let id = Uuid::new_v4();
        let progress_id = id.clone();
        let error_id = id.clone();

        let (tx, mut rx) = mpsc::unbounded_channel::<f32>();

        if let Some(progress_hook) = self.on_progress.clone() {
            tokio::spawn(async move {
                while let Some(p) = rx.recv().await {
                    (progress_hook)(progress_id, p);
                }
            });
        } else {
            tokio::spawn(async move {
                while rx.recv().await.is_some() {}
            });
        }

        let error_hook = self.on_error.clone();
        let handle = tokio::spawn(async move {
            let result = f(tx).await;
            if let Err(ref e) = result {
                if let Some(hook) = error_hook {
                    hook(error_id, e.to_string());
                }
            }
            result
        });

        self.tasks
            .lock()
            .await
            .insert(id.clone(), ManagedTask { handle });

        id
    }

    /// Cancel a running task if it exists.
    pub async fn cancel(&self, id: Uuid) {
        if let Some(task) = self.tasks.lock().await.remove(&id) {
            task.handle.abort();
            if let Some(cb) = &self.on_cancel {
                cb(id);
            }
        }
    }

    /// Wait for a task to finish and remove it from management.
    pub async fn join(&self, id: Uuid) -> Result<(), BoxError> {
        if let Some(task) = self.tasks.lock().await.remove(&id) {
            match task.handle.await {
                Ok(res) => res,
                Err(e) => {
                    if e.is_cancelled() {
                        Ok(())
                    } else {
                        Err(Box::new(e))
                    }
                }
            }
        } else {
            Ok(())
        }
    }
}

