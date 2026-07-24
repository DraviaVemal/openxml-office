module openxml_office_go_test

go 1.22.5

require draviavemal_openxml_office v0.0.0

require github.com/google/flatbuffers v24.12.23+incompatible // indirect

// Consume the in-repo Go wrapper directly instead of a published module.
replace draviavemal_openxml_office => ../../go
