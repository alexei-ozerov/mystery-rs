pub struct PlayerActions<'f> {
    pub describe: Vec<&'f str>,
    pub examine: Vec<&'f str>,
    pub talk: Vec<&'f str>,
    pub walk: Vec<&'f str>,
}
