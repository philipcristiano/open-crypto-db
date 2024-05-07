# open-crypto-db
Database of Cryptocurrencies

## Types

### Currency


Data in `/data/currencies`

`id` - UUID
`name` - User friendly name

Sources defined map the symbol from the source to this currency

API

`/currencies/[ID]`

Returns

`id`, `name`

### Source

Source for identifiers of other types

Data in `/data/sources`

API

`/sources/[SOURCE]/currencies/[SYMBOL]`

Returns

`id`, `name`
