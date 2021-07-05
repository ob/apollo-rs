use crate::TokenKind;

/// A node in the immutable tree. It has other nodes and tokens as children.
pub type SyntaxNode = rowan::SyntaxNode<GraphQLLanguage>;
/// A leaf node in the AST.
pub type SyntaxToken = rowan::SyntaxToken<GraphQLLanguage>;
/// A `SyntaxNode` or a `SyntaxToken`.
pub type SyntaxElement = rowan::SyntaxElement<GraphQLLanguage>;
/// Children of a `SyntaxNode`.
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<GraphQLLanguage>;
/// Children of a `SyntaxElement`.
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<GraphQLLanguage>;

/// A language implementation for use in `Rowan`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GraphQLLanguage {}

impl rowan::Language for GraphQLLanguage {
    type Kind = TokenKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= TokenKind::Root as u16);
        unsafe { std::mem::transmute::<u16, TokenKind>(raw.0) }
    }
    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}
