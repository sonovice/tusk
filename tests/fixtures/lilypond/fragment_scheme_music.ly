\version "2.24.0"

%% Scheme expressions in music position (music_embedded / embedded_scm_active)

{
  c4
  %% Scheme list call returning music
  #(ly:export (make-music 'SkipEvent))
  d4
  %% Scheme identifier reference in music position
  #myMusicVar
  e4
  %% Embedded LilyPond inside Scheme returning music
  ##{ f4 #}
  g4
}
