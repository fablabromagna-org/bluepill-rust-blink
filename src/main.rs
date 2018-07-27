
// BluePill Rust Blink !
//
// Esempio minimale di programma RUST 
// scritto per funzionare su BluePill (STM32F103)
// maurizio.conti@fablabromagna.org - Luglio 2018
//
// Per ora non lampeggia neanche... accende solo il led sulla board.

// RUST con questo comando si libera di tutto quello che
// serve per funzionare su un normale sistema operativo
// e si prepara a lavorare sul bare metal (STM32) 
#![no_std]

// ecco la lib che mappa i registri della cpu STM32F103
extern crate stm32f1;

// In RUST serve sempre una panic_abort.
// E' una routine dove il sistema atterra in caso di problemi gravi
// A noi per ora non interessa di definirla con precisione,
// quindi la dichiariamo esterna
extern crate panic_abort;

// In RUST ciò che si definisce si deve poi anche usare.
// Per questo motivo commentiamo le altre define...
// Noi lavoriamo a 10MHz.
//const MODE_INPUT: u8 = 0b00;
const MODE_OUTPUT_10MHZ: u8 = 0b01;
//const MODE_OUTPUT_2MHz: u8 = 0b10;
//const MODE_OUTPUT_50MHz: u8 = 0b11;

const CNF_OUTPUT_PUSHPULL: u8 = 0b00;
//const CNF_INPUT_FLOATING: u8 = 0b01;

fn main() {

    // unwrap() è il modo che RUST ha per evitare null pointer
    // Piuttosto che usare un puntatore nullo, RUST chiama panic_abort!! 
    let peripherals = stm32f1::stm32f100::Peripherals::take().unwrap();
    let rcc = peripherals.RCC;
    let port_c = peripherals.GPIOC;

    // Accende il clock sulla GPIOC
    rcc.apb2enr.write(|w| w.iopcen().bit(true));
    
    // Configura il DDR
    port_c.crh.write(|w| unsafe {
        w
            .mode13().bits(MODE_OUTPUT_10MHZ)
            .cnf13().bits(CNF_OUTPUT_PUSHPULL)
    });

    // Accende i bit
    port_c.bsrr.write(|w|
        w
            .bs13().clear_bit()
    );
}