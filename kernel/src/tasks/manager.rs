use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_cron_scheduler::JobScheduler;
use tracing::error;
use crate::config::server_config;
use crate::tasks::task_one;

/// 调度器管理器
pub struct SchedulerManager {
    scheduler: Arc<RwLock<Option<JobScheduler>>>,
}

impl SchedulerManager {
    /// 创建新的调度器管理器
    pub fn new() -> Self {
        Self {
            scheduler: Arc::new(RwLock::new(None)),
        }
    }

    /// 启动所有定时任务
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        if server_config().cron {
            let mut scheduler_lock = self.scheduler.write().await;
            // 创建调度器
            let sched = JobScheduler::new().await?;
            // 添加定时任务
            self.add_jobs(&sched).await?;
            // 启动调度器
            sched.start().await?;
            // 保存调度器实例
            *scheduler_lock = Some(sched);

            println!("{} cron 定时任务调度器已启动成功!!!", "✅");
            println!();
        }
        Ok(())
    }

    /// 添加具体的任务到调度器
    async fn add_jobs(&self, sched: &JobScheduler) -> Result<(), Box<dyn std::error::Error>> {
        // 示例任务
        if let Ok(job) = task_one::handle_task() {
            sched.add(job).await?; // 加入调度器
        }

        Ok(())
    }

    /// 优雅关闭调度器
    pub async fn shutdown(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut scheduler_lock = self.scheduler.write().await;

        if let Some(mut sched) = scheduler_lock.take() {
            sched.shutdown().await?;
            println!(
                "\n❌ cron定时任务调度器已关闭 [{}]",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
            );
        } else {
            if server_config().cron {
                error!("调度器实例不存在");
            }
        }

        Ok(())
    }

    /// 获取调度器句柄，用于在 select! 中等待关闭信号
    /// 返回一个 future，可以等待调度器完成关闭
    pub fn shutdown_future(&self) -> impl Future<Output = ()> + '_ {
        async move {
            if let Err(e) = self.shutdown().await {
                error!("\n 关闭调度器时出错: {}\n", e);
            }
        }
    }
}
