% Grace note constructs

% Standard grace
\grace c16 d4

% Acciaccatura
\acciaccatura d8 c4

% Appoggiatura
\appoggiatura d8 c2

% Grace with braces (multiple notes)
\grace { c16 d16 } e4

% Acciaccatura with braces
\acciaccatura { c16 d16 } e2

% afterGrace without fraction
\afterGrace c2 { d16 e16 }

% afterGrace with fraction
\afterGrace 3/4 c2 { d16 }

% afterGrace with different fraction
\afterGrace 7/8 f1 { g8 }
