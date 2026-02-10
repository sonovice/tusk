\version "2.24.0"
\score {
  {
    \tempo "Allegro" 4 = 120
    c4 d e f
    \tempo 2 = 60
    g2 a
    \tempo "Andante"
    b4 c d e
    \tempo "Vivace" 4. = 132-144
    f4 g a b
    \mark \default
    c4 d e f
    \mark "A"
    g4 a b c
    \mark 5
    d4 e f g
    \textMark "Fine"
    a4 b c d
  }
}
