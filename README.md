
# RIBBIT

```

                           '.  ckO,
                         .xKOXXkkNko                            ';
                        0xoclodOkkKKKd;.      .       .        .... .
                        do'.  ':lxK0KkXWxkOOkOddl';;.  .  .      .,        ..;.
                         .:.     ,dxXOWKcxdKcxkkkdokOKll.. ..  .  ;   .  ..   '
             .    .       .c      'dx0OKddXKOdx0KXxcOxkNKo   '.   :. . .'
            . .. ;,      . c,      ::xdNkkdlxxkOdkkK000WXX0l'.'. .d.' ...''..
         ....... ,  ....  .;x'     .cxONkKXNO0K0KXKNKKXWWWNWNk:;.,d;,l','.'  . ...
        ...'',::c'.........c0Oc..  .'oOX0NOXkKx0XNXOxKWWWXXK0dl',l0dkc;;.... ... ..
       .....;cx.';.....,::;:xNN0l,..'';OXNKkdxNWKkkxXKOKKOXWKO0dokN0l;'.. .  .
      .....';oklccdodl:clllolONXKKl,,;;oNKO0kxNX0OKNXNXN0XKdxKKOdXWOx:,..
     .';:,'..':,.,,,,'......:XXNXxk::;.dNKX0ONXKxxOOkO0OOOKX0kkKXXkoc.
     ..,;::,,:;......''.':,;:,;dx';ld0xkOXKKkoclkkx0XX0OOxOkkXK0d:;,'..
             .    ...':clc::;,'',;lOKO000kxkOOxcllkdooOKxkKOkdc,,..
                   .,:;:ocooccllc::cc:;,',,'.....oo,.oc,'..
                            .......   ...'..    c'....
```
Rust RIBBIT client.

## Quick start
```rust
extern crate rust_RIBBIT;
use rust_RIBBIT::client as client;

fn frog_say() {
  let mut frog = client::RIBBITClient::new();
  let tip = frog.frog_tip();
  println!("{}", tip);
}
```
  
