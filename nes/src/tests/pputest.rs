mod pputest {

    //use crate::ppu::*;

    /* 
    #[test]
    pub fn test_control_reg() {
        
        let mut control = regcontrol::RegControl { reg: 0x00 };
        
        control.set_nmi_enabled(true);
        assert_eq!(control.reg, 0b10000000);
        control.set_nmi_enabled(false);
        assert_eq!(control.reg, 0b00000000);

        control.set_name_x(true);
        assert_eq!(control.reg, 0b00000001);
        control.set_name_y(true);
        assert_eq!(control.reg, 0b00000011);
        control.set_name_x(false);
        assert_eq!(control.reg, 0b00000010);
        control.set_name_y(false);
        assert_eq!(control.reg, 0b00000000);

        control.set_bg(true);
        assert_eq!(control.reg, 0b00010000);
        control.set_bg(false);
        assert_eq!(control.reg, 0b00000000);
    }
    */

    /*
    #[test]
    pub fn test_loopy_reg() {
        let mut loopy = regloopy::RegLoopy{ reg: 0x00 };

        loopy.set_coarse_x(0b10110);
        assert_eq!(loopy.reg, 0b00000000_00010110);
        assert_eq!(loopy.get_coarse_x(), 0b10110);

        loopy.set_coarse_y(0b01010);
        assert_eq!(loopy.reg, 0b00000001_01010110);
        assert_eq!(loopy.get_coarse_y(), 0b01010);

        loopy.set_nametable_x(true);
        assert_eq!(loopy.get_nametable_x(), true);
        assert_eq!(loopy.get_nametable_y(), false);
        loopy.set_nametable_y(true);
        assert_eq!(loopy.get_nametable_y(), true);
        assert_eq!(loopy.reg, 0b00001101_01010110);
        loopy.set_nametable_x(false);
        assert_eq!(loopy.reg, 0b00001001_01010110);

        loopy.set_fine_y(0b011);
        assert_eq!(loopy.reg, 0b00111001_01010110);

        loopy.set_fine_y(0b101);
        assert_eq!(loopy.reg, 0b01011001_01010110);
    }
    */
}