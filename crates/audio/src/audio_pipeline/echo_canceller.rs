/// Echo cancellation only matters for live calls, which this build does not include.
#[derive(Clone, Default)]
pub struct EchoCanceller;

impl EchoCanceller {
    pub fn process_reverse_stream(&mut self, _buf: &mut [i16]) {}

    pub fn process_stream(&mut self, _buf: &mut [i16]) -> anyhow::Result<()> {
        Ok(())
    }
}
