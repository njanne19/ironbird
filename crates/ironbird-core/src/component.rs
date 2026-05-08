

// Definition of what it means to be
// a basic simulation component. A
// simulation component is an 
// object that has a list of channels 
// that it subscribes to and publishes,
// and has a step function that accepts a 
// timestamp.
pub trait SimulationComponent {

    /// Describes the channels that this comopnent is 
    /// interested in publishing.
    fn get_publishers(&self) -> ChannelSet {

    }

    /// Describes the channels that this component is interested
    /// in subscribing to.
    fn get_subscriptions(&self) -> Vec<ChannelDescription> {

    }

}
