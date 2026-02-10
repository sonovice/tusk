\version "2.24.0"

\paper {
  indent = 0
  ragged-right = ##t
  ragged-last = ##f
}

\layout {
  \context {
    \Score
    \remove "Bar_number_engraver"
  }
  \context {
    \Staff
    \consists "Span_arpeggio_engraver"
  }
}

\midi {
  \context {
    \Score
    midiMinimumVolume = #0.2
  }
}

\score {
  \new Staff { c4 d e f }
  \layout {
    ragged-right = ##t
  }
  \midi { }
}
