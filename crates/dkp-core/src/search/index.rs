use serde::{Deserialize, Serialize};
use tantivy::{
    collector::TopDocs,
    doc,
    query::QueryParser,
    schema::{Field, SchemaBuilder, Value, STORED, STRING, TEXT},
    Index, TantivyDocument,
};

use crate::{error::DkpResult, pack::loader::Pack, DkpError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub asset_type: String,
    pub title: String,
    pub excerpt: String,
    pub score: f32,
    pub source_file: String,
}

struct Fields {
    id: Field,
    asset_type: Field,
    title: Field,
    body: Field,
    source_file: Field,
}

pub struct SearchIndex {
    index: Index,
    fields: Fields,
}

impl SearchIndex {
    pub fn build(pack: &Pack) -> DkpResult<Self> {
        let mut builder = SchemaBuilder::new();
        let id = builder.add_text_field("id", STRING | STORED);
        let asset_type = builder.add_text_field("asset_type", STRING | STORED);
        let title = builder.add_text_field("title", TEXT | STORED);
        let body = builder.add_text_field("body", TEXT | STORED);
        let source_file = builder.add_text_field("source_file", STRING | STORED);
        let schema = builder.build();
        let fields = Fields {
            id,
            asset_type,
            title,
            body,
            source_file,
        };

        let index = Index::create_in_ram(schema);
        let mut writer = index
            .writer(15_000_000)
            .map_err(|e| DkpError::SearchIndex(e.to_string()))?;

        if let Some(gf) = pack.load_glossary()? {
            for t in &gf.terms {
                let body = format!(
                    "{} {} {}",
                    t.definition,
                    t.aliases.join(" "),
                    t.tags.join(" ")
                );
                writer
                    .add_document(doc!(
                        fields.id => t.id.as_str(),
                        fields.asset_type => "term",
                        fields.title => t.term.as_str(),
                        fields.body => body.as_str(),
                        fields.source_file => "machine/glossary.json",
                    ))
                    .map_err(|e| DkpError::SearchIndex(e.to_string()))?;
            }
        }

        for c in &pack.load_chunks()? {
            let body = format!("{} {}", c.chunk_text, c.tags.join(" "));
            writer
                .add_document(doc!(
                    fields.id => c.id.as_str(),
                    fields.asset_type => "chunk",
                    fields.title => c.title.as_str(),
                    fields.body => body.as_str(),
                    fields.source_file => "machine/retrieval_chunks.jsonl",
                ))
                .map_err(|e| DkpError::SearchIndex(e.to_string()))?;
        }

        if let Some(rf) = pack.load_rules()? {
            for r in &rf.rules {
                let body = format!("{} {}", r.description, r.tags.join(" "));
                writer
                    .add_document(doc!(
                        fields.id => r.id.as_str(),
                        fields.asset_type => "rule",
                        fields.title => r.title.as_str(),
                        fields.body => body.as_str(),
                        fields.source_file => "machine/rules.json",
                    ))
                    .map_err(|e| DkpError::SearchIndex(e.to_string()))?;
            }
        }

        if let Some(cf) = pack.load_constraints()? {
            for c in cf.all_constraints() {
                let body = format!("{} {}", c.description, c.tags.join(" "));
                writer
                    .add_document(doc!(
                        fields.id => c.id.as_str(),
                        fields.asset_type => "constraint",
                        fields.title => c.title.as_str(),
                        fields.body => body.as_str(),
                        fields.source_file => "machine/constraints.json",
                    ))
                    .map_err(|e| DkpError::SearchIndex(e.to_string()))?;
            }
        }

        writer
            .commit()
            .map_err(|e| DkpError::SearchIndex(e.to_string()))?;

        Ok(Self { index, fields })
    }

    pub fn search(&self, query: &str, limit: usize) -> DkpResult<Vec<SearchResult>> {
        let reader = self
            .index
            .reader()
            .map_err(|e| DkpError::SearchIndex(e.to_string()))?;
        let searcher = reader.searcher();

        let parser = QueryParser::for_index(&self.index, vec![self.fields.title, self.fields.body]);

        let parsed = parser
            .parse_query(query)
            .map_err(|e| DkpError::SearchIndex(e.to_string()))?;

        let top_docs: Vec<(f32, _)> = searcher
            .search(&parsed, &TopDocs::with_limit(limit).order_by_score())
            .map_err(|e| DkpError::SearchIndex(e.to_string()))?;

        let mut results = Vec::with_capacity(top_docs.len());
        for (score, addr) in top_docs {
            let doc: TantivyDocument = searcher
                .doc(addr)
                .map_err(|e| DkpError::SearchIndex(e.to_string()))?;

            let get = |f: Field| -> String {
                doc.get_first(f)
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string()
            };

            let body = get(self.fields.body);
            let excerpt = if body.len() > 120 {
                format!("{}…", &body[..120])
            } else {
                body
            };

            results.push(SearchResult {
                id: get(self.fields.id),
                asset_type: get(self.fields.asset_type),
                title: get(self.fields.title),
                excerpt,
                score,
                source_file: get(self.fields.source_file),
            });
        }

        Ok(results)
    }
}
