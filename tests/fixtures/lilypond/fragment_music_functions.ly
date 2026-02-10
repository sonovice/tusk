\version "2.24.0"

% Built-in functions already covered by other fixtures
% (\grace, \tuplet, \relative, \transpose, etc.)

% Generic music function with a braced music argument
\someFunction { c4 d e f }

% Function with string argument and music
\tag "part" { c4 d e f }

% Function with scheme argument and music
\keepWithTag #'print { c4 d e f }

% Function with multiple arguments
\partCombine { c4 e g c' } { e4 g c' e' }

% Function with numeric argument
\magnifyMusic 0.63 { c4 d e f }

% Partial function with \etc
\tag #'score \etc

% Bare identifier (no arguments)
\myMelody
