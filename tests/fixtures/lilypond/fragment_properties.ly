\version "2.24.0"

% Override examples
\override NoteHead.color = #red
\override Staff.TimeSignature.color = #green
\override Beam.gap-count = 5

% Revert examples
\revert NoteHead.color
\revert Staff.BarLine.color

% Set examples
\set Staff.instrumentName = "Piano"
\set Staff.useBassFigureExtenders = ##t

% Unset examples
\unset Staff.keyAlterations

% Once modifier
\once \override NoteHead.color = #red

% Music with property operations
{
  \override NoteHead.color = #red
  c4 d e f
  \revert NoteHead.color
  g4 a b c
}

% Context with property operations
\new Staff \with {
  \override TimeSignature.color = #green
  \set instrumentName = "Piano"
} {
  c4 d e f
}

% Tweak as post-event
{ c4\tweak color #red -. }
