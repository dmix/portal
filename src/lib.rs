// mod db;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::str::FromStr;

static HOME: &'static str = "/Users/dmix/";
static HISTORY: &'static str = "/Users/dmix/.z";
// static HISTORY: &'static str = "/Users/dmix/.zsh_history-utf8";

#[derive(Debug)]
pub struct Dir {
    pub path: String,
    rank: f32,
    timestamp: u32,
}

impl Dir {
    fn new(zpath: String) -> Dir {
        let p: Vec<_> = zpath.split('|').collect();
        let rank: f32 = FromStr::from_str(&p[1]).unwrap();
        let timestamp: u32 = FromStr::from_str(&p[2]).unwrap();

        Dir {
            path: String::from(p[0]),
            rank: rank,
            timestamp: timestamp,
        }
    }
}

pub struct Config {
    pub query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Please provide a query argument to search directories");
        }

        let query = args[1].clone();
        let filename = String::from(HISTORY);

        Ok(Config { query, filename })
    }
}

pub fn run(config: &Config) -> Result<(String), Box<dyn Error>> {
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    let contents = fs::read_to_string(&config.filename)?;
    // println!("Contents {}", contents);

    Ok(contents)
}

// iconv -f UTF-8 -t UTF-8//IGNORE .bash_history > .bash_history-utf8
// iconv -f UTF-8 -t UTF-8//IGNORE .zsh_history > .zsh_history-utf8

fn home(path: &str) -> String {
    path.replace("~/", HOME)
}

fn valid_file(path: &str) -> bool {
    let blacklist = ["/", ".", "./.", "..", "../", "../..", "../../.."];
    if blacklist.contains(&path) {
        return false;
    }

    let clean_path = home(&path);
    if Path::new(&clean_path).exists() {
        // TODO: ignore paths in current directory
        return true;
    }

    return false;
}

pub fn search<'a>(query: &str, contents: &'a String) -> Vec<Dir> {
    // let x = db::init;

    let mut results = Vec::new();

    for line in contents.lines() {
        let dir = Dir::new(String::from(line));

        if dir.path.contains(query) {
            if valid_file(&dir.path) {
                results.push(dir);
            }
        }
    }

    // results.sort_by_key(|x| x.rank as u32);
    results.sort_by_key(|x| x.timestamp);
    results
}

// // -----------------------------------------------------------------------
// // Tantivy
// // -----------------------------------------------------------------------
//
// #[macro_use]
// extern crate tantivy;
// use tantivy::collector::TopDocs;
// use tantivy::query::QueryParser;
// use tantivy::schema::*;
// use tantivy::Index;
// use tantivy::ReloadPolicy;
// use tempdir::TempDir;
//
// pub fn init<'a>() -> tantivy::Result<&'a Schema> {
//     // Let's create a temporary directory for the
//     // sake of this example
//     let index_path = TempDir::new("portal")?;
//
//     // # Defining the schema
//     //
//     // The Tantivy index requires a very strict schema.
//     // The schema declares which fields are in the index,
//     // and for each field, its type and "the way it should
//     // be indexed".
//
//     // first we need to define a schema ...
//     let mut schema_builder = Schema::builder();
//
//     // Our first field is directory.
//     // We want full-text search for it, and we also want
//     // to be able to retrieve the document after the search.
//     //
//     // `TEXT | STORED` is some syntactic sugar to describe
//     // that.
//     //
//     // `TEXT` means the field should be tokenized and indexed,
//     // along with its term frequency and term positions.
//     //
//     // `STORED` means that the field will also be saved
//     // in a compressed, row-oriented key-value store.
//     // This store is useful to reconstruct the
//     // documents that were selected during the search phase.
//     let count_options = IntOptions::default()
//         .set_stored()
//         .set_fast(Cardinality::SingleValue);
//     let timestamp_options = IntOptions::default()
//         .set_stored()
//         .set_fast(Cardinality::SingleValue);
//     schema_builder.add_text_field("directory", STRING | STORED);
//     schema_builder.add_u64_field("count", FAST);
//     schema_builder.add_u64_field("timestamp", FAST | INDEXED);
//
//     let schema = schema_builder.build();
//
//     Ok(&schema)
// }
//
// pub fn seed(schema: &Schema, index_path: &TempDir) {
//     // # Indexing documents
//     //
//     // Let's create a brand new index.
//     //
//     // This will actually just save a meta.json
//     // with our schema in the directory.
//     let index = Index::create_in_dir(&index_path, schema.clone())?;
//
//     // To insert document we need an index writer.
//     // There must be only one writer at a time.
//     // This single `IndexWriter` is already
//     // multithreaded.
//     //
//     // Here we give tantivy a budget of `50MB`.
//     // Using a bigger heap for the indexer may increase
//     // throughput, but 50 MB is already plenty.
//     let mut index_writer = index.writer(50_000_000)?;
//
//     // Let's index our documents!
//     // We first need a handle on the directory and the timestamp field.
//
//     // ### Adding documents
//     //
//     // We can create a document manually, by setting the fields
//     // one by one in a Document object.
//     let directory = schema.get_field("directory").unwrap();
//     let timestamp = schema.get_field("timestamp").unwrap();
//
//     let mut old_man_doc = Document::default();
//     old_man_doc.add_text(directory, "The Old Man and the Sea");
//     old_man_doc.add_text(
//         timestamp,
//         "He was an old man who fished alone in a skiff in the Gulf Stream and \
//          he had gone eighty-four days now without taking a fish.",
//     );
//
//     // ... and add it to the `IndexWriter`.
//     index_writer.add_document(old_man_doc);
//
//     // For convenience, tantivy also comes with a macro to
//     // reduce the boilerplate above.
//     index_writer.add_document(doc!(
//     directory => "Of Mice and Men",
//     timestamp => "A few miles south of Soledad, the Salinas River drops in close to the hillside \
//             bank and runs deep and green. The water is warm too, for it has slipped twinkling \
//             over the yellow sands in the sunlight before reaching the narrow pool. On one \
//             side of the river the golden foothill slopes curve up to the strong and rocky \
//             Gabilan Mountains, but on the valley side the water is lined with trees—willows \
//             fresh and green with every spring, carrying in their lower leaf junctures the \
//             debris of the winter’s flooding; and sycamores with mottled, white, recumbent \
//             limbs and branches that arch over the pool"
//     ));
//
//     index_writer.add_document(doc!(
//     directory => "Of Mice and Men",
//     timestamp => "A few miles south of Soledad, the Salinas River drops in close to the hillside \
//             bank and runs deep and green. The water is warm too, for it has slipped twinkling \
//             over the yellow sands in the sunlight before reaching the narrow pool. On one \
//             side of the river the golden foothill slopes curve up to the strong and rocky \
//             Gabilan Mountains, but on the valley side the water is lined with trees—willows \
//             fresh and green with every spring, carrying in their lower leaf junctures the \
//             debris of the winter’s flooding; and sycamores with mottled, white, recumbent \
//             limbs and branches that arch over the pool"
//     ));
//
//     // Multivalued field just need to be repeated.
//     index_writer.add_document(doc!(
//     directory => "Frankenstein",
//     directory => "The Modern Prometheus",
//     timestamp => "You will rejoice to hear that no disaster has accompanied the commencement of an \
//              enterprise which you have regarded with such evil forebodings.  I arrived here \
//              yesterday, and my first task is to assure my dear sister of my welfare and \
//              increasing confidence in the success of my undertaking."
//     ));
//
//     // This is an example, so we will only index 3 documents
//     // here. You can check out tantivy's tutorial to index
//     // the English wikipedia. Tantivy's indexing is rather fast.
//     // Indexing 5 million articles of the English wikipedia takes
//     // around 3 minutes on my computer!
//
//     // ### Committing
//     //
//     // At this point our documents are not searchable.
//     //
//     //
//     // We need to call .commit() explicitly to force the
//     // index_writer to finish processing the documents in the queue,
//     // flush the current index to the disk, and advertise
//     // the existence of new documents.
//     //
//     // This call is blocking.
//     index_writer.commit()?;
//
//     // If `.commit()` returns correctly, then all of the
//     // documents that have been added are guaranteed to be
//     // persistently indexed.
//     //
//     // In the scenario of a crash or a power failure,
//     // tantivy behaves as if has rolled back to its last
//     // commit.
// }
//
// pub fn read(schema: &Schema, index: &Index) {
//     // # Searching
//     //
//     // ### Searcher
//     //
//     // A reader is required to get search the index.
//     // It acts as a `Searcher` pool that reloads itself,
//     // depending on a `ReloadPolicy`.
//     //
//     // For a search server you will typically create one reader for the entire lifetime of your
//     // program, and acquire a new searcher for every single request.
//     //
//     // In the code below, we rely on the 'ON_COMMIT' policy: the reader
//     // will reload the index automatically after each commit.
//     let reader = index
//         .reader_builder()
//         .reload_policy(ReloadPolicy::OnCommit)
//         .try_into()?;
//
//     // We now need to acquire a searcher.
//     //
//     // A searcher points to snapshotted, immutable version of the index.
//     //
//     // Some search experience might require more than
//     // one query. Using the same searcher ensures that all of these queries will run on the
//     // same version of the index.
//     //
//     // Acquiring a `searcher` is very cheap.
//     //
//     // You should acquire a searcher every time you start processing a request and
//     // and release it right after your query is finished.
//     let searcher = reader.searcher();
//
//     // ### Query
//
//     let directory = schema.get_field("directory").unwrap();
//     let timestamp = schema.get_field("timestamp").unwrap();
//
//     // The query parser can interpret human queries.
//     // Here, if the user does not specify which
//     // field they want to search, tantivy will search
//     // in both directory and timestamp.
//     let query_parser = QueryParser::for_index(&index, vec![directory, timestamp]);
//
//     // QueryParser may fail if the query is not in the right
//     // format. For user facing applications, this can be a problem.
//     // A ticket has been opened regarding this problem.
//     let query = query_parser.parse_query("sea whale")?;
//
//     // A query defines a set of documents, as
//     // well as the way they should be scored.
//     //
//     // A query created by the query parser is scored according
//     // to a metric called Tf-Idf, and will consider
//     // any document matching at least one of our terms.
//
//     // ### Collectors
//     //
//     // We are not interested in all of the documents but
//     // only in the top 10. Keeping track of our top 10 best documents
//     // is the role of the TopDocs.
//
//     // We can now perform our query.
//     let top_docs = searcher.search(&query, &TopDocs::with_limit(10));
//
//     // The actual documents still need to be
//     // retrieved from Tantivy's store.
//     //
//     // Since the timestamp field was not configured as stored,
//     // the document returned will only contain
//     // a directory.
//     for (_score, doc_address) in top_docs {
//         let retrieved_doc = searcher.doc(doc_address)?;
//         println!("{}", schema.to_json(&retrieved_doc));
//     }
// }
