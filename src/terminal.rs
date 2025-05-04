use std::sync::mpsc::{self, Receiver};
use std::thread;
extern crate termios;
use std::io::{self, Read};
use termios::{ECHO, ICANON, TCSANOW, Termios, tcsetattr};

pub fn spawn_stdin_channel() -> Receiver<u8> {
    let stdin = 0; // couldn't get std::os::unix::io::FromRawFd to work 
    // on /dev/stdin or /dev/tty
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone(); // make a mutable copy of termios 
    // that we will modify
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let mut reader = io::stdin();

    let (tx, rx) = mpsc::channel::<u8>();
    let mut buffer: [u8; 1] = [0; 1]; // read exactly one byte
    thread::spawn(move || {
        loop {
            reader.read(&mut buffer).unwrap();

            tx.send(buffer[0]).unwrap();
        }
    });
    rx
}

pub fn get_terminal() -> Termios {
    Termios::from_fd(0).unwrap()
}

pub fn cleanup(terminal: &Termios) {
    tcsetattr(0, TCSANOW, terminal).unwrap();
}
