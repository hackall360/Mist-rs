use std::io;
use std::sync::{Arc, Mutex};

use mist_core::helpers::task_manager::TaskManager;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn multiple_concurrent_tasks() {
    let mut manager = TaskManager::new();

    let progress = Arc::new(Mutex::new(Vec::new()));
    let errors = Arc::new(Mutex::new(Vec::new()));
    let cancellations = Arc::new(Mutex::new(Vec::new()));

    {
        let progress = progress.clone();
        manager.on_progress(move |id, p| {
            progress.lock().unwrap().push((id, p));
        });
    }

    {
        let errors = errors.clone();
        manager.on_error(move |id, err| {
            errors.lock().unwrap().push((id, err));
        });
    }

    {
        let cancellations = cancellations.clone();
        manager.on_cancel(move |id| {
            cancellations.lock().unwrap().push(id);
        });
    }

    // Task 1: completes successfully and reports progress
    let id_ok = manager
        .spawn(|tx| async move {
            for i in 0..=5 {
                tx.send(i as f32 / 5.0).unwrap();
                sleep(Duration::from_millis(10)).await;
            }
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        })
        .await;

    // Task 2: fails immediately
    let id_err = manager
        .spawn(|tx| async move {
            tx.send(0.2).unwrap();
            Err::<(), Box<dyn std::error::Error + Send + Sync>>(io::Error::new(io::ErrorKind::Other, "fail").into())
        })
        .await;

    // Task 3: long running, will be cancelled
    let id_cancel = manager
        .spawn(|tx| async move {
            loop {
                tx.send(0.0).unwrap();
                sleep(Duration::from_millis(50)).await;
            }
            #[allow(unreachable_code)]
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        })
        .await;

    sleep(Duration::from_millis(120)).await;

    manager.cancel(id_cancel).await;

    manager.join(id_ok).await.expect("join ok");
    assert!(manager.join(id_err).await.is_err());
    manager.join(id_cancel).await.expect("join cancel");

    let progress = progress.lock().unwrap().clone();
    assert!(progress.iter().any(|(id, p)| *id == id_ok && (*p - 1.0).abs() < f32::EPSILON));

    let errors = errors.lock().unwrap();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].0, id_err);

    let cancellations = cancellations.lock().unwrap();
    assert_eq!(cancellations.len(), 1);
    assert_eq!(cancellations[0], id_cancel);
}

