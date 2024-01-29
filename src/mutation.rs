pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    // mutation{
    //   add(a:1,b:2)
    // }
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    // mutation{
    //   addWithInput(input:{foo:"foo",bar:42})
    // }
    async fn add_with_input(&self, input: MyInput) -> i32 {
        input.foo.len() as i32 + input.bar
    }
}

#[derive(async_graphql::InputObject)]
struct MyInput {
    foo: String,
    bar: i32,
}
