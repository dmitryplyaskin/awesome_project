use juniper::FieldResult;
use juniper::RootNode;

#[derive(GraphQLObject)]
#[graphql(description = "Wow, it's work!")]
struct User {
  age: String,
  name: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "New User")]
struct NewUser {
  age: String,
  name: String,
}

pub struct QueryRoot;

graphql_object!(QueryRoot: () |&self| {
    field user(&executor) -> FieldResult<User> {
        Ok(User{
            age: "22".to_owned(),
            name: "Dmitry".to_owned(),
        })
    },
    field user_old(&executor) -> FieldResult<User> {
        Ok(User{
            age: "63".to_owned(),
            name: "Alex".to_owned(),
        })
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: () |&self| {
    field newUser(&executor, new_user: NewUser) -> FieldResult<User> {
        Ok(User{
            age: format!("age-{}", new_user.age),
            name: new_user.name,
        })
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {})
}
