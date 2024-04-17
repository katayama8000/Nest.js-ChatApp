- Command (write) を 使用する方法
    - CommandHandler
        - CommandHandler という trait が存在する
        ```rust
        pub trait CommandHandler: HasCircleRepository {
        async fn create_circle(&self, input: CreateCircleInput) -> anyhow::Result<CreateCircleOutput> {
            // 省略
            self.circle_repository()
                .create(&circle)
                .map(|_| CreateCircleOutput {
                    circle_id: circle_id.into(),
                    owner_id: circle.owner.id.into(),
                })
            }
        }
        ```
        - これは、interface の HasCircleRepository を実装していることが前提とする
        - CommandHander を struct に実装すると、その struct は HasCircleRepository を実装している必要がある

    - server.rs
        - Appstate を 構築 (new) している
        ```rust
        pub fn new() -> anyhow::Result<AppState> {
            struct CommandHandlerStruct {
                circle_repository: Arc<dyn CircleRepositoryInterface + Send + Sync>,
            }
            impl HasCircleRepository for CommandHandlerStruct {
                fn circle_repository(&self) -> Arc<dyn CircleRepositoryInterface + Send + Sync> {
                    self.circle_repository.clone()
                }
            }
            impl CommandHandler for CommandHandlerStruct {}
            let command_handler = CommandHandlerStruct {
                circle_repository: Arc::new(CircleRepository::new()),
            };

            struct QueryHandlerStruct {}
            impl QueryHandler for QueryHandlerStruct {}
            let query_handler = QueryHandlerStruct {};
            Ok(AppState::new(
                Arc::new(command_handler),
                Arc::new(query_handler),
            ))
        }
        ```
        - CommandHandlerStruct フィールドに circle_repository を持つ
            - circle_repository はレポジトリに実装されている trait を持つ
        - HasCircleRepository を実装している
            - circle_repository を持っているよ〜
            - 中身は circle_repository を返すだけ
            - 実態ではなく、型情報だけを知っていれば良い
            - 実態は後から注入される

        - CommandHandler を実装している
            - HasCircleRepository を実装していることが前提
            - CommandHandler の中身は、デフォルト実装で、usecase の実行のようなことを行なっている

        - CommandHandlerStruct
            - repository の実体をインスタンス化して、状態を保持している
            - この構造体を Appstate で保持している



