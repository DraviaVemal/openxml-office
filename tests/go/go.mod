module openxml_office_go_test

go 1.25.0

require (
	draviavemal_openxml_office v0.0.0
	github.com/xuri/excelize/v2 v2.11.0
)

require (
	github.com/google/flatbuffers v24.12.23+incompatible // indirect
	github.com/richardlehane/mscfb v1.0.7 // indirect
	github.com/richardlehane/msoleps v1.0.6 // indirect
	github.com/tiendc/go-deepcopy v1.7.2 // indirect
	github.com/xuri/efp v0.0.1 // indirect
	github.com/xuri/nfp v0.0.2-0.20250530014748-2ddeb826f9a9 // indirect
	golang.org/x/crypto v0.53.0 // indirect
	golang.org/x/net v0.56.0 // indirect
	golang.org/x/text v0.38.0 // indirect
)

// Consume the in-repo Go wrapper directly instead of a published module.
replace draviavemal_openxml_office => ../../go
