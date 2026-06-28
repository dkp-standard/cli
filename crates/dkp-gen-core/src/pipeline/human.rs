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
