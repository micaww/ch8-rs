use rodio::{source::SineWave, OutputStream, Sink};

pub struct Speaker {
    _stream: OutputStream,
    sink: Sink
}

impl Speaker {
    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        sink.pause();
        sink.append(SineWave::new(440));

        Speaker {
            _stream,
            sink
        }
    }

    /// Starts the sound.
    pub fn start(&self) {
        self.sink.play();
    }

    /// Stops the sound.
    pub fn stop(&self) {
        self.sink.pause();
    }
}