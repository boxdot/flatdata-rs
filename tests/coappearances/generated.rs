// Do not edit: This code was generated by flatdata's generator.


pub mod schema {
    pub mod structs {
        pub const META: &str = r#"namespace coappearances {
struct Meta
{
    title_ref : u32 : 32;
    author_ref : u32 : 32;
}
}

"#;
        pub const CHARACTER: &str = r#"namespace coappearances {
struct Character
{
    name_ref : u32 : 32;
}
}

"#;
        pub const COAPPEARANCE: &str = r#"namespace coappearances {
struct Coappearance
{
    a_ref : u32 : 16;
    b_ref : u32 : 16;
    count : u32 : 16;
    first_chapter_ref : u32 : 16;
}
}

"#;
        pub const NICKNAME: &str = r#"namespace coappearances {
struct Nickname
{
    ref : u32 : 32;
}
}

"#;
        pub const DESCRIPTION: &str = r#"namespace coappearances {
struct Description
{
    ref : u32 : 32;
}
}

"#;
        pub const UNARY_RELATION: &str = r#"namespace coappearances {
struct UnaryRelation
{
    kind_ref : u32 : 32;
    to_ref : u32 : 16;
}
}

"#;
        pub const BINARY_RELATION: &str = r#"namespace coappearances {
struct BinaryRelation
{
    kind_ref : u32 : 32;
    to_a_ref : u32 : 16;
    to_b_ref : u32 : 16;
}
}

"#;
        pub const CHAPTER: &str = r#"namespace coappearances {
struct Chapter
{
    major : u8 : 4;
    minor : u8 : 7;
}
}

"#;
        pub const INVARIANTS: &str = r#"namespace coappearances {
struct Invariants
{
    max_degree : u32 : 16;
    max_degree_ref : u32 : 16;
    min_degree : u32 : 16;
    min_degree_ref : u32 : 16;
    num_connected_components : u32 : 16;
}
}

"#;
        pub const DEGREE: &str = r#"namespace coappearances {
struct Degree
{
    value : u32 : 16;
}
}

"#;
        pub const INDEX_TYPE32: &str = r#""#;
        pub const STATISTICS: &str = r#"namespace coappearances {
struct Invariants
{
    max_degree : u32 : 16;
    max_degree_ref : u32 : 16;
    min_degree : u32 : 16;
    min_degree_ref : u32 : 16;
    num_connected_components : u32 : 16;
}
}

namespace coappearances {
struct Degree
{
    value : u32 : 16;
}
}

namespace coappearances {
@bound_implicitly( characters : .coappearances.Graph.vertices, .coappearances.Statistics.vertex_degrees )
archive Statistics
{
    invariants : .coappearances.Invariants;
    vertex_degrees : vector< .coappearances.Degree >;
}
}

"#;
        pub const GRAPH: &str = r#"namespace coappearances {
struct Meta
{
    title_ref : u32 : 32;
    author_ref : u32 : 32;
}
}

namespace coappearances {
struct Character
{
    name_ref : u32 : 32;
}
}

namespace coappearances {
struct Coappearance
{
    a_ref : u32 : 16;
    b_ref : u32 : 16;
    count : u32 : 16;
    first_chapter_ref : u32 : 16;
}
}

namespace coappearances {
struct Nickname
{
    ref : u32 : 32;
}
}

namespace coappearances {
struct Description
{
    ref : u32 : 32;
}
}

namespace coappearances {
struct UnaryRelation
{
    kind_ref : u32 : 32;
    to_ref : u32 : 16;
}
}

namespace coappearances {
struct BinaryRelation
{
    kind_ref : u32 : 32;
    to_a_ref : u32 : 16;
    to_b_ref : u32 : 16;
}
}

namespace coappearances {
struct Chapter
{
    major : u8 : 4;
    minor : u8 : 7;
}
}

namespace coappearances {
struct Invariants
{
    max_degree : u32 : 16;
    max_degree_ref : u32 : 16;
    min_degree : u32 : 16;
    min_degree_ref : u32 : 16;
    num_connected_components : u32 : 16;
}
}

namespace coappearances {
struct Degree
{
    value : u32 : 16;
}
}

namespace coappearances {
@bound_implicitly( characters : .coappearances.Graph.vertices, .coappearances.Statistics.vertex_degrees )
archive Statistics
{
    invariants : .coappearances.Invariants;
    vertex_degrees : vector< .coappearances.Degree >;
}
}

namespace coappearances {
@bound_implicitly( characters : .coappearances.Graph.vertices, .coappearances.Graph.vertices_data )
archive Graph
{
    @explicit_reference( .coappearances.Meta.title_ref, .coappearances.Graph.strings )
    @explicit_reference( .coappearances.Meta.author_ref, .coappearances.Graph.strings )
    meta : .coappearances.Meta;
    @explicit_reference( .coappearances.Character.name_ref, .coappearances.Graph.strings )
    vertices : vector< .coappearances.Character >;
    @explicit_reference( .coappearances.Coappearance.a_ref, .coappearances.Graph.vertices )
    @explicit_reference( .coappearances.Coappearance.b_ref, .coappearances.Graph.vertices )
    @explicit_reference( .coappearances.Coappearance.first_chapter_ref, .coappearances.Graph.chapters )
    edges : vector< .coappearances.Coappearance >;
    @explicit_reference( .coappearances.Nickname.ref, .coappearances.Graph.strings )
    @explicit_reference( .coappearances.Description.ref, .coappearances.Graph.strings )
    @explicit_reference( .coappearances.UnaryRelation.kind_ref, .coappearances.Graph.strings )
    @explicit_reference( .coappearances.UnaryRelation.to_ref, .coappearances.Graph.vertices )
    @explicit_reference( .coappearances.BinaryRelation.kind_ref, .coappearances.Graph.strings )
    @explicit_reference( .coappearances.BinaryRelation.to_a_ref, .coappearances.Graph.vertices )
    @explicit_reference( .coappearances.BinaryRelation.to_b_ref, .coappearances.Graph.vertices )
    vertices_data : multivector< 32, .coappearances.Nickname, .coappearances.Description, .coappearances.UnaryRelation, .coappearances.BinaryRelation >;
    chapters : vector< .coappearances.Chapter >;
    strings : raw_data;
    @optional
    statistics : archive .coappearances.Statistics;
}
}

"#;
    }

    pub mod resources {
        pub mod statistics {
pub const INVARIANTS: &str = r#"namespace coappearances {
struct Invariants
{
    max_degree : u32 : 16;
    max_degree_ref : u32 : 16;
    min_degree : u32 : 16;
    min_degree_ref : u32 : 16;
    num_connected_components : u32 : 16;
}
}

namespace coappearances {
archive Statistics
{
    invariants : .coappearances.Invariants;
}
}

"#;
pub const VERTEX_DEGREES: &str = r#"namespace coappearances {
struct Degree
{
    value : u32 : 16;
}
}

namespace coappearances {
archive Statistics
{
    vertex_degrees : vector< .coappearances.Degree >;
}
}

"#;
pub const CHARACTERS: &str = r#"namespace coappearances {
@bound_implicitly( characters : .coappearances.Graph.vertices, .coappearances.Statistics.vertex_degrees )
archive Statistics
{
}
}

"#;
        }
        pub mod graph {
pub const META: &str = r#"namespace coappearances {
struct Meta
{
    title_ref : u32 : 32;
    author_ref : u32 : 32;
}
}

namespace coappearances {
archive Graph
{
    @explicit_reference( .coappearances.Meta.title_ref, .coappearances.Graph.strings )
    @explicit_reference( .coappearances.Meta.author_ref, .coappearances.Graph.strings )
    meta : .coappearances.Meta;
}
}

"#;
pub const VERTICES: &str = r#"namespace coappearances {
struct Character
{
    name_ref : u32 : 32;
}
}

namespace coappearances {
archive Graph
{
    @explicit_reference( .coappearances.Character.name_ref, .coappearances.Graph.strings )
    vertices : vector< .coappearances.Character >;
}
}

"#;
pub const EDGES: &str = r#"namespace coappearances {
struct Coappearance
{
    a_ref : u32 : 16;
    b_ref : u32 : 16;
    count : u32 : 16;
    first_chapter_ref : u32 : 16;
}
}

namespace coappearances {
archive Graph
{
    @explicit_reference( .coappearances.Coappearance.a_ref, .coappearances.Graph.vertices )
    @explicit_reference( .coappearances.Coappearance.b_ref, .coappearances.Graph.vertices )
    @explicit_reference( .coappearances.Coappearance.first_chapter_ref, .coappearances.Graph.chapters )
    edges : vector< .coappearances.Coappearance >;
}
}

"#;
pub const VERTICES_DATA: &str = r#"namespace coappearances {
struct Nickname
{
    ref : u32 : 32;
}
}

namespace coappearances {
struct Description
{
    ref : u32 : 32;
}
}

namespace coappearances {
struct UnaryRelation
{
    kind_ref : u32 : 32;
    to_ref : u32 : 16;
}
}

namespace coappearances {
struct BinaryRelation
{
    kind_ref : u32 : 32;
    to_a_ref : u32 : 16;
    to_b_ref : u32 : 16;
}
}

namespace coappearances {
archive Graph
{
    @explicit_reference( .coappearances.Nickname.ref, .coappearances.Graph.strings )
    @explicit_reference( .coappearances.Description.ref, .coappearances.Graph.strings )
    @explicit_reference( .coappearances.UnaryRelation.kind_ref, .coappearances.Graph.strings )
    @explicit_reference( .coappearances.UnaryRelation.to_ref, .coappearances.Graph.vertices )
    @explicit_reference( .coappearances.BinaryRelation.kind_ref, .coappearances.Graph.strings )
    @explicit_reference( .coappearances.BinaryRelation.to_a_ref, .coappearances.Graph.vertices )
    @explicit_reference( .coappearances.BinaryRelation.to_b_ref, .coappearances.Graph.vertices )
    vertices_data : multivector< 32, .coappearances.Nickname, .coappearances.Description, .coappearances.UnaryRelation, .coappearances.BinaryRelation >;
}
}

"#;
pub const CHAPTERS: &str = r#"namespace coappearances {
struct Chapter
{
    major : u8 : 4;
    minor : u8 : 7;
}
}

namespace coappearances {
archive Graph
{
    chapters : vector< .coappearances.Chapter >;
}
}

"#;
pub const STRINGS: &str = r#"namespace coappearances {
archive Graph
{
    strings : raw_data;
}
}

"#;
pub const STATISTICS: &str = r#"namespace coappearances {
struct Invariants
{
    max_degree : u32 : 16;
    max_degree_ref : u32 : 16;
    min_degree : u32 : 16;
    min_degree_ref : u32 : 16;
    num_connected_components : u32 : 16;
}
}

namespace coappearances {
struct Degree
{
    value : u32 : 16;
}
}

namespace coappearances {
@bound_implicitly( characters : .coappearances.Graph.vertices, .coappearances.Statistics.vertex_degrees )
archive Statistics
{
    invariants : .coappearances.Invariants;
    vertex_degrees : vector< .coappearances.Degree >;
}
}

namespace coappearances {
archive Graph
{
    @optional
    statistics : archive .coappearances.Statistics;
}
}

"#;
pub const CHARACTERS: &str = r#"namespace coappearances {
@bound_implicitly( characters : .coappearances.Graph.vertices, .coappearances.Graph.vertices_data )
archive Graph
{
}
}

"#;
        }
    }
}

/// Meta information about the book.
define_struct!(
    Meta,
    RefMeta,
    RefMutMeta,
    schema::structs::META,
    8,
    (title_ref, set_title_ref, u32, 0, 32),
    (author_ref, set_author_ref, u32, 32, 32));

/// A character.
define_struct!(
    Character,
    RefCharacter,
    RefMutCharacter,
    schema::structs::CHARACTER,
    4,
    (name_ref, set_name_ref, u32, 0, 32));

/// An appearance of two characters in the same scene.
///
/// count - multiplicity of the coappearance.
/// first_chapter_ref - a reference to the first chapter in which characters appear. How to get the
/// full range of chapters is described in 'coappearances.cpp:read'.
define_struct!(
    Coappearance,
    RefCoappearance,
    RefMutCoappearance,
    schema::structs::COAPPEARANCE,
    8,
    (a_ref, set_a_ref, u32, 0, 16),
    (b_ref, set_b_ref, u32, 16, 16),
    (count, set_count, u32, 32, 16),
    (first_chapter_ref, set_first_chapter_ref, u32, 48, 16));

/// A nickname or an alternative name of a character.
define_struct!(
    Nickname,
    RefNickname,
    RefMutNickname,
    schema::structs::NICKNAME,
    4,
    (ref_, set_ref, u32, 0, 32));

/// A description of a character.
define_struct!(
    Description,
    RefDescription,
    RefMutDescription,
    schema::structs::DESCRIPTION,
    4,
    (ref_, set_ref, u32, 0, 32));

/// A relation of a character to another one.
define_struct!(
    UnaryRelation,
    RefUnaryRelation,
    RefMutUnaryRelation,
    schema::structs::UNARY_RELATION,
    6,
    (kind_ref, set_kind_ref, u32, 0, 32),
    (to_ref, set_to_ref, u32, 32, 16));

/// A relation of a character to two other characters.
define_struct!(
    BinaryRelation,
    RefBinaryRelation,
    RefMutBinaryRelation,
    schema::structs::BINARY_RELATION,
    8,
    (kind_ref, set_kind_ref, u32, 0, 32),
    (to_a_ref, set_to_a_ref, u32, 32, 16),
    (to_b_ref, set_to_b_ref, u32, 48, 16));

/// A chapter in the book.
define_struct!(
    Chapter,
    RefChapter,
    RefMutChapter,
    schema::structs::CHAPTER,
    2,
    (major, set_major, u8, 0, 4),
    (minor, set_minor, u8, 4, 7));


define_struct!(
    Invariants,
    RefInvariants,
    RefMutInvariants,
    schema::structs::INVARIANTS,
    10,
    (max_degree, set_max_degree, u32, 0, 16),
    (max_degree_ref, set_max_degree_ref, u32, 16, 16),
    (min_degree, set_min_degree, u32, 32, 16),
    (min_degree_ref, set_min_degree_ref, u32, 48, 16),
    (num_connected_components, set_num_connected_components, u32, 64, 16));


define_struct!(
    Degree,
    RefDegree,
    RefMutDegree,
    schema::structs::DEGREE,
    2,
    (value, set_value, u32, 0, 16));

/// Builtin type to for MultiVector index
define_index!(
    IndexType32,
    RefIndexType32,
    RefMutIndexType32,
    schema::structs::INDEX_TYPE32,
    4,
    32
);



define_archive!(Statistics, StatisticsBuilder,
    schema::structs::STATISTICS;
    // struct resources
    (invariants, set_invariants,
        Invariants, schema::resources::statistics::INVARIANTS, false);
    // vector resources
    (vertex_degrees, set_vertex_degrees, start_vertex_degrees,
        Degree, schema::resources::statistics::VERTEX_DEGREES, false);
    // multivector resources
;
    // raw data resources
;
    // subarchives
);


/// Builtin union type of Nickname, Description, UnaryRelation, BinaryRelation.
define_variadic_struct!(VerticesData, RefVerticesData, BuilderVerticesData,
    IndexType32,
    0 => (Nickname, add_nickname),
    1 => (Description, add_description),
    2 => (UnaryRelation, add_unary_relation),
    3 => (BinaryRelation, add_binary_relation));

define_archive!(Graph, GraphBuilder,
    schema::structs::GRAPH;
    // struct resources
    (meta, set_meta,
        Meta, schema::resources::graph::META, false);
    // vector resources
    (vertices, set_vertices, start_vertices,
        Character, schema::resources::graph::VERTICES, false),
    (edges, set_edges, start_edges,
        Coappearance, schema::resources::graph::EDGES, false),
    (chapters, set_chapters, start_chapters,
        Chapter, schema::resources::graph::CHAPTERS, false);
    // multivector resources
    (vertices_data, start_vertices_data,
        VerticesData, schema::resources::graph::VERTICES_DATA,
        vertices_data_index, IndexType32, false);
    // raw data resources
    (strings, set_strings,
        schema::resources::graph::STRINGS, false);
    // subarchives
    (statistics,
        Statistics, StatisticsBuilder,
        schema::resources::graph::STATISTICS, true));

