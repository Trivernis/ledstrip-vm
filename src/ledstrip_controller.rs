use std::io;
use std::io::Write;
use std::net::TcpStream;

const STATE_COMMAND_PREFIX: u8 = 0x71;
const PROGRAM_COMMAND_PREFIX: u8 = 0x61;

pub enum StateStripCommand {
    On = 0x23,
    Off = 0x24,
}

pub enum ProgramStripCommand {
    SevenCrossFade = 0x25,
    RedGradual = 0x26,
    GreenGradual = 0x27,
    BlueGradual = 0x28,
    WhiteGradual = 0x2c,
    RedGreenCross = 0x2d,
    RedBlueCross = 0x2e,
    GreenBlueCross = 0x2f,
    SevenStrobe = 0x30,
    RedStrobe = 0x31,
    GreenStrobe = 0x32,
    BlueStrobe = 0x33,
    WhiteStrobe = 0x37,
    SevenJumping = 0x38,
}

#[derive(Debug)]
pub struct LedStripController {
    stream: Option<TcpStream>,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl LedStripController {
    pub fn new(ip: &str, port: usize) -> io::Result<Self> {
        let stream = if let Ok(tcp_stream) = TcpStream::connect(format!("{}:{}", ip, port)) {
            Some(tcp_stream)
        } else {
            eprintln!("Failed to connect. Writing sent data to stdout.");
            None
        };

        Ok(Self {
            stream,
            r: 0,
            g: 0,
            b: 0,
        })
    }

    fn send(&mut self, message: &[u8]) -> io::Result<()> {
        if let Some(stream) = &mut self.stream {
            stream.write(message)?;
            Ok(())
        } else {
            println!("Send: {:?}", message);
            Ok(())
        }
    }

    /// Send an rgb color to the led strip
    pub fn send_rgb_color(&mut self, r: u8, g: u8, b: u8) -> io::Result<()> {
        self.r = r;
        self.g = g;
        self.b = b;
        let message = create_message(&[0x31, r, g, b, 0xf0]);
        self.send(&message)?;

        Ok(())
    }

    /// Sets the state of the strip to either on or off
    pub fn set_state(&mut self, cmd: StateStripCommand) -> io::Result<()> {
        self.send(&create_message(&[STATE_COMMAND_PREFIX, cmd as u8]))?;

        Ok(())
    }
    /// Sends a strip command with a specified speed
    /// that is one of 0x01 0x06, 0x10, 0x1c
    pub fn send_command(&mut self, cmd: ProgramStripCommand, speed: u8) -> io::Result<()> {
        self.send(&create_message(&[PROGRAM_COMMAND_PREFIX, cmd as u8, speed]))?;

        Ok(())
    }
}

/// Create a message for the led strip
fn create_message(data: &[u8]) -> Vec<u8> {
    let mut data = data.clone().to_vec();
    data.append(&mut vec![0x0f]);
    let mut sum = 0u128;
    for i in &data {
        sum += *i as u128;
    }
    data.push(sum as u8 & 255);

    data
}
