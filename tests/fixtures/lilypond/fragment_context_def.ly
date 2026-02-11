\version "2.24.0"

\layout {
  \context {
    \Staff
    \accepts "CueVoice"
    \denies "Voice"
  }
  \context {
    \Score
    \defaultchild "Staff"
    \description "The main score context"
    \name "Score"
    \alias "Score"
  }
}

\new Staff {
  c4 d e f
}
