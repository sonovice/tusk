\version "2.24.0"

%% Scheme expression coverage: booleans, numbers, strings, symbols, lists, identifiers

% Boolean values
\set Staff.useBassFigureExtenders = ##t
\set Staff.voltaSpannerDuration = ##f

% Numeric values (integer)
\override Staff.fontSize = #-2

% Identifier values
\override NoteHead.color = #red

% Quoted symbol
\keepWithTag #'print { c4 d e f }

% S-expression list
\override Glissando.color = #(rgb-color 1 0 0)

% Scheme in markup
\markup \with-color #red "colored text"
\markup \abs-fontsize #16 "large text"

% Score context
\score {
  \new Staff {
    \override NoteHead.color = #red
    \set Staff.useBassFigureExtenders = ##t
    c4 d e f |
    \revert NoteHead.color
    g4 a b c |
  }
}
