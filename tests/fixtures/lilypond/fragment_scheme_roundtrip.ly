\version "2.24.0"

%% Scheme expression roundtrip coverage

\header {
  title = "Scheme Roundtrip"
  tagline = ##f
}

\score {
  \new Staff {
    \override Staff.fontSize = #-2
    \override NoteHead.color = #red
    \set Staff.useBassFigureExtenders = ##t
    \override Glissando.color = #(rgb-color 1 0 0)
    \keepWithTag #'print { c4 d e f }
    \revert NoteHead.color
    g4 a b c |
  }
}
