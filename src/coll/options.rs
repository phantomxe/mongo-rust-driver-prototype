//! Options for collection-level operations.
use bson::{self, Bson, bson, doc};
use common::{ReadPreference, WriteConcern};
use Error::ArgumentError;
use Result;

/// Describes the type of cursor to return on collection queries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CursorType {
    NonTailable,
    Tailable,
    TailableAwait,
}

impl Default for CursorType {
    fn default() -> Self {
        CursorType::NonTailable
    }
}

/// Describes the type of document to return on write operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReturnDocument {
    Before,
    After,
}

impl ReturnDocument {
    pub fn as_bool(&self) -> bool {
        match *self {
            ReturnDocument::Before => false,
            ReturnDocument::After => true,
        }
    }
}

/// Marker interface for writes that can be batched together.
#[derive(Debug, Clone, PartialEq)]
pub enum WriteModel {
    InsertOne { document: bson::Document },
    DeleteOne { filter: bson::Document },
    DeleteMany { filter: bson::Document },
    ReplaceOne {
        filter: bson::Document,
        replacement: bson::Document,
        upsert: Option<bool>,
    },
    UpdateOne {
        filter: bson::Document,
        update: bson::Document,
        upsert: Option<bool>,
    },
    UpdateMany {
        filter: bson::Document,
        update: bson::Document,
        upsert: Option<bool>,
    },
}

/// Options for aggregation queries.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AggregateOptions {
    pub allow_disk_use: Option<bool>,
    pub use_cursor: Option<bool>,
    pub batch_size: i32,
    pub max_time_ms: Option<i64>,
    pub read_preference: Option<ReadPreference>,
}

impl AggregateOptions {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<AggregateOptions> for bson::Document {
    fn from(options: AggregateOptions) -> Self {
        let mut document = bson::Document::new();

        if let Some(allow_disk_use) = options.allow_disk_use {
            document.insert("allowDiskUse", allow_disk_use);
        }

        // useCursor not currently used by the driver.


        let cursor = doc! { "batchSize": options.batch_size };
        document.insert("cursor", cursor);

        // maxTimeMS is not currently used by the driver.

        // read_preference is used directly by Collection::aggregate.

        document
    }
}

/// Options for count queries.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CountOptions {
    pub skip: Option<i64>,
    pub limit: Option<i64>,
    pub hint: Option<String>,
    pub hint_doc: Option<bson::Document>,
    pub max_time_ms: Option<i64>,
    pub read_preference: Option<ReadPreference>,
}

impl CountOptions {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<CountOptions> for bson::Document {
    fn from(options: CountOptions) -> Self {
        let mut document = bson::Document::new();

        if let Some(skip) = options.skip {
            document.insert("skip", skip);
        }

        if let Some(limit) = options.limit {
            document.insert("limit", limit);
        }

        if let Some(hint) = options.hint {
            document.insert("hint", hint);
        }

        if let Some(hint_doc) = options.hint_doc {
            document.insert("hint_doc", hint_doc);
        }

        // maxTimeMS is not currently used by the driver.

        // read_preference is used directly by Collection::count.

        document
    }
}

/// Options for distinct queries.
#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DistinctOptions {
    pub max_time_ms: Option<i64>,
    pub read_preference: Option<ReadPreference>,
}

impl DistinctOptions {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Options for collection queries.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FindOptions {
    pub allow_partial_results: bool,
    pub no_cursor_timeout: bool,
    pub oplog_replay: bool,
    pub skip: Option<i64>,
    pub limit: Option<i64>,
    pub cursor_type: CursorType,
    pub batch_size: Option<i32>,
    pub comment: Option<String>,
    pub max_time_ms: Option<i64>,
    pub modifiers: Option<bson::Document>,
    pub projection: Option<bson::Document>,
    pub sort: Option<bson::Document>,
    pub read_preference: Option<ReadPreference>,
}

impl FindOptions {
    /// Creates a new FindOptions struct with default parameters.
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<FindOptions> for bson::Document {
    fn from(options: FindOptions) -> Self {
        let mut document = bson::Document::new();

        // `allow_partial_results`, `no_cursor_timeout`, `oplog_relay`, and `cursor_type` are used by
        // wire_protocol::OpQueryFlags.
        //
        // `max_time_ms` and `modifiers` are not currently used by the driver.
        //
        // read_preference is used directly by Collection::find_with_command_type.

        if let Some(projection) = options.projection {
            document.insert("projection", projection);
        }

        if let Some(skip) = options.skip {
            document.insert("skip", skip);
        }

        if let Some(limit) = options.limit {
            document.insert("limit", limit);
        }

        if let Some(batch_size) = options.batch_size {
            document.insert("batchSize", batch_size);
        }

        if let Some(sort) = options.sort {
            document.insert("sort", sort);
        }

        document
    }
}

/// Options for `findOneAndDelete` operations.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FindOneAndDeleteOptions {
    pub max_time_ms: Option<i64>,
    pub projection: Option<bson::Document>,
    pub sort: Option<bson::Document>,
    pub write_concern: Option<WriteConcern>,
}

impl FindOneAndDeleteOptions {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<FindOneAndDeleteOptions> for bson::Document {
    fn from(options: FindOneAndDeleteOptions) -> Self {
        let mut document = bson::Document::new();

        // max_time_ms is not currently used by the driver

        if let Some(projection) = options.projection {
            document.insert("fields", projection);
        }

        if let Some(sort) = options.sort {
            document.insert("sort", sort);
        }

        if let Some(write_concern) = options.write_concern {
            document.insert("writeConcern", write_concern.to_bson());
        }

        document
    }
}

/// Options for `findOneAndUpdate` operations.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FindOneAndUpdateOptions {
    pub return_document: Option<ReturnDocument>,
    pub max_time_ms: Option<i64>,
    pub projection: Option<bson::Document>,
    pub sort: Option<bson::Document>,
    pub upsert: Option<bool>,
    pub write_concern: Option<WriteConcern>,
    pub array_filters: Option<bson::Document>,
}

impl FindOneAndUpdateOptions {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<FindOneAndUpdateOptions> for bson::Document {
    fn from(options: FindOneAndUpdateOptions) -> Self {
        let mut document = bson::Document::new();

        if let Some(return_document) = options.return_document {
            document.insert("new", return_document.as_bool());
        }

        // max_time_ms is not currently used by the driver

        if let Some(projection) = options.projection {
            document.insert("fields", projection);
        }

        if let Some(sort) = options.sort {
            document.insert("sort", sort);
        }

        if let Some(upsert) = options.upsert {
            document.insert("upsert", upsert);
        }

        if let Some(write_concern) = options.write_concern {
            document.insert("writeConcern", write_concern.to_bson());
        }

        if let Some(array_filters) = options.array_filters {
            document.insert("arrayFilters",  array_filters);
        }

        document
    }
}

/// Options for index operations.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IndexOptions {
    #[serde(skip_serializing_if="Option::is_none")]
    pub background: Option<bool>,

    #[serde(rename="expireAfterSeconds", skip_serializing_if="Option::is_none")]
    pub expire_after_seconds: Option<i32>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub sparse: Option<bool>,

    #[serde(rename="storageEngine", skip_serializing_if="Option::is_none")]
    pub storage_engine: Option<bson::Document>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub unique: Option<bool>,

    #[serde(rename="v", skip_serializing_if="Option::is_none")]
    pub version: Option<i32>,

    // Options for text indexes
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_language: Option<String>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub language_override: Option<String>,

    #[serde(rename="textIndexVersion", skip_serializing_if="Option::is_none")]
    pub text_version: Option<i32>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub weights: Option<bson::Document>,

    // Options for 2dsphere indexes
    #[serde(rename="2dsphereIndexVersion", skip_serializing_if="Option::is_none")]
    pub sphere_version: Option<i32>,

    // Options for 2d indexes
    #[serde(skip_serializing_if="Option::is_none")]
    pub bits: Option<i32>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub max: Option<f64>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub min: Option<f64>,

    // Options for geoHaystack indexes
    #[serde(rename="bucketSize", skip_serializing_if="Option::is_none")]
    pub bucket_size: Option<i32>,
}

impl IndexOptions {
    pub fn new() -> Self {
        Default::default()
    }
}

/// A single index model.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IndexModel {
    #[serde(rename="key")]
    pub keys: bson::Document,

    #[serde(flatten)]
    pub options: IndexOptions,
}

impl IndexModel {
    pub fn new(keys: bson::Document, options: Option<IndexOptions>) -> IndexModel {
        IndexModel {
            keys: keys,
            options: options.unwrap_or_else(IndexOptions::new),
        }
    }

    /// Returns the name of the index as specified by the options, or
    /// as automatically generated using the keys.
    pub fn name(&self) -> Result<String> {
        Ok(match self.options.name {
            Some(ref name) => name.to_owned(),
            None => self.generate_index_name()?,
        })
    }

    /// Generates the index name from keys.
    /// Auto-generated names have the form "key1_val1_key2_val2..."
    pub fn generate_index_name(&self) -> Result<String> {
        let mut name = String::new();
        for (key, bson) in self.keys.iter() {
            if !name.is_empty() {
                name.push_str("_");
            }

            name.push_str(key);
            name.push('_');
            match *bson {
                Bson::I32(ref i) => name.push_str(&format!("{}", i)),
                Bson::String(ref s)
                    if s == "text" || s == "hashed" || s == "2d" || s == "2dsphere" || s == "geoHaystack" => {
                    name.push_str(s)
                }
                _ => {
                    return Err(ArgumentError(String::from(
                        r#"Index model keys must map to i32, "text", "hashed", "2d", "2dsphere" or "geoHaystack"."#,
                    )))
                }
            }
        }
        Ok(name)
    }

    /// Converts the model to its BSON document representation.
    pub fn to_bson(&self) -> Result<bson::Document> {
        let mut doc = doc!{ "key": self.keys.clone() };

        if let Some(val) = self.options.background {
            doc.insert("background", val);
        }
        if let Some(val) = self.options.expire_after_seconds {
            doc.insert("expireAfterSeconds", val);
        }
        if let Some(ref val) = self.options.name {
            doc.insert("name", val);
        } else {
            doc.insert("name", self.generate_index_name()?);
        }
        if let Some(val) = self.options.sparse {
            doc.insert("sparse", val);
        }
        if let Some(ref val) = self.options.storage_engine {
            doc.insert("storageEngine", bson::Bson::Document(val.clone()));
        }
        if let Some(val) = self.options.unique {
            doc.insert("unique", val);
        }
        if let Some(val) = self.options.version {
            doc.insert("v", val);
        }
        if let Some(ref val) = self.options.default_language {
            doc.insert("default_language", val);
        }
        if let Some(ref val) = self.options.language_override {
            doc.insert("language_override", val);
        }
        if let Some(val) = self.options.text_version {
            doc.insert("textIndexVersion", val);
        }
        if let Some(ref val) = self.options.weights {
            doc.insert("weights", val.clone());
        }
        if let Some(val) = self.options.sphere_version {
            doc.insert("2dsphereIndexVersion", val);
        }
        if let Some(val) = self.options.bits {
            doc.insert("bits", val);
        }
        if let Some(val) = self.options.max {
            doc.insert("max", val);
        }
        if let Some(val) = self.options.min {
            doc.insert("min", val);
        }
        if let Some(val) = self.options.bucket_size {
            doc.insert("bucketSize", val);
        }

        Ok(doc)
    }
}

/// Options for insertMany operations.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct InsertManyOptions {
    pub ordered: Option<bool>,
    pub write_concern: Option<WriteConcern>,
}

impl InsertManyOptions {
    pub fn new() -> Self {
        Default::default()
    }
}

impl From<InsertManyOptions> for bson::Document {
    fn from(options: InsertManyOptions) -> Self {
        let mut document = bson::Document::new();

        if let Some(ordered) = options.ordered {
            document.insert("ordered", ordered);
        }

        if let Some(write_concern) = options.write_concern {
            document.insert("writeConcern", write_concern.to_bson());
        }

        document
    }
}

/// Options for update operations.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct UpdateOptions {
    pub upsert: Option<bool>,
    pub write_concern: Option<WriteConcern>,
}

impl UpdateOptions {
    pub fn new() -> UpdateOptions {
        Default::default()
    }
}

pub type ReplaceOptions = UpdateOptions;

#[cfg(test)]
mod test {
    use super::*;

    fn build_populated_index_opts() -> IndexOptions {
        // Note, this setup is not actually valid, but tests (de)serialization effectively.
        let mut opts = IndexOptions::default();
        opts.background = Some(true);
        opts.expire_after_seconds = Some(10);
        opts.name = Some("test".to_string());
        opts.sparse = Some(true);
        opts.storage_engine = Some(doc!{"mmapv1": true}); // Not sure about the actual shape `:)`.
        opts.unique = Some(true);
        opts.version = Some(2);
        opts.default_language = Some("en_us".to_string());
        opts.language_override = Some("en_us".to_string());
        opts.text_version = Some(3);
        opts.weights = Some(doc!{"test_field": 10});
        opts.sphere_version = Some(3);
        opts.bits = Some(26);
        opts.max = Some(-180.0);
        opts.min = Some(180.0);
        opts.bucket_size = Some(10);
        opts
    }

    #[test]
    fn serde_and_manual_serialization_should_match_with_defaults() {
        let keys = doc!{"test_field": -1};
        let opts = IndexOptions::default();
        let model = IndexModel::new(keys, Some(opts));

        let manual_ser = model.to_bson().unwrap();
        let mut serde_ser = bson::to_bson(&model).unwrap();

        // NOTE: manual serialization is currently supplementing the index name instead of letting
        // the server do it. So let's assert the shape of the serde `name` & then update it from
        // the manual serialization for the overall assert.
        assert!(serde_ser.as_document().unwrap().get("name").is_none()); // Serde should skip "name" if not specified.
        serde_ser = if let bson::Bson::Document(mut doc) = serde_ser {
            doc.insert("name", manual_ser.get_str("name").unwrap());
            bson::Bson::Document(doc)
        } else {
            panic!("Expected a bson::Bson::Document(_).");
        };
        assert_eq!(bson::Bson::Document(manual_ser), serde_ser);
    }

    #[test]
    fn serde_and_manual_serialization_should_match_with_values() {
        let keys = doc!{"test_field": "text"};
        let opts = build_populated_index_opts();
        let model = IndexModel::new(keys, Some(opts));

        let manual_ser = model.to_bson().unwrap();
        let serde_ser = bson::to_bson(&model).unwrap();

        assert_eq!(bson::Bson::Document(manual_ser), serde_ser);
    }

    #[test]
    fn to_and_from_serde_should_be_idempotent() {
        let keys = doc!{"test_field": "text"};
        let opts = build_populated_index_opts();
        let model = IndexModel::new(keys, Some(opts.clone()));

        let serde_ser = bson::to_bson(&model).expect("Expected valid BSON serialization.");
        let de: IndexModel = bson::from_bson(serde_ser).expect("Expected valid BSON deserialization.");

        assert_eq!(doc!{"test_field": "text"}, de.keys);
        assert_eq!(opts, de.options);
    }
}
