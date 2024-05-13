// Gurleen Dhaliwal
// Life - Homework 1

#![no_main]
#![no_std]

// Importing modules
mod life;
use life::*;

// Importing panic handler and RTT print utilities
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print, rprint};

use core::ops::RangeFrom;
use cortex_m_rt::entry;
use embedded_hal::{blocking::delay::DelayMs, digital::v2::InputPin};
use microbit::{board::Board, display::blocking::Display, hal::timer::Timer, pac::TIMER0};
use nanorand::Rng;


#[entry]
fn init() -> ! {
    // Initializing RTT print utility
    rtt_init_print!();

    // Taking control of the micro:bit board
    let board = Board::take().unwrap();
    let mut display = Display::new(board.display_pins);
    let mut timer = Timer::new(board.TIMER0);

    // Initializing the game board and seed generator
    let mut life_board = [[0; 5]; 5];
    let mut seeds = 9..; 
    life_board = initialize_board(&mut life_board, seeds.next().unwrap());
    rprintln!("Initial Board: ");
    print_board(&life_board);

    // Main game loop
    loop {
        // Checking if button A is pressed
        if let Ok(true) = board.buttons.button_a.is_low() {
            press_a(&mut life_board, &mut display, &mut timer, &mut seeds);
        } 
        // Checking if button B is pressed
        else if let Ok(true) = board.buttons.button_b.is_low() {
            press_b(&mut life_board, &mut display, &mut timer);
        } 
        // If no button is pressed
        else {
            // Displaying the game board
            display_board(&mut timer, &life_board, &mut display);
            timer.delay_ms(100u16);
            rprintln!("Current Board: ");
            print_board(&life_board);
            life(&mut life_board);

            // Checking if the game is over
            if done(&life_board) {
                timer.delay_ms(3000u16);

                // Checking if button A is pressed after game over
                if let Ok(true) = board.buttons.button_a.is_low() {
                    rprintln!("Board A continues after Game");
                    press_a(&mut life_board, &mut display, &mut timer, &mut seeds);
                    continue;
                } 
                // Checking if button B is pressed after game over
                else if let Ok(true) = board.buttons.button_b.is_low() {
                    rprintln!("Board B continues after Game");
                    press_b(&mut life_board, &mut display, &mut timer);
                    continue;
                } 
                // If no button is pressed after game over
                else {
                    // Generating a new random board
                    let seed = seeds.next().unwrap();
                    rprintln!("New seed: {}", seed);
                    life_board = initialize_board(&mut life_board, seed);
                }
            }
        }
    }

    //panic!("Done")
}

// Function to display the game board
fn display_board(timer: &mut Timer<TIMER0>, life_board: &[[u8; 5]; 5], display: &mut Display) {
    display.show(timer, *life_board, 1000);
}

// Print board
fn print_board(board_print: &[[u8; 5]]) {
    for row in board_print.iter() {
        for &cell in row.iter() {
            rprint!("{}", cell);
        }
        rprintln!();
    }
    rprintln!();
}

// Generate random board
fn initialize_board(life_board: &mut [[u8; 5]; 5], seed: u128) -> [[u8; 5]; 5] {
    let mut rng = nanorand::Pcg64::new_seed(seed);
    for r in 0..5 {
        for c in 0..5 {
            let b: bool = rng.generate();
            life_board[r][c] = b as u8;
        }
    }
    *life_board
}

// Complement the game board
fn complement(life_board: &mut [[u8; 5]], complement_board: &mut [[u8; 5]]) {
    for r in 0..5 {
        for c in 0..5 {
            if life_board[r][c] == 0 {
                complement_board[r][c] = 1;
            } else {
                complement_board[r][c] = 0
            }
        }
    }
}

// Function to handle button A press
fn press_a(
    life_board: &mut [[u8; 5]; 5],
    display: &mut Display,
    timer: &mut Timer<TIMER0>,
    seeds: &mut RangeFrom<u128>,
) {
    let seed = seeds.next().unwrap();
    rprintln!("New seed: {}", seed);
    *life_board = initialize_board(life_board, seed);
    display_board(timer, life_board, display);
    timer.delay_ms(100u16);
    rprintln!("Pressed 'A', board continues");
}

// Function for B 
fn press_b(
    life_board: &mut [[u8; 5]; 5],
    display: &mut Display,
    timer: &mut Timer<TIMER0>,
) {
    let mut complement_board = [[0; 5]; 5];
    complement(life_board, &mut complement_board);
    rprintln!("Pressed 'B', board continues");
    print_board(&complement_board);
    display_board(timer, &complement_board, display);
    timer.delay_ms(500u16);
}



