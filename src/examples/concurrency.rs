pub struct ConcExample;

impl ConcExample {
    pub fn run_example() {
        ConcExample::create_thread();
        crate::core::utils::Utils::print_line_separator();
    }

    fn create_thread() {
        std::thread::spawn(|| ConcExample::work_for_new_thread());
        ConcExample::work_for_current_thread();

        //Eplain why without join, the process can end and the
        //new thread prints won't be seen in the monitor.
    }

    fn work_for_new_thread() {
        let sleep_time_micro: u64 = 2;
        let iteration: u64 = 7;
        for i in 0..iteration {
            println!(
                "New thread: number is: {i}, let's sleep \
                 for a {sleep_time_micro} micros"
            );
            std::thread::sleep(std::time::Duration::from_micros(
                sleep_time_micro,
            ))
        }
    }

    fn work_for_current_thread() {
        let sleep_time_micro: u64 = 2000;
        let iteration: u64 = 5;
        for i in 0..iteration {
            println!(
                "Old thread: number is: {i}, let's sleep \
                for a {sleep_time_micro} micros"
            );

            std::thread::sleep(std::time::Duration::from_micros(
                sleep_time_micro,
            ))
        }
    }
}