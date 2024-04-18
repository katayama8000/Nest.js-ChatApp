use crate::api::handler::graphql::types::mutation_root::MutationRoot;

pub async fn resolve(
    _: &MutationRoot,
    context: &Context<'_>,
    input: CreateFileInput,
) -> graphql::Result<CreateFileOutput> {
    let data = context.data_unchecked::<GraphQLData>();
    let input = command::create_file::Input::from(input);
    let output = data.command_handler.create_file(input).await?;
    let output = CreateFileOutput::from(output);
    Ok(output)
}
