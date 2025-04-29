pub use sea_orm_migration::prelude::*;

mod m20250125_083500_create_roles_table;
mod m20250125_085421_create_images_table;
mod m20250125_085443_create_users_table;
mod m20250420_204700_create_categories_table;
mod m20250125_093836_create_pages_table;
mod m20250125_085500_create_contents_table;
mod m20250428_133436_seed_admin_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![

            Box::new(m20250125_083500_create_roles_table::Migration),
            Box::new(m20250125_085421_create_images_table::Migration),
            Box::new(m20250125_085443_create_users_table::Migration),
            Box::new(m20250420_204700_create_categories_table::Migration),
            Box::new(m20250125_093836_create_pages_table::Migration),
            Box::new(m20250125_085500_create_contents_table::Migration),
            Box::new(m20250428_133436_seed_admin_user::Migration),

        ]
    }
}
