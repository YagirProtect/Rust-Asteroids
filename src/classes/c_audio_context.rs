use rodio::source::{SineWave, Source};
use std::time::Duration;
use rodio::OutputStream;

#[derive(Default)]
pub struct AudioContext {
    stream: Option<rodio::OutputStream>,
}

impl AudioContext {
    pub fn new() -> Self {
        let stream = rodio::OutputStreamBuilder::open_default_stream()
            .expect("open default audio stream");
        Self {
            stream: Some(stream)
        }
    }

    pub fn beep(&self, freq_hz: f32, ms: u64, volume: f32) {

        let mixer = match &self.stream{
            None => {
                return;
            }
            Some(stream) => {
                stream
            }
        };

        let sink = rodio::Sink::connect_new(mixer.mixer());
        sink.set_volume(volume);

        let src = SineWave::new(freq_hz)
            .take_duration(Duration::from_millis(ms))
            .amplify(0.20);

        sink.append(src);
        sink.detach();
    }
    pub fn beep_shoot(&self) {
        self.beep(950.0, 35, 0.35);
    }

    pub fn beep_asteroid_hit(&self) {
        self.beep(100.0, 65, 0.45);
    }

    pub fn beep_shoot_enemy(&self) {
        self.beep(500.0, 55, 0.45);
    }

    pub fn beep_death(&self) {
        self.beep_seq(
            &[
                (520.0, 70, 0.35),
                (320.0, 90, 0.35),
                (180.0, 140, 0.35),
            ],
            20,
        );
    }


    pub fn beep_seq(&self, seq: &[(f32, u64, f32)], gap_ms: u64) {
        let Some(stream) = &self.stream else { return; };

        let sink = rodio::Sink::connect_new(stream.mixer());
        sink.set_volume(1.0);

        for (freq, ms, vol) in seq {
            let s = SineWave::new(*freq)
                .take_duration(Duration::from_millis(*ms))
                .amplify(*vol);

            sink.append(s);

            if gap_ms > 0 {
                sink.append(rodio::source::Zero::new(1, 44_100).take_duration(Duration::from_millis(gap_ms)));
            }
        }

        sink.detach();
    }
}