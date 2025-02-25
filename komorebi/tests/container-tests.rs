use komorebi::container::Container;
use komorebi::window::Window;


#[test]
fn test_add_window_to_container() {
    // Creates an empty container
    let mut container = Container::default();

    // Mock Window
    let test_window = Window { hwnd: 12345 };
    container.add_window(test_window);

    // Ensure the window was added
    assert_eq!(container.windows().len(), 1);

    // Ensure the window is focused
    assert_eq!(container.focused_window_idx(), 0);
}

#[test]
fn test_remove_window_by_idx() {
    // Creates an empty container
    let mut container = Container::default();

    // Mock Windows
    let window1 = Window { hwnd: 12345 };
    let window2 = Window { hwnd: 67890 };

    container.add_window(window1);
    container.add_window(window2);

    // Ensure the window was added
    assert_eq!(container.windows().len(), 2);

    let removed_window = container.remove_window_by_idx(0);

    // Ensure the window was removed
    assert!(removed_window.is_some());
    assert_eq!(removed_window.unwrap().hwnd, 12345);
    assert_eq!(container.windows().len(), 1);
}

#[test]
fn test_remove_focused_window() {
    let mut container = Container::default();
    let window1 = Window { hwnd: 12345 };
    let window2 = Window { hwnd: 67890 };

    container.add_window(window1);
    container.add_window(window2);

    container.focus_window(1);
    let removed_window = container.remove_focused_window();

    assert!(removed_window.is_some());
    assert_eq!(removed_window.unwrap().hwnd, 67890);
    assert_eq!(container.windows().len(), 1);
    assert_eq!(container.focused_window_idx(), 0);
}
