#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub(crate) enum Message {
	CatppuccinFrappeTheme,
	CatppuccinLatteTheme,
	CatppuccinMacchiatoTheme,
	CatppuccinMochaTheme,
	DarkTheme,
	DefaultSettings,
	DraculaTheme,
	GruvboxDarkTheme,
	GruvboxLightTheme,
	KanagawaDragonTheme,
	KanagawaLotusTheme,
	KanagawaWaveTheme,
	LightTheme,
	MoonflyTheme,
	NightflyTheme,
	NordTheme,
	Scale(f32),
	SolarizedDarkTheme,
	SolarizedLightTheme,
	TokyoNightLightTheme,
	TokyoNightStormTheme,
	TokyoNightTheme,
	OxocarbonTheme,
	SettingsPage,
	StartPage,
	Exit,
}