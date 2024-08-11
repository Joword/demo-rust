
trait Name {
    song_id: u32
    anchor_time: u32
}

trait RecodeData {
    audio: &str
    duration: f64
    channels: i32
    sample_rate: i32
    sample_size: i32
}