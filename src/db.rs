use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(u64);

#[derive(Default, Deserialize, Serialize)]
pub struct UserPermissions {
    pub create_pages: bool,
    pub edit_pages: bool,
}

impl UserPermissions {
    pub const fn no_permissions() -> Self {
        Self {
            create_pages: false,
            edit_pages: false,
        }
    }

    pub const fn admin() -> Self {
        Self {
            create_pages: true,
            edit_pages: true,
        }
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct UserAccount {
    pub id: UserId,
    pub username: String,
    pub permissions: UserPermissions,
}

#[derive(Default, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct WikiArticleId(String);

#[derive(Deserialize, Serialize)]
pub struct WikiEdit {
    pub author_id: UserId,
    pub timestamp: u64,
}

#[derive(Default, Deserialize, Serialize)]
pub struct WikiArticle {
    pub id: WikiArticleId,

    /// In markdown, with images embedded inside as Base64
    pub content: String,

    pub history: Vec<WikiEdit>,
}

pub struct Database {
    root_dir: PathBuf,

    users: HashMap<UserId, UserAccount>,
    wiki_articles: HashMap<WikiArticleId, WikiArticle>,
}

impl Database {
    pub async fn load_or_init<P: Into<PathBuf>>(root_dir: P) -> Result<Self> {
        let root_dir = root_dir.into();

        if root_dir.exists() {
            Self::load(root_dir)
        } else {
            Self::init(root_dir).await
        }
    }

    fn new(root_dir: PathBuf) -> Self {
        Self {
            root_dir,

            users: HashMap::new(),
            wiki_articles: HashMap::new(),
        }
    }

    fn load(root_dir: PathBuf) -> Result<Self> {
        let mut db = Self::new(root_dir);

        let users_json = std::fs::read_to_string(db.root_dir.join("users.json"))?;
        db.users = serde_json::from_str(&users_json)?;

        let wiki_folder_path = db.root_dir.join("wiki_articles");

        for entry in std::fs::read_dir(&wiki_folder_path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            if metadata.is_file() {
                // We can try to load this article!
                let json = std::fs::read_to_string(entry.path())?;
                let article: WikiArticle = serde_json::from_str(&json)?;
                db.wiki_articles.insert(article.id.clone(), article);
            } else {
                error!("wiki_articles folder contains a directory! Ignoring the directory...");
            }
        }

        Ok(db)
    }

    async fn init(root_dir: PathBuf) -> Result<Self> {
        let mut db = Self::new(root_dir);
        db.users.insert(
            UserId(0),
            UserAccount {
                id: UserId(0),
                username: String::from("luci"),
                permissions: UserPermissions::admin(),
            },
        );
        db.sync_to_disk().await?;
        Ok(db)
    }

    async fn sync_to_disk(&self) -> Result<()> {
        let wiki_folder_path = self.root_dir.join("wiki_articles");

        // Ensure the right folder structure exists!
        tokio::fs::create_dir_all(&wiki_folder_path).await?;

        // Write to the users file
        let users_json = serde_json::to_string(&self.users)?;
        tokio::fs::write(self.root_dir.join("users.json"), users_json).await?;

        // Write the individual articles to disk
        for (id, article) in &self.wiki_articles {
            let json = serde_json::to_string(article)?;
            tokio::fs::write(wiki_folder_path.join(format!("{}.json", id.0)), json).await?;
        }

        Ok(())
    }
}
