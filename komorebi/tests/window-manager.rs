
#[cfg(test)]
mod tests {
    use komorebi::WindowManagerEvent;
    use komorebi::{window_manager::WindowManager, DATA_DIR};
    use crossbeam_channel::Sender;
    use crossbeam_channel::Receiver;
    use crossbeam_channel::bounded;


    #[test]
    fn test_create_window_manager() {
        let (_sender, receiver): (Sender<WindowManagerEvent>, Receiver<WindowManagerEvent>) =  bounded(1);
        let socket = Some(DATA_DIR.join("komorebi-test.sock"));
        let wm = WindowManager::new(receiver, socket);
        assert!(wm.is_ok());
    }
}
