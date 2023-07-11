//go:build tools
// +build tools

// Package tools records tool dependencies. It cannot actually be compiled.
package tools

import (
	_ "github.com/kisielk/errcheck"
	_ "golang.org/x/vuln/cmd/govulncheck"
)
