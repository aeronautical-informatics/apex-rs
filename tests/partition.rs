use apex_rs_macros::partition;

mod deps;
use deps::dummy::Dummy;

#[partition(deps::dummy::Dummy)]
mod hello {
    #[sampling_out(msg_size = "10KB")]
    struct Channel1;

    #[sampling_in(refresh_period = "500ms")]
    #[sampling_in(msg_size = "25KB")]
    struct Channel2;

    #[queuing_out(msg_count = 20, msg_size = "12KB", discipline = "FIFO")]
    struct Channel3;

    #[start(pre)]
    fn pre_start() {
        // i.e. init custom logging
    }

    #[start(cold)]
    fn cold_start(ctx: cold::Context) {
        ctx.warm_start();
    }

    #[start(warm)]
    fn warm_start(ctx: warm::Context) {
        ctx.init_aperiodic2().unwrap();
        ctx.init_aperiodic2().unwrap();
        ctx.init_channel1().unwrap();
        ctx.init_channel2().unwrap();
        // Maybe we do not always want to initialize channel3
        // ctx.init_channel3().unwrap();
    }

    #[aperiodic(
        time_capacity = "Infinite",
        stack_size = "10KB",
        base_priority = 1,
        deadline = "Soft"
    )]
    fn aperiodic2(ctx: aperiodic2::Context) {}

    #[periodic(
        period = "10ms",
        time_capacity = "Infinite",
        stack_size = "10KB",
        base_priority = 1,
        deadline = "Hard"
    )]
    fn periodic3(ctx: periodic3::Context) {}
}
