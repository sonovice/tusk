\version "2.24.0"

% Basic volta repeat
\repeat volta 2 { c4 d e f }

% Volta repeat with alternatives
\repeat volta 2 { c4 d e f } \alternative { { g2 } { a2 } }

% Unfold repeat
\repeat unfold 4 { c8 d }

% Percent repeat
\repeat percent 4 { c4 d e f }

% Tremolo repeat
\repeat tremolo 8 { c16 d }

% Nested repeat
\repeat volta 2 { \repeat unfold 3 { c8 d } e4 }

% Volta with three alternatives
\repeat volta 3 { c4 d e f } \alternative { { g2 } { a2 } { b2 } }

% Segno repeat
\repeat segno 2 { c4 d e f }
