use chrono::NaiveDateTime;
use fnv::FnvHashMap as HashMap;
use kstring::KString;
use rust_decimal::Decimal;

/// Fundamental representation of geographic features in OpenStreetMap
///
/// <https://wiki.openstreetmap.org/wiki/Elements>
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Element {
    Node(Node),
    Way(Way),
    Relation(Relation),
}

impl Element {
    pub fn id(&self) -> Id {
        match self {
            Element::Node(Node { id, .. })
            | Element::Way(Way { id, .. })
            | Element::Relation(Relation { id, .. }) => *id,
        }
    }

    pub fn tags(&self) -> &HashMap<KString, KString> {
        match self {
            Element::Node(Node { tags, .. })
            | Element::Way(Way { tags, .. })
            | Element::Relation(Relation { tags, .. }) => tags,
        }
    }

    pub fn info(&self) -> Option<&Info> {
        match self {
            Element::Node(Node { info, .. })
            | Element::Way(Way { info, .. })
            | Element::Relation(Relation { info, .. }) => info.as_ref(),
        }
    }
}

/// [Element] identifier
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Id(pub i64);

/// Single point in space
///
/// <https://wiki.openstreetmap.org/wiki/Node>
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node {
    pub id: Id,
    pub tags: HashMap<KString, KString>,
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
    pub tags: HashMap<KString, KString>,
    pub info: Option<Info>,
    pub refs: Vec<Id>,
}

/// Ordered list of [Element]s
///
/// <https://wiki.openstreetmap.org/wiki/Relation>
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Relation {
    pub id: Id,
    pub tags: HashMap<KString, KString>,
    pub info: Option<Info>,
    pub members: Vec<Member>,
}

/// [Element] in a [Relation]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Member {
    pub id: Id,
    pub ty: MemberType,
    /// Describes the function of this member in its relation
    ///
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
///
/// <https://wiki.openstreetmap.org/wiki/Elements#Common_attributes>
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Info {
    /// Version number of this revision of the [Element]
    ///
    /// Starts at 1 and incremented with each update
    pub version: i32,
    /// Time of last modification
    pub timestamp: Option<NaiveDateTime>,
    /// Group of edits that this version belongs to
    ///
    /// <https://wiki.openstreetmap.org/wiki/Changeset>
    pub changeset: Option<i64>,
    /// ID of user who performed the last modification
    pub uid: Option<i32>,
    /// Display name of the user
    ///
    /// This will change without a version increment if the user modifies their display name.
    pub user: Option<KString>,
    /// Whether a [Element] is visible or not
    ///
    /// Assume this to be true if it is [None]. If [Some(false)], the [Element] was deleted
    /// and was returned by a history call.
    pub visible: Option<bool>,
}
