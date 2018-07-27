# bluepill-rust-blink
Progetto iniziale per accedere al led GPIO13 presente sulla scheda Blue Pill.

Gira in linguaggio RUST su STM32F103 in ambiente Visual Studio code.

Link iniziali da cui sono partito:
- https://medium.com/@ly.lee/coding-the-stm32-blue-pill-with-rust-and-visual-studio-code-b21615d8a20
- http://nercury.github.io/rust/embedded/experiments/2018/04/29/rust-embedded-01-discovery-vl-flipping-bits.html

Nella cartella .vscode ci sono due file
- task.json (definizione di attività di compilazione RUST per vscode)
- launch.json (definizione di run e debug)  

Ci sono alcuni file importanti ai fini dell'uso con i microcontrollori con RUST:
- memory.x (contiene la mappa della memoria del device, gli indirizzi dello heap e dello stack)
- loader.gdb (contiene la configurazione del gdb. Ma attenzione, alcune cose vanno date comunque a linea di comando quindi guardare anche al task.json in .vscode per capire bene cosa fare)
- cargo.toml (è il file di progetto per RUST (nome del progetto autore etc))
- .cargo\config (file di configurazione dell'ambiente di compilazione)
