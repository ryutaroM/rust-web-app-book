use adapter::{database::ConnectionPool, repository::health::HealthCheckRepositoryImpl};
use kernel::repository::health::HealthRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<dyn HealthRepository>,
}

impl AppRegistry {
    pub fn new(pool: ConnectionPool) -> Self {
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        Self {
            health_check_repository,
        }
    }

    pub fn health_check_repository(&self) -> Arc<dyn HealthRepository> {
        self.health_check_repository.clone()
    }
}
