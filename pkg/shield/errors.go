package shield

import "errors"

var (
	ErrForwardChain = errors.New("forward chain")
	ErrInvalidValue = errors.New("invalid value")
	ErrProvider     = errors.New("provider")
)
