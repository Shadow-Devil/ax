#[macro_export]
macro_rules! ax_test {
    [$test_name:ident; $($bytes:expr),*; $asserts:expr] => {
        // Call the other macro with empty setup code
        ax_test!($test_name; $($bytes),*; |_: &mut Axecutor| {}; $asserts);
    };
    [$test_name:ident; $($bytes:expr),*; $setup:expr; $asserts:expr] => {
		#[test]
		fn $test_name () {
            use rand::Rng;

			let bytes = &[$($bytes),*];

            // Always use a random rip
            let random_rip = rand::thread_rng().gen::<u64>() & 0x0000_ffff_ffff_ffff;

            let mut ax = Axecutor::new(bytes, random_rip).unwrap();

            $setup(&mut ax);

            ax.execute().unwrap();

            $asserts(ax);
		}
    };
}

// TODO: Flags
#[macro_export]
macro_rules! assert_reg_value {
    [b; $axecutor:expr; $reg:expr; $value:expr] => {
        let wrap = RegisterWrapper::from($reg);
        assert!($reg.is_gpr8());
        let val = $axecutor.reg_read_8(wrap);
        assert_eq!(
            &val, &$value,
            "expected register {:?} to have value {:?}, but got {}",
            $reg, $value, &val
        );
    };
    [w; $axecutor:expr; $reg:expr; $value:expr] => {
        let wrap = RegisterWrapper::from($reg);
        assert!($reg.is_gpr16());
        let val = $axecutor.reg_read_16(wrap);
        assert_eq!(
            &val, &$value,
            "expected register {:?} to have value {:?}, but got {}",
            $reg, $value, &val
        );
    };
    [d; $axecutor:expr; $reg:expr; $value:expr] => {
        let wrap = RegisterWrapper::from($reg);
        assert!($reg.is_gpr32());
        let val = $axecutor.reg_read_32(wrap);
        assert_eq!(
            &val, &$value,
            "expected register {:?} to have value {:?}, but got {}",
            $reg, $value, &val
        );
    };
    [q; $axecutor:expr; $reg:expr; $value:expr] => {
        let wrap = RegisterWrapper::from($reg);
        assert!($reg.is_gpr64());
        let val = $axecutor.reg_read_64(wrap);
        assert_eq!(
            &val, &$value,
            "expected register {:?} to have value {:?}, but got {}",
            $reg, $value, &val
        );
    };
}

#[macro_export]
macro_rules! write_reg_value {
    (b; $axecutor:expr; $reg:expr; $value:expr) => {
        let wrap = RegisterWrapper::from($reg);
        assert!($reg.is_gpr8());
        $axecutor.reg_write_8(wrap, $value);
    };
    (w; $axecutor:expr; $reg:expr; $value:expr) => {
        let wrap = RegisterWrapper::from($reg);
        assert!($reg.is_gpr16());
        $axecutor.reg_write_16(wrap, $value);
    };
    (d; $axecutor:expr; $reg:expr; $value:expr) => {
        let wrap = RegisterWrapper::from($reg);
        assert!($reg.is_gpr32());
        $axecutor.reg_write_32(wrap, $value);
    };
    (q; $axecutor:expr; $reg:expr; $value:expr) => {
        let wrap = RegisterWrapper::from($reg);
        assert!($reg.is_gpr64());
        $axecutor.reg_write_64(wrap, $value);
    };
}
