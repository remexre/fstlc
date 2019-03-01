: fstlc-read-block ( addr u -- x_0 ... x_u )
  0 ?do dup i + @ swap loop drop ;
: fstlc-write-block ( x_0 ... x_u u -- addr )
  dup here 0 rot swap ?do i 2 + pick , loop r> discard >r ;
