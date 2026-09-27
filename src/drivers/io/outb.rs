
// author: whekkees (Daniil)

use core::arch::asm;


pub unsafe fn outb(port : u16, value : u8 ) { 

    asm!(
        "out al, dx",
        in("dx") port,
        in("al") value,
        options(nostack, preserves_flags),
    );
}

/*
сначала мы записываем 1 байт в портовый регистр чтобы передать состояние портового регистра 
потом мы читаем порт из port и кладем его в портовый регистр dx
дальше мы читаем состояние из value и кладем его в регистр al что значит что состояние будет читаться по 1 байту

 */