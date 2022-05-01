extern crate native_windows_gui as nwg;

use nwg::NativeUi;

#[derive(Default)]
pub struct NewTicketWindow {
    window: nwg::Window,
    select_customer_button: nwg::Button,
    new_customer_button: nwg::Button,
    cancel_button: nwg::Button,
}

impl NewTicketWindow {
    fn select_customer(&self) {

    }

    fn new_customer(&self) {

    }

    fn close_window(&self) {
        nwg::stop_thread_dispatch();
    }
}

pub mod new_ticket_window {
    use native_windows_gui as nwg;
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::ops::Deref;

    pub struct NewTicketWindowUI {
        inner: Rc<NewTicketWindow>,
        default_handler: RefCell<Option<nwg::EventHandler>>,
    }

    impl nwg::NativeUi<NewTicketWindowUI> for NewTicketWindow {
        fn build_ui(mut data: Self) -> Result<NewTicketWindowUI, nwg::NwgError> {
            use nwg::Event as E;

            nwg::Window::builder()
                .flags(nwg::WindowFlags::MAXIMIZE_BOX | nwg::WindowFlags::VISIBLE)
                .maximized(true)
                .title("New Ticket")
                .build(&mut data.window)?;

            nwg::Button::builder()
                .size((250, 60))
                .position((800, 40))
                .text("New Ticket")
                .parent(&data.window)
                .build(&mut data.select_customer_button)?;

            nwg::Button::builder()
                .size((250, 60))
                .position((800, 120))
                .text("New Customer")
                .parent(&data.window)
                .build(&mut data.new_customer_button)?;

            nwg::Button::builder()
                .size((250, 60))
                .position((800, 180))
                .text("Cancel")
                .parent(&data.window)
                .build(&mut data.cancel_button)?;

            let ui = NewTicketWindowUI {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };

            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnButtonClick => {
                            if &handle == &ui.select_customer_button { NewTicketWindow::select_customer(&ui); }
                            else if &handle == &ui.new_customer_button { NewTicketWindow::new_customer(&ui); }
                            else if &handle == &ui.cancel_button { NewTicketWindow::close_window(&ui); }
                        },

                        _ => {},
                    }
                }
            };

            *ui.default_handler.borrow_mut() = Some(
                nwg::full_bind_event_handler(&ui.window.handle, handle_events)
            );

            Ok(ui)
        }
    }

    impl Drop for NewTicketWindowUI {
        fn drop(&mut self) {
            let handler = self.default_handler.borrow();
            if handler.is_some() {
                nwg::unbind_event_handler(handler.as_ref().unwrap());
            }
        }
    }

    impl Deref for NewTicketWindowUI {
        type Target = NewTicketWindow;
        fn deref(&self) -> &NewTicketWindow {
            &self.inner
        }
    }
}