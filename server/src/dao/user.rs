use super::dto;
use crate::core::{
    model::{User, UserId},
    service::{GettingUserError, SavingUserError, UserService},
};
use diesel::{
    mysql::MysqlConnection,
    r2d2::{ConnectionManager, Pool},
    result::Error as DieselError,
    Connection, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl,
};

pub type ConnectionsPool = Pool<ConnectionManager<MysqlConnection>>;

pub struct UserDao {
    pool: ConnectionsPool,
}

impl UserDao {
    pub fn new(pool: ConnectionsPool) -> Self {
        Self { pool }
    }
}

impl UserService for UserDao {
    fn get_by_id(&self, id: UserId) -> Result<User, GettingUserError> {
        let connection = self
            .pool
            .get()
            .map_err(|err| GettingUserError::DbError(err.to_string()))?;

        get_by_id(&connection, id.0)
            .map_err(|err| GettingUserError::DbError(err.to_string()))?
            .map(|user_dto| User {
                id: UserId(user_dto.id),
                name: user_dto.name,
            })
            .ok_or(GettingUserError::UserWasNotFound)
    }

    fn list(&self) -> Result<Vec<User>, GettingUserError> {
        let connection = self
            .pool
            .get()
            .map_err(|err| GettingUserError::DbError(err.to_string()))?;

        let users = list(&connection)
            .map_err(|err| GettingUserError::DbError(err.to_string()))?
            .into_iter()
            .map(|user_dto| User {
                id: UserId(user_dto.id),
                name: user_dto.name,
            })
            .collect();
        Ok(users)
    }

    fn save(&self, mut user: User) -> Result<User, SavingUserError> {
        let connection = match self.pool.get() {
            Ok(connection) => connection,
            Err(err) => return Err(SavingUserError(err.to_string(), user)),
        };

        if user.id.is_exist() {
            match update(&connection, dto::User {
                id: user.id.into(),
                name: user.name.clone(),
            }) {
                Ok(is_updated) if is_updated => Ok(user),
                Ok(_) => Err(SavingUserError("User was not updated".to_string(), user)),
                Err(err) => Err(SavingUserError(err.to_string(), user)),
            }
        } else {
            match insert(&connection, dto::NewUser { name: &user.name }) {
                Ok(id) => {
                    user.id = UserId(id);
                    Ok(user)
                }
                Err(err) => Err(SavingUserError(err.to_string(), user)),
            }
        }
    }
}

pub fn get_by_id(connection: &MysqlConnection, user_id: u32) -> Result<Option<dto::User>, DieselError> {
    use crate::dao::schema::users::dsl::*;

    users
        .filter(id.eq(user_id))
        .get_result::<dto::User>(connection)
        .optional()
}

pub fn insert(connection: &MysqlConnection, user: dto::NewUser) -> Result<u32, DieselError> {
    use crate::dao::schema::users::dsl::*;

    connection.transaction::<_, DieselError, _>(|| {
        diesel::insert_into(users).values(&user).execute(connection)?;

        users.filter(name.eq(&user.name)).select(id).get_result(connection)
    })
}

pub fn update(connection: &MysqlConnection, user: dto::User) -> Result<bool, DieselError> {
    use crate::dao::schema::users::dsl::*;

    diesel::update(&user)
        .set(name.eq(&user.name))
        .execute(connection)
        .map(|rows| rows > 0)
}

pub fn list(connection: &MysqlConnection) -> Result<Vec<dto::User>, DieselError> {
    use crate::dao::schema::users::dsl::*;

    users.order(id).load::<dto::User>(connection)
}
