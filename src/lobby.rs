pub trait Game {
    type Setup: GameSetup;
    type Api: GameApi;
}

pub trait GameSetup {
    fn min_players(&self) -> usize;
    fn max_players(&self) -> usize;
}
pub trait GameApi {}

pub struct PlayerInfo {
    pub name: String,
}

pub trait GameEntry<G: Game> {
    type Lobby: Lobby<G>;

    fn is_password_protected() -> bool;
    fn connect(&mut self, info: PlayerInfo, pwd: Option<String>) -> GameToken;
    fn lobby<'a>(&'a mut self, token: GameToken) -> &'a Self::Lobby;
    fn game<'a>(&'a mut self, token: GameToken) -> &'a G::Api;
}

pub enum GameError {
    Unknown,
}

pub struct GameToken(String);

pub trait Lobby<G: Game> {
    fn set_setup(&mut self, setup: G::Setup);
    fn toggle_ready(&mut self);
    fn start(&mut self);
    fn state(&self) -> LobbyState<G>;
}

pub struct LobbyState<G: Game> {
    pub players: Vec<PlayerInfo>,
    pub ready: Vec<bool>,
    pub admin: usize,
    pub started: bool,
    pub setup: G::Setup,
}

pub trait MasterStateGame : Game {
    type MasterState;
    fn create(setup: Self::Setup) -> Self::MasterState;
    fn scoped_to<'a>(state: &'a mut Self::MasterState) -> &'a mut Self::Api;
}
