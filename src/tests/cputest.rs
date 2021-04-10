mod cputest {

    use crate::cpu::cpu::Cpu;
    use crate::mem::debugram::DebugRam;

    // Tests from https://skilldrick.github.io/easy6502/
    
    #[test]
    pub fn test_prg1() {
        /*
        LDA #$01
        STA $0200
        LDA #$05
        STA $0201
        LDA #$08
        STA $0202
         */
        let prg = vec![
            0xa9, 0x01, 0x8d, 0x00, 0x02, 0xa9, 0x05, 
            0x8d, 0x01, 0x02, 0xa9, 0x08, 0x8d, 0x02, 0x02
        ];
        let prgsize = prg.len() as u16;
        let mut cpu = Cpu::new(Box::new(DebugRam::new(prg)));
        while cpu.pc < prgsize {
            cpu.clock();
        }
        assert_eq!(cpu.bus.read(0x0200), 0x01);
        assert_eq!(cpu.bus.read(0x0201), 0x05);
        assert_eq!(cpu.bus.read(0x0202), 0x08);
        assert_eq!(cpu.bus.read(0x0203), 0x00);
    }

    #[test]
    pub fn test_prg2() {
        /*
        LDA #$c0  ;Load the hex value $c0 into the A register
        TAX       ;Transfer the value in the A register to X
        INX       ;Increment the value in the X register
        ADC #$c4  ;Add the hex value $c4 to the A register
         */
        let prg = vec![0xa9, 0xc0, 0xaa, 0xe8, 0x69, 0xc4];
        let prgsize = prg.len() as u16;
        let mut cpu = Cpu::new(Box::new(DebugRam::new(prg)));
        while cpu.pc < prgsize {
            cpu.clock();
        }
        assert_eq!(cpu.ac, 0x84);
    }

    #[test]
    pub fn test_prg3() {
        /*
        LDX #$08
        decrement:
        DEX
        STX $0200
        CPX #$03
        BNE decrement
        STX $0201
         */
        let prg = vec![
            0xa2, 0x08, 0xca, 0x8e, 0x00, 0x02, 0xe0, 
            0x03, 0xd0, 0xf8, 0x8e, 0x01, 0x02
        ];
        let prgsize = prg.len() as u16;
        let mut cpu = Cpu::new(Box::new(DebugRam::new(prg)));
        while cpu.pc < prgsize {
            cpu.clock();
        }
        assert_eq!(cpu.x, 0x03);
        assert_eq!(cpu.read(0x0200), 0x03);
        assert_eq!(cpu.read(0x0201), 0x03);
    }

    #[test]
    pub fn test_prg4() {
        /*
        LDA #$01
        CMP #$02
        BNE notequal
        STA $22
        notequal:
        NOP
         */
        let prg = vec![0xa9, 0x01, 0xc9, 0x02, 0xd0, 0x02, 0x85, 0x22, 0xea];
        let prgsize = prg.len() as u16;
        let mut cpu = Cpu::new(Box::new(DebugRam::new(prg)));
        while cpu.pc < prgsize {
            cpu.clock();
        }
        assert_eq!(cpu.ac, 0x01);
    }
}