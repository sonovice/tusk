\version "2.24.0"

\book {
  \header { title = "Test Book" }

  \bookpart {
    \header { subtitle = "Part 1" }
    \score {
      { c4 d e f }
    }
  }

  \bookpart {
    \header { subtitle = "Part 2" }
    \score {
      { g4 a b c' }
    }
  }
}
