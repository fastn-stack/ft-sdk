// TODO: remove this when https://github.com/diesel-rs/diesel/issues/4302 is resolved
pub(crate) struct NoInstrumentation;

impl diesel::connection::Instrumentation for NoInstrumentation {
    fn on_connection_event(&mut self, _: diesel::connection::InstrumentationEvent<'_>) {}
}
