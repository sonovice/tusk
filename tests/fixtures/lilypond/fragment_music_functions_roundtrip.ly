\version "2.24.0"

% Music function calls with notes for roundtrip testing
{
  \someFunction { c4 d e f } g4
  \tag "part" { a4 b c' d' } e'4
  \magnifyMusic 0.63 { f4 g a b } c'4
}
