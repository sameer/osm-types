use chrono::NaiveDateTime;
use fnv::FnvHashMap as HashMap;
use kstring::KString;
use rust_decimal::Decimal;

/// Fundamental representation of geographic features in OpenStreetMap
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Element {
    Node(Node),
    Way(Way),
    Relation(Relation),
}

/// [Element] identifier
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Id(pub i64);

/// Single point in space
///
/// <https://wiki.openstreetmap.org/wiki/Node>
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub id: Id,
    pub attributes: HashMap<KString, KString>,
    pub info: Option<Info>,
    pub lat: Decimal,
    pub lon: Decimal,
}

/// Ordered list of [Node]s
///
/// <https://wiki.openstreetmap.org/wiki/Way>
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Way {
    pub id: Id,
    pub attributes: HashMap<KString, KString>,
    pub info: Option<Info>,
    pub refs: Vec<Id>,
}

/// Ordered list of [Element]s
///
/// <https://wiki.openstreetmap.org/wiki/Relation>
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Relation {
    pub id: Id,
    pub attributes: HashMap<KString, KString>,
    pub info: Option<Info>,
    pub members: Vec<Member>,
}

/// [Element] in a [Relation]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Member {
    pub id: Id,
    pub ty: MemberType,
    /// <https://wiki.openstreetmap.org/wiki/Relation#Roles>
    pub role: Option<KString>,
}

/// Type of [Element] represented by [Member]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum MemberType {
    Node,
    Way,
    Relation,
}

/// Non-geographical information about a [Element]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Info {
    /// Version number of this revision of the [Element]
    pub version: i32,
    /// Time of last edit
    pub timestamp: Option<NaiveDateTime>,
    /// Group of edits that this version belongs to
    ///
    /// <https://wiki.openstreetmap.org/wiki/Changeset>
    pub changeset: Option<i64>,
    /// User ID
    pub uid: Option<i32>,
    /// Username
    pub user: Option<KString>,
    /// Whether a [Element] is visible or not
    ///
    /// Assume this to be true if it is [None]. If [Some(false)], the [Element] was deleted.
    pub visible: Option<bool>,
}
