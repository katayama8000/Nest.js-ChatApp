#[derive(Default)]
pub struct Query;

#[async_graphql::Object]
impl Query {
    // query {
    //   howdy
    // }
    pub async fn howdy(&self) -> &'static str {
        "partner"
    }

    // query {
    //   hello(name: "foo")
    // }
    pub async fn hello(&self, name: String) -> String {
        format!("Hello, {}!", name)
    }

    // 新しい複雑なクエリの例を追加
    // query {
    //   complexQuery {
    //     field1
    //     field2
    //     subField {
    //       subField1
    //       subField2
    //     }
    //   }
    // }
    pub async fn complex_query(&self) -> ComplexQueryResult {
        ComplexQueryResult {
            field1: "Value1".to_string(),
            field2: 42,
            sub_field: ComplexSubField {
                sub_field1: true,
                sub_field2: "SubValue2".to_string(),
            },
        }
    }

    // pub async fn json_placeholder(&self) -> async_graphql::Result<serde_json::Value> {
    //     let client = reqwest::Client::new();
    //     let res = client
    //         .get("https://jsonplaceholder.typicode.com/todos/1")
    //         .send()
    //         .await?;
    //     let body = res.text().await?;
    //     let json: serde_json::Value = serde_json::from_str(&body)?;
    //     Ok(json)
    // }
}

#[derive(async_graphql::SimpleObject)]
pub struct ComplexQueryResult {
    pub field1: String,
    pub field2: i32,
    pub sub_field: ComplexSubField,
}

#[derive(async_graphql::SimpleObject)]
pub struct ComplexSubField {
    pub sub_field1: bool,
    pub sub_field2: String,
}
