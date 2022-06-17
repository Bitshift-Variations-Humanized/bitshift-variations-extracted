#![feature(read_buf)]

type DynErr = Box<dyn std::error::Error>;
type Result<T = (), E = DynErr> = std::result::Result<T,E>;

use std::io::Read;
const K: usize = 1024;
const TEST_WINDOW_SIZE: usize = 256 * K;

fn main() -> Result {
    let mut comm = std::process::Command::new("./bitshift-variations");
    comm.stdout(std::process::Stdio::piped());
    comm.stdin (std::process::Stdio::null() );
    comm.stderr(std::process::Stdio::null() );
    let mut child = comm.spawn()?;
    let mut stdout = child.stdout.take().unwrap().bytes();
    let mut buf = [0;TEST_WINDOW_SIZE];
    for i in 0..TEST_WINDOW_SIZE {
        buf[i] = stdout.next().unwrap().unwrap();
    }
    let first_window = buf.clone();
    let mut i = 0;
    loop {
        buf.rotate_left(1);
        buf[TEST_WINDOW_SIZE - 1] = stdout.next().unwrap().unwrap();
        if buf == first_window {
            println!("{i}");
            break;
        }
        if i % (2 * K) == 0 {
            println!("{i}");
        }
        i += 1;
    }

    let mut collect_comm = std::process::Command::new("./bitshift-variations");
    collect_comm.stdout(std::process::Stdio::piped());
    collect_comm.stdin (std::process::Stdio::null() );
    collect_comm.stderr(std::process::Stdio::null() );
    let mut collect_child = collect_comm.spawn()?;
    let mut collect_stdout = collect_child.stdout.take().unwrap().take(i.try_into().unwrap());
    let mut collect_buf = Vec::with_capacity(i);
    std::io::copy(&mut collect_stdout, &mut collect_buf)?;
    std::fs::write("bitshift-variations-extracted.bin", &collect_buf)?;
    let mut wav_output_file = std::fs::File::create("bitshift-variations-extracted.wav")?;
    wav::write(wav::Header::new(wav::WAV_FORMAT_PCM, 1, 8000, 8), &wav::BitDepth::Eight(collect_buf), &mut wav_output_file)?;
    Ok(())
}
