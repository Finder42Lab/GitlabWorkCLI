use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(about = "Инициализировать проект")]
    Init,

    #[clap(subcommand, about = "Глобальная конфигурация (alias: gc)", alias = "gc")]
    GlobalConfig(GlobalConfigCommands),

    #[clap(about = "Переключиться на ветку задачи (alias: cot)", alias = "cot")]
    CheckoutTask { task_iid: u64 },

    #[clap(about = "Переключиться на ветку фичи (alias: cof)", alias = "cof")]
    CheckoutFeature { feature_iid: u16 },

    #[clap(about = "Создать MR (alias: mr)", alias = "mr")]
    MergeRequest {
        /// Исходная ветка
        #[arg(short, long)]
        source: Option<String>,

        /// Целевая ветка
        #[arg(short, long)]
        target: Option<String>,

        /// Добавить ревьюверов в MR
        #[arg(short, long)]
        review: bool
    },
}

#[derive(Subcommand, Debug)]
pub enum GlobalConfigCommands {
    #[clap(about = "Установить токен (alias: st)", alias = "st", hide = false)]
    SetToken { token: String },
    #[clap(about = "Установить gitlab хост (alias: sh)", alias = "sh", hide = false)]
    SetHost { host: String },
}