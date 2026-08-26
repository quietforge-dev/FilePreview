use std::{
    path::{Path, PathBuf},
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use tauri::{AppHandle, Emitter};

use crate::error::AppError;

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkspaceFilesChanged {
    workspace_path: String,
}

pub struct FileWatchService {
    watcher: Mutex<Option<RecommendedWatcher>>,
    watched_root: Arc<Mutex<Option<PathBuf>>>,
}

impl Default for FileWatchService {
    fn default() -> Self {
        Self {
            watcher: Mutex::new(None),
            watched_root: Arc::new(Mutex::new(None)),
        }
    }
}

impl FileWatchService {
    pub fn watch_workspace(&self, app: &AppHandle, root: &Path) -> Result<(), AppError> {
        self.ensure_watcher(app)?;
        let root = root.to_path_buf();
        let previous = self
            .watched_root
            .lock()
            .expect("文件监听路径锁已损坏")
            .replace(root.clone());
        let mut watcher = self.watcher.lock().expect("文件监听器锁已损坏");
        let watcher = watcher.as_mut().expect("文件监听器应已初始化");
        if let Some(previous) = previous {
            let _ = watcher.unwatch(&previous);
        }
        watcher.watch(&root, RecursiveMode::Recursive)?;
        Ok(())
    }

    fn ensure_watcher(&self, app: &AppHandle) -> Result<(), AppError> {
        if self.watcher.lock().expect("文件监听器锁已损坏").is_some() {
            return Ok(());
        }

        let (sender, receiver) = mpsc::channel();
        let watched_root = Arc::clone(&self.watched_root);
        let app_handle = app.clone();
        thread::spawn(move || dispatch_changes(receiver, watched_root, app_handle));
        let watcher = RecommendedWatcher::new(
            move |event: notify::Result<notify::Event>| {
                if event.is_ok() {
                    let _ = sender.send(());
                }
            },
            Config::default(),
        )?;
        *self.watcher.lock().expect("文件监听器锁已损坏") = Some(watcher);
        Ok(())
    }
}

fn dispatch_changes(
    receiver: mpsc::Receiver<()>,
    watched_root: Arc<Mutex<Option<PathBuf>>>,
    app: AppHandle,
) {
    while receiver.recv().is_ok() {
        thread::sleep(Duration::from_millis(500));
        while receiver.try_recv().is_ok() {}
        let root = watched_root.lock().expect("文件监听路径锁已损坏").clone();
        if let Some(root) = root {
            let _ = app.emit(
                "workspace-files-changed",
                WorkspaceFilesChanged {
                    workspace_path: root.to_string_lossy().to_string(),
                },
            );
        }
    }
}
