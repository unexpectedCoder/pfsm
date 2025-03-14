use std::{collections::HashMap, fmt::Debug, hash::Hash};


trait State {}


struct FSM<T>
where T: State + Debug + Clone + Hash + Eq + PartialEq
{
    state: T,
    schema: HashMap<(T, T), bool>
}


impl<T> FSM<T>
where T: State + Debug + Clone + Hash + Eq + PartialEq
{
    fn state(&self) -> Option<&T>
    {
        Some(&self.state)
    }

    fn new(initial: &T) -> Self
    {
        FSM { state: initial.clone(), schema: HashMap::new() }
    }

    fn from_schema(initial: &T, schema: &[(T, T)]) -> Self
    {
        let mut s: HashMap<(T, T), bool> = HashMap::new();
        for si in schema {
            s.insert(si.clone(), true);
        }

        Self{ state: initial.clone(), schema: s }
    }

    fn change_state(&mut self, to_state: &T)
    {
        let key = (self.state.clone(), to_state.clone());
        let state = &self.state;
        self.schema.get(&key).expect(
            &format!("unable change `{state:?}` -> `{to_state:?}`")
        );
        
        self.state = to_state.clone();
    }
}


#[cfg(test)]
mod test {
    use super::*;


    #[derive(Debug, Hash, Clone, PartialEq, Eq)]
    enum TrafficLightState {
        RED,
        YELLOW,
        GREEN
    }


    impl State for TrafficLightState {}


    #[test]
    fn test_new()
    {
        let fsm =
            FSM::new(&TrafficLightState::RED);
        assert_eq!(fsm.state().unwrap(), &TrafficLightState::RED);
    }

    #[test]
    fn test_from_schema()
    {
        let schema = get_schema();

        let mut fsm =
            FSM::from_schema(&TrafficLightState::RED, &schema);
        assert_eq!(fsm.state().unwrap(), &TrafficLightState::RED);

        fsm.change_state(&TrafficLightState::YELLOW);
        assert_eq!(fsm.state().unwrap(), &TrafficLightState::YELLOW);

        fsm.change_state(&TrafficLightState::GREEN);
        assert_eq!(fsm.state().unwrap(), &TrafficLightState::GREEN);

        fsm.change_state(&TrafficLightState::YELLOW);
        assert_eq!(fsm.state().unwrap(), &TrafficLightState::YELLOW);

        fsm.change_state(&TrafficLightState::RED);
        assert_eq!(fsm.state().unwrap(), &TrafficLightState::RED);
    }

    #[test]
    #[should_panic]
    fn test_invalid_change_state()
    {
        let schema = get_schema();

        let mut fsm =
            FSM::from_schema(&TrafficLightState::RED, &schema);
        fsm.change_state(&TrafficLightState::GREEN);
    }

    fn get_schema() -> Vec<(TrafficLightState, TrafficLightState)>
    {
        [
            (TrafficLightState::RED, TrafficLightState::YELLOW),
            (TrafficLightState::YELLOW, TrafficLightState::GREEN),
            (TrafficLightState::GREEN, TrafficLightState::YELLOW),
            (TrafficLightState::YELLOW, TrafficLightState::RED)
        ].to_vec()
    }
}
