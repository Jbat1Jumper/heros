
trait DeltaState: Clone {
    type Delta: Clone;

    fn apply(&mut self, d: Self::Delta);
}
