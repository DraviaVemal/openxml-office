package main

import (
	"fmt"
	"io"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
	"strings"
	"time"
)

const (
	repository = "DraviaVemal/openxml-office"
	libDir     = "lib"
)

func main() {
	version := gitVersion()

	compiler := ""
	if runtime.GOOS == "windows" {
		compiler = detectWindowsCompiler()
	}

	assetName, err := libAssetNameForPlatform(runtime.GOOS, runtime.GOARCH, compiler)
	if err != nil {
		fail(err)
	}

	outputName, err := libFileNameForPlatform(runtime.GOOS, runtime.GOARCH, compiler)
	if err != nil {
		fail(err)
	}

	if err := os.MkdirAll(libDir, 0o755); err != nil {
		fail(err)
	}

	baseURL := fmt.Sprintf("https://github.com/%s/releases/download/%s", repository, version)

	if err := download(baseURL+"/"+assetName, filepath.Join(libDir, outputName)); err != nil {
		fail(fmt.Errorf("download static lib: %w", err))
	}
	if err := download(baseURL+"/headers.h", filepath.Join(libDir, "headers.h")); err != nil {
		fail(fmt.Errorf("download header: %w", err))
	}

	fmt.Printf("fetchlib: fetched FFI %s for %s/%s\n", version, runtime.GOOS, runtime.GOARCH)
}

func gitVersion() string {
	output, err := exec.Command("git", "describe", "--tags", "--match", "v*", "--abbrev=0").Output()
	if err != nil {
		return "0.0.0"
	}
	version := strings.TrimSpace(string(output))
	if version == "" {
		return "0.0.0"
	}
	return version
}

func libAssetName() (string, error) {
	return libAssetNameForPlatform(runtime.GOOS, runtime.GOARCH, "")
}

func libFileNameForPlatform(goos, goarch, compiler string) (string, error) {
	platform := goos + "-" + goarch
	switch platform {
	case "linux-amd64":
		return "libdraviavemal_openxml_office_ffi-linux-amd64.a", nil
	case "windows-amd64":
		if isMSVCCompiler(compiler) {
			return "draviavemal_openxml_office_ffi.lib", nil
		}
		return "libdraviavemal_openxml_office_ffi.a", nil
	case "darwin-amd64":
		return "libdraviavemal_openxml_office_ffi-darwin-amd64.a", nil
	case "darwin-arm64":
		return "libdraviavemal_openxml_office_ffi-darwin-arm64.a", nil
	default:
		return "", fmt.Errorf("unsupported platform: %s", platform)
	}
}

func libAssetNameForPlatform(goos, goarch, compiler string) (string, error) {
	platform := goos + "-" + goarch
	switch platform {
	case "linux-amd64":
		return "libdraviavemal_openxml_office_ffi-linux-amd64.a", nil
	case "windows-amd64":
		if compiler == "" {
			compiler = detectWindowsCompiler()
		}
		if isMSVCCompiler(compiler) {
			return "draviavemal_openxml_office_ffi-windows-amd64.lib", nil
		}
		return "libdraviavemal_openxml_office_ffi-windows-amd64.a", nil
	case "darwin-amd64":
		return "libdraviavemal_openxml_office_ffi-darwin-amd64.a", nil
	case "darwin-arm64":
		return "libdraviavemal_openxml_office_ffi-darwin-arm64.a", nil
	default:
		return "", fmt.Errorf("unsupported platform: %s", platform)
	}
}

func detectWindowsCompiler() string {
	for _, candidate := range []string{"cl.exe", "cl", "x86_64-w64-mingw32-gcc", "gcc"} {
		if _, err := exec.LookPath(candidate); err == nil {
			return candidate
		}
	}
	return ""
}

func isMSVCCompiler(compiler string) bool {
	if compiler == "" {
		return false
	}
	compiler = strings.ToLower(compiler)
	return strings.Contains(compiler, "cl") || strings.Contains(compiler, "cl.exe") || strings.Contains(compiler, "msvc")
}

func download(url, destination string) error {
	client := &http.Client{Timeout: 5 * time.Minute}
	response, err := client.Get(url)
	if err != nil {
		return err
	}
	defer response.Body.Close()
	if response.StatusCode != http.StatusOK {
		return fmt.Errorf("GET %s: %s", url, response.Status)
	}

	tempPath := destination + ".tmp"
	file, err := os.Create(tempPath)
	if err != nil {
		return err
	}
	if _, err := io.Copy(file, response.Body); err != nil {
		file.Close()
		os.Remove(tempPath)
		return err
	}
	if err := file.Close(); err != nil {
		os.Remove(tempPath)
		return err
	}
	return os.Rename(tempPath, destination)
}

func fail(err error) {
	fmt.Fprintln(os.Stderr, "fetchlib:", err)
	os.Exit(1)
}
