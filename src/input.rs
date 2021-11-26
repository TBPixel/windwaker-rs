use std::io;

use bit_field::BitField;
use dolphin_memory::Dolphin;

const INPUT_RANGE: usize = 0x803A4E20;

/**
 *
803A4DF0,4 - Control stick horizontal axis. Float from -1 to 1.
803A4DF4,4 - Control stick vertical axis. Float from -1 to 1.
803A4E00,4 - C-stick horizontal axis. Float from -1 to 1.
803A4E04,4 - C-stick vertical axis. Float from -1 to 1.
803A4E20,1 - Bitfield of whether certain buttons are currently being pressed down.
  01 - A button is down
  02 - L button is down
  04 - R button is down
  08 - Z button is down
  10 - D-pad up arrow button is down
  20 - D-pad down arrow button is down
  40 - D-pad right arrow button is down
  80 - D-pad left arrow button is down
803A4E21,1 - Bitfield of whether certain buttons are currently being pressed down.
  01 - ??? button is down
  02 - ??? button is down
  04 - ??? button is down
  08 - ??? button is down
  10 - Start button is down
  20 - Y button is down
  40 - X button is down
  80 - B button is down



803A4E22,1 - Bitfield of whether certain buttons were just pressed this frame.
  01 - A button was just pressed this frame
  02 - L button was just pressed this frame
  04 - R button was just pressed this frame
  08 - Z button was just pressed this frame
  10 - D-pad up arrow button was just pressed this frame
  20 - D-pad down arrow button was just pressed this frame
  40 - D-pad right arrow button was just pressed this frame
  80 - D-pad left arrow button was just pressed this frame



803A4E23,1 - Bitfield of whether certain buttons were just pressed this frame.
  01 - ??? button was just pressed this frame
  02 - ??? button was just pressed this frame
  04 - ??? button was just pressed this frame
  08 - ??? button was just pressed this frame
  10 - Start button was just pressed this frame
  20 - Y button was just pressed this frame
  40 - X button was just pressed this frame
  80 - B button was just pressed this frame
 */

#[derive(Default, Debug, Clone, Copy)]
pub struct Inputs {
    pub dpad_left_just_pressed: bool,
}

impl Inputs {
    pub fn read(&mut self, d: &Dolphin) -> io::Result<Self> {
        let inputs = d.read_u32(INPUT_RANGE, None)?;
        // let pressed = d.read_u8(JustPressed1, None)?;
        // let pressed = InputBitfieldJustPressed1(pressed);

        self.dpad_left_just_pressed = inputs.get_bit(31);

        Ok(*self)
        // Ok(inputs.get_bits(0..32))
        // self.bitfield = inputs.get_bits(0..32);
        // self.dpad_left_just_pressed = inputs.get_bit(32);
        // println!("{:#034b}", inputs.get_bits(0..32));
        // self.dpad_left_just_pressed = pressed.get_dpad_left() == 1;
        // Ok(*self)
    }
}
