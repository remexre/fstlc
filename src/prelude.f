: fstlc-make-pair ( x y -- x,y ) here rot , swap , ;
: fstlc-fst ( x,y -- x ) @ ;
: fstlc-snd ( x,y -- y ) 4 + @ ;
