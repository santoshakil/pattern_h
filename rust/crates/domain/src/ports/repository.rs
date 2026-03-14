use async_trait::async_trait;
use errors::StorageError;

#[async_trait]
pub trait Repository<T: Send + Sync, Id: Send + Sync>: Send + Sync {
    async fn find_by_id(&self, id: &Id) -> Result<Option<T>, StorageError>;
    async fn find_all(&self) -> Result<Vec<T>, StorageError>;
    async fn save(&self, entity: &T) -> Result<(), StorageError>;
    async fn delete(&self, id: &Id) -> Result<bool, StorageError>;
}
