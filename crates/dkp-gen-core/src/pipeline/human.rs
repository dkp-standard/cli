use crate::error::GenResult;
use crate::pipeline::context::PipelineContext;
use crate::prompt::templates;

pub async fn run(ctx: &PipelineContext) -> GenResult<()> {
    // Human layer assets are independent — generate in parallel
    let (r1, r2, r3, r4) = tokio::join!(
        generate_handbook(ctx),
        generate_quickstart(ctx),
        generate_faq(ctx),
        generate_examples(ctx),
    );
    r1?;
    r2?;
    r3?;
    r4?;
    Ok(())
}

async fn generate_handbook(ctx: &PipelineContext) -> GenResult<()> {
    let path = ctx.human_path().join("handbook.md");
    if !ctx.should_generate(&path) {
        return Ok(());
    }
    let (sys, user) = templates::prompt_handbook(&ctx.domain, &ctx.pack_name);
    let text = ctx.generate("handbook", &sys, &user).await?;
    ctx.write_text(&path, text.trim())
}

async fn generate_quickstart(ctx: &PipelineContext) -> GenResult<()> {
    let path = ctx.human_path().join("quickstart.md");
    if !ctx.should_generate(&path) {
        return Ok(());
    }
    let (sys, user) = templates::prompt_quickstart(&ctx.domain, &ctx.pack_name);
    let text = ctx.generate("quickstart", &sys, &user).await?;
    ctx.write_text(&path, text.trim())
}

async fn generate_faq(ctx: &PipelineContext) -> GenResult<()> {
    let path = ctx.human_path().join("faq.md");
    if !ctx.should_generate(&path) {
        return Ok(());
    }
    let (sys, user) = templates::prompt_faq(&ctx.domain, &ctx.pack_name);
    let text = ctx.generate("faq", &sys, &user).await?;
    ctx.write_text(&path, text.trim())
}

async fn generate_examples(ctx: &PipelineContext) -> GenResult<()> {
    let path = ctx.human_path().join("examples.md");
    if !ctx.should_generate(&path) {
        return Ok(());
    }
    let (sys, user) = templates::prompt_examples(&ctx.domain, &ctx.pack_name);
    let text = ctx.generate("examples", &sys, &user).await?;
    ctx.write_text(&path, text.trim())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GenConfig;
    use crate::llm::mock::MockClient;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tempfile::TempDir;

    fn test_ctx(tmp: &TempDir, client: MockClient) -> PipelineContext {
        PipelineContext {
            pack_dir: tmp.path().to_path_buf(),
            domain: "testing".to_string(),
            pack_name: "test-pack".to_string(),
            config: GenConfig::default(),
            client: Arc::new(client),
            progress: None,
            verbose: false,
        }
    }

    #[tokio::test]
    async fn generates_all_four_human_assets() {
        let tmp = TempDir::new().unwrap();
        let client = MockClient::with_fallback("Generated content.");
        let ctx = test_ctx(&tmp, client);

        run(&ctx).await.unwrap();

        assert!(ctx.human_path().join("handbook.md").exists());
        assert!(ctx.human_path().join("quickstart.md").exists());
        assert!(ctx.human_path().join("faq.md").exists());
        assert!(ctx.human_path().join("examples.md").exists());
        assert_eq!(
            std::fs::read_to_string(ctx.human_path().join("handbook.md")).unwrap(),
            "Generated content."
        );
    }

    #[tokio::test]
    async fn skips_generation_when_file_already_exists_and_not_overwriting() {
        let tmp = TempDir::new().unwrap();
        let client = MockClient::with_fallback("Should not be used.");
        let ctx = test_ctx(&tmp, client);

        std::fs::create_dir_all(ctx.human_path()).unwrap();
        std::fs::write(ctx.human_path().join("handbook.md"), "Existing content.").unwrap();

        run(&ctx).await.unwrap();

        assert_eq!(
            std::fs::read_to_string(ctx.human_path().join("handbook.md")).unwrap(),
            "Existing content."
        );
    }

    #[tokio::test]
    async fn llm_refusal_propagates_as_error() {
        let tmp = TempDir::new().unwrap();
        // Empty fallback + no fixtures means MockClient always returns LlmRefusal.
        let client = MockClient::new(HashMap::new(), "");
        let ctx = test_ctx(&tmp, client);

        let result = run(&ctx).await;
        assert!(result.is_err());
    }
}
