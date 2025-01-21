module sapphire

go 1.23.4

replace (
	github.com/rylenko/sapphire/pkg/shield => ./pkg/shield
	github.com/rylenko/sapphire/pkg/shieldprovider => ./pkg/shieldprovider
)

require (
	github.com/rylenko/sapphire/pkg/shield v0.0.0-00010101000000-000000000000
	github.com/rylenko/sapphire/pkg/shieldprovider v0.0.0-00010101000000-000000000000
)
