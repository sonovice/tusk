% Lyric mode constructs (Phase 20)
\score {
  <<
    \new Voice = "melody" {
      c4 d e f
    }
    \new Lyrics \lyricsto "melody" {
      do re mi fa
    }
  >>
}

% Lyricmode with hyphens and extenders
\lyricmode {
  Hal -- le -- lu -- jah __
}

% Addlyrics
{ c4 d e f }
\addlyrics { one two three four }

% Lyrics shorthand (expands to \new Lyrics \lyricmode)
\lyrics { la la la la }
