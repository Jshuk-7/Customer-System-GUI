extern crate native_windows_gui as nwg;

#[path = "./new_ticket_window.rs"]
mod new_ticket_window;
use nwg::NativeUi;
use new_ticket_window::NewTicketWindow;
use new_ticket_window::new_ticket_window::NewTicketWindowUI;

#[derive(Default)]
pub struct MainWindow {
    window: nwg::Window,
    new_order_button: nwg::Button,
    pickup_button: nwg::Button,
    quick_ticket_button: nwg::Button,
    rack_button: nwg::Button,
    ticket_operations_button: nwg::Button,
    delivery_route_button: nwg::Button,
    search_button: nwg::Button,
    reports_button: nwg::Button,
    manager_button: nwg::Button,
}

impl MainWindow {
    fn new_order(&self) -> NewTicketWindowUI {
        let ui = NewTicketWindow::build_ui(
            Default::default()
        ).expect("Failed");
        nwg::dispatch_thread_events();
        ui
    }

    fn pickup_order(&self) {
        println!("Pickup clicked");
    }

    fn quick_ticket(&self) {
        println!("Quick ticket clicked");
    }

    fn rack(&self) {
        println!("Rack clicked");
    }

    fn ticket_opetations(&self) {
        println!("Ticket operations clicked");
    }

    fn delivery_route(&self) {
        println!("Delivery route clicked");
    }

    fn search(&self) {
        println!("Delivery route clicked");
    }

    fn reports(&self) {
        println!("Reports clicked");
    }

    fn manager(&self) {
        println!("Manager clicked");
    }

    fn exit_app(&self) {
        nwg::stop_thread_dispatch();
    }
}

mod main_window {
    use native_windows_gui as nwg;
    use super::*;
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::ops::Deref;

    pub struct MainWindowUi {
        inner: Rc<MainWindow>,
        default_handler: RefCell<Option<nwg::EventHandler>>,
    }

    impl nwg::NativeUi<MainWindowUi> for MainWindow {
        fn build_ui(mut data: Self) -> Result<MainWindowUi, nwg::NwgError> {
            use nwg::Event as E;
            
            nwg::Window::builder()
                .flags(nwg::WindowFlags::MAIN_WINDOW | nwg::WindowFlags::VISIBLE)
                .maximized(true)
                .title("Customer Gui")
                .build(&mut data.window)?;

            nwg::Button::builder()
                .size((300, 80))
                .position((10, 40))
                .text("New Order")
                .parent(&data.window)
                .build(&mut data.new_order_button)?;

            nwg::Button::builder()
                .size((300, 80))
                .position((310, 40))
                .text("Pickup Ticket")
                .parent(&data.window)
                .build(&mut data.pickup_button)?;

            nwg::Button::builder()
                .size((300, 80))
                .position((610, 40))
                .text("Quick Ticket")
                .parent(&data.window)
                .build(&mut data.quick_ticket_button)?;

            nwg::Button::builder()
                .size((300, 80))
                .position((10, 140))
                .text("Rack Number")
                .parent(&data.window)
                .build(&mut data.rack_button)?;

            nwg::Button::builder()
                .size((300, 80))
                .position((310, 140))
                .text("Ticket Operations")
                .parent(&data.window)
                .build(&mut data.ticket_operations_button)?;

            nwg::Button::builder()
                .size((300, 80))
                .position((610, 140))
                .text("Delivery Route")
                .parent(&data.window)
                .build(&mut data.delivery_route_button)?;

            nwg::Button::builder()
                .size((300, 80))
                .position((10, 240))
                .text("Search")
                .parent(&data.window)
                .build(&mut data.search_button)?;

            nwg::Button::builder()
                .size((300, 80))
                .position((310, 240))
                .text("Reports")
                .parent(&data.window)
                .build(&mut data.reports_button)?;

            nwg::Button::builder()
                .size((300, 80))
                .position((610, 240))
                .text("Manager")
                .parent(&data.window)
                .build(&mut data.manager_button)?;

            let ui = MainWindowUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };

            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnButtonClick => {
                            if &handle == &ui.new_order_button { MainWindow::new_order(&ui); }
                            else if &handle == &ui.pickup_button { MainWindow::pickup_order(&ui); }
                            else if &handle == &ui.quick_ticket_button { MainWindow::quick_ticket(&ui); }
                            else if &handle == &ui.rack_button { MainWindow::rack(&ui); }
                            else if &handle == &ui.ticket_operations_button { MainWindow::ticket_opetations(&ui); }
                            else if &handle == &ui.delivery_route_button { MainWindow::delivery_route(&ui); }
                            else if &handle == &ui.search_button { MainWindow::search(&ui); }
                            else if &handle == &ui.reports_button { MainWindow::reports(&ui); }
                            else if &handle == &ui.manager_button { MainWindow::manager(&ui); }
                        },

                        E::OnWindowClose => if &handle == &ui.window { MainWindow::exit_app(&ui); },

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

    impl Drop for MainWindowUi {
        fn drop(&mut self) {
            let handler = self.default_handler.borrow();
            if handler.is_some() {
                nwg::unbind_event_handler(handler.as_ref().unwrap());
            }
        }
    }

    impl Deref for MainWindowUi {
        type Target = MainWindow;
        fn deref(&self) -> &MainWindow {
            &self.inner
        }
    }
}