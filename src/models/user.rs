use chrono::{NaiveDateTime , Local}; // This type is used for date field in Diesel.

use crate::schema::users;
use crate::errors::MyError;

use diesel::PgConnection;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable )]
#[table_name = "users"]
pub struct User {
    #[serde(skip)]
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    pub avatar: Option<String>,
    pub biography: Option<String>,
    pub created_at: NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize , Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub avatar: Option<String>,
    pub biography: Option<String>,
    pub created_at: NaiveDateTime
}

impl User {
    pub fn create(register_user: RegisterUser, connection: &PgConnection) ->
     Result<User, MyError> {
        use diesel::RunQueryDsl;

        Ok(diesel::insert_into(users::table)
            .values(NewUser {
                username: register_user.username,
                email: register_user.email,
                password: Self::hash_password(register_user.password)?,
                avatar: register_user.avatar,
                biography: register_user.biography,
                created_at: Local::now().naive_local()
                
            })
            .get_result(connection)?)
    }

    pub fn hash_password(plain: String) -> Result<String, MyError> {
        Ok(hash(plain, DEFAULT_COST)?)
    }
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
    pub avatar: Option<String>,
    pub biography: Option<String>,
}

impl RegisterUser {
    pub fn validates(self) ->
     Result<RegisterUser, MyError> {
         if self.password == self.password_confirmation {
             Ok(self)
         } else {
             Err(
                 MyError::PasswordNotMatch(
                     "Password and Password Confirmation does not match".to_string()
                 )
             )
         }
    }
}

#[derive(Deserialize)]
pub struct AuthUser {
    pub email: String,
    pub password: String
}

impl AuthUser {

     // The good thing about ? syntax and have a custom error is 
    // that the code would look very straightforward, I mean, 
    // the other way would imply a lot of pattern matching 
    // making it look ugly. 
    pub fn login(&self, connection: &PgConnection) ->
     Result<User, MyError> {
        use bcrypt::verify;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use diesel::ExpressionMethods;
        use crate::schema::users::dsl::email;

        let mut records =
            users::table
                .filter(email.eq(&self.email))
                .load::<User>(connection)?;

        let user =
            records
                .pop()
                .ok_or(MyError::DBError(diesel::result::Error::NotFound))?;

        let verify_password =
            verify(&self.password, &user.password)
                .map_err( |_error| {
                    MyError::WrongPassword(
                        "Wrong password, check again please".to_string()
                    )
                })?;

        if verify_password {
            Ok(user)
        } else {
            Err(MyError::WrongPassword(
                "Wrong password, check again please".to_string()
            ))
        }

    }
}