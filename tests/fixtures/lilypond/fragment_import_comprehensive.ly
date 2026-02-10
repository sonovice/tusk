\version "2.24.0"

\header {
  title = "Comprehensive Import Test"
  composer = "Test"
}

melody = \relative c' {
  \clef treble
  \key d \major
  \time 3/4
  \tempo 4 = 120
  \mark \default
  d4\p e( fis)
  \tuplet 3/2 { g8 a b }
  \repeat volta 2 {
    a4.\f\< b8 cis4\!
  }
  \alternative {
    { d4 r r }
    { cis4 r r }
  }
  \grace { e16 } d4-.
  \acciaccatura { fis8 } g4\fermata
  \bar "|."
}

\score {
  \new Staff = "main" \melody
  \layout { }
}
