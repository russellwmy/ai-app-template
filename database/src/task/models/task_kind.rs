use serde::{Deserialize, Serialize};

use crate::{CallbackTask, DownloadTask, IndexingTask, UploadTask};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind", content = "payload")]
pub enum TaskKind {
    CallbackTask(CallbackTask),
    DownloadTask(DownloadTask),
    IndexingTask(IndexingTask),
    UploadTask(UploadTask),
}

impl From<CallbackTask> for TaskKind {
    fn from(value: CallbackTask) -> Self {
        TaskKind::CallbackTask(value)
    }
}

impl From<DownloadTask> for TaskKind {
    fn from(value: DownloadTask) -> Self {
        TaskKind::DownloadTask(value)
    }
}

impl From<IndexingTask> for TaskKind {
    fn from(value: IndexingTask) -> Self {
        TaskKind::IndexingTask(value)
    }
}

impl From<UploadTask> for TaskKind {
    fn from(value: UploadTask) -> Self {
        TaskKind::UploadTask(value)
    }
}
