# FrameTimer
A rust drop in frame timer

BETA VERSION

This is a frame timer for rust, meant to be used to insure that game loops stay to a certain rate

It is meant to be as simple as can be
First create it like this 

~~~rust
let mut frame_timer: FrameTimer = FrameTimer::new();
~~~

At the start of your game loop call
~~~rust
frame_timer.frame_start();
~~~

At the end of your game loop call
~~~rust 
frame_timer.frame_end();
~~~

and that will limit your game to 60 frames a second


Future features
1. Add in a delta frame calculation and varible
2. Make the frame timer smarter so that it may 
3. Open to suggestions