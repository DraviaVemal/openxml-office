package main

import "testing"

func TestLibAssetNameForPlatform(t *testing.T) {
	cases := []struct {
		name     string
		goos     string
		goarch   string
		compiler string
		want     string
		wantErr  bool
	}{
		{"linux-amd64", "linux", "amd64", "", "libdraviavemal_openxml_office_ffi-linux-amd64.a", false},
		{"darwin-amd64", "darwin", "amd64", "", "libdraviavemal_openxml_office_ffi-darwin-amd64.a", false},
		{"darwin-arm64", "darwin", "arm64", "", "libdraviavemal_openxml_office_ffi-darwin-arm64.a", false},
		{"windows-amd64-msvc", "windows", "amd64", "cl.exe", "draviavemal_openxml_office_ffi-windows-amd64.lib", false},
		{"windows-amd64-gcc", "windows", "amd64", "x86_64-w64-mingw32-gcc", "libdraviavemal_openxml_office_ffi-windows-amd64.a", false},
		{"windows-amd64-default", "windows", "amd64", "", "libdraviavemal_openxml_office_ffi-windows-amd64.a", false},
		{"unsupported", "plan9", "amd64", "", "", true},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got, err := libAssetNameForPlatform(tc.goos, tc.goarch, tc.compiler)
			if (err != nil) != tc.wantErr {
				t.Fatalf("libAssetNameForPlatform(%q, %q, %q) error = %v, wantErr %v", tc.goos, tc.goarch, tc.compiler, err, tc.wantErr)
			}
			if got != tc.want {
				t.Fatalf("libAssetNameForPlatform(%q, %q, %q) = %q, want %q", tc.goos, tc.goarch, tc.compiler, got, tc.want)
			}
		})
	}
}

func TestLibFileNameForPlatform(t *testing.T) {
	cases := []struct {
		name     string
		goos     string
		goarch   string
		compiler string
		want     string
		wantErr  bool
	}{
		{"linux-amd64", "linux", "amd64", "", "libdraviavemal_openxml_office_ffi-linux-amd64.a", false},
		{"windows-amd64-msvc", "windows", "amd64", "cl.exe", "draviavemal_openxml_office_ffi.lib", false},
		{"windows-amd64-gcc", "windows", "amd64", "x86_64-w64-mingw32-gcc", "libdraviavemal_openxml_office_ffi.a", false},
		{"darwin-amd64", "darwin", "amd64", "", "libdraviavemal_openxml_office_ffi-darwin-amd64.a", false},
		{"unsupported", "plan9", "amd64", "", "", true},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got, err := libFileNameForPlatform(tc.goos, tc.goarch, tc.compiler)
			if (err != nil) != tc.wantErr {
				t.Fatalf("libFileNameForPlatform(%q, %q, %q) error = %v, wantErr %v", tc.goos, tc.goarch, tc.compiler, err, tc.wantErr)
			}
			if got != tc.want {
				t.Fatalf("libFileNameForPlatform(%q, %q, %q) = %q, want %q", tc.goos, tc.goarch, tc.compiler, got, tc.want)
			}
		})
	}
}

func TestIsMSVCCompiler(t *testing.T) {
	cases := []struct {
		compiler string
		want     bool
	}{
		{"cl.exe", true},
		{"CL", true},
		{"msvc", true},
		{"x86_64-w64-mingw32-gcc", false},
		{"gcc", false},
		{"", false},
	}

	for _, tc := range cases {
		got := isMSVCCompiler(tc.compiler)
		if got != tc.want {
			t.Fatalf("isMSVCCompiler(%q) = %v, want %v", tc.compiler, got, tc.want)
		}
	}
}
