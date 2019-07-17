// -----------------------------------------------------------------------
// Database
// -----------------------------------------------------------------------

use portal::Dir;
use tantivy;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, ReloadPolicy};
use tempdir::TempDir;

pub struct DB {
    schema: Schema,
    index: Index,
    index_path: TempDir,
}

pub fn init() -> tantivy::Result<DB> {
    let index_path = TempDir::new("portal_tmp_dir")?;

    let mut schema_builder = Schema::builder();

    let timestamp_options = IntOptions::default()
        .set_stored()
        .set_fast(Cardinality::SingleValue);
    schema_builder.add_text_field("path", TEXT | STORED);
    // schema_builder.add_u64_field("count", FAST);
    schema_builder.add_u64_field("timestamp", timestamp_options);

    let schema = schema_builder.build();
    let index = Index::create_in_dir(&index_path, schema.clone())?;

    Ok(DB {
        schema,
        index,
        index_path,
    })
}

pub fn seed() -> tantivy::Result<Vec<Dir>> {
    println!("DB SEED");

    let entries = vec![
        Dir::new("/Users/dmix/dev/_rust/portal", 1557849352),
        Dir::new("/Users/dmix/dev/_elixir/issues", 1561657040),
        Dir::new("/Users/dmix/dev/_nim/karax/examples", 1549258325),
    ];

    Ok(entries)
}

pub fn add_entries(db: &DB, entries: Vec<Dir>) {
    println!("DB ADDING {} ENTRIES", entries.len());
    let mut index_writer = db.index.writer(50_000_000).unwrap();

    let path = db.schema.get_field("path").unwrap();
    let timestamp = db.schema.get_field("timestamp").unwrap();

    for e in entries {
        println!("DB ADDING {}", e.path);
        let mut entry = Document::default();
        entry.add_text(path, &e.path);
        entry.add_u64(timestamp, e.timestamp as u64);
        index_writer.add_document(entry);
    }

    match index_writer.commit() {
        Ok(_) => println!("Saved new entries to DB!"),
        Err(err) => println!("Error adding entries db! {:?}", err),
    }
}

pub fn query(db: &DB, query_term: &str) {
    println!("DB QUERY");

    let path = db.schema.get_field("path").unwrap();
    // let timestamp = db.schema.get_field("timestamp").unwrap();

    let reader = &db
        .index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()
        .unwrap();
    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(&db.index, vec![path]);
    let query = query_parser.parse_query(query_term).unwrap();

    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(50))
        .expect("could not parse query");

    println!("TopDocs results: {:?}", top_docs);
    for (_d, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address).unwrap();
        println!("Result: {}", db.schema.to_json(&retrieved_doc));
    }
}
