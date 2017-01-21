use std::rc::Rc;


pub struct EventHandler<V, S> {
    state: Rc<Option<S>>,
    reducer: fn(V, &mut S)
}

impl<V, S> EventHandler<V, S> {
    pub fn new(reducer: fn(V, &mut S)) -> Self {
        EventHandler {
            state: Rc::new(None),
            reducer: reducer
        }
    }
    fn call(&self, value: V) {
        let state = Rc::get_mut(&mut self.state).unwrap();
        (self.reducer)(value, &mut state.unwrap())

    }
}
