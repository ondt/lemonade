use cairo_sys;
use cairo;
use xcb;

use cairo::XCBSurface;

pub trait Dock {
    fn create_surface(&self) -> cairo::Surface;
    fn dock(&self);
    fn set_size(&mut self, u16, u16);
    //fn set_pos(&self, i16, i16);
    //fn show();
    //fn set_offset();
}

pub struct XCB {
    pub conn: xcb::Connection,
    win: u32,
    screen_num: i32,

    size: (u16, u16), // (w, h)
    pos: (i16, i16), // (x, y)
}

impl XCB {
    pub fn new() -> XCB {

        // Create XCB struct to return
        let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
        let win = conn.generate_id();
        let size: (u16, u16) = (1, 1); // minimum size
        let pos: (i16, i16) = (0, 0);

        let x = XCB {
            conn,
            win,
            screen_num,
            size,
            pos,
        };

        // Use the previous struct and create the window
        let values = [
            (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_KEY_PRESS | xcb::EVENT_MASK_EXPOSURE),
        ];

        xcb::create_window(&x.conn,
                           xcb::COPY_FROM_PARENT as u8,
                           x.win,
                           x.get_screen().root(),
                           x.pos.0, x.pos.1,
                           x.size.0, x.size.1,
                           0,
                           xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
                           x.get_screen().root_visual(),
                           &values);

        xcb::map_window(&x.conn, x.win);

        return x;
    }

    // TODO handle Result<>
    fn get_atom(&self, name: &str) -> xcb::Atom {
        let atom = xcb::intern_atom(&self.conn, false, name);

        let reply = atom.get_reply().unwrap().atom();
        return reply;
    }

    // TODO somehow store this value in the struct
    // generates lifetime errors for now
    fn get_screen(&self) -> xcb::Screen {
        let setup = self.conn.get_setup();
        let screen = setup.roots().nth(self.screen_num as usize).unwrap();

        return screen;
    }

    fn get_visual(&self) -> xcb::Visualtype {
        for root in self.conn.get_setup().roots() {
            for depth in root.allowed_depths() {
                for v in depth.visuals() {
                    if v.visual_id() == self.get_screen().root_visual() {
                        return v;
                    }
                }
            }
        }
        panic!("Failed to find visual type");
    }
}

impl Dock for XCB {
    fn create_surface(&self) -> cairo::Surface {

        // Prepare cairo variables
        let cr_conn = unsafe {
            cairo::XCBConnection::from_raw_none(
                self.conn.get_raw_conn() as *mut cairo_sys::xcb_connection_t)
        };

        let cr_draw = cairo::XCBDrawable(self.win);

        let cr_visual = unsafe {
            cairo::XCBVisualType::from_raw_none(
                &mut self.get_visual().base as *mut xcb::ffi::xcb_visualtype_t
                                            as *mut cairo_sys::xcb_visualtype_t)
        };

        // Create the surface using previous variables
        return cairo::Surface::create(
            &cr_conn, &cr_draw, &cr_visual,
            self.size.0 as i32, self.size.1 as i32);
    }

    fn dock(&self) {
        let data = [
            self.get_atom("_NET_WM_WINDOW_TYPE_DOCK"),
        ];

        xcb::change_property(&self.conn,
                             xcb::PROP_MODE_REPLACE as u8,
                             self.win,
                             self.get_atom("_NET_WM_WINDOW_TYPE"),
                             xcb::ATOM_ATOM,
                             32,
                             &data);
    }

    fn set_size(&mut self, w: u16, h: u16) {
        xcb::configure_window(&self.conn, self.win, &[
                (xcb::CONFIG_WINDOW_WIDTH as u16, w as u32),
                (xcb::CONFIG_WINDOW_HEIGHT as u16, h as u32),
        ]);

        self.size = (w, h);
    }
}