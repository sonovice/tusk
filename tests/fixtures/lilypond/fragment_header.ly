\version "2.24.0"

\header {
  title = "Test Suite"
  subtitle = "For Header Parsing"
  composer = "J.S. Bach"
  arranger = "Claude"
  poet = "Anonymous"
  opus = "BWV 1"
  piece = "Prelude"
  dedication = "For testing"
  copyright = "Public Domain"
  tagline = ##f
}

\score {
  \new Staff { c4 d e f }
  \header {
    piece = "Nested Header"
  }
  \layout { }
  \midi { }
}
