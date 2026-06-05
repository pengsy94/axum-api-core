
/// 自动管理时间戳的 Model
///
/// 为实现了该 trait 的 ActiveModel 在 insert/update 时自动设置
/// `created_at` 和 `updated_at` 字段。
///
/// # 用法
/// ```ignore
/// impl TimeStamp for sys_user::ActiveModel {
///     fn set_created_at(&mut self, time: ChronoDateTimeUtc) {
///         self.created_at = Set(Some(time));
///     }
///     fn set_updated_at(&mut self, time: ChronoDateTimeUtc) {
///         self.updated_at = Set(Some(time));
///     }
/// }
/// ```
pub trait TimeStamp {
    fn set_created_at(&mut self, time: ChronoDateTimeUtc);
    fn set_updated_at(&mut self, time: ChronoDateTimeUtc);

    /// 插入前调用：设置 created_at + updated_at
    fn before_insert(&mut self) {
        let now = chrono::Utc::now();
        self.set_created_at(now);
        self.set_updated_at(now);
    }

    /// 更新前调用：刷新 updated_at
    fn before_update(&mut self) {
        self.set_updated_at(chrono::Utc::now());
    }
}

/// 软删除接口
///
/// 为实现了该 trait 的 Model 提供软删除能力。
/// 查询时需要手动过滤 `deleted_at.is_null()`。
///
/// # 用法
/// ```ignore
/// impl SoftDelete for sys_user::ActiveModel {
///     fn set_deleted_at(&mut self, time: Option<ChronoDateTimeUtc>) {
///         self.deleted_at = Set(time);
///     }
/// }
/// ```
pub trait SoftDelete {
    fn set_deleted_at(&mut self, time: Option<ChronoDateTimeUtc>);

    /// 软删除：设置 deleted_at 为当前时间
    fn soft_delete(&mut self) {
        self.set_deleted_at(Some(chrono::Utc::now()));
    }

    /// 恢复：清除 deleted_at
    fn restore(&mut self) {
        self.set_deleted_at(None);
    }
}

/// 便捷类型别名
pub type ChronoDateTimeUtc = chrono::DateTime<chrono::Utc>;
