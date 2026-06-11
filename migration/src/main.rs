use migration::Migrator;
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;

const DB_URL: &str = "DATABASE_URL";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var(DB_URL).unwrap_or_else(|_| {
        eprintln!("请设置 {} 环境变量", DB_URL);
        std::process::exit(1);
    });

    let conn = Database::connect(&database_url).await.unwrap_or_else(|e| {
        eprintln!("数据库连接失败: {}", e);
        std::process::exit(1);
    });

    let args: Vec<String> = std::env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match command {
        "up" | "run" => {
            println!("运行迁移...");
            Migrator::up(&conn, None).await.unwrap();
            println!("✅ 迁移完成");
        }
        "down" | "rollback" => {
            println!("回滚迁移...");
            Migrator::down(&conn, None).await.unwrap();
            println!("✅ 回滚完成");
        }
        "fresh" => {
            println!("重置数据库...");
            Migrator::fresh(&conn).await.unwrap();
            println!("✅ 重置完成");
        }
        "status" => {
            println!("迁移状态:");
            Migrator::status(&conn).await.unwrap();
        }
        _ => {
            println!("用法: cargo run -p migration <command>");
            println!();
            println!("命令:");
            println!("  up        运行所有待执行的迁移");
            println!("  down      回滚最后一个迁移");
            println!("  fresh     回滚所有迁移后重新运行");
            println!("  status    查看迁移状态");
        }
    }
}
