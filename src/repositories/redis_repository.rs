use deadpool_redis::{
    Config,
    Pool,
    Runtime,
};

use deadpool_redis::Connection;
use std::collections::HashMap;
use rand::Rng;

use redis::AsyncCommands;

use crate::models::link::Link;

pub fn short_code(size: usize) -> String {
    let chars = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();

    (0..size)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars[idx] as char
        })
        .collect()
}

#[derive(Clone)]
pub struct RedisStore {
    pool: Pool,
}

impl RedisStore {
    pub fn new(cfg: Config) -> Self {
        Self {
            pool: cfg
            .create_pool(Some(Runtime::Tokio1))
            .expect("Erro ao criar pool Redis")
        }
    }

    async fn conn(&self) -> Result<Connection, deadpool_redis::PoolError>{
        self.pool.get().await
    }

    pub async fn create_link(&self, url: &str) -> Result<String, String> {
        let code = short_code(12);

        let mut conn = 
            self
            .conn()
            .await
            .map_err(|e| e.to_string())?;
        
        let _:() = 
            conn
            .hset_multiple(
                &code, 
                &[("url", url), ("clicks", "0")])
            .await
            .map_err(|e| e.to_string())?;
        
        let _: u32 = conn
        .lpush("links", &code)
        .await
        .map_err(|e| e.to_string())?;
        
        Ok(code)
    }
    
    async fn get_link(&self, code: &str) -> Result<Link, String> {
        let mut conn = self
            .conn()
            .await
            .map_err(|e| e.to_string())?;

        let link = self
            .get_link_conn(code, &mut conn)
            .await?;

        Ok(link)
    }

    pub async fn del_code(
    &self,
    code: &str,
    ) -> Result<(), String> {

        let mut conn = self
            .conn()
            .await
            .map_err(|e| e.to_string())?;

        //esse trecho é uma adaptação de IA ainda não entendo o que tudo isso faz, só sei que juntos, fazem uma requisição ao redis, de maneira atomica, e com varias operações por vez, ainda não sei se faz rollback sozinho
        let (hash, list): (i32, i32) = redis::pipe()
            .atomic()
            .del(code)
            .lrem("links", 1, code)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        if hash == 0 && list == 0 {
            return Err("Código não encontrado".to_string());
        }

        Ok(())
    }

    async fn get_link_conn(
        &self,
        code: &str,
        conn: &mut Connection,
    ) -> Result<Link, String>{
        let data: HashMap<String, String> = 
            conn
            .hgetall(code)
            .await
            .map_err(|e| e.to_string())?;

        if data.is_empty() {
            return Err("Link não encontrado".to_string());
        }

        let url = match data.get("url") {
            Some(t) => t,
            None => return Err("Url não encontrado".to_string())
        };

        let clicks = match data.get("clicks") {
            Some(t) => t
                .parse::<u32>()
                .map_err(|_| "Clicks inválido".to_string())?,
            None => return Err("Clicks não encontrado".to_string()),
        };

        Ok(Link {
            code: code.to_string(),
            original_url: url.to_string(),
            clicks:clicks     
        })
    }
        

    pub async fn get_url(&self, code: &str) -> Result<String, String>{
        let mut conn = 
            self
            .conn()
            .await
            .map_err(|e| e.to_string())?;

        let data: HashMap<String, String> = 
            conn
            .hgetall(code)
            .await
            .map_err(|e| e.to_string())?;

        if data.is_empty() {
            return Err("Link não encontrado".to_string());
        }

        let url = match data.get("url") {
            Some(t) => t,
            None => return Err("Url não encontrado".to_string())
        };

        let _: u32 = 
            conn
            .hincr(code, "clicks", "1")
            .await
            .map_err(|e| e.to_string())?;

        Ok(url.to_string())

    }

    pub async fn lasted_links(&self) -> Result<Vec<Link>, String>{
        let mut conn = 
            self
            .conn()
            .await
            .map_err(|e| e.to_string())?;

        let codes: Vec<String> = 
            conn
            .lrange("links", 0, 49)
            .await
            .map_err(|e| e.to_string())?;

        let mut links: Vec<Link> = vec![];
        for code in codes {
            match self.get_link_conn(&code, &mut conn).await{
                Ok(value) => links.push(value),
                Err(msg) => links.push(Link { code, original_url: format!("ERRO: {msg}"), clicks: 0 }),
            };
        }
        Ok(links)
    }
}