use std::{collections::HashMap, fmt::Debug, hash::Hash};


pub type Action = Box<dyn Fn()>;


pub struct Transition<S: Copy> {
    next_state: S,
    action: Option<Action>
}


impl<S: Copy> Transition<S> {
    pub fn create(next_state: S, action: Option<Action>) -> Self
    {
        Self{ next_state, action }
    }
}


pub trait FSM<S: Copy, E: Copy> {
    /// Initializes the state machine with an initial state and transition map.
    fn initialize(initial: S,
                  transitions: HashMap<(S, E), Transition<S>>) -> Self;

    
    /// Triggers an event, causing the state machine to transition
    /// if a valid transition exists.
    fn trigger(&mut self, event: E) -> Result<(), String>;

    
    /// Returns the current state of the state machine.
    fn state(&self) -> S;
}


pub struct StateMachine<S: Copy, E: Copy> {
    state: S,
    transitions: HashMap<(S, E), Transition<S>>
}


impl<S, E> FSM<S, E> for StateMachine<S, E>
where S: Copy + Hash + Eq + Debug, E: Copy + Hash + Eq + Debug
{
    fn initialize(
        initial: S,
        transitions: HashMap<(S, E), Transition<S>>
    ) -> Self
    {
        Self{ state: initial, transitions }
    }


    fn trigger(&mut self, event: E) -> Result<(), String>
    {
        let key = (self.state, event);
        
        if let Some(transition) = self.transitions.get(&key) {
            if let Some(action) = &transition.action {
                action();
            }
            self.state = transition.next_state;
            return Ok(());
        }
        
        Err(format!(
            "No transition found for event '{:?}' from state '{:?}'",
            event, self.state
        ))
    }


    fn state(&self) -> S
    {
        self.state
    }
}


#[cfg(test)]
mod test {
    use super::*;


    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum TrafficLightState {
        Red,
        Yellow,
        Green
    }


    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum TrafficLightEvent {
        RedTimeout,
        Yellow2GreenTimeout,
        Yellow2RedTimeout,
        GreenTimeout
    }


    type State = TrafficLightState;
    type Event = TrafficLightEvent;


    struct TrafficLight {
        fsm: StateMachine<TrafficLightState, TrafficLightEvent>
    }


    impl TrafficLight {
        fn init_fsm(
            initial: State,
            transitions: HashMap<(State, Event), Transition<State>>
        ) -> Self
        {
            Self{ fsm: StateMachine::initialize(initial, transitions) }
        }
    }


    fn create_traffic_light() -> TrafficLight
    {
        let initial = State::Red;
        let mut transitions: HashMap<(State, Event), Transition<State>> =
            HashMap::new();

        transitions.insert(
            (State::Red, Event::RedTimeout),
            Transition::create(
                State::Yellow,
                Some(Box::new(|| action("Red -> Yellow")))
            )
        );

        transitions.insert(
            (State::Yellow, Event::Yellow2GreenTimeout),
            Transition::create(
                State::Green,
                Some(Box::new(|| action("Yellow -> Green")))
            )
        );

        transitions.insert(
            (State::Green, Event::GreenTimeout),
            Transition::create(
                State::Yellow,
                Some(Box::new(|| action("Green -> Yellow")))
            )
        );

        transitions.insert(
            (State::Yellow, Event::Yellow2RedTimeout),
            Transition::create(
                State::Red,
                Some(Box::new(|| action("Yellow -> Red")))
            )
        );

        TrafficLight::init_fsm(initial, transitions)
    }


    fn action(msg: &str)
    {
        println!("{msg}");
    }


    #[test]
    fn test_initialize()
    {
        let tl = create_traffic_light();
        assert_eq!(tl.fsm.state(), State::Red);
    }


    #[test]
    fn test_trigger()
    {
        let mut tl = create_traffic_light();

        assert!(tl.fsm.trigger(Event::RedTimeout).is_ok());
        assert_eq!(tl.fsm.state(), State::Yellow);
        
        assert!(tl.fsm.trigger(Event::Yellow2GreenTimeout).is_ok());
        assert_eq!(tl.fsm.state(), State::Green);

        assert!(tl.fsm.trigger(Event::GreenTimeout).is_ok());
        assert_eq!(tl.fsm.state(), State::Yellow);

        assert!(tl.fsm.trigger(Event::Yellow2RedTimeout).is_ok());
        assert_eq!(tl.fsm.state(), State::Red);
    }


    #[test]
    fn test_incorrect_trigger()
    {
        let mut tl = create_traffic_light();

        assert_eq!(tl.fsm.state(), State::Red);
        assert!(!tl.fsm.trigger(Event::GreenTimeout).is_ok());
    }
}
