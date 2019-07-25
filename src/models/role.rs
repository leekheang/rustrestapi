use crate::schema::roles;
use diesel::PgConnection;

#[derive(Serialize, Deserialize)]
pub struct  RoleList(pub Vec<Role>);

#[derive(Queryable, Serialize, Deserialize)]
pub struct Role {
    pub id: i32,
    pub title: String
}


impl Role {

    pub fn find(id: &i32, connection: &PgConnection) -> Result<Role, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        roles::table.find(id).first(connection)
    }

    pub fn update( id: &i32, new_role: &NewRole, connection: &PgConnection) -> 
        Result<(), diesel::result::Error> {
            use diesel::QueryDsl;
            use diesel::RunQueryDsl;
            use crate::schema::roles::dsl;

            diesel::update(dsl::roles.find(id))
                .set(new_role)
                .execute(connection)?;
                // .get_result(connection)
             
            Ok(())
        }
}    

impl RoleList {
    pub fn list(connection: &PgConnection) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::roles::dsl::*;

        let role_result = 
             roles
             .limit(10)
             .load::<Role>(connection)
             .expect("Error loading roles");

         RoleList(role_result)
    }

}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="roles"]
pub struct NewRole {
    pub title: String
}


impl NewRole {
    pub fn create(&self, connection: &PgConnection) -> Result<Role, diesel::result::Error> {
        use diesel::RunQueryDsl;
        
        diesel::insert_into(roles::table)
             .values(self) 
             .get_result(connection)      

    }

}
