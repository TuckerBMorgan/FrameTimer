use std::time::{SystemTime, Duration};
use std::thread::sleep;

pub struct FrameTimer {
    n_frame_per_second_length: Duration,
    //I allocate the frame_duration varible here so that I can avoid an allocation each frame
    frame_duration: Duration,
    sleep_duration: Duration,
    frame_start: SystemTime,
    frame_end: SystemTime,
}

impl FrameTimer {
    pub fn new() -> Self {

        let sleep_duration = Duration::from_millis(0);
        //TODO: Make this more then just a 60 FPS timer
        let sixty_frames_per_second_length = Duration::from_millis(16);
        
        let frame_duration = Duration::from_millis(0);
        let frame_start = SystemTime::now();
        let frame_end = SystemTime::now();

        FrameTimer {
            n_frame_per_second_length: sixty_frames_per_second_length,
            frame_duration: frame_duration,
            sleep_duration: sleep_duration,
            frame_start: frame_start,
            frame_end: frame_end
        }
    }

    pub fn frame_start(&mut self) {
        self.frame_start = SystemTime::now();
    }

    pub fn frame_end(&mut self) {
        self.frame_end = SystemTime::now();
        self.frame_duration = self.frame_start.duration_since(self.frame_start).unwrap();
        if self.frame_duration.as_secs() > 0 {
            return;
        }

        if self.sleep_duration.subsec_nanos() < self.n_frame_per_second_length.subsec_nanos() {
            self.sleep_duration = self.n_frame_per_second_length - self.frame_duration;
            sleep(self.sleep_duration);
        }
    }
}