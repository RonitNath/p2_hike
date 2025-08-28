use std::sync::Arc;
use axum::response::Html;
use tera::{Context, Tera};

use crate::error::AppError;

#[derive(Clone)]
pub struct AppState {
    tera: Arc<Tera>,
    env: Env
}

#[derive(Clone, PartialEq)]
pub enum Env {
    Dev,
    Prod,
}

impl AppState {
    pub fn new() -> Self {
        let tera = Tera::new("templates/**/*").expect("failed to initialize Tera");
        let env = std::env::var("ENV").unwrap_or("dev".to_string());
        let env = match env.as_str() {
            "dev" => Env::Dev,
            "prod" => Env::Prod,
            _ => panic!("invalid environment: {}", env),
        };
        Self { tera: Arc::new(tera), env }
    }

    pub fn render(&self, template: &str, ctx: Option<Context>) -> Result<Html<String>, AppError> {
        let ctx = ctx.unwrap_or_else(Context::new);

        let html = if self.env == Env::Dev {
            let tera = Tera::new("templates/**/*").expect("failed to initialize Tera");
            tera.render(template, &ctx)
        } else {
            self.tera.render(template, &ctx)
        }?;

        Ok(Html(html))
    }
}