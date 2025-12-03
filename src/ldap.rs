use nu_protocol::LabeledError;
use tokio::task::JoinHandle;
use tokio_util::{sync::CancellationToken, task::TaskTracker};

#[derive(Debug)]
pub struct LdapPlugin {
    pub main_runtime: tokio::runtime::Runtime,
    pub tasks: TaskTracker,
    pub cancel: CancellationToken,
}

impl LdapPlugin {
    pub fn new() -> Self {
        LdapPlugin {
            main_runtime: tokio::runtime::Runtime::new().unwrap(),
            tasks: TaskTracker::new(),
            cancel: CancellationToken::new(),
        }
    }

    pub fn spawn<F>(&self, task: F) -> JoinHandle<F::Output>
    where
        F: Future<Output = Result<(), LabeledError>> + Send + 'static,
    {
        self.tasks.spawn(task)
    }

    pub fn spawn_blocking<F>(&self, task: F) -> JoinHandle<F::Output>
    where
        F: FnOnce() -> Result<(), LabeledError> + Send + 'static,
    {
        self.tasks.spawn_blocking(task)
    }

    pub async fn close(&self) {
        self.tasks.close();
        self.tasks.wait().await;
    }
}
