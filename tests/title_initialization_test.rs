mod tests {
    use dewitrusty::ui::initializer::Initializer;
    use dewitrusty::ui::slint_exports::AppWindow;

    #[test]
    fn test_title_correct() {
        i_slint_backend_testing::init_no_event_loop();
        let app = AppWindow::new().unwrap();
        Initializer::default().run(&app);
        assert_eq!(app.get_window_title(), "Dew-It");
    }
}