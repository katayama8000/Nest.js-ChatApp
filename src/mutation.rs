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

    // mutation{
    //   postJson(title:"aaa",userId:2,body:"eee"){
    //     userId
    //     title
    //     body
    //   }
    // }
    async fn post_json(
        &self,
        title: String,
        body: String,
        user_id: i32,
    ) -> async_graphql::Result<JsonPlaceholderMutationResult> {
        // POSTデータ
        let post_data = serde_json::json!({
            "userId": user_id,
            "title": title,
            "body": body,
        });

        // POSTリクエスト
        let client = reqwest::Client::new();
        let response = client
            .post("https://jsonplaceholder.typicode.com/posts")
            .json(&post_data)
            .send()
            .await?;

        let body = response.text().await?;
        println!("{}", body);
        let json: JsonPlaceholderMutationResult = serde_json::from_str(&body)?;
        Ok(json)
    }
}

#[derive(async_graphql::SimpleObject, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonPlaceholderMutationResult {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub user_id: i32,
}

#[derive(async_graphql::InputObject)]
struct MyInput {
    foo: String,
    bar: i32,
}
